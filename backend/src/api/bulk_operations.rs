//! Bulk Operations API
//! Handles background job queue for bulk file operations

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BackgroundJob {
    pub id: String,
    pub job_type: String,
    pub status: String,
    pub priority: i32,
    pub payload: String,
    pub result: Option<String>,
    pub error: Option<String>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub scheduled_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBulkJobRequest {
    pub job_type: String,
    pub file_paths: Vec<String>,
    pub operation: String,
    pub destination: Option<String>,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CancelJobRequest {
    pub reason: Option<String>,
}

/// Create a new bulk operation job
async fn create_bulk_job(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateBulkJobRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let payload = serde_json::json!({
        "user_id": user.id,
        "operation": req.operation,
        "file_paths": req.file_paths,
        "destination": req.destination,
        "total_files": req.file_paths.len(),
    });

    sqlx::query(
        r#"
        INSERT INTO background_jobs (
            id, job_type, status, priority, payload, 
            scheduled_at, created_at, updated_at
        )
        VALUES (?, ?, 'pending', ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&job_id)
    .bind(&req.job_type)
    .bind(req.priority.unwrap_or(5))
    .bind(payload.to_string())
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create bulk job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "job_id": job_id,
            "message": "Bulk operation job created",
            "total_files": req.file_paths.len(),
        })),
    ))
}

/// Get job status and progress
async fn get_job_status(
    State(state): State<AppState>,
    user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let job = sqlx::query_as::<_, BackgroundJob>(
        r#"
        SELECT * FROM background_jobs
        WHERE id = ?
        "#,
    )
    .bind(&job_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Parse payload to check ownership
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .unwrap_or_else(|_| serde_json::json!({}));
    
    if let Some(owner_id) = payload.get("user_id").and_then(|v| v.as_str()) {
        if owner_id != user.id {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Calculate progress
    let progress = if job.status == "completed" {
        100
    } else if job.status == "running" {
        // Try to parse progress from result field
        if let Some(ref result) = job.result {
            if let Ok(result_json) = serde_json::from_str::<serde_json::Value>(result) {
                result_json.get("progress")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(50) as i32
            } else {
                50
            }
        } else {
            50
        }
    } else {
        0
    };

    Ok(Json(serde_json::json!({
        "job_id": job.id,
        "job_type": job.job_type,
        "status": job.status,
        "progress": progress,
        "priority": job.priority,
        "attempts": job.attempts,
        "error": job.error,
        "created_at": job.created_at,
        "started_at": job.started_at,
        "completed_at": job.completed_at,
        "payload": payload,
    })))
}

/// List all jobs for current user
async fn list_jobs(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let jobs = sqlx::query_as::<_, BackgroundJob>(
        r#"
        SELECT * FROM background_jobs
        WHERE payload LIKE ?
        ORDER BY created_at DESC
        LIMIT 100
        "#,
    )
    .bind(format!("%\"user_id\":\"{}%", user.id))
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let jobs_with_progress: Vec<serde_json::Value> = jobs
        .into_iter()
        .map(|job| {
            let progress = if job.status == "completed" {
                100
            } else if job.status == "running" {
                50
            } else {
                0
            };

            serde_json::json!({
                "job_id": job.id,
                "job_type": job.job_type,
                "status": job.status,
                "progress": progress,
                "priority": job.priority,
                "created_at": job.created_at,
                "completed_at": job.completed_at,
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "jobs": jobs_with_progress,
        "total": jobs_with_progress.len(),
    })))
}

/// Cancel a pending or running job
async fn cancel_job(
    State(state): State<AppState>,
    user: UserInfo,
    Path(job_id): Path<String>,
    Json(_req): Json<CancelJobRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job = sqlx::query_as::<_, BackgroundJob>(
        "SELECT * FROM background_jobs WHERE id = ?"
    )
    .bind(&job_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check ownership
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .unwrap_or_else(|_| serde_json::json!({}));
    
    if let Some(owner_id) = payload.get("user_id").and_then(|v| v.as_str()) {
        if owner_id != user.id {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Only allow canceling pending or running jobs
    if job.status != "pending" && job.status != "running" {
        return Err(StatusCode::BAD_REQUEST);
    }

    let now = Utc::now().to_rfc3339();
    sqlx::query(
        r#"
        UPDATE background_jobs
        SET status = 'cancelled', 
            error = 'Cancelled by user',
            completed_at = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&now)
    .bind(&now)
    .bind(&job_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "message": "Job cancelled successfully"
    })))
}

/// Delete a completed job from history
async fn delete_job(
    State(state): State<AppState>,
    user: UserInfo,
    Path(job_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let job = sqlx::query_as::<_, BackgroundJob>(
        "SELECT * FROM background_jobs WHERE id = ?"
    )
    .bind(&job_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check ownership
    let payload: serde_json::Value = serde_json::from_str(&job.payload)
        .unwrap_or_else(|_| serde_json::json!({}));
    
    if let Some(owner_id) = payload.get("user_id").and_then(|v| v.as_str()) {
        if owner_id != user.id {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    sqlx::query("DELETE FROM background_jobs WHERE id = ?")
        .bind(&job_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/bulk/jobs", post(create_bulk_job).get(list_jobs))
        .route("/bulk/jobs/:job_id", get(get_job_status).delete(delete_job))
        .route("/bulk/jobs/:job_id/cancel", post(cancel_job))
}
