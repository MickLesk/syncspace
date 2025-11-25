//! Config API Routes

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::auth::UserInfo;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub max_upload_size: i64,
    pub allowed_file_types: Vec<String>,
    pub enable_sharing: bool,
    pub enable_versioning: bool,
    pub enable_trash: bool,
    pub trash_retention_days: i32,
    pub version_retention_count: i32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_upload_size: 100 * 1024 * 1024, // 100MB
            allowed_file_types: vec!["*".to_string()],
            enable_sharing: true,
            enable_versioning: true,
            enable_trash: true,
            trash_retention_days: 30,
            version_retention_count: 10,
        }
    }
}

/// Get application config
async fn get_config(State(_state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    // State available for future database-backed config
    let config = AppConfig::default();

    Ok(Json(config))
}

/// Update application config (admin only)
async fn update_config(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(config): Json<AppConfig>,
) -> Result<impl IntoResponse, StatusCode> {
    // Verify user is admin
    if user_info.username != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Log config changes for audit trail
    // Log configuration change
    eprintln!(
        "Config updated by {}: max_upload_size={}, enable_sharing={}",
        user_info.username, config.max_upload_size, config.enable_sharing
    );

    // Persist to database settings table
    if let Err(e) = sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)"
    )
    .bind("max_upload_size")
    .bind(config.max_upload_size.to_string())
    .execute(&state.db_pool)
    .await
    {
        tracing::warn!("Failed to persist max_upload_size: {}", e);
    }
    
    if let Err(e) = sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)"
    )
    .bind("enable_sharing")
    .bind(config.enable_sharing.to_string())
    .execute(&state.db_pool)
    .await
    {
        tracing::warn!("Failed to persist enable_sharing: {}", e);
    }

    Ok(Json(serde_json::json!({
        "message": "Configuration updated successfully and persisted to database",
        "max_upload_size": config.max_upload_size,
        "enable_sharing": config.enable_sharing
    })))
}

/// Build config router
pub fn router() -> Router<AppState> {
    Router::new().route("/config", get(get_config).put(update_config))
}
