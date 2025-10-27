//! Performance API endpoints

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use crate::{services, AppState, auth::UserInfo};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/performance/cache/stats", get(get_cache_stats))
        .route("/performance/system/info", get(get_system_info))
}

async fn get_cache_stats(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::performance::get_cache_stats(&state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_system_info(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::performance::get_system_info(&state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
