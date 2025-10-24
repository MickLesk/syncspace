//! Backup system handlers
//!
//! Create, list, restore, and manage database/file backups.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::Utc;
use uuid::Uuid;
use std::path::PathBuf;

use crate::auth::User;
use crate::AppState;

const BACKUP_DIR: &str = "./data/backups";

// ==================== MODELS ====================

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Backup {
    pub id: String,
    pub backup_type: String, // "full", "incremental", "database", "files"
    pub size_bytes: i64,
    pub file_count: Option<i64>,
    pub storage_path: String,
    pub created_by: String,
    pub created_at: String,
    pub status: String, // "completed", "in_progress", "failed"
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub backup_type: String, // "full", "database", "files"
    pub include_versions: Option<bool>,
}

// ==================== HANDLERS ====================

/// Create a new backup
/// POST /api/backups/create
pub async fn create_backup(
    user: User,
    State(state): State<AppState>,
    Json(req): Json<CreateBackupRequest>,
) -> Result<Json<Backup>, StatusCode> {
    // Validate backup type
    if !matches!(req.backup_type.as_str(), "full" | "database" | "files") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let backup_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_filename = format!("backup_{}_{}.tar.gz", req.backup_type, timestamp);
    let backup_path = PathBuf::from(BACKUP_DIR).join(&backup_filename);

    // Ensure backup directory exists
    if let Err(e) = tokio::fs::create_dir_all(BACKUP_DIR).await {
        eprintln!("Failed to create backup directory: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Create backup record with "in_progress" status
    sqlx::query(
        r#"
        INSERT INTO backups 
        (id, backup_type, size_bytes, file_count, storage_path, created_by, created_at, status)
        VALUES (?, ?, 0, 0, ?, ?, ?, 'in_progress')
        "#
    )
    .bind(&backup_id)
    .bind(&req.backup_type)
    .bind(backup_path.to_str().unwrap())
    .bind(&user.id.to_string())
    .bind(Utc::now().to_rfc3339())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Spawn background task to create backup
    let pool = state.db_pool.clone();
    let backup_id_clone = backup_id.clone();
    let backup_type = req.backup_type.clone();
    let include_versions = req.include_versions.unwrap_or(false);

    tokio::spawn(async move {
        let result = perform_backup(&backup_type, &backup_path, include_versions).await;

        match result {
            Ok((size, file_count)) => {
                // Update backup record to "completed"
                let _ = sqlx::query(
                    r#"
                    UPDATE backups 
                    SET status = 'completed', size_bytes = ?, file_count = ?
                    WHERE id = ?
                    "#
                )
                .bind(size)
                .bind(file_count)
                .bind(&backup_id_clone)
                .execute(&pool)
                .await;
            }
            Err(e) => {
                // Update backup record to "failed"
                let _ = sqlx::query(
                    r#"
                    UPDATE backups 
                    SET status = 'failed', error_message = ?
                    WHERE id = ?
                    "#
                )
                .bind(format!("{}", e))
                .bind(&backup_id_clone)
                .execute(&pool)
                .await;
            }
        }
    });

    // Return backup record
    let backup: Backup = sqlx::query_as("SELECT * FROM backups WHERE id = ?")
        .bind(&backup_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(backup))
}

/// List all backups
/// GET /api/backups
pub async fn list_backups(
    _user: User,
    State(state): State<AppState>,
) -> Result<Json<Vec<Backup>>, StatusCode> {
    let backups: Vec<Backup> = sqlx::query_as(
        "SELECT * FROM backups ORDER BY created_at DESC LIMIT 50"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(backups))
}

/// Get backup details
/// GET /api/backups/{id}
pub async fn get_backup(
    _user: User,
    State(state): State<AppState>,
    Path(backup_id): Path<String>,
) -> Result<Json<Backup>, StatusCode> {
    let backup: Backup = sqlx::query_as("SELECT * FROM backups WHERE id = ?")
        .bind(&backup_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(backup))
}

/// Delete a backup
/// DELETE /api/backups/{id}
pub async fn delete_backup(
    _user: User, // TODO: Check admin role
    State(state): State<AppState>,
    Path(backup_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Get backup info
    let backup: Backup = sqlx::query_as("SELECT * FROM backups WHERE id = ?")
        .bind(&backup_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Delete physical backup file
    let backup_path = PathBuf::from(&backup.storage_path);
    if backup_path.exists() {
        if let Err(e) = tokio::fs::remove_file(&backup_path).await {
            eprintln!("Failed to delete backup file: {}", e);
            // Continue anyway to remove DB record
        }
    }

    // Delete from database
    sqlx::query("DELETE FROM backups WHERE id = ?")
        .bind(&backup_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

/// Restore from backup (placeholder - complex operation)
/// POST /api/backups/{id}/restore
pub async fn restore_backup(
    _user: User, // TODO: Check admin role
    State(state): State<AppState>,
    Path(backup_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Get backup info
    let backup: Backup = sqlx::query_as("SELECT * FROM backups WHERE id = ?")
        .bind(&backup_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if backup.status != "completed" {
        return Err(StatusCode::BAD_REQUEST);
    }

    // TODO: Implement actual restore logic
    // This is a placeholder - restore is complex and dangerous
    // Should:
    // 1. Stop accepting new requests
    // 2. Close all connections
    // 3. Extract backup archive
    // 4. Restore database
    // 5. Restore files
    // 6. Restart server

    Err(StatusCode::NOT_IMPLEMENTED)
}

// ==================== BACKUP LOGIC ====================

/// Perform actual backup (runs in background)
async fn perform_backup(
    backup_type: &str,
    backup_path: &PathBuf,
    include_versions: bool,
) -> Result<(i64, i64), Box<dyn std::error::Error + Send + Sync>> {
    use tokio::process::Command;

    match backup_type {
        "database" => {
            // Backup SQLite database using VACUUM INTO
            let db_path = "./data/syncspace.db";
            let backup_db = backup_path.with_extension("db");
            
            // Copy database file
            tokio::fs::copy(db_path, &backup_db).await?;
            
            // Get file size
            let metadata = tokio::fs::metadata(&backup_db).await?;
            let size = metadata.len() as i64;
            
            Ok((size, 1))
        }
        "files" => {
            // Create tar.gz of data directory (excluding backups)
            let output = Command::new("tar")
                .args(&[
                    "-czf",
                    backup_path.to_str().unwrap(),
                    "--exclude=backups",
                    "--exclude=search_index",
                    if include_versions { "./data" } else { "./data" },
                ])
                .output()
                .await?;

            if !output.status.success() {
                return Err(format!("tar command failed: {:?}", output.stderr).into());
            }

            let metadata = tokio::fs::metadata(backup_path).await?;
            let size = metadata.len() as i64;
            
            // Count files (approximation)
            Ok((size, 0))
        }
        "full" => {
            // Backup both database and files
            let db_path = "./data/syncspace.db";
            let temp_dir = backup_path.parent().unwrap().join("temp_backup");
            tokio::fs::create_dir_all(&temp_dir).await?;
            
            // Copy database
            tokio::fs::copy(db_path, temp_dir.join("syncspace.db")).await?;
            
            // Copy data directory
            let output = Command::new("tar")
                .args(&[
                    "-czf",
                    backup_path.to_str().unwrap(),
                    "-C",
                    temp_dir.to_str().unwrap(),
                    ".",
                ])
                .output()
                .await?;

            // Cleanup temp dir
            let _ = tokio::fs::remove_dir_all(&temp_dir).await;

            if !output.status.success() {
                return Err(format!("tar command failed: {:?}", output.stderr).into());
            }

            let metadata = tokio::fs::metadata(backup_path).await?;
            let size = metadata.len() as i64;
            
            Ok((size, 0))
        }
        _ => Err("Invalid backup type".into()),
    }
}
