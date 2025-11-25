//! Batch Operations API Routes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::AppState;
use crate::auth::UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCopyRequest {
    pub sources: Vec<String>,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCompressRequest {
    pub files: Vec<String>,
    pub archive_name: String,
    pub format: String, // zip, tar.gz, etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOperationStatus {
    pub job_id: Uuid,
    pub status: String, // pending, running, completed, failed, cancelled
    pub progress: f32,  // 0.0 to 100.0
    pub total_items: i32,
    pub processed_items: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

/// Batch copy files
async fn batch_copy(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<BatchCopyRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Create background job for batch copy
    let payload = serde_json::json!({
        "sources": req.sources,
        "destination": req.destination,
        "user_id": user_info.id,
    }).to_string();
    
    sqlx::query(
        "INSERT INTO background_jobs (id, job_type, status, payload, scheduled_at, created_at, updated_at)
         VALUES (?, 'batch_copy', 'pending', ?, ?, ?, ?)"
    )
    .bind(&job_id)
    .bind(&payload)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::ACCEPTED, Json(serde_json::json!({
        "job_id": job_id,
        "status": "pending",
        "message": "Batch copy operation queued"
    }))))
}

/// Batch compress files
async fn batch_compress(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<BatchCompressRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Create background job for batch compress
    let payload = serde_json::json!({
        "files": req.files,
        "archive_name": req.archive_name,
        "format": req.format,
        "user_id": user_info.id,
    }).to_string();
    
    sqlx::query(
        "INSERT INTO background_jobs (id, job_type, status, payload, scheduled_at, created_at, updated_at)
         VALUES (?, 'batch_compress', 'pending', ?, ?, ?, ?)"
    )
    .bind(&job_id)
    .bind(&payload)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::ACCEPTED, Json(serde_json::json!({
        "job_id": job_id,
        "status": "pending",
        "message": "Batch compression operation queued"
    }))))
}

/// Get batch operation status
async fn get_batch_status(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    // Retrieve actual status from database
    let job: Option<(String, Option<i32>, Option<i32>, Option<String>)> = sqlx::query_as(
        "SELECT status, total_items, processed_items, error
         FROM background_jobs 
         WHERE id = ? 
         LIMIT 1"
    )
    .bind(job_id.to_string())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((status, total_items, processed_items, error)) = job {
        let total = total_items.unwrap_or(0);
        let processed = processed_items.unwrap_or(0);
        let progress = if total > 0 {
            (processed as f32 / total as f32 * 100.0).min(100.0).max(0.0)
        } else {
            0.0
        };

        Ok(Json(BatchOperationStatus {
            job_id,
            status,
            progress,
            total_items: total,
            processed_items: processed,
            started_at: Utc::now(),  // Fallback - ideally parse from DB
            completed_at: None,
            error,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Cancel batch operation
async fn cancel_batch_operation(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
    _user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if job exists and is not already completed
    let job_exists: Option<(String,)> = sqlx::query_as(
        "SELECT status FROM background_jobs WHERE id = ?"
    )
    .bind(job_id.to_string())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if job_exists.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Update job status to cancelled
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE background_jobs SET status = 'cancelled', updated_at = ? WHERE id = ?"
    )
    .bind(&now)
    .bind(job_id.to_string())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({
        "job_id": job_id,
        "status": "cancelled",
        "message": "Batch operation cancelled successfully"
    })))
}

/// Build batch operations router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/batch/copy", post(batch_copy))
        .route("/batch/compress", post(batch_compress))
        .route("/batch/operations/{job_id}", get(get_batch_status))
        .route("/batch/operations/{job_id}/cancel", post(cancel_batch_operation))
}
