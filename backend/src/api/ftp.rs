//! FTP Sync API endpoints
//! Manages FTP connections and synchronization

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    auth::UserInfo,
    ftp_sync::{
        self, CreateFtpConnectionRequest, FtpConnection, FtpFileEntry, SyncResult,
        UpdateFtpConnectionRequest,
    },
    AppState,
};

/// List all FTP connections for user
async fn list_connections(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<FtpConnection>>, StatusCode> {
    ftp_sync::list_ftp_connections(&state.db_pool, &user.id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to list FTP connections: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Get single FTP connection
async fn get_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<Json<FtpConnection>, StatusCode> {
    ftp_sync::get_ftp_connection(&state.db_pool, &id, &user.id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to get FTP connection: {}", e);
            StatusCode::NOT_FOUND
        })
}

/// Create new FTP connection
async fn create_connection(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateFtpConnectionRequest>,
) -> Result<(StatusCode, Json<FtpConnection>), StatusCode> {
    ftp_sync::create_ftp_connection(&state.db_pool, &user.id, req)
        .await
        .map(|conn| (StatusCode::CREATED, Json(conn)))
        .map_err(|e| {
            tracing::error!("Failed to create FTP connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Update FTP connection
async fn update_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
    Json(req): Json<UpdateFtpConnectionRequest>,
) -> Result<Json<FtpConnection>, StatusCode> {
    ftp_sync::update_ftp_connection(&state.db_pool, &id, &user.id, req)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to update FTP connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Delete FTP connection
async fn delete_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    ftp_sync::delete_ftp_connection(&state.db_pool, &id, &user.id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| {
            tracing::error!("Failed to delete FTP connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Test FTP connection
async fn test_connection(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let connection = ftp_sync::get_ftp_connection(&state.db_pool, &id, &user.id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    match ftp_sync::test_ftp_connection(&connection).await {
        Ok(true) => Ok(Json(serde_json::json!({
            "success": true,
            "message": "Connection successful"
        }))),
        Ok(false) => Ok(Json(serde_json::json!({
            "success": false,
            "message": "Connection failed"
        }))),
        Err(e) => Ok(Json(serde_json::json!({
            "success": false,
            "message": format!("Connection error: {}", e)
        }))),
    }
}

/// List remote files
async fn list_remote_files(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<Json<Vec<FtpFileEntry>>, StatusCode> {
    let connection = ftp_sync::get_ftp_connection(&state.db_pool, &id, &user.id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    ftp_sync::list_remote_files(&connection, None)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to list remote files: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// Trigger sync for connection
async fn trigger_sync(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user: UserInfo,
) -> Result<Json<SyncResult>, StatusCode> {
    let connection = ftp_sync::get_ftp_connection(&state.db_pool, &id, &user.id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    ftp_sync::sync_ftp(&state.db_pool, &connection)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("FTP sync failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ftp/connections", get(list_connections).post(create_connection))
        .route(
            "/ftp/connections/{id}",
            get(get_connection).put(update_connection).delete(delete_connection),
        )
        .route("/ftp/connections/{id}/test", post(test_connection))
        .route("/ftp/connections/{id}/files", get(list_remote_files))
        .route("/ftp/connections/{id}/sync", post(trigger_sync))
}
