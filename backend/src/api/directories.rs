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
    pub name: String,
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
pub struct DirectoryTreeResponse {
    pub directories: Vec<DirectoryInfo>,
    pub total: usize,
}

// ==================== ROUTER ====================

pub fn router() -> Router<AppState> {
    Router::new()
        // Get directory tree
        .route("/dirs/tree", axum::routing::get(get_directory_tree))
        // Batch move directories/files
        .route("/dirs/batch/move", post(batch_move_handler))
        // Create directory (using query param or request body for path)
        .route("/dirs/create", post(create_dir_handler))
        // SPECIFIC routes MUST come BEFORE catch-all routes!
        // Move directory
        .route("/dirs/{dir_id}/move", put(move_dir_handler))
        // Rename directory
        .route("/dirs/{dir_id}/rename", put(rename_dir_handler))
        // Delete directory (soft delete to trash)
        .route("/dirs/{dir_id}", delete(delete_dir_handler))
        // Create directory with path parameter uses POST /dirs with body now
        // REMOVED catch-all to avoid conflicts - use /dirs/create instead
}

// ==================== HANDLERS ====================

/// Create a new directory
async fn create_dir_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateDirRequest>,
) -> Result<Json<DirectoryInfo>, StatusCode> {
    let path = req.parent_path.unwrap_or_else(|| "/".to_string()) + "/" + &req.name;
    services::directory::create_directory(&state, &user, &path)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Create directory from path parameter (for API compatibility)
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
