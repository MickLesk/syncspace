/* // Cron Jobs API endpoints - disabled pending job system refactor
// When re-enabled, will provide:
// - POST /api/cron - Create scheduled job
// - GET /api/cron - List jobs
// - PUT /api/cron/{id} - Update job
// - DELETE /api/cron/{id} - Delete job
//
// Currently using simple cron-based execution via background tasks
//
//! Cron Jobs API Endpoints

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete, put},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::auth::UserInfo;
use crate::cron;
use crate::jobs::{CronJob, JobType};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cron", post(create_cron_job_handler))
        .route("/cron", get(list_cron_jobs_handler))
        .route("/cron/{id}", get(get_cron_job_handler))
        .route("/cron/{id}", put(update_cron_job_handler))
        .route("/cron/{id}", delete(delete_cron_job_handler))
        .route("/cron/{id}/enable", post(enable_cron_job_handler))
        .route("/cron/{id}/disable", post(disable_cron_job_handler))
}

// ============================================================================
// Request/Response Models
// ============================================================================

#[derive(Debug, Deserialize)]
struct CreateCronJobRequest {
    name: String,
    job_type: String,
    cron_expression: String,
    payload: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct UpdateCronJobRequest {
    cron_expression: Option<String>,
    payload: Option<serde_json::Value>,
}

// ============================================================================
// Handlers
// ============================================================================

/// Create new cron job
#[tracing::instrument(skip(state, _user, req))]
async fn create_cron_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Json(req): Json<CreateCronJobRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let job_type = JobType::from_str(&req.job_type);
    
    let id = cron::create_cron_job(
        &state.db_pool,
        req.name,
        job_type,
        req.cron_expression,
        req.payload,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to create cron job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(serde_json::json!({
        "id": id,
        "status": "created"
    })))
}

/// List all cron jobs
#[tracing::instrument(skip(state, _user))]
async fn list_cron_jobs_handler(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<CronJob>>, StatusCode> {
    let cron_jobs = cron::list_cron_jobs(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list cron jobs: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(cron_jobs))
}

/// Get specific cron job
#[tracing::instrument(skip(state, _user))]
async fn get_cron_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<CronJob>, StatusCode> {
    let cron_job = cron::get_cron_job(&state.db_pool, &id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get cron job: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(cron_job))
}

/// Update cron job
#[tracing::instrument(skip(state, _user, req))]
async fn update_cron_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
    Json(req): Json<UpdateCronJobRequest>,
) -> Result<StatusCode, StatusCode> {
    cron::update_cron_job(
        &state.db_pool,
        &id,
        req.cron_expression,
        req.payload,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to update cron job: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(StatusCode::OK)
}

/// Delete cron job
#[tracing::instrument(skip(state, _user))]
async fn delete_cron_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    cron::delete_cron_job(&state.db_pool, &id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete cron job: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(StatusCode::OK)
}

/// Enable cron job
#[tracing::instrument(skip(state, _user))]
async fn enable_cron_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    cron::set_cron_job_enabled(&state.db_pool, &id, true)
        .await
        .map_err(|e| {
            tracing::error!("Failed to enable cron job: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(StatusCode::OK)
}

/// Disable cron job
#[tracing::instrument(skip(state, _user))]
async fn disable_cron_job_handler(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    cron::set_cron_job_enabled(&state.db_pool, &id, false)
        .await
        .map_err(|e| {
            tracing::error!("Failed to disable cron job: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(StatusCode::OK)
}
*/  // End of commented cron API

use axum::Router;

pub fn router() -> Router<crate::AppState> {
    Router::new()
    // Job scheduler routes will be added when job system is refactored
    // Placeholder for future cron job management endpoints
}
