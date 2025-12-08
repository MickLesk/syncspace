//! Notifications API endpoints
//! Implements notification management using services layer

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::json;
use crate::{auth::UserInfo, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/notifications", get(get_notifications))
        .route("/notifications", delete(delete_all_notifications))
        .route("/notifications/{id}/read", post(mark_notification_read))
        .route("/notifications/read-all", post(mark_all_notifications_read))
        .route("/notifications/{id}", delete(delete_notification))
}

async fn get_notifications(user_info: UserInfo, State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    match sqlx::query_as::<_, (String, String, String, String, bool)>(
        "SELECT id, type, title, message, is_read FROM notifications WHERE user_id = ? ORDER BY created_at DESC LIMIT 100"
    )
    .bind(&user_info.id)
    .fetch_all(&state.db_pool)
    .await
    {
        Ok(notifications) => {
            let result: Vec<_> = notifications.into_iter()
                .map(|(id, notification_type, title, message, is_read)| {
                    json!({
                        "id": id,
                        "type": notification_type,
                        "title": title,
                        "message": message,
                        "read": is_read
                    })
                })
                .collect();
            Ok(Json(json!({ "notifications": result })))
        }
        Err(e) => {
            tracing::error!("Failed to fetch notifications: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_all_notifications(user_info: UserInfo, State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    match sqlx::query("DELETE FROM notifications WHERE user_id = ?")
        .bind(&user_info.id)
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to delete all notifications: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn mark_notification_read(user_info: UserInfo, State(state): State<AppState>, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
    match sqlx::query("UPDATE notifications SET is_read = 1 WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_info.id)
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to mark notification as read: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn mark_all_notifications_read(user_info: UserInfo, State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    match sqlx::query("UPDATE notifications SET is_read = 1 WHERE user_id = ?")
        .bind(&user_info.id)
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to mark all notifications as read: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_notification(user_info: UserInfo, State(state): State<AppState>, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
    match sqlx::query("DELETE FROM notifications WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_info.id)
        .execute(&state.db_pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to delete notification: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
