use crate::auth::UserInfo;
//! Backup API endpoints

use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post}, Json, Router};
use serde::Deserialize;
use crate::{auth::User, services::backup, AppState};

#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub backup_type: String,
    pub include_versions: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/backups", get(list_backups))
        .route("/backups/create", post(create_backup))
        .route("/backups/:backup_id", get(get_backup).delete(delete_backup))
        .route("/backups/:backup_id/verify", post(verify_backup))
        .route("/backups/cleanup", post(cleanup_backups))
}

async fn list_backups(State(state): State<AppState>, user: UserInfo) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    backup_service::list(&state, &user).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_backup(State(state): State<AppState>, user: UserInfo, Json(req): Json<CreateBackupRequest>) -> Result<Json<serde_json::Value>, StatusCode> {
    backup_service::create(&state, &user, req).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_backup(State(state): State<AppState>, user: UserInfo, Path(backup_id): Path<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    backup_service::get(&state, &user, &backup_id).await.map(Json).map_err(|_| StatusCode::NOT_FOUND)
}

async fn delete_backup(State(state): State<AppState>, user: UserInfo, Path(backup_id): Path<String>) -> Result<StatusCode, StatusCode> {
    backup_service::delete(&state, &user, &backup_id).await.map(|_| StatusCode::NO_CONTENT).map_err(|_| StatusCode::NOT_FOUND)
}

async fn verify_backup(State(state): State<AppState>, user: UserInfo, Path(backup_id): Path<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    backup_service::verify(&state, &user, &backup_id).await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn cleanup_backups(State(state): State<AppState>, user: UserInfo) -> Result<StatusCode, StatusCode> {
    backup_service::cleanup(&state, &user).await.map(|_| StatusCode::OK).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
