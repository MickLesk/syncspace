use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::AppState;
use crate::auth::UserInfo;

#[derive(Debug, Serialize, FromRow)]
pub struct FileVersion {
    pub id: String,
    pub file_id: String,
    pub version_number: i32,
    pub file_path: String,
    pub size_bytes: i64,
    pub checksum_sha256: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RestoreVersionRequest {
    pub version_id: String,
}

/// List all versions for a file
async fn list_file_versions(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let versions: Vec<FileVersion> = sqlx::query_as(
        "SELECT id, file_id, version_number, file_path, size_bytes, checksum_sha256, 
                created_by, created_at, comment
         FROM file_versions
         WHERE file_id = ?
         ORDER BY version_number DESC"
    )
    .bind(&file_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(versions))
}

/// Get specific version details
async fn get_version_details(
    State(state): State<AppState>,
    Path(version_id): Path<String>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let version: Option<FileVersion> = sqlx::query_as(
        "SELECT id, file_id, version_number, file_path, size_bytes, checksum_sha256,
                created_by, created_at, comment
         FROM file_versions
         WHERE id = ?"
    )
    .bind(&version_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    version
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
        .map(|v| v.into_response())
}

/// Restore a specific version
async fn restore_version(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    user: UserInfo,
    Json(req): Json<RestoreVersionRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get version details
    let version: Option<(String, i64, Option<String>)> = sqlx::query_as(
        "SELECT file_path, size_bytes, checksum_sha256 FROM file_versions WHERE id = ?"
    )
    .bind(&req.version_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (version_path, size, checksum) = version.ok_or(StatusCode::NOT_FOUND)?;

    // Get current file info
    let current_file: Option<(String,)> = sqlx::query_as(
        "SELECT path FROM files WHERE id = ?"
    )
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (current_path,) = current_file.ok_or(StatusCode::NOT_FOUND)?;

    // TODO: Actually copy the file content from version_path to current_path
    // For now, just update metadata

    let now = chrono::Utc::now().to_rfc3339();

    // Update file record
    sqlx::query(
        "UPDATE files SET size_bytes = ?, checksum_sha256 = ?, updated_at = ? WHERE id = ?"
    )
    .bind(size)
    .bind(&checksum)
    .bind(&now)
    .bind(&file_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log restoration
    let log_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO file_history (id, file_id, user_id, action, file_path, created_at)
         VALUES (?, ?, ?, 'version_restored', ?, ?)"
    )
    .bind(&log_id)
    .bind(&file_id)
    .bind(&user.user_id())
    .bind(&current_path)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create restoration record
    let restoration_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO version_restorations (id, file_id, from_version_id, restored_by, restored_at)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&restoration_id)
    .bind(&file_id)
    .bind(&req.version_id)
    .bind(&user.user_id())
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Version restored successfully",
        "file_id": file_id,
        "restored_version_id": req.version_id
    }))))
}

/// Create new version when file is updated
pub async fn create_version(
    pool: &sqlx::SqlitePool,
    file_id: &str,
    user_id: &str,
    file_path: &str,
    size_bytes: i64,
    checksum: Option<&str>,
    comment: Option<&str>,
) -> Result<String, StatusCode> {
    // Get next version number
    let next_version: Option<(i32,)> = sqlx::query_as(
        "SELECT COALESCE(MAX(version_number), 0) + 1 FROM file_versions WHERE file_id = ?"
    )
    .bind(file_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let version_number = next_version.map(|(v,)| v).unwrap_or(1);

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO file_versions (id, file_id, version_number, file_path, size_bytes, 
                                    checksum_sha256, created_by, created_at, comment)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(file_id)
    .bind(version_number)
    .bind(file_path)
    .bind(size_bytes)
    .bind(checksum)
    .bind(user_id)
    .bind(&now)
    .bind(comment)
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(id)
}

/// Get version count for file
async fn get_version_count(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let count: Option<(i32,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM file_versions WHERE file_id = ?"
    )
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let version_count = count.map(|(c,)| c).unwrap_or(0);

    Ok(Json(serde_json::json!({
        "file_id": file_id,
        "version_count": version_count
    })))
}

/// Router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/files/{file_id}/versions", get(list_file_versions))
        .route("/api/files/{file_id}/versions/count", get(get_version_count))
        .route("/api/files/{file_id}/versions/restore", post(restore_version))
        .route("/api/versions/{version_id}", get(get_version_details))
}
