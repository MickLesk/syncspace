//! File service - Full implementation
use crate::{auth::UserInfo, models::FileInfo, AppState, FileChangeEvent};
use anyhow::{anyhow, Result};
use axum::extract::Multipart;
use bytes::Bytes;
use chrono::Utc;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

const DATA_DIR: &str = "./data";

pub async fn list_files(state: &AppState, user: &UserInfo, path: &str) -> Result<Vec<FileInfo>> {
    if let Ok(Some(cached)) = state.cache_manager.get_directory_listing(path).await {
        return Ok(cached.into_iter().map(|e| FileInfo {
            id: Uuid::new_v4(), 
            name: e.name, 
            path: path.to_string(),
            size: e.size as i64, 
            is_directory: e.is_directory, 
            owner_id: Uuid::parse_str(&user.id).unwrap_or_default(),
            created_at: Utc::now(), 
            modified_at: Utc::now(),
            parent_id: None,
        }).collect());
    }
    
    let target = Path::new(DATA_DIR).join(path);
    let mut entries = Vec::new();
    let mut dir = fs::read_dir(&target).await.map_err(|_| anyhow!("Directory not found"))?;
    
    while let Ok(Some(e)) = dir.next_entry().await {
        if let Ok(meta) = e.metadata().await {
            let name = e.file_name().to_string_lossy().to_string();
            entries.push(FileInfo {
                id: Uuid::new_v4(), 
                name, 
                path: path.to_string(),
                size: meta.len() as i64, 
                is_directory: meta.is_dir(), 
                owner_id: Uuid::parse_str(&user.id).unwrap_or_default(),
                created_at: Utc::now(), 
                modified_at: Utc::now(),
                parent_id: None,
            });
        }
    }
    Ok(entries)
}

pub async fn download_file(state: &AppState, user: &UserInfo, path: &str) -> Result<tokio::fs::File> {
    let file_path = Path::new(DATA_DIR).join(path);
    let file = fs::File::open(&file_path).await.map_err(|_| anyhow!("File not found"))?;
    
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO file_history (id, user_id, action, file_path, status, created_at) VALUES (?, ?, 'downloaded', ?, 'success', datetime('now'))")
        .bind(&log_id).bind(&user.id).bind(path).execute(&state.db_pool).await;
    
    Ok(file)
}

pub async fn upload_file(state: &AppState, user: &UserInfo, path: &str, data: Vec<u8>) -> Result<FileInfo> {
    let target = Path::new(DATA_DIR).join(path);
    if let Some(parent) = target.parent() { fs::create_dir_all(parent).await?; }
    
    let tmp_name = format!("{}.{}.tmp", target.file_name().and_then(|n| n.to_str()).unwrap_or("upload"), Uuid::new_v4());
    let tmp_path = target.with_file_name(tmp_name);
    
    let mut tmp_file = fs::File::create(&tmp_path).await?;
    tmp_file.write_all(&data).await?;
    tmp_file.flush().await?;
    fs::rename(&tmp_path, &target).await?;
    
    let log_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO file_history (id, user_id, action, file_path, status, created_at) VALUES (?, ?, 'created', ?, 'success', datetime('now'))")
        .bind(&log_id).bind(&user.id).bind(path).execute(&state.db_pool).await;
    
    let _ = state.fs_tx.send(FileChangeEvent::new(path.to_string(), "create".to_string()));
    
    Ok(FileInfo {
        id: Uuid::new_v4(), 
        name: target.file_name().unwrap().to_string_lossy().to_string(),
        path: path.to_string(), 
        size: data.len() as i64, 
        is_directory: false,
        owner_id: Uuid::parse_str(&user.id).unwrap_or_default(), 
        created_at: Utc::now(), 
        modified_at: Utc::now(),
        parent_id: None,
    })
}

pub async fn delete_file(state: &AppState, user: &UserInfo, path: &str) -> Result<()> {
    let file_path = Path::new(DATA_DIR).join(path);
    if !file_path.exists() { return Err(anyhow!("Not found")); }
    
    if file_path.is_file() {
        fs::remove_file(&file_path).await?;
    } else {
        fs::remove_dir_all(&file_path).await?;
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
