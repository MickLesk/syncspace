//! File Versioning API Routes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::AppState;
use crate::auth::UserInfo;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileVersion {
    pub id: String,
    pub file_id: String,
    pub version_number: i32,
    pub file_path: String,
    pub file_size: i64,
    pub created_by: String,
    pub created_at: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVersionMetadataRequest {
    pub comment: Option<String>,
}

/// List all versions of a file
async fn list_versions(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let versions = sqlx::query_as::<_, FileVersion>(
        r#"
        SELECT id, file_id, version_number, file_path, file_size,
               created_by, created_at, comment
        FROM file_versions
        WHERE file_id = ?
        ORDER BY version_number DESC
        "#,
    )
    .bind(&file_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(versions))
}

/// Create a new version
async fn create_version(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    user_info: UserInfo,
    Json(req): Json<CreateVersionRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get current max version number
    let max_version: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(version_number), 0) FROM file_versions WHERE file_id = ?"
    )
    .bind(&file_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);
    
    let version_id = Uuid::new_v4().to_string();
    let new_version_number = max_version + 1;
    let now = Utc::now().to_rfc3339();
    
    // TODO: Copy actual file to versioned storage
    
    sqlx::query(
        r#"
        INSERT INTO file_versions 
        (id, file_id, version_number, file_path, file_size, created_by, created_at, comment)
        VALUES (?, ?, ?, '', 0, ?, ?, ?)
        "#,
    )
    .bind(&version_id)
    .bind(&file_id)
    .bind(new_version_number)
    .bind(&user_info.id)
    .bind(&now)
    .bind(&req.comment)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "id": version_id,
        "version_number": new_version_number
    }))))
}

/// Get version info
async fn get_version(
    State(state): State<AppState>,
    Path(version_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let version = sqlx::query_as::<_, FileVersion>(
        r#"
        SELECT id, file_id, version_number, file_path, file_size,
               created_by, created_at, comment
        FROM file_versions
        WHERE id = ?
        "#,
    )
    .bind(&version_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(version))
}

/// Delete a version
async fn delete_version(
    State(state): State<AppState>,
    Path(version_id): Path<String>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM file_versions WHERE id = ? AND created_by = ?"
    )
    .bind(&version_id)
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    
    Ok(StatusCode::NO_CONTENT)
}

/// Restore a version (make it current)
async fn restore_version(
    State(state): State<AppState>,
    Path(version_id): Path<String>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement actual file restoration
    Ok(Json(serde_json::json!({
        "message": "Version restored successfully"
    })))
}

/// Get diff between two versions
async fn get_version_diff(
    State(state): State<AppState>,
    Path((from_version_id, to_version_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement diff logic
    Ok(Json(serde_json::json!({
        "from_version": from_version_id,
        "to_version": to_version_id,
        "diff": "Not implemented"
    })))
}

/// Download a specific version
async fn download_version(
    State(state): State<AppState>,
    Path(version_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implement file download
    Err::<Json<serde_json::Value>, _>(StatusCode::NOT_IMPLEMENTED)
}

/// Get version metadata
async fn get_version_metadata(
    State(state): State<AppState>,
    Path(version_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let version = get_version(State(state), Path(version_id)).await?;
    Ok(version)
}

/// Update version metadata
async fn update_version_metadata(
    State(state): State<AppState>,
    Path(version_id): Path<String>,
    Json(req): Json<UpdateVersionMetadataRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query(
        "UPDATE file_versions SET comment = ? WHERE id = ?"
    )
    .bind(&req.comment)
    .bind(&version_id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({
        "message": "Metadata updated successfully"
    })))
}

/// Build versions router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/versions/file/{file_id}", get(list_versions))
        .route("/versions/file/{file_id}/create", post(create_version))
        .route("/versions/version/{version_id}", get(get_version).delete(delete_version))
        .route("/versions/version/{version_id}/restore", post(restore_version))
        .route("/versions/diff/{from_version_id}/{to_version_id}", get(get_version_diff))
        .route("/versions/version/{version_id}/download", get(download_version))
        .route("/versions/version/{version_id}/metadata", get(get_version_metadata).put(update_version_metadata))
}
