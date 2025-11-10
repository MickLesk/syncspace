//! System API endpoints (stats, storage, health)

use crate::auth::UserInfo;

use crate::{services, AppState};
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/stats", get(get_stats))
        .route("/system/storage", get(get_storage_info))
        .route("/config/info", get(get_config_info))
    // Manual sync endpoint temporarily disabled - sync runs automatically on startup
    // .route("/system/sync", post(sync_filesystem))
}

// Status endpoint is now public and handled in main.rs

async fn get_stats(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // User info available for future per-user stats
    let stats = services::system::get_stats(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    serde_json::to_value(stats)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_storage_info(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // User info available for quota checks
    let storage = services::system::get_storage_info(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    serde_json::to_value(storage)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_config_info(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::system::get_config_info(&state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[allow(dead_code)]
async fn sync_filesystem(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!(
        "🔄 Manual filesystem sync triggered by user: {}",
        user.username
    );

    let user_id_str = user.id.to_string();
    let count = crate::services::sync_service::sync_filesystem_to_db(&state.db_pool, &user_id_str)
        .await
        .map_err(|e| {
            eprintln!("❌ Filesystem sync failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "synced_count": count,
        "message": format!("{} files synced to database", count)
    })))
}
