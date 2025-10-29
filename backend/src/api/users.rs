//! User management API endpoints

use axum::{extract::State, http::StatusCode, routing::{get, put}, Json, Router};
use serde::{Deserialize, Serialize};
use crate::{auth::UserInfo, services, AppState, handlers};

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub avatar_base64: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub default_view: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub preferences: serde_json::Value,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/profile", get(get_profile).put(update_profile))
        .route("/users/settings", get(handlers::user_handlers::get_user_settings).put(handlers::user_handlers::update_user_settings))
        .route("/users/preferences", get(handlers::user_handlers::get_user_preferences).put(handlers::user_handlers::update_user_preferences))
}

/// Get current authenticated user info (for token validation)
async fn get_current_user(user: UserInfo) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": user.user_id,
        "username": user.username
    })))
}

async fn get_profile(State(state): State<AppState>, user: UserInfo) -> Result<Json<serde_json::Value>, StatusCode> {
    services::get_profile(&state, &user).await.map(Json).map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_profile(State(state): State<AppState>, user: UserInfo, Json(req): Json<UpdateProfileRequest>) -> Result<StatusCode, StatusCode> {
    services::update_profile(&state, &user, req).await.map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn get_settings(State(state): State<AppState>, user: UserInfo) -> Result<Json<serde_json::Value>, StatusCode> {
    services::get_settings(&state, &user).await.map(Json).map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_settings(State(state): State<AppState>, user: UserInfo, Json(req): Json<UpdateSettingsRequest>) -> Result<StatusCode, StatusCode> {
    services::update_settings(&state, &user, req).await.map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn get_preferences(State(state): State<AppState>, user: UserInfo) -> Result<Json<serde_json::Value>, StatusCode> {
    services::get_preferences(&state, &user).await.map(Json).map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_preferences(State(state): State<AppState>, user: UserInfo, Json(req): Json<UpdatePreferencesRequest>) -> Result<StatusCode, StatusCode> {
    services::update_preferences(&state, &user, req.preferences).await.map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}
