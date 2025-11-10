//! File Versioning API Routes with Differential Storage and Compression

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::PathBuf;

use crate::auth::UserInfo;
use crate::services::version_storage_service;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileVersion {
    pub id: String,
    pub file_id: String,
    pub version_number: i32,
    pub storage_path: Option<String>,
    pub original_size: i64,
    pub compressed_size: i64,
    pub is_compressed: bool,
    pub is_differential: bool,
    pub base_version_id: Option<String>,
    pub checksum: String,
    pub created_by: String,
    pub created_at: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub file_path: String,
    pub comment: Option<String>,
}

/// List all versions of a file
async fn list_versions(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let versions = sqlx::query_as::<_, FileVersion>(
        r#"
        SELECT id, file_id, version_number, storage_path, original_size, compressed_size,
               is_compressed, is_differential, base_version_id, checksum, 
               created_by, created_at, comment
        FROM file_versions
        WHERE file_id = ?
        ORDER BY version_number DESC
        "#,
    )
    .bind(&file_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(versions))
}

/// Create a new version with differential storage and compression
async fn create_version(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
    user: UserInfo,
    Json(req): Json<CreateVersionRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let file_path = PathBuf::from(&req.file_path);

    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    match version_storage_service::create_version(
        &state.db_pool,
        &file_id,
        &file_path,
        &user.id,
        req.comment.as_deref(),
    )
    .await
    {
        Ok(version_metadata) => {
            let version = FileVersion {
                id: version_metadata.id,
                file_id: version_metadata.file_id,
                version_number: version_metadata.version_number,
                storage_path: Some(version_metadata.storage_path),
                original_size: version_metadata.original_size,
                compressed_size: version_metadata.compressed_size,
                is_compressed: version_metadata.is_compressed,
                is_differential: version_metadata.is_differential,
                base_version_id: version_metadata.base_version_id,
                checksum: version_metadata.checksum,
                created_by: version_metadata.created_by,
                created_at: version_metadata.created_at,
                comment: version_metadata.comment,
            };

            Ok((StatusCode::CREATED, Json(version)))
        }
        Err(e) => {
            eprintln!("Error creating version: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a version
async fn delete_version(
    State(state): State<AppState>,
    Path((file_id, version_id)): Path<(String, String)>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let version: Option<(String,)> =
        sqlx::query_as("SELECT storage_path FROM file_versions WHERE id = ? AND file_id = ?")
            .bind(&version_id)
            .bind(&file_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((storage_path,)) = version {
        let _ = tokio::fs::remove_file(&storage_path).await;

        let result = sqlx::query("DELETE FROM file_versions WHERE id = ?")
            .bind(&version_id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if result.rows_affected() > 0 {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Restore a version (returns file content)
async fn restore_version(
    State(state): State<AppState>,
    Path((_file_id, version_id)): Path<(String, String)>,
    _user: UserInfo,
) -> Result<Response, StatusCode> {
    match version_storage_service::restore_version(&state.db_pool, &version_id).await {
        Ok(content) => {
            let body = Bytes::from(content);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"restored_v{}.dat\"", version_id),
                )
                .body(body.into())
                .unwrap())
        }
        Err(e) => {
            eprintln!("Error restoring version: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/versions/{file_id}",
            get(list_versions).post(create_version),
        )
        .route("/versions/{file_id}/{version_id}", delete(delete_version))
        .route(
            "/versions/{file_id}/{version_id}/restore",
            post(restore_version),
        )
}
