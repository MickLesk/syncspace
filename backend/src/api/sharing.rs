//! File sharing API endpoints

use crate::auth::UserInfo;

use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post, put}, Json, Router};
use serde::{Deserialize, Serialize};
use crate::{services, AppState};

#[derive(Debug, Deserialize)]
pub struct CreateShareRequest {
    pub file_path: String,
    pub permission: String,
    pub expires_at: Option<String>,
    pub password: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/shares", get(list_shares).post(create_share))
        .route("/shares/:share_id", get(get_share).put(update_share).delete(delete_share))
}

async fn list_shares(State(state): State<AppState>, user: UserInfo) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    services::sharing::list_shares(&state, &user).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_share(State(state): State<AppState>, user: UserInfo, Json(req): Json<CreateShareRequest>) -> Result<Json<serde_json::Value>, StatusCode> {
    services::sharing::create_share(&state, &user, req).await.map(Json).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn get_share(State(state): State<AppState>, Path(share_id): Path<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    services::sharing::get_share(&state, &share_id).await.map(Json).map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_share(State(state): State<AppState>, user: UserInfo, Path(share_id): Path<String>, Json(req): Json<serde_json::Value>) -> Result<StatusCode, StatusCode> {
    services::sharing::update_share(&state, &user, &share_id, req).await.map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn delete_share(State(state): State<AppState>, user: UserInfo, Path(share_id): Path<String>) -> Result<StatusCode, StatusCode> {
    services::sharing::delete_share(&state, &user, &share_id).await.map(|_| StatusCode::NO_CONTENT).map_err(|_| StatusCode::NOT_FOUND)
}
