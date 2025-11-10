//! Background Jobs API Endpoints
//! 
//! Provides REST API for job management and monitoring.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::auth::UserInfo;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/jobs/stats", get(get_job_stats_handler))
        .route("/jobs", get(list_jobs_handler))
        .route("/jobs/{job_id}", get(get_job_handler))
        .route("/jobs/{job_id}/cancel", post(cancel_job_handler))
        .route("/jobs/cleanup", post(cleanup_jobs_handler))
}

#[derive(Debug, Serialize)]
struct JobStatsResponse {
    pending: i64,
    running: i64,
    completed: i64,
    failed: i64,
    total: i64,
}

#[derive(Debug, Deserialize)]
struct ListJobsQuery {
    status: Option<String>,
    #[serde(default = "default_limit")]
    limit: i32,
    #[serde(default)]
    offset: i32,
}

fn default_limit() -> i32 {
    50
}

#[derive(Debug, Serialize)]
struct JobResponse {
    id: String,
    job_type: String,
    status: String,
    priority: i32,
    attempts: i32,
    max_retries: i32,
    created_at: String,
    scheduled_at: Option<String>,
    started_at: Option<String>,
    completed_at: Option<String>,
    error_message: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct JobRow {
    id: String,
    job_type: String,
    status: String,
    priority: i32,
    attempts: i32,
    max_retries: i32,
    created_at: String,
    scheduled_at: Option<String>,
    started_at: Option<String>,
    completed_at: Option<String>,
    error_message: Option<String>,
}

#[tracing::instrument(skip(state, _user))]
async fn get_job_stats_handler(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<JobStatsResponse>, StatusCode> {
    let pending: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM job_queue WHERE status = 'pending'")
        .fetch_one(&state.db_pool).await.map_err(|e| {
            tracing::error!("Failed to count pending jobs: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let running: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM job_queue WHERE status = 'running'")
        .fetch_one(&state.db_pool).await.map_err(|e| {
            tracing::error!("Failed to count running jobs: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let completed: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM job_queue WHERE status = 'completed'")
        .fetch_one(&state.db_pool).await.map_err(|e| {
            tracing::error!("Failed to count completed jobs: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let failed: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM job_queue WHERE status = 'failed'")
        .fetch_one(&state.db_pool).await.map_err(|e| {
            tracing::error!("Failed to count failed jobs: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(JobStatsResponse {
        pending: pending.0,
        running: running.0,
        completed: completed.0,
        failed: failed.0,
        total: pending.0 + running.0 + completed.0 + failed.0,
    }))
}

#[tracing::instrument(skip(state, _user))]
async fn list_jobs_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<ListJobsQuery>,
) -> Result<Json<Vec<JobResponse>>, StatusCode> {
    let mut sql = String::from("SELECT * FROM job_queue WHERE 1=1");
    
    if let Some(status) = &query.status {
        sql.push_str(&format!(" AND status = '{}'", status));
    }
    
    sql.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", query.limit, query.offset));
    
    let jobs: Vec<JobRow> = sqlx::query_as(&sql).fetch_all(&state.db_pool).await.map_err(|e| {
        tracing::error!("Failed to list jobs: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let response: Vec<JobResponse> = jobs.into_iter().map(|job| JobResponse {
        id: job.id, job_type: job.job_type, status: job.status, priority: job.priority,
        attempts: job.attempts, max_retries: job.max_retries, created_at: job.created_at,
        scheduled_at: job.scheduled_at, started_at: job.started_at,
        completed_at: job.completed_at, error_message: job.error_message,
    }).collect();
    
    Ok(Json(response))
}

#[tracing::instrument(skip(state, _user))]
async fn get_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<Json<JobResponse>, StatusCode> {
    let job: JobRow = sqlx::query_as("SELECT * FROM job_queue WHERE id = ?")
        .bind(&job_id).fetch_one(&state.db_pool).await.map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(Json(JobResponse {
        id: job.id, job_type: job.job_type, status: job.status, priority: job.priority,
        attempts: job.attempts, max_retries: job.max_retries, created_at: job.created_at,
        scheduled_at: job.scheduled_at, started_at: job.started_at,
        completed_at: job.completed_at, error_message: job.error_message,
    }))
}

#[tracing::instrument(skip(state, _user))]
async fn cancel_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query("UPDATE job_queue SET status = 'cancelled', completed_at = ? WHERE id = ? AND status IN ('pending', 'running')")
        .bind(Utc::now().to_rfc3339()).bind(&job_id).execute(&state.db_pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    
    Ok(Json(serde_json::json!({"success": true, "message": "Job cancelled"})))
}

#[tracing::instrument(skip(state, _user))]
async fn cleanup_jobs_handler(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let cutoff = Utc::now() - chrono::Duration::days(30);
    let result = sqlx::query("DELETE FROM job_queue WHERE status IN ('completed', 'failed', 'cancelled') AND completed_at < ?")
        .bind(cutoff.to_rfc3339()).execute(&state.db_pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({"deleted": result.rows_affected()})))
}