//! Tags API endpoints

use crate::auth::UserInfo;

use crate::{services, AppState};
use axum::{
    extract::{Path, Query, State},
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

#[derive(Debug, Deserialize)]
pub struct FilePathQuery {
    pub path: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/{tag_id}", delete(delete_tag))
        .route("/file-tags", post(tag_file))
        .route("/file-tags/{file_tag_id}", delete(untag_file))
}

/// File-scoped tags routes using query parameters
/// Frontend should call: /api/file-tags/list?path={path}
pub fn file_tags_router() -> Router<AppState> {
    Router::new()
        .route("/file-tags/list", get(list_file_tags))
        .route("/file-tags/create", post(create_file_tag))
        .route("/file-tags/update/{tag_id}", put(update_file_tag))
        .route("/file-tags/delete/{tag_id}", delete(delete_file_tag))
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
        .map(|_| {
            // Log activity
            let state_clone = state.clone();
            let user_id = user.id.clone();
            let file_id_clone = file_id.to_string();
            let tag_id_clone = tag_id.to_string();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::TAG_ADD,
                    &file_id_clone,
                    "",
                    None,
                    None,
                    "success",
                    None,
                    Some(serde_json::json!({ "tag_id": tag_id_clone })),
                ).await;
            });
            StatusCode::CREATED
        })
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
    let file_id = parts[0].to_string();
    let tag_id = parts[1].to_string();
    
    services::tag::untag_file(&state, &user, &file_id, &tag_id)
        .await
        .map(|_| {
            // Log activity
            let state_clone = state.clone();
            let user_id = user.id.clone();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::TAG_REMOVE,
                    &file_id,
                    "",
                    None,
                    None,
                    "success",
                    None,
                    Some(serde_json::json!({ "tag_id": tag_id })),
                ).await;
            });
            StatusCode::NO_CONTENT
        })
        .map_err(|_| StatusCode::NOT_FOUND)
}

// ============================================================================
// FILE-SCOPED TAG ENDPOINTS with Query Parameters
// ============================================================================

/// GET /api/file-tags/list?path={path} - List all tags for a file
async fn list_file_tags(
    State(state): State<AppState>,
    Query(query): Query<FilePathQuery>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // Get tags for this specific file path
    let tags = services::tag::list_by_file(&state, &user, &query.path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        tags.into_iter()
            .map(|t| serde_json::to_value(t).unwrap_or_default())
            .collect(),
    ))
}

/// POST /api/file-tags/create?path={path} - Create tag for a file
async fn create_file_tag(
    State(state): State<AppState>,
    Query(query): Query<FilePathQuery>,
    user: UserInfo,
    Json(req): Json<CreateTagRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let tag = services::tag::create_for_file(&state, &user, &query.path, &req.name, req.color)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let json = serde_json::to_value(tag).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(json)))
}

/// PUT /api/file-tags/update/{tag_id}?path={path} - Update tag
async fn update_file_tag(
    State(state): State<AppState>,
    Path(tag_id): Path<String>,
    Query(_query): Query<FilePathQuery>,
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

/// DELETE /api/file-tags/delete/{tag_id}?path={path} - Delete tag
async fn delete_file_tag(
    State(state): State<AppState>,
    Path(tag_id): Path<String>,
    Query(_query): Query<FilePathQuery>,
    user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    services::tag::delete(&state, &user, &tag_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::NOT_FOUND)
}
