//! Tags API endpoints

use crate::auth::UserInfo;

use crate::{services, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/{tag_id}", delete(delete_tag))
        .route("/file-tags", post(tag_file))
        .route("/file-tags/{file_tag_id}", delete(untag_file))
}

/// File-scoped tags routes: /files/{path}/tags/*
/// This is the primary interface for frontend
pub fn file_tags_router() -> Router<AppState> {
    Router::new()
        .route(
            "/files/*path/tags",
            get(list_file_tags).post(create_file_tag),
        )
        .route(
            "/files/*path/tags/:tag_id",
            put(update_file_tag).delete(delete_file_tag),
        )
}

async fn list_tags(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let tags = services::tag::list(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        tags.into_iter()
            .map(|t| serde_json::to_value(t).unwrap_or_default())
            .collect(),
    ))
}

async fn create_tag(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateTagRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tag = services::tag::create(&state, &user, &req.name, req.color)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    serde_json::to_value(tag)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_tag(
    State(state): State<AppState>,
    user: UserInfo,
    Path(tag_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    services::tag::delete(&state, &user, &tag_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn tag_file(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    let file_id = req
        .get("file_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let tag_id = req
        .get("tag_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    services::tag::tag_file(&state, &user, file_id, tag_id)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn untag_file(
    State(state): State<AppState>,
    user: UserInfo,
    Path(file_tag_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let parts: Vec<&str> = file_tag_id.split('_').collect();
    if parts.len() != 2 {
        return Err(StatusCode::BAD_REQUEST);
    }
    services::tag::untag_file(&state, &user, parts[0], parts[1])
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}

// ============================================================================
// FILE-SCOPED TAG ENDPOINTS: /files/{path}/tags/*
// ============================================================================

/// GET /files/{path}/tags - List all tags for a file
async fn list_file_tags(
    State(state): State<AppState>,
    Path(path): Path<String>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // Get tags for this specific file path
    let tags = services::tag::list_by_file(&state, &user, &path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        tags.into_iter()
            .map(|t| serde_json::to_value(t).unwrap_or_default())
            .collect(),
    ))
}

/// POST /files/{path}/tags - Create tag for a file
async fn create_file_tag(
    State(state): State<AppState>,
    Path(path): Path<String>,
    user: UserInfo,
    Json(req): Json<CreateTagRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let tag = services::tag::create_for_file(&state, &user, &path, &req.name, req.color)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let json = serde_json::to_value(tag).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(json)))
}

/// PUT /files/{path}/tags/{tag_id} - Update tag
async fn update_file_tag(
    State(state): State<AppState>,
    Path((path, tag_id)): Path<(String, String)>,
    user: UserInfo,
    Json(req): Json<CreateTagRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tag = services::tag::update(&state, &user, &tag_id, &req.name, req.color)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    serde_json::to_value(tag)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// DELETE /files/{path}/tags/{tag_id} - Delete tag
async fn delete_file_tag(
    State(state): State<AppState>,
    Path((path, tag_id)): Path<(String, String)>,
    user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    services::tag::delete(&state, &user, &tag_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}
