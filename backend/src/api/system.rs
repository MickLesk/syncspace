use crate::auth::UserInfo;
//! System API endpoints (stats, storage, health)

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use crate::{auth::User, services::system, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/status", get(get_status))
        .route("/stats", get(get_stats))
        .route("/system/storage", get(get_storage_info))
        .route("/config/info", get(get_config_info))
}

async fn get_status(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    services::system::get_status(&state).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_stats(State(state): State<AppState>, user: UserInfo) -> Result<Json<serde_json::Value>, StatusCode> {
    services::system::get_stats(&state, &user).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_storage_info(State(state): State<AppState>, user: UserInfo) -> Result<Json<serde_json::Value>, StatusCode> {
    services::system::get_storage_info(&state, &user).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_config_info(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    services::system::get_config_info(&state).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
