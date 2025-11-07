//! Background Jobs API Endpoints
//! 
//! Provides REST API for job management and monitoring.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::auth::UserInfo;
use crate::jobs::{self, JobType, JobStatus};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/jobs", post(enqueue_job_handler))
        .route("/jobs", get(list_jobs_handler))
        .route("/jobs/:job_id", get(get_job_handler))
        .route("/jobs/:job_id/cancel", post(cancel_job_handler))
        .route("/jobs/cleanup", post(cleanup_jobs_handler))
        .route("/jobs/stats", get(get_job_stats_handler))
}

// ============================================================================
// Request/Response Models
// ============================================================================

#[derive(Debug, Deserialize)]
struct EnqueueJobRequest {
    job_type: String,
    payload: serde_json::Value,
    #[serde(default = "default_priority")]
    priority: i32,
    scheduled_at: Option<String>,
}

fn default_priority() -> i32 {
    5
}

#[derive(Debug, Serialize)]
struct EnqueueJobResponse {
    job_id: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct ListJobsQuery {
    status: Option<String>,
    job_type: Option<String>,
    #[serde(default = "default_limit")]
    limit: i32,
    #[serde(default)]
    offset: i32,
}

fn default_limit() -> i32 {
    50
}

#[derive(Debug, Serialize)]
struct JobStatsResponse {
    pending: i64,
    running: i64,
    completed: i64,
    failed: i64,
    total: i64,
}

// ============================================================================
// Handlers
// ============================================================================

/// Enqueue a new background job
#[tracing::instrument(skip(state, _user, req))]
async fn enqueue_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Json(req): Json<EnqueueJobRequest>,
) -> Result<Json<EnqueueJobResponse>, StatusCode> {
    let job_type = JobType::from_str(&req.job_type);
    
    let scheduled_at = req.scheduled_at
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    
    let job_id = jobs::enqueue_job(
        &state.pool,
        job_type,
        req.payload,
        req.priority,
        scheduled_at,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to enqueue job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(EnqueueJobResponse {
        job_id,
        status: "pending".to_string(),
    }))
}

/// List jobs with optional filters
#[tracing::instrument(skip(state, _user))]
async fn list_jobs_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<ListJobsQuery>,
) -> Result<Json<Vec<jobs::BackgroundJob>>, StatusCode> {
    let status = query.status.map(|s| JobStatus::from_str(&s));
    let job_type = query.job_type.map(|s| JobType::from_str(&s));
    
    let jobs_list = jobs::list_jobs(
        &state.pool,
        status,
        job_type,
        query.limit,
        query.offset,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to list jobs: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(jobs_list))
}

/// Get specific job by ID
#[tracing::instrument(skip(state, _user))]
async fn get_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<Json<jobs::BackgroundJob>, StatusCode> {
    let job = jobs::get_job(&state.pool, &job_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get job: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(job))
}

/// Cancel a pending job
#[tracing::instrument(skip(state, _user))]
async fn cancel_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    jobs::cancel_job(&state.pool, &job_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to cancel job: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(StatusCode::OK)
}

/// Cleanup old completed/failed jobs
#[tracing::instrument(skip(state, _user))]
async fn cleanup_jobs_handler(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let deleted = jobs::cleanup_old_jobs(&state.pool, 30)
        .await
        .map_err(|e| {
            tracing::error!("Failed to cleanup jobs: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(serde_json::json!({
        "deleted": deleted
    })))
}

/// Get job statistics
#[tracing::instrument(skip(state, _user))]
async fn get_job_stats_handler(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<JobStatsResponse>, StatusCode> {
    let pending: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM background_jobs WHERE status = 'pending'"
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let running: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM background_jobs WHERE status = 'running'"
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let completed: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM background_jobs WHERE status = 'completed'"
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let failed: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM background_jobs WHERE status = 'failed'"
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let total = pending.0 + running.0 + completed.0 + failed.0;
    
    Ok(Json(JobStatsResponse {
        pending: pending.0,
        running: running.0,
        completed: completed.0,
        failed: failed.0,
        total,
    }))
}
