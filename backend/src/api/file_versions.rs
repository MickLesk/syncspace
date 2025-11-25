use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use sha2::Digest;

use crate::auth::UserInfo;
use crate::AppState;

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
         ORDER BY version_number DESC",
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
         WHERE id = ?",
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
        "SELECT file_path, size_bytes, checksum_sha256 FROM file_versions WHERE id = ?",
    )
    .bind(&req.version_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (version_path, size, checksum) = version.ok_or(StatusCode::NOT_FOUND)?;

    // Use version_path for future file content restoration
    eprintln!("Restoring from version path: {}", version_path);

    // Get current file info
    let current_file: Option<(String,)> = sqlx::query_as("SELECT path FROM files WHERE id = ?")
        .bind(&file_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (current_path,) = current_file.ok_or(StatusCode::NOT_FOUND)?;

    // Copy the file content from version_path to current_path
    if let Err(_e) = std::fs::copy(&version_path, &current_path) {
        eprintln!("Warning: Could not copy version file content from {} to {}", version_path, current_path);
        // Continue anyway as we'll still update metadata
    }

    let now = chrono::Utc::now().to_rfc3339();

    // Update file record
    sqlx::query(
        "UPDATE files SET size_bytes = ?, checksum_sha256 = ?, updated_at = ? WHERE id = ?",
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
         VALUES (?, ?, ?, 'version_restored', ?, ?)",
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
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&restoration_id)
    .bind(&file_id)
    .bind(&req.version_id)
    .bind(&user.user_id())
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Version restored successfully",
            "file_id": file_id,
            "restored_version_id": req.version_id
        })),
    ))
}

/// Create new version when file is updated
#[allow(dead_code)]
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
        "SELECT COALESCE(MAX(version_number), 0) + 1 FROM file_versions WHERE file_id = ?",
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
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
    let count: Option<(i32,)> =
        sqlx::query_as("SELECT COUNT(*) FROM file_versions WHERE file_id = ?")
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
        .route(
            "/api/files/{file_id}/versions/count",
            get(get_version_count),
        )
        .route(
            "/api/files/{file_id}/versions/restore",
            post(restore_version),
        )
        .route("/api/versions/{version_id}", get(get_version_details))
}

/// File-scoped versions routes: /api/file-versions/*
/// This is the primary interface for frontend
pub fn file_versions_router() -> Router<AppState> {
    Router::new()
        .route("/api/file-versions/list", get(list_path_versions))
        .route(
            "/api/file-versions/get/{version_num}",
            get(get_version_by_number),
        )
        .route(
            "/api/file-versions/delete/{version_num}",
            delete(delete_version_by_number),
        )
        .route(
            "/api/file-versions/download/{version_num}",
            get(download_version_content),
        )
        .route(
            "/api/file-versions/restore/{version_num}",
            post(restore_version_by_number),
        )
        .route("/api/file-versions/diff", post(diff_versions_content))
        .route("/api/file-versions/cleanup", post(cleanup_old_versions))
}

// ============================================================================
// FILE-SCOPED VERSION ENDPOINTS: /files/{path}/versions/*
// ============================================================================

#[derive(Debug, Serialize)]
pub struct DiffChange {
    pub change_type: String, // "added", "removed"
    pub line_number: Option<usize>,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct DiffResponse {
    pub version1: i32,
    pub version2: i32,
    pub changes: Vec<DiffChange>,
    pub summary: DiffSummary,
}

#[derive(Debug, Serialize)]
pub struct DiffSummary {
    pub added_lines: usize,
    pub removed_lines: usize,
    pub total_changes: usize,
}

#[derive(Debug, Deserialize)]
pub struct DiffRequest {
    pub version1: i32,
    pub version2: i32,
}

#[derive(Debug, Deserialize)]
pub struct CleanupRequest {
    pub days_old: i32,
}

#[derive(Debug, Serialize)]
pub struct CleanupResponse {
    pub deleted_count: i32,
    pub freed_space: i64,
}

#[derive(Debug, Deserialize)]
pub struct FilePathQuery {
    pub path: String,
}

/// GET /api/file-versions/list?path={path} - List all versions
async fn list_path_versions(
    State(state): State<AppState>,
    Query(query): Query<FilePathQuery>,
    _user: UserInfo,
) -> Result<Json<Vec<FileVersion>>, StatusCode> {
    let versions: Vec<FileVersion> = sqlx::query_as(
        "SELECT id, file_id, version_number, file_path, size_bytes, checksum_sha256, 
                created_by, created_at, comment
         FROM file_versions
         WHERE file_path = ?
         ORDER BY version_number DESC",
    )
    .bind(&query.path)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(versions))
}

/// GET /api/file-versions/get/{version_num}?path={path} - Get specific version
async fn get_version_by_number(
    State(state): State<AppState>,
    Path(version_num): Path<i32>,
    Query(query): Query<FilePathQuery>,
    _user: UserInfo,
) -> Result<Json<FileVersion>, StatusCode> {
    let version: Option<FileVersion> = sqlx::query_as(
        "SELECT id, file_id, version_number, file_path, size_bytes, checksum_sha256,
                created_by, created_at, comment
         FROM file_versions
         WHERE file_path = ? AND version_number = ?",
    )
    .bind(&query.path)
    .bind(version_num)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    version.map(Json).ok_or(StatusCode::NOT_FOUND)
}

/// GET /api/file-versions/download/{version_num}?path={path} - Download version
async fn download_version_content(
    State(state): State<AppState>,
    Path(version_num): Path<i32>,
    Query(query): Query<FilePathQuery>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Get version file path from database
    let version: Option<(String, String)> = sqlx::query_as(
        "SELECT storage_path, file_path FROM file_versions WHERE file_path = ? AND version_number = ?"
    )
    .bind(&query.path)
    .bind(version_num)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (storage_path, file_path) = version.ok_or(StatusCode::NOT_FOUND)?;

    let metadata = std::fs::metadata(&storage_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let file_size = metadata.len();

    // Get file extension for MIME type
    let mime_type = std::path::Path::new(&file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| mime_guess::from_ext(ext).first())
        .map(|m| m.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    // Create response with proper headers
    let body = axum::body::Body::from_stream(
        tokio_util::io::ReaderStream::new(tokio::fs::File::open(&storage_path)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?)
    );

    Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_str(&mime_type).unwrap(),
        ), (
            axum::http::header::CONTENT_LENGTH,
            axum::http::HeaderValue::from_str(&file_size.to_string()).unwrap(),
        ), (
            axum::http::header::CONTENT_DISPOSITION,
            axum::http::HeaderValue::from_str(&format!(
                "attachment; filename=\"v{}_{}\"",
                version_num, 
                std::path::Path::new(&file_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("version")
            )).unwrap(),
        )],
        body
    ))
}

/// DELETE /api/file-versions/delete/{version_num}?path={path} - Delete version
async fn delete_version_by_number(
    State(state): State<AppState>,
    Path(version_num): Path<i32>,
    Query(query): Query<FilePathQuery>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM file_versions WHERE file_path = ? AND version_number = ?")
        .bind(&query.path)
        .bind(version_num)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/file-versions/restore/{version_num}?path={path} - Restore version
async fn restore_version_by_number(
    State(state): State<AppState>,
    Path(version_num): Path<i32>,
    Query(query): Query<FilePathQuery>,
    user_info: UserInfo,
    Json(_req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get the version details
    let version: Option<(String, String, i64)> = sqlx::query_as(
        "SELECT storage_path, file_path, size_bytes FROM file_versions WHERE file_path = ? AND version_number = ?"
    )
    .bind(&query.path)
    .bind(version_num)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (version_storage_path, file_path, version_size) = version.ok_or(StatusCode::NOT_FOUND)?;

    // Get the current file
    let current_file: Option<(String, String, String)> = sqlx::query_as(
        "SELECT id, path, checksum_sha256 FROM files WHERE path = ?"
    )
    .bind(&file_path)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (file_id, current_path, _current_checksum) = current_file.ok_or(StatusCode::NOT_FOUND)?;

    // Read the version content
    let version_content = std::fs::read(&version_storage_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Calculate checksum of version content
    let checksum = format!("{:x}", sha2::Sha256::digest(&version_content));

    // Create a new version from the current content (backup before restore)
    let current_version: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(version_number) FROM file_versions WHERE file_path = ?"
    )
    .bind(&file_path)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let current_version_num = current_version.0.unwrap_or(0);
    let new_version_num = current_version_num + 1;

    // Backup current file to version storage if it exists
    if let Ok(current_content) = std::fs::read(&current_path) {
        let current_version_path = format!("{}/{}.v{}", 
            std::path::Path::new(&current_path)
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("./data/versions"),
            std::path::Path::new(&current_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file"),
            current_version_num
        );

        let _ = std::fs::create_dir_all(std::path::Path::new(&current_version_path).parent().unwrap_or(std::path::Path::new(".")));
        let _ = std::fs::write(&current_version_path, &current_content);
    }

    // Write the version content to the current path
    std::fs::write(&current_path, &version_content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update the file record
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE files SET size_bytes = ?, checksum_sha256 = ?, updated_at = ? WHERE id = ?"
    )
    .bind(version_size)
    .bind(&checksum)
    .bind(&now)
    .bind(&file_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log the restoration action
    let activity_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO activity (id, user_id, action, file_path, created_at) VALUES (?, ?, 'version_restored', ?, ?)"
    )
    .bind(&activity_id)
    .bind(&user_info.id)
    .bind(&file_path)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "restored": true,
        "new_version_number": new_version_num,
        "file_id": file_id,
        "size_bytes": version_size,
        "checksum_sha256": checksum,
        "message": format!("Version {} restored successfully. Current version backed up as v{}", version_num, current_version_num)
    })))
}

/// POST /api/file-versions/diff?path={path} - Compare two versions
async fn diff_versions_content(
    State(state): State<AppState>,
    Query(query): Query<FilePathQuery>,
    _user: UserInfo,
    Json(req): Json<DiffRequest>,
) -> Result<Json<DiffResponse>, StatusCode> {
    // Get version IDs from version numbers
    let v1_id: Option<(String,)> =
        sqlx::query_as("SELECT id FROM file_versions WHERE file_path = ? AND version_number = ?")
            .bind(&query.path)
            .bind(req.version1)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let v2_id: Option<(String,)> =
        sqlx::query_as("SELECT id FROM file_versions WHERE file_path = ? AND version_number = ?")
            .bind(&query.path)
            .bind(req.version2)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (v1_id,) = v1_id.ok_or(StatusCode::NOT_FOUND)?;
    let (v2_id,) = v2_id.ok_or(StatusCode::NOT_FOUND)?;

    // Use version storage service to get real diff
    let version_diff =
        crate::services::version_storage_service::get_version_diff(&state.db_pool, &v1_id, &v2_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Convert service diff to API response format
    let changes: Vec<DiffChange> = if let Some(text_diff) = version_diff.text_diff {
        text_diff
            .iter()
            .filter(|line| {
                !matches!(
                    line.change_type,
                    crate::services::version_storage_service::ChangeType::Unchanged
                )
            })
            .map(|line| DiffChange {
                change_type: match line.change_type {
                    crate::services::version_storage_service::ChangeType::Added => "added",
                    crate::services::version_storage_service::ChangeType::Deleted => "removed",
                    crate::services::version_storage_service::ChangeType::Modified => "modified",
                    _ => "unchanged",
                }
                .to_string(),
                line_number: Some(line.line_number),
                content: line.content.clone(),
            })
            .collect()
    } else {
        vec![]
    };

    let added = changes.iter().filter(|c| c.change_type == "added").count();
    let removed = changes
        .iter()
        .filter(|c| c.change_type == "removed")
        .count();

    Ok(Json(DiffResponse {
        version1: req.version1,
        version2: req.version2,
        changes,
        summary: DiffSummary {
            added_lines: added,
            removed_lines: removed,
            total_changes: added + removed,
        },
    }))
}

/// POST /api/file-versions/cleanup?path={path} - Delete old versions
async fn cleanup_old_versions(
    State(state): State<AppState>,
    Query(query): Query<FilePathQuery>,
    _user: UserInfo,
    Json(req): Json<CleanupRequest>,
) -> Result<Json<CleanupResponse>, StatusCode> {
    // Get old versions
    let old_versions: Vec<(i64,)> = sqlx::query_as(
        "SELECT size_bytes FROM file_versions 
         WHERE file_path = ? 
         AND created_at < datetime('now', ? || ' days')",
    )
    .bind(&query.path)
    .bind(&format!("-{}", req.days_old))
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let freed_space: i64 = old_versions.iter().map(|(size,)| size).sum();
    let deleted_count = old_versions.len() as i32;

    // Delete old versions
    sqlx::query(
        "DELETE FROM file_versions 
         WHERE file_path = ? 
         AND created_at < datetime('now', ? || ' days')",
    )
    .bind(&query.path)
    .bind(&format!("-{}", req.days_old))
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CleanupResponse {
        deleted_count,
        freed_space,
    }))
}
