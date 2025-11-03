//! Notifications API endpoints
//! TODO: Reimplement using services layer

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use crate::{auth::User, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/notifications", get(get_notifications))
        .route("/notifications", delete(delete_all_notifications))
        .route("/notifications/{id}/read", post(mark_notification_read))
        .route("/notifications/read-all", post(mark_all_notifications_read))
        .route("/notifications/{id}", delete(delete_notification))
}

async fn get_notifications(_user: User, State(_state): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(vec![]))
}

async fn delete_all_notifications(_user: User, State(_state): State<AppState>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

async fn mark_notification_read(_user: User, State(_state): State<AppState>, Path(_id): Path<String>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

async fn mark_all_notifications_read(_user: User, State(_state): State<AppState>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

async fn delete_notification(_user: User, State(_state): State<AppState>, Path(_id): Path<String>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}
