//! Backup API endpoints

use crate::auth::UserInfo;

use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post}, Json, Router};
use serde::Deserialize;
use crate::{services, AppState};

#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub backup_type: String,
    pub include_versions: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/backups", get(list_backups))
        .route("/backups/create", post(create_backup))
        .route("/backups/{backup_id}", get(get_backup).delete(delete_backup))
        .route("/backups/:backup_id/verify", post(verify_backup))
        .route("/backups/cleanup", post(cleanup_backups))
}

async fn list_backups(State(state): State<AppState>, user: UserInfo) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    services::backup::list_backups(&state, &user).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_backup(State(state): State<AppState>, user: UserInfo, Json(req): Json<CreateBackupRequest>) -> Result<Json<serde_json::Value>, StatusCode> {
    services::backup::create_backup(&state, &user, &req.backup_type).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_backup(State(state): State<AppState>, user: UserInfo, Path(backup_id): Path<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({"id": backup_id})))
}

async fn delete_backup(State(state): State<AppState>, user: UserInfo, Path(backup_id): Path<String>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

async fn verify_backup(State(state): State<AppState>, user: UserInfo, Path(backup_id): Path<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({"valid": true})))
}

async fn cleanup_backups(State(state): State<AppState>, user: UserInfo) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}
