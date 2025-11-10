//! Sync Service - Synchronize filesystem with database
//!
//! This service ensures that all files in the data directory are properly
//! registered in the database with correct metadata (size, timestamps, etc.)

use chrono::Utc;
use sqlx::SqlitePool;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

const DATA_DIR: &str = "./data";

/// Scan the data directory and sync all files and folders to the database
pub async fn sync_filesystem_to_db(
    pool: &SqlitePool,
    default_owner_id: &str,
) -> anyhow::Result<usize> {
    println!("🔄 Starting filesystem sync to database...");

    let mut synced_count = 0;
    let data_path = Path::new(DATA_DIR);

    if !data_path.exists() {
        println!("⚠️  Data directory does not exist, skipping sync");
        return Ok(0);
    }

    // Recursively scan directory (files and folders)
    synced_count += scan_directory(pool, data_path, "", None, default_owner_id).await?;

    println!("✅ Filesystem sync complete: {} files synced", synced_count);
    Ok(synced_count)
}

/// Recursively scan a directory and sync files and folders
fn scan_directory<'a>(
    pool: &'a SqlitePool,
    dir_path: &'a Path,
    relative_path: &'a str,
    parent_folder_id: Option<&'a str>,
    owner_id: &'a str,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<usize>> + 'a>> {
    Box::pin(async move {
        let mut count = 0;
        let mut entries = fs::read_dir(dir_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            // Skip database files, search index, and versions folder
            if file_name == "syncspace.db"
                || file_name == "syncspace.db-shm"
                || file_name == "syncspace.db-wal"
                || file_name == "search_index"
                || file_name == "versions"
            {
                continue;
            }

            // Construct relative path from data directory
            let file_relative_path = if relative_path.is_empty() {
                file_name.clone()
            } else {
                format!("{}/{}", relative_path, file_name)
            };

            if metadata.is_dir() {
                // Insert folder into database if not exists
                let folder_id = Uuid::new_v4().to_string();
                let now = Utc::now().to_rfc3339();

                let folder_exists: bool = sqlx::query_scalar(
                    "SELECT COUNT(*) > 0 FROM folders WHERE path = ? AND is_deleted = 0",
                )
                .bind(&file_relative_path)
                .fetch_one(pool)
                .await?;

                let current_folder_id = if !folder_exists {
                    sqlx::query(
                    "INSERT INTO folders (id, name, path, parent_id, owner_id, is_deleted, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, 0, ?, ?)"
                )
                .bind(&folder_id)
                .bind(&file_name)
                .bind(&file_relative_path)
                .bind(parent_folder_id)
                .bind(owner_id)
                .bind(&now)
                .bind(&now)
                .execute(pool)
                .await?;

                    println!("  📁 Synced folder: {}", file_relative_path);
                    folder_id
                } else {
                    // Get existing folder ID
                    sqlx::query_scalar::<_, String>(
                        "SELECT id FROM folders WHERE path = ? AND is_deleted = 0",
                    )
                    .bind(&file_relative_path)
                    .fetch_one(pool)
                    .await?
                };

                // Recursively process subdirectories with current folder as parent
                count += scan_directory(
                    pool,
                    &path,
                    &file_relative_path,
                    Some(&current_folder_id),
                    owner_id,
                )
                .await?;
            } else {
                // Check if file already exists in database
                let exists: bool = sqlx::query_scalar(
                    "SELECT COUNT(*) > 0 FROM files WHERE path = ? AND is_deleted = 0",
                )
                .bind(&file_relative_path)
                .fetch_one(pool)
                .await?;

                if !exists {
                    // Insert file into database
                    let file_id = Uuid::new_v4().to_string();
                    let size_bytes = metadata.len() as i64;
                    let now = Utc::now().to_rfc3339();

                    sqlx::query(
                    "INSERT INTO files (id, name, path, owner_id, size_bytes, storage_path, is_deleted, version, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, ?, 0, 1, ?, ?)"
                )
                .bind(&file_id)
                .bind(&file_name)
                .bind(&file_relative_path)
                .bind(owner_id)
                .bind(size_bytes)
                .bind(&file_relative_path)
                .bind(&now)
                .bind(&now)
                .execute(pool)
                .await?;

                    count += 1;
                    println!("  📄 Synced: {} ({} bytes)", file_relative_path, size_bytes);
                } else {
                    println!("  ⏭️  Skipped (already in DB): {}", file_relative_path);
                }
            }
        }

        Ok(count)
    })
}
