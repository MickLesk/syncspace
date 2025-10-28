//! Config API Routes

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, put},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::auth::UserInfo;

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
async fn get_config(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Load from database or config file
    let config = AppConfig::default();
    
    Ok(Json(config))
}

/// Update application config
async fn update_config(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(config): Json<AppConfig>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Save to database or config file
    // TODO: Add admin permission check
    
    Ok(Json(serde_json::json!({
        "message": "Configuration updated successfully"
    })))
}

/// Build config router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/config", get(get_config).put(update_config))
}
