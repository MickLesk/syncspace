//! File service - Full implementation
use crate::{auth::UserInfo, models::FileInfo, AppState, FileChangeEvent};
use anyhow::{anyhow, Result};
use chrono::Utc;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

const DATA_DIR: &str = "./data";

pub async fn list_files(state: &AppState, user: &UserInfo, path: &str) -> Result<Vec<FileInfo>> {
    // SECURITY: Validate path (allow empty string for root)
    let safe_path = if path.is_empty() {
        String::new()
    } else {
        crate::security::validate_file_path(path)
            .map_err(|_| anyhow!("Invalid directory path"))?
    };
    
    let target = Path::new(DATA_DIR).join(&safe_path);
    let mut entries = Vec::new();
    let mut dir = fs::read_dir(&target).await.map_err(|_| anyhow!("Directory not found"))?;
    
    while let Ok(Some(e)) = dir.next_entry().await {
        if let Ok(meta) = e.metadata().await {
            let name = e.file_name().to_string_lossy().to_string();
            let is_dir = meta.is_dir();
            
            // Construct the file path relative to DATA_DIR
            let file_path = if path.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", path.trim_end_matches('/'), name)
            };
            
            // Try to get file info from database first
            #[derive(sqlx::FromRow)]
            struct FileRow {
                id: String,
                name: String,
                size_bytes: i64,
                owner_id: String,
                created_at: String,
                updated_at: String,
            }
            
            let db_result: Option<FileRow> = sqlx::query_as(
                "SELECT id, name, size_bytes, owner_id, created_at, updated_at 
                 FROM files 
                 WHERE path = ? AND is_deleted = 0 
                 LIMIT 1"
            )
            .bind(&file_path)
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten();
            
            // Check if file is marked as deleted in DB (should be hidden)
            let is_deleted: bool = sqlx::query_scalar(
                "SELECT COUNT(*) > 0 FROM files WHERE path = ? AND is_deleted = 1"
            )
            .bind(&file_path)
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or(false);
            
            // Skip files that are marked as deleted
            if is_deleted {
                continue;
            }
            
            // Debug log for troubleshooting
            if db_result.is_none() && !is_dir {
                eprintln!("[list_files] File not in DB: {} (using filesystem size: {} bytes)", file_path, meta.len());
            }
            
            // Use DB info if available, otherwise use filesystem metadata
            if let Some(db_file) = db_result {
                entries.push(FileInfo {
                    id: Uuid::parse_str(&db_file.id).unwrap_or_default(),
                    name: db_file.name,
                    path: file_path,
                    size: db_file.size_bytes,
                    is_directory: is_dir,
                    owner_id: Uuid::parse_str(&db_file.owner_id).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&db_file.created_at)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    modified_at: chrono::DateTime::parse_from_rfc3339(&db_file.updated_at)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    parent_id: None,
                });
            } else {
                // Fallback to filesystem metadata for directories or files not in DB
                entries.push(FileInfo {
                    id: Uuid::new_v4(),
                    name,
                    path: file_path,
                    size: if is_dir { 0 } else { meta.len() as i64 },
                    is_directory: is_dir,
                    owner_id: Uuid::parse_str(&user.id).unwrap_or_default(),
                    created_at: Utc::now(),
                    modified_at: Utc::now(),
                    parent_id: None,
                });
            }
        }
    }
    Ok(entries)
}

pub async fn download_file(state: &AppState, user: &UserInfo, path: &str) -> Result<tokio::fs::File> {
    // SECURITY: Validate file path to prevent directory traversal
    let safe_path = crate::security::validate_file_path(path)
        .map_err(|_| anyhow!("Invalid file path"))?;
    
    let file_path = Path::new(DATA_DIR).join(&safe_path);
    let file = fs::File::open(&file_path).await.map_err(|_| anyhow!("File not found"))?;
    
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO file_history (id, user_id, action, file_path, status, created_at) VALUES (?, ?, 'downloaded', ?, 'success', datetime('now'))")
        .bind(&log_id).bind(&user.id).bind(path).execute(&state.db_pool).await;
    
    Ok(file)
}

pub async fn upload_file(state: &AppState, user: &UserInfo, path: &str, data: Vec<u8>) -> Result<FileInfo> {
    // SECURITY: Validate and sanitize file path
    let safe_path = crate::security::validate_file_path(path)
        .map_err(|_| anyhow!("Invalid file path"))?;
    
    // SECURITY: Validate filename
    let filename = Path::new(&safe_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow!("Invalid filename"))?;
    
    let safe_filename = crate::security::validate_filename(filename)
        .map_err(|_| anyhow!("Invalid filename"))?;
    
    let target = Path::new(DATA_DIR).join(&safe_path);
    if let Some(parent) = target.parent() { fs::create_dir_all(parent).await?; }
    
    let tmp_name = format!("{}.{}.tmp", target.file_name().and_then(|n| n.to_str()).unwrap_or("upload"), Uuid::new_v4());
    let tmp_path = target.with_file_name(tmp_name);
    
    let mut tmp_file = fs::File::create(&tmp_path).await?;
    tmp_file.write_all(&data).await?;
    tmp_file.flush().await?;
    fs::rename(&tmp_path, &target).await?;
    
    // CRITICAL FIX: Create database entry with CORRECT column names!
    let file_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // SAFE: Extract filename, fallback to "upload" if path is invalid
    let filename = target
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("upload")
        .to_string();
    
    let size_bytes = data.len() as i64;
    
    eprintln!("[upload_file] Uploading: {} ({} bytes) to path: {}", filename, size_bytes, path);
    
    sqlx::query(
        "INSERT INTO files (id, name, path, owner_id, size_bytes, storage_path, is_deleted, version, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, 0, 1, ?, ?)"
    )
    .bind(&file_id)
    .bind(&filename)
    .bind(path)
    .bind(&user.id)
    .bind(size_bytes)
    .bind(path) // storage_path = same as path for now
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await?;
    
    eprintln!("[upload_file] DB insert successful for: {}", path);
    
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO file_history (id, file_id, user_id, action, status, created_at) VALUES (?, ?, ?, 'created', 'success', datetime('now'))")
        .bind(&log_id).bind(&file_id).bind(&user.id).execute(&state.db_pool).await;
    
    let _ = state.fs_tx.send(FileChangeEvent::new(path.to_string(), "create".to_string()));
    
    Ok(FileInfo {
        id: Uuid::parse_str(&file_id).unwrap_or_default(), 
        name: filename,
        path: path.to_string(), 
        size: size_bytes, 
        is_directory: false,
        owner_id: Uuid::parse_str(&user.id).unwrap_or_default(), 
        created_at: Utc::now(), 
        modified_at: Utc::now(),
        parent_id: None,
    })
}

pub async fn delete_file(state: &AppState, user: &UserInfo, path: &str) -> Result<()> {
    // SECURITY: Validate file path
    let safe_path = crate::security::validate_file_path(path)
        .map_err(|_| anyhow!("Invalid file path"))?;
    
    // SOFT DELETE: Mark file as deleted in DB instead of actually deleting it
    let now = Utc::now().to_rfc3339();
    
    // Update the file record to mark as deleted
    let result = sqlx::query(
        "UPDATE files SET is_deleted = 1, updated_at = ? WHERE path = ? AND is_deleted = 0"
    )
    .bind(&now)
    .bind(path)
    .execute(&state.db_pool)
    .await?;
    
    // If no file was found in DB, it might be a directory or untracked file
    if result.rows_affected() == 0 {
        // Check if physical file/directory exists
        let file_path = Path::new(DATA_DIR).join(path);
        if !file_path.exists() {
            return Err(anyhow!("Not found"));
        }
        // For directories or files not in DB, we still soft-delete by creating a DB entry
        let file_id = Uuid::new_v4().to_string();
        let meta = fs::metadata(&file_path).await?;
        let filename = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        sqlx::query(
            "INSERT INTO files (id, name, path, owner_id, size_bytes, storage_path, is_deleted, version, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, 1, 1, ?, ?)"
        )
        .bind(&file_id)
        .bind(&filename)
        .bind(path)
        .bind(&user.id)
        .bind(meta.len() as i64)
        .bind(path)
        .bind(&now)
        .bind(&now)
        .execute(&state.db_pool)
        .await?;
    }
    
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO file_history (id, user_id, action, file_path, status, created_at) VALUES (?, ?, 'deleted', ?, 'success', datetime('now'))")
        .bind(&log_id).bind(&user.id).bind(path).execute(&state.db_pool).await;
    
    let _ = state.fs_tx.send(FileChangeEvent::new(path.to_string(), "delete".to_string()));
    Ok(())
}

pub async fn rename_file(state: &AppState, user: &UserInfo, old_path: &str, new_path: &str) -> Result<()> {
    let old = Path::new(DATA_DIR).join(old_path);
    let new = Path::new(DATA_DIR).join(new_path);
    if let Some(parent) = new.parent() { fs::create_dir_all(parent).await?; }
    
    fs::rename(old, new).await?;
    
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO file_history (id, user_id, action, file_path, status, created_at) VALUES (?, ?, 'renamed', ?, 'success', datetime('now'))")
        .bind(&log_id).bind(&user.id).bind(new_path).execute(&state.db_pool).await;
    
    let _ = state.fs_tx.send(FileChangeEvent::new(new_path.to_string(), "rename".to_string()));
    Ok(())
}

pub async fn move_file(state: &AppState, user: &UserInfo, old_path: &str, new_path: &str) -> Result<()> {
    rename_file(state, user, old_path, new_path).await
}

pub async fn copy_file(state: &AppState, user: &UserInfo, source_path: &str, dest_path: &str) -> Result<()> {
    let src = Path::new(DATA_DIR).join(source_path);
    let dst = Path::new(DATA_DIR).join(dest_path);
    if let Some(parent) = dst.parent() { fs::create_dir_all(parent).await?; }
    
    fs::copy(src, dst).await?;
    
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO file_history (id, user_id, action, file_path, status, created_at) VALUES (?, ?, 'copied', ?, 'success', datetime('now'))")
        .bind(&log_id).bind(&user.id).bind(dest_path).execute(&state.db_pool).await;
    
    let _ = state.fs_tx.send(FileChangeEvent::new(dest_path.to_string(), "copy".to_string()));
    Ok(())
}

pub async fn get_recent_files(state: &AppState, user: &UserInfo, limit: i64) -> Result<Vec<FileInfo>> {
    // Query file_history for recent file access by this user
    #[derive(sqlx::FromRow)]
    struct RecentFileRow {
        file_path: String,
        id: Option<String>,
        name: Option<String>,
        size_bytes: Option<i64>,
        owner_id: Option<String>,
        created_at: Option<String>,
        updated_at: Option<String>,
    }
    
    let recent: Vec<RecentFileRow> = sqlx::query_as(
        "SELECT DISTINCT fh.file_path, f.id, f.name, f.size_bytes, f.owner_id, f.created_at, f.updated_at
         FROM file_history fh
         LEFT JOIN files f ON f.path = fh.file_path AND f.is_deleted = 0
         WHERE fh.user_id = ? AND fh.action IN ('downloaded', 'created', 'renamed')
         ORDER BY fh.created_at DESC
         LIMIT ?"
    )
    .bind(&user.id)
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await?;

    let mut files = Vec::new();
    for row in recent {
        // Only include files that still exist in the files table
        if let (Some(id), Some(name), Some(size_bytes), Some(owner_id), Some(created_at), Some(updated_at)) = 
            (row.id, row.name, row.size_bytes, row.owner_id, row.created_at, row.updated_at) {
            files.push(FileInfo {
                id: Uuid::parse_str(&id).unwrap_or_default(),
                name,
                path: row.file_path,
                size: size_bytes,
                is_directory: false,
                owner_id: Uuid::parse_str(&owner_id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                modified_at: chrono::DateTime::parse_from_rfc3339(&updated_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                parent_id: None,
            });
        }
    }
    
    Ok(files)
}
