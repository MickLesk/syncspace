//! Directory operations API endpoints
//! Handles directory creation, moving, deletion with proper hierarchy management

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::auth::UserInfo;
use crate::models::DirectoryInfo;
use crate::services;
use crate::AppState;

// ==================== REQUEST/RESPONSE TYPES ====================

#[derive(Debug, Deserialize)]
pub struct CreateDirRequest {
    // Support both formats:
    // 1. { "path": "folder/subfolder" } - full path from frontend
    // 2. { "name": "folder", "parent_path": "/some/path" } - legacy format
    pub path: Option<String>,
    pub name: Option<String>,
    pub parent_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MoveDirRequest {
    pub new_parent_path: String,
}

#[derive(Debug, Deserialize)]
pub struct RenameDirRequest {
    pub new_name: String,
}

#[derive(Debug, Deserialize)]
pub struct BatchMoveRequest {
    pub paths: Vec<String>,
    pub target_path: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DirectoryTreeResponse {
    pub directories: Vec<DirectoryInfo>,
    pub total: usize,
}

// ==================== ROUTER ====================

pub fn router() -> Router<AppState> {
    Router::new()
        // Get directory tree - SPECIFIC routes FIRST
        .route("/dirs/tree", axum::routing::get(get_directory_tree))
        // Batch move directories/files
        .route("/dirs/batch/move", post(batch_move_handler))
        // Create directory (using path in request body)
        .route("/dirs/create", post(create_dir_handler))
        // Move directory - specific ID patterns
        .route("/dirs/{dir_id}/move", put(move_dir_handler))
        // Rename directory
        .route("/dirs/{dir_id}/rename", put(rename_dir_handler))
        // Delete directory (soft delete to trash)
        .route("/dirs/{dir_id}", delete(delete_dir_handler))
    // NOTE: Cannot use /dirs/{*path} catch-all after specific routes - Axum limitation
    // Frontend must use /dirs/create with path in body instead
}

// ==================== HANDLERS ====================

/// Create a new directory
async fn create_dir_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateDirRequest>,
) -> Result<Json<DirectoryInfo>, StatusCode> {
    // Support both formats:
    // 1. path: "folder/subfolder" (new format from frontend)
    // 2. name + parent_path (legacy format)
    let path = if let Some(p) = req.path {
        p
    } else if let Some(name) = req.name {
        let parent = req.parent_path.unwrap_or_else(|| "/".to_string());
        format!("{}/{}", parent.trim_end_matches('/'), name)
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    services::directory::create_directory(&state, &user, &path)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Create directory from path parameter (for API compatibility)
#[allow(dead_code)]
async fn create_dir_from_path(
    State(state): State<AppState>,
    user: UserInfo,
    Path(path): Path<String>,
) -> Result<StatusCode, StatusCode> {
    services::directory::create_directory(&state, &user, &path)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Move a directory to a new parent
async fn move_dir_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(dir_id): Path<String>,
    Json(req): Json<MoveDirRequest>,
) -> Result<StatusCode, StatusCode> {
    services::directory::move_directory(&state, &user, &dir_id, &req.new_parent_path)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Rename a directory
async fn rename_dir_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(dir_id): Path<String>,
    Json(req): Json<RenameDirRequest>,
) -> Result<StatusCode, StatusCode> {
    services::directory::rename_directory(&state, &user, &dir_id, &req.new_name)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Delete a directory (moves to trash)
async fn delete_dir_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(dir_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    services::directory::delete_directory(&state, &user, &dir_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}

/// Batch move multiple items to a target directory
async fn batch_move_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<BatchMoveRequest>,
) -> Result<StatusCode, StatusCode> {
    services::directory::batch_move(&state, &user, req.paths, &req.target_path)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Get complete directory tree
async fn get_directory_tree(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::directory::get_directory_tree(&state, &user, "")
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
