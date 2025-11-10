//! Performance API endpoints

use crate::{auth::UserInfo, services, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub job_type: String,
    pub parameters: serde_json::Value,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/performance/metrics", get(get_metrics))
        .route("/performance/metrics/history", get(get_metrics_history))
        .route("/performance/cache/stats", get(get_cache_stats))
        .route("/performance/cache/clear", post(clear_cache))
        .route("/performance/jobs", get(list_jobs).post(create_job))
        .route("/performance/jobs/{job_id}/status", get(get_job_status))
        .route("/performance/system/info", get(get_system_info))
}

async fn get_metrics(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::performance::get_metrics(&state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_metrics_history(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::performance::get_metrics_history(&state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_cache_stats(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::performance::get_cache_stats(&state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn clear_cache(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::performance::clear_cache(&state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn list_jobs(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // Query job_queue table for active/pending jobs
    let queue = crate::jobs::queue::JobQueue::new(std::sync::Arc::new(state.db_pool.clone()));
    let jobs = queue.list_jobs(None, None, 50, 0).await.map_err(|e| {
        tracing::error!("Failed to list jobs: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let job_list = jobs
        .into_iter()
        .map(|job| {
            serde_json::json!({
                "id": job.id,
                "job_type": job.job_type,
                "status": job.status,
                "created_at": job.created_at,
                "started_at": job.started_at,
                "completed_at": job.completed_at,
                "error": job.error,
                "attempts": job.attempts,
            })
        })
        .collect();

    Ok(Json(job_list))
}

async fn create_job(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateJobRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Create a new job
    let job = crate::jobs::types::Job {
        id: Uuid::new_v4().to_string(),
        job_type: req.job_type.clone(),
        status: "pending".to_string(),
        payload: req.parameters.to_string(),
        result: None,
        error: None,
        attempts: 0,
        max_attempts: 3,
        created_at: chrono::Utc::now().to_rfc3339(),
        started_at: None,
        completed_at: None,
        scheduled_for: None,
        created_by: Some(user.id),
    };

    // Enqueue the job
    let queue = crate::jobs::queue::JobQueue::new(std::sync::Arc::new(state.db_pool.clone()));
    let enqueued_job = queue.enqueue(job).await.map_err(|e| {
        tracing::error!("Failed to enqueue job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::json!({
        "job_id": enqueued_job.id,
        "job_type": enqueued_job.job_type,
        "status": enqueued_job.status,
        "message": "Job created successfully"
    })))
}

async fn get_job_status(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get job by ID from database
    let queue = crate::jobs::queue::JobQueue::new(std::sync::Arc::new(state.db_pool.clone()));
    let job = queue.get_job(&job_id.to_string()).await.map_err(|e| {
        tracing::error!("Failed to get job status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match job {
        Some(job) => Ok(Json(serde_json::json!({
            "job_id": job.id,
            "job_type": job.job_type,
            "status": job.status,
            "created_at": job.created_at,
            "started_at": job.started_at,
            "completed_at": job.completed_at,
            "error": job.error,
            "result": job.result,
            "attempts": job.attempts,
            "max_attempts": job.max_attempts,
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_system_info(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    services::performance::get_system_info(&state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
