#![allow(dead_code)]

//! Database health monitoring API

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::database_monitor::{analyze_database, check_health, checkpoint_wal, vacuum_database};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(get_health))
        .route("/checkpoint", post(trigger_checkpoint))
        .route("/analyze", post(trigger_analyze))
        .route("/vacuum", post(trigger_vacuum))
}

/// Get database health status
///
/// GET /api/db/health
///
/// Returns comprehensive database health information including:
/// - Connection pool status
/// - Query statistics
/// - Database size
/// - Performance metrics
async fn get_health(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    match check_health(&state.db_pool, &state.db_health_monitor).await {
        Ok(health) => Ok(Json(health)),
        Err(e) => {
            eprintln!("❌ Health check failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Trigger a WAL checkpoint
///
/// POST /api/db/checkpoint
///
/// Forces a Write-Ahead Log checkpoint to sync data to the main database file.
/// This can help reduce WAL file size and improve read performance.
///
/// Requires: Admin authentication
async fn trigger_checkpoint(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match checkpoint_wal(&state.db_pool).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "WAL checkpoint completed"
        }))),
        Err(e) => {
            eprintln!("❌ WAL checkpoint failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Trigger database analysis
///
/// POST /api/db/analyze
///
/// Updates SQLite query planner statistics to improve query performance.
/// Should be run periodically or after major data changes.
///
/// Requires: Admin authentication
async fn trigger_analyze(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    match analyze_database(&state.db_pool).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "Database analysis completed"
        }))),
        Err(e) => {
            eprintln!("❌ Database analysis failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Trigger database vacuum
///
/// POST /api/db/vacuum
///
/// Rebuilds the database file to reclaim unused space.
/// Warning: This can take a long time for large databases!
///
/// Requires: Admin authentication
async fn trigger_vacuum(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    match vacuum_database(&state.db_pool).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "Database vacuum completed"
        }))),
        Err(e) => {
            eprintln!("❌ Database vacuum failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
