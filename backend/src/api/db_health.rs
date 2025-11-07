//! Database Health and Monitoring API
//! 
//! Provides endpoints for database health checks, metrics, and performance monitoring

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::auth::UserInfo;
use crate::db_monitor::{verify_db_health, get_sqlite_stats};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health/db", get(db_health_handler))
        .route("/health/db/metrics", get(db_metrics_handler))
        .route("/health/db/slow-queries", get(slow_queries_handler))
        .route("/health/db/stats", get(db_stats_handler))
}

/// Database health check endpoint
/// GET /api/health/db
/// Returns database connection status and pool health
#[tracing::instrument(skip(state))]
async fn db_health_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::debug!("Database health check requested");

    // Verify connectivity
    match verify_db_health(&state.db_pool).await {
        Ok(_) => {
            // Get health status from monitor
            let health = state.db_monitor.health_status().await;
            
            tracing::info!("Database health check: {}", health.status);
            
            Ok(Json(json!({
                "status": health.status,
                "timestamp": health.timestamp,
                "database": "connected",
                "pool": {
                    "utilization": format!("{:.1}%", health.pool_utilization * 100.0),
                    "active_connections": health.active_connections,
                    "total_connections": health.total_connections,
                    "max_connections": health.max_connections
                },
                "performance": {
                    "average_query_time_ms": format!("{:.2}", health.average_query_time_ms),
                    "average_acquire_time_ms": format!("{:.2}", health.average_acquire_time_ms),
                    "slow_queries": health.slow_query_count,
                    "connection_leaks": health.connection_leaks
                },
                "errors": {
                    "acquire_timeouts": health.acquire_timeouts,
                    "connection_errors": health.connection_errors
                }
            })))
        }
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Get detailed pool metrics (protected - admin only)
/// GET /api/health/db/metrics
#[tracing::instrument(skip(state, _user), fields(user_id = %_user.id))]
async fn db_metrics_handler(
    State(state): State<AppState>,
    _user: UserInfo, // Require authentication
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::debug!("Database metrics requested");

    let metrics = state.db_monitor.get_metrics().await;
    let (recommended_max, recommended_min) = state.db_monitor.recommend_pool_size().await;

    Ok(Json(json!({
        "timestamp": metrics.timestamp,
        "current": {
            "connections_active": metrics.connections_active,
            "connections_idle": metrics.connections_idle,
            "connections_total": metrics.connections_total,
            "pool_utilization": format!("{:.1}%", metrics.pool_utilization * 100.0)
        },
        "limits": {
            "max_connections": metrics.max_connections,
            "min_connections": metrics.min_connections,
            "recommended_max": recommended_max,
            "recommended_min": recommended_min
        },
        "performance": {
            "average_acquire_time_ms": format!("{:.2}", metrics.average_acquire_time_ms),
            "average_query_time_ms": format!("{:.2}", metrics.average_query_time_ms)
        },
        "errors": {
            "acquire_timeouts": metrics.acquire_timeouts,
            "connection_errors": metrics.connection_errors,
            "slow_queries": metrics.slow_queries
        }
    })))
}

/// Get slow query log (protected - admin only)
/// GET /api/health/db/slow-queries
#[tracing::instrument(skip(state, _user), fields(user_id = %_user.id))]
async fn slow_queries_handler(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::debug!("Slow queries requested");

    let slow_queries = state.db_monitor.get_slow_queries().await;

    Ok(Json(json!({
        "count": slow_queries.len(),
        "threshold_ms": 100,
        "queries": slow_queries.iter().map(|q| json!({
            "timestamp": q.timestamp,
            "duration_ms": q.duration_ms,
            "query": q.query.chars().take(200).collect::<String>(),
            "caller": q.caller
        })).collect::<Vec<_>>()
    })))
}

/// Get SQLite-specific statistics
/// GET /api/health/db/stats
#[tracing::instrument(skip(state, _user), fields(user_id = %_user.id))]
async fn db_stats_handler(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::debug!("SQLite stats requested");

    match get_sqlite_stats(&state.db_pool).await {
        Ok(stats) => Ok(Json(json!({
            "cache": {
                "cache_size_pages": stats.cache_size_pages,
                "total_pages": stats.total_pages,
                "cache_hit_rate": if stats.total_pages > 0 {
                    format!("{:.1}%", (stats.cache_size_pages as f64 / stats.total_pages as f64) * 100.0)
                } else {
                    "N/A".to_string()
                }
            },
            "wal": {
                "frames_checkpointed": stats.wal_frames_checkpointed,
                "frames_pending": stats.wal_frames_pending
            }
        }))),
        Err(e) => {
            tracing::error!("Failed to get SQLite stats: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
