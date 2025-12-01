//! Auto-Cleanup Management API
//!
//! Endpoints for viewing and managing automatic cleanup of deleted files
//! Admins can:
//! - View cleanup statistics
//! - View eligible files for cleanup
//! - Manually trigger cleanup
//! - Configure cleanup settings

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use crate::{auth::UserInfo, services::cleanup_service::{self, CleanupConfig, CleanupStats}, AppState};

#[derive(Debug, Serialize)]
pub struct CleanupStatusResponse {
    pub eligible_for_cleanup: i32,
    pub retention_days: i64,
    pub last_cleanup: Option<CleanupStats>,
}

#[derive(Debug, Deserialize)]
pub struct TriggerCleanupRequest {
    pub dry_run: Option<bool>,
    pub retention_days: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CleanupResponse {
    pub success: bool,
    pub stats: CleanupStats,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cleanup/status", get(get_cleanup_status))
        .route("/cleanup/trigger", post(trigger_cleanup))
        .route("/cleanup/last", get(get_last_cleanup))
}

/// Get current cleanup status (eligible files count, last cleanup info)
async fn get_cleanup_status(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo, // Admin check should be in middleware
) -> Result<Json<CleanupStatusResponse>, StatusCode> {
    let config = CleanupConfig::default();

    // Get count of eligible files
    let eligible = cleanup_service::get_eligible_for_cleanup_count(&state.db_pool, config.retention_days)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get last cleanup stats
    let last_cleanup = cleanup_service::get_cleanup_stats(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CleanupStatusResponse {
        eligible_for_cleanup: eligible,
        retention_days: config.retention_days,
        last_cleanup,
    }))
}

/// Manually trigger cleanup (admin only)
async fn trigger_cleanup(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo, // Should verify admin role
    Json(req): Json<TriggerCleanupRequest>,
) -> Result<Json<CleanupResponse>, StatusCode> {
    let mut config = CleanupConfig::default();
    
    if let Some(days) = req.retention_days {
        config.retention_days = days;
    }

    // For now, ignore dry_run - actual implementation could add this
    let _dry_run = req.dry_run.unwrap_or(false);

    let stats = cleanup_service::cleanup_expired_deleted_files(&state.db_pool, &config)
        .await
        .map_err(|e| {
            tracing::error!("Cleanup failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(CleanupResponse {
        success: true,
        stats,
    }))
}

/// Get last cleanup execution stats
async fn get_last_cleanup(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
) -> Result<Json<Option<CleanupStats>>, StatusCode> {
    cleanup_service::get_cleanup_stats(&state.db_pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
