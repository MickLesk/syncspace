//! User profile and settings handlers

use crate::auth;
use crate::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub language: String,
    pub theme: String,
    pub default_view: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub view_mode: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub sidebar_collapsed: Option<bool>,
    pub recent_searches: Option<Vec<String>>,
}

/// Get user settings
pub async fn get_user_settings_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<UserSettings>, StatusCode> {
    let settings: Option<(String, String, String)> = sqlx::query_as(
        "SELECT language, theme, default_view FROM users WHERE id = ?"
    )
    .bind(&user.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some((language, theme, default_view)) = settings {
        Ok(Json(UserSettings {
            language,
            theme,
            default_view,
        }))
    } else {
        Ok(Json(UserSettings {
            language: "en".to_string(),
            theme: "light".to_string(),
            default_view: "grid".to_string(),
        }))
    }
}

/// Update user settings
pub async fn update_user_settings_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(settings): Json<UserSettings>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query(
        "UPDATE users SET language = ?, theme = ?, default_view = ? WHERE id = ?"
    )
    .bind(&settings.language)
    .bind(&settings.theme)
    .bind(&settings.default_view)
    .bind(&user.id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::OK)
}

/// Get user preferences
pub async fn get_user_preferences_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<JsonValue>, StatusCode> {
    let prefs: Option<(String,)> = sqlx::query_as(
        "SELECT preferences FROM users WHERE id = ?"
    )
    .bind(&user.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some((prefs_json,)) = prefs {
        let parsed: JsonValue = serde_json::from_str(&prefs_json).unwrap_or(serde_json::json!({}));
        Ok(Json(parsed))
    } else {
        Ok(Json(serde_json::json!({})))
    }
}

/// Update user preferences
pub async fn update_user_preferences_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(preferences): Json<JsonValue>,
) -> Result<StatusCode, StatusCode> {
    let prefs_json = serde_json::to_string(&preferences)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    sqlx::query(
        "UPDATE users SET preferences = ? WHERE id = ?"
    )
    .bind(&prefs_json)
    .bind(&user.id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::OK)
}
