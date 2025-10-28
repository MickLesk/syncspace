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
        .route("/shares/{share_id}", get(get_share).put(update_share).delete(delete_share))
        .route("/shares/{share_id}/permissions", put(update_share_permissions))
        .route("/shared-with-me", get(list_shared_with_me))
}

async fn list_shares(State(state): State<AppState>, user: UserInfo) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let shares = services::sharing::list_shares(&state, &user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(shares.into_iter().map(|s| serde_json::to_value(s).unwrap_or_default()).collect()))
}

async fn create_share(State(state): State<AppState>, user: UserInfo, Json(req): Json<CreateShareRequest>) -> Result<Json<serde_json::Value>, StatusCode> {
    let share = services::sharing::create_share(&state, &user, &req.file_path, false).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    serde_json::to_value(share).map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_share(State(state): State<AppState>, Path(share_id): Path<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    let share = services::sharing::get_share(&state, &share_id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    serde_json::to_value(share).map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_share(State(state): State<AppState>, user: UserInfo, Path(share_id): Path<String>, Json(req): Json<serde_json::Value>) -> Result<StatusCode, StatusCode> {
    services::sharing::update_share(&state, &user, &share_id, req).await.map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn delete_share(State(state): State<AppState>, user: UserInfo, Path(share_id): Path<String>) -> Result<StatusCode, StatusCode> {
    services::sharing::delete_share(&state, &user, &share_id).await.map(|_| StatusCode::NO_CONTENT).map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_share_permissions(
    State(state): State<AppState>,
    user: UserInfo,
    Path(share_id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    // Reuse update_share for now
    services::sharing::update_share(&state, &user, &share_id, req).await.map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn list_shared_with_me(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // TODO: Implement actual query for files shared with current user
    let shares = services::sharing::list_shares(&state, &user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(shares.into_iter().map(|s| serde_json::to_value(s).unwrap_or_default()).collect()))
}
