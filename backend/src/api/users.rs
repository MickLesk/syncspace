//! User management API endpoints

use crate::{auth::UserInfo, services, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub role: Option<String>,
    pub status: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/list", get(list_all_users))
        .route("/users/profile", get(get_profile).put(update_profile))
        .route("/users/settings", get(get_settings).put(update_settings))
        .route(
            "/users/preferences",
            get(get_preferences).put(update_preferences),
        )
}

/// Get current authenticated user info (for token validation)
async fn get_current_user(user: UserInfo) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "id": user.user_id(),
        "username": user.username
    })))
}

async fn get_profile(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::get_profile(&state, &user)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_profile(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<StatusCode, StatusCode> {
    // SECURITY: Validate email if provided and not empty
    if let Some(ref email) = req.email {
        if !email.is_empty() {
            crate::security::validate_email(email).map_err(|_| StatusCode::BAD_REQUEST)?;
        }
    }

    let user_id = user.id.clone();
    services::update_profile(&state, &user, req)
        .await
        .map(|_| {
            // Log activity
            let state_clone = state.clone();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::PROFILE_UPDATE,
                    "",
                    "",
                    None,
                    None,
                    "success",
                    None,
                    None,
                ).await;
            });
            StatusCode::OK
        })
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn get_settings(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::get_settings(&state, &user)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_settings(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<UpdateSettingsRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_id = user.id.clone();
    let metadata = serde_json::json!({
        "theme": req.theme,
        "language": req.language,
        "default_view": req.default_view
    });
    services::update_settings(&state, &user, req)
        .await
        .map(|_| {
            // Log activity
            let state_clone = state.clone();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::SETTINGS_CHANGE,
                    "",
                    "",
                    None,
                    None,
                    "success",
                    None,
                    Some(metadata),
                ).await;
            });
            StatusCode::OK
        })
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn get_preferences(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::get_preferences(&state, &user)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn update_preferences(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<UpdatePreferencesRequest>,
) -> Result<StatusCode, StatusCode> {
    services::update_preferences(&state, &user, req.preferences)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn list_all_users(
    State(state): State<AppState>,
    user: UserInfo,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let users = services::list_users(&state, &user, query.role, query.status)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "data": users,
        "count": users.len()
    })))
}
