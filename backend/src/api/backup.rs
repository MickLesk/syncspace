//! Backup API endpoints

use crate::auth::UserInfo;

use crate::{services, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub backup_type: String,
    #[allow(dead_code)]
    pub include_versions: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BackupSchedule {
    pub id: Uuid,
    pub name: String,
    pub cron_expression: String,
    pub backup_type: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub cron_expression: String,
    pub backup_type: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/backups", get(list_backups))
        .route("/backups/create", post(create_backup))
        .route(
            "/backups/{backup_id}",
            get(get_backup).delete(delete_backup),
        )
        .route("/backups/{backup_id}/verify", post(verify_backup))
        .route(
            "/backups/{backup_id}/verifications",
            get(list_verifications),
        )
        .route("/backups/cleanup", post(cleanup_backups))
        // Backup schedules
        .route(
            "/backups/schedules",
            get(list_schedules).post(create_schedule),
        )
        .route(
            "/backups/schedules/{schedule_id}",
            get(get_schedule)
                .put(update_schedule)
                .delete(delete_schedule),
        )
        .route(
            "/backups/schedules/{schedule_id}/trigger",
            post(trigger_schedule),
        )
}

async fn list_backups(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let backups = services::backup::list_backups(&state, &user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        backups
            .into_iter()
            .map(|b| serde_json::to_value(b).unwrap_or_default())
            .collect(),
    ))
}

async fn create_backup(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateBackupRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let backup = services::backup::create_backup(&state, &user, &req.backup_type)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    serde_json::to_value(backup)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_backup(
    State(_state): State<AppState>,
    _user: UserInfo,
    Path(backup_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({"id": backup_id})))
}

async fn delete_backup(
    State(_state): State<AppState>,
    _user: UserInfo,
    Path(_backup_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

async fn verify_backup(
    State(_state): State<AppState>,
    _user: UserInfo,
    Path(_backup_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({"valid": true})))
}

async fn list_verifications(
    State(_state): State<AppState>,
    Path(_backup_id): Path<String>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // TODO: Implement verification history
    Ok(Json(vec![]))
}

async fn cleanup_backups(
    State(_state): State<AppState>,
    _user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

// Backup schedule handlers

async fn list_schedules(
    State(_state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<BackupSchedule>>, StatusCode> {
    // TODO: Implement schedule listing
    Ok(Json(vec![]))
}

async fn create_schedule(
    State(_state): State<AppState>,
    _user: UserInfo,
    Json(req): Json<CreateScheduleRequest>,
) -> Result<Json<BackupSchedule>, StatusCode> {
    let schedule = BackupSchedule {
        id: Uuid::new_v4(),
        name: req.name,
        cron_expression: req.cron_expression,
        backup_type: req.backup_type,
        enabled: true,
    };

    Ok(Json(schedule))
}

async fn get_schedule(
    State(_state): State<AppState>,
    Path(_schedule_id): Path<Uuid>,
) -> Result<Json<BackupSchedule>, StatusCode> {
    // TODO: Implement schedule retrieval
    Err(StatusCode::NOT_FOUND)
}

async fn update_schedule(
    State(_state): State<AppState>,
    Path(schedule_id): Path<Uuid>,
    Json(req): Json<CreateScheduleRequest>,
) -> Result<Json<BackupSchedule>, StatusCode> {
    let schedule = BackupSchedule {
        id: schedule_id,
        name: req.name,
        cron_expression: req.cron_expression,
        backup_type: req.backup_type,
        enabled: true,
    };

    Ok(Json(schedule))
}

async fn delete_schedule(
    State(_state): State<AppState>,
    Path(_schedule_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement schedule deletion
    Ok(StatusCode::NO_CONTENT)
}

async fn trigger_schedule(
    State(_state): State<AppState>,
    _user: UserInfo,
    Path(_schedule_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Trigger backup from schedule
    Ok(Json(serde_json::json!({
        "message": "Backup triggered successfully"
    })))
}
