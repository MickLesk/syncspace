//! File versioning handlers
//!
//! Implements automatic file versioning and restore.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::Utc;
use uuid::Uuid;

use crate::auth::User;
use crate::AppState;

// ==================== MODELS ====================

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct FileVersion {
    pub id: String,
    pub file_id: String,
    pub version_number: i64,
    pub storage_path: String,
    pub size_bytes: i64,
    pub checksum_sha256: String,
    pub created_at: String,
    pub created_by: String,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileVersionWithMetadata {
    #[serde(flatten)]
    pub version: FileVersion,
    pub file_name: String,
    pub is_current: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub comment: Option<String>,
}

// ==================== HELPERS ====================

/// Create a new version when file is updated
pub async fn create_version(
    pool: &sqlx::SqlitePool,
    file_id: &str,
    storage_path: &str,
    size_bytes: i64,
    checksum: &str,
    user_id: &str,
    comment: Option<String>,
) -> Result<FileVersion, sqlx::Error> {
    // Get next version number
    let last_version: Option<(i64,)> = sqlx::query_as(
        "SELECT MAX(version_number) FROM file_versions WHERE file_id = ?"
    )
    .bind(file_id)
    .fetch_optional(pool)
    .await?;

    let version_number = last_version.map(|v| v.0).unwrap_or(0) + 1;
    let version_id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO file_versions 
        (id, file_id, version_number, storage_path, size_bytes, checksum_sha256, created_at, created_by, comment)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&version_id)
    .bind(file_id)
    .bind(version_number)
    .bind(storage_path)
    .bind(size_bytes)
    .bind(checksum)
    .bind(Utc::now().to_rfc3339())
    .bind(user_id)
    .bind(&comment)
    .execute(pool)
    .await?;

    let version: FileVersion = sqlx::query_as(
        "SELECT * FROM file_versions WHERE id = ?"
    )
    .bind(&version_id)
    .fetch_one(pool)
    .await?;

    Ok(version)
}

// ==================== HANDLERS ====================

/// List all versions of a file
/// GET /api/files/{file_id}/versions
pub async fn list_versions(
    user: User,
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<Json<Vec<FileVersionWithMetadata>>, StatusCode> {
    // Verify file ownership
    let file: Option<(String, String)> = sqlx::query_as(
        "SELECT owner_id, name FROM files WHERE id = ?"
    )
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (_owner_id, file_name) = match file {
        None => return Err(StatusCode::NOT_FOUND),
        Some((owner, name)) => {
            if owner != user.id.to_string() {
                return Err(StatusCode::FORBIDDEN);
            }
            (owner, name)
        }
    };

    // Get current file checksum
    let current_checksum: Option<(String,)> = sqlx::query_as(
        "SELECT checksum_sha256 FROM files WHERE id = ?"
    )
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let current_checksum = current_checksum.map(|c| c.0);

    // Get all versions
    let versions: Vec<FileVersion> = sqlx::query_as(
        r#"
        SELECT * FROM file_versions 
        WHERE file_id = ?
        ORDER BY version_number DESC
        "#
    )
    .bind(&file_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Vec<FileVersionWithMetadata> = versions
        .into_iter()
        .map(|v| {
            let is_current = current_checksum.as_ref() == Some(&v.checksum_sha256);
            FileVersionWithMetadata {
                version: v,
                file_name: file_name.clone(),
                is_current,
            }
        })
        .collect();

    Ok(Json(result))
}

/// Restore a specific version
/// POST /api/files/{file_id}/versions/{version_id}/restore
pub async fn restore_version(
    user: User,
    State(state): State<AppState>,
    Path((file_id, version_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    // Verify file ownership
    let file: Option<(String, String, String)> = sqlx::query_as(
        "SELECT owner_id, storage_path, checksum_sha256 FROM files WHERE id = ?"
    )
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (_owner_id, current_storage_path, current_checksum) = match file {
        None => return Err(StatusCode::NOT_FOUND),
        Some((owner, path, checksum)) => {
            if owner != user.id.to_string() {
                return Err(StatusCode::FORBIDDEN);
            }
            (owner, path, checksum)
        }
    };

    // Get version to restore
    let version: FileVersion = sqlx::query_as(
        "SELECT * FROM file_versions WHERE id = ? AND file_id = ?"
    )
    .bind(&version_id)
    .bind(&file_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    // Create version of current file before restoring
    create_version(
        &state.db_pool,
        &file_id,
        &current_storage_path,
        version.size_bytes,
        &current_checksum,
        &user.id.to_string(),
        Some("Auto-backup before restore".to_string()),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update file to point to old version
    sqlx::query(
        r#"
        UPDATE files 
        SET storage_path = ?,
            size_bytes = ?,
            checksum_sha256 = ?,
            updated_at = ?
        WHERE id = ?
        "#
    )
    .bind(&version.storage_path)
    .bind(version.size_bytes)
    .bind(&version.checksum_sha256)
    .bind(Utc::now().to_rfc3339())
    .bind(&file_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO activity_log (id, user_id, action, resource_type, resource_id, timestamp, metadata)
        VALUES (?, ?, 'version_restored', 'file', ?, ?, ?)
        "#
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user.id.to_string())
    .bind(&file_id)
    .bind(Utc::now().to_rfc3339())
    .bind(format!("{{\"version_id\": \"{}\", \"version_number\": {}}}", version_id, version.version_number))
    .execute(&state.db_pool)
    .await;

    Ok(StatusCode::NO_CONTENT)
}

/// Delete a specific version
/// DELETE /api/files/{file_id}/versions/{version_id}
pub async fn delete_version(
    user: User,
    State(state): State<AppState>,
    Path((file_id, version_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    // Verify file ownership
    let file: Option<(String,)> = sqlx::query_as(
        "SELECT owner_id FROM files WHERE id = ?"
    )
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match file {
        None => return Err(StatusCode::NOT_FOUND),
        Some((owner,)) => {
            if owner != user.id.to_string() {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    // Verify version exists
    let version: Option<FileVersion> = sqlx::query_as(
        "SELECT * FROM file_versions WHERE id = ? AND file_id = ?"
    )
    .bind(&version_id)
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if version.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let version = version.unwrap();

    // Delete version from DB
    sqlx::query("DELETE FROM file_versions WHERE id = ?")
        .bind(&version_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: Delete physical file from storage if not used by other versions/files

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO activity_log (id, user_id, action, resource_type, resource_id, timestamp, metadata)
        VALUES (?, ?, 'version_deleted', 'file', ?, ?, ?)
        "#
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&user.id.to_string())
    .bind(&file_id)
    .bind(Utc::now().to_rfc3339())
    .bind(format!("{{\"version_number\": {}}}", version.version_number))
    .execute(&state.db_pool)
    .await;

    Ok(StatusCode::NO_CONTENT)
}

/// Get version count for a file
/// GET /api/files/{file_id}/versions/count
pub async fn version_count(
    user: User,
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verify file ownership
    let file: Option<(String,)> = sqlx::query_as(
        "SELECT owner_id FROM files WHERE id = ?"
    )
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match file {
        None => return Err(StatusCode::NOT_FOUND),
        Some((owner,)) => {
            if owner != user.id.to_string() {
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM file_versions WHERE file_id = ?"
    )
    .bind(&file_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "file_id": file_id,
        "version_count": count.0,
    })))
}
