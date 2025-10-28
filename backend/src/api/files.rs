//! File operations API endpoints
//! Handles upload, download, delete, rename, move, copy

use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;

use crate::auth::UserInfo;
use crate::models::FileInfo;
use crate::services;
use crate::AppState;

// ==================== REQUEST/RESPONSE TYPES ====================

#[derive(Debug, Deserialize)]
pub struct RenameRequest {
    pub new_path: String,
}

#[derive(Debug, Deserialize)]
pub struct MoveRequest {
    pub new_path: String,
}

#[derive(Debug, Deserialize)]
pub struct CopyRequest {
    pub new_path: String,
}

#[derive(Debug, Serialize)]
pub struct FileListResponse {
    pub files: Vec<FileInfo>,
    pub total: usize,
}

// ==================== ROUTER ====================

pub fn router() -> Router<AppState> {
    Router::new()
        // Upload file (multipart) - specific route first
        .route("/upload-multipart", post(upload_multipart_handler))
        // Get thumbnail - specific route
        .route("/thumbnails/{file_id}", get(get_thumbnail_handler))
        // Recent files
        .route("/files/recent", get(list_recent_files))
        // List files (root)
        .route("/files", get(list_files_root))
        // Download file
        .route("/file/{*path}", get(download_file_handler))
        // Upload file (raw body) - handle both with and without path
        .route("/upload", post(upload_file_to_root))
        .route("/upload/", post(upload_file_to_root))
        .route("/upload/{*path}", post(upload_file_handler))
        // Rename file
        .route("/rename/{*path}", put(rename_file_handler))
        // Move file
        .route("/move/{*path}", put(move_file_handler))
        // Copy file
        .route("/copy/{*path}", post(copy_file_handler))
        // List files in directory / Delete file - combined route with multiple methods
        .route("/files/{*path}", get(list_files_handler).delete(delete_file_handler))
}

// ==================== HANDLERS ====================

/// List files in root directory
async fn list_files_root(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<FileInfo>>, StatusCode> {
    services::list_files(&state, &user, "")
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// List files in a directory
async fn list_files_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(path): Path<String>,
) -> Result<Json<Vec<FileInfo>>, StatusCode> {
    services::list_files(&state, &user, &path)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

/// Download a file
async fn download_file_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let file_handle = services::download_file(&state, &user, &path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let stream = ReaderStream::new(file_handle);
    let body = Body::from_stream(stream);

    Ok(body)
}

/// Upload a file (raw body)
async fn upload_file_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(path): Path<String>,
    body: axum::body::Bytes,
) -> Result<StatusCode, StatusCode> {
    services::upload_file(&state, &user, &path, body.to_vec())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Upload file to root directory (when path is empty)
async fn upload_file_to_root(
    State(state): State<AppState>,
    user: UserInfo,
    body: axum::body::Bytes,
) -> Result<StatusCode, StatusCode> {
    services::upload_file(&state, &user, "", body.to_vec())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// List recent files
async fn list_recent_files(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<FileInfo>>, StatusCode> {
    // TODO: Implement recent files service
    // For now return empty list
    Ok(Json(vec![]))
}

/// Upload a file (multipart form)
async fn upload_multipart_handler(
    State(state): State<AppState>,
    user: UserInfo,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement upload_multipart service function
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Delete a file or directory
async fn delete_file_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(path): Path<String>,
) -> Result<StatusCode, StatusCode> {
    services::delete_file(&state, &user, &path)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}

/// Rename a file
async fn rename_file_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(old_path): Path<String>,
    Json(req): Json<RenameRequest>,
) -> Result<StatusCode, StatusCode> {
    services::rename_file(&state, &user, &old_path, &req.new_path)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Move a file
async fn move_file_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(old_path): Path<String>,
    Json(req): Json<MoveRequest>,
) -> Result<StatusCode, StatusCode> {
    services::move_file(&state, &user, &old_path, &req.new_path)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Copy a file
async fn copy_file_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(source_path): Path<String>,
    Json(req): Json<CopyRequest>,
) -> Result<StatusCode, StatusCode> {
    services::copy_file(&state, &user, &source_path, &req.new_path)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Get file thumbnail
async fn get_thumbnail_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    services::get_thumbnail(&state, &user, &file_id)
        .await
        .map(|bytes| {
            (
                [(axum::http::header::CONTENT_TYPE, "image/jpeg")],
                bytes,
            )
        })
        .map_err(|_| StatusCode::NOT_FOUND)
}
