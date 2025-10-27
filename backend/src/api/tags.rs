use crate::auth::UserInfo;
//! Tags API endpoints

use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post}, Json, Router};
use serde::Deserialize;
use crate::{auth::User, services::tag, AppState};

#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/:tag_id", delete(delete_tag))
        .route("/file-tags", post(tag_file))
        .route("/file-tags/:file_tag_id", delete(untag_file))
}

async fn list_tags(State(state): State<AppState>, user: UserInfo) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    services::tag::list(&state, &user).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_tag(State(state): State<AppState>, user: UserInfo, Json(req): Json<CreateTagRequest>) -> Result<Json<serde_json::Value>, StatusCode> {
    services::tag::create(&state, &user, req).await.map(Json).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn delete_tag(State(state): State<AppState>, user: UserInfo, Path(tag_id): Path<String>) -> Result<StatusCode, StatusCode> {
    services::tag::delete(&state, &user, &tag_id).await.map(|_| StatusCode::NO_CONTENT).map_err(|_| StatusCode::NOT_FOUND)
}

async fn tag_file(State(state): State<AppState>, user: UserInfo, Json(req): Json<serde_json::Value>) -> Result<StatusCode, StatusCode> {
    services::tag::tag_file(&state, &user, req).await.map(|_| StatusCode::CREATED).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn untag_file(State(state): State<AppState>, user: UserInfo, Path(file_tag_id): Path<String>) -> Result<StatusCode, StatusCode> {
    services::tag::untag_file(&state, &user, &file_tag_id).await.map(|_| StatusCode::NO_CONTENT).map_err(|_| StatusCode::NOT_FOUND)
}
