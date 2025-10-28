//! Notification Handlers
//!
//! Implements endpoints for:
//! - GET /api/notifications - Get user notifications
//! - POST /api/notifications/:id/read - Mark notification as read
//! - POST /api/notifications/read-all - Mark all as read
//! - DELETE /api/notifications/:id - Delete notification

use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

// ==================== MODELS ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub action_url: Option<String>,
    pub action_label: Option<String>,
    pub is_read: bool,
    pub read_at: Option<String>,
    pub priority: String,
    pub related_file_id: Option<String>,
    pub related_user_id: Option<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NotificationList {
    pub notifications: Vec<Notification>,
    pub unread_count: i64,
    pub total_count: i64,
}

// ==================== HANDLERS ====================

/// GET /api/notifications - Get user notifications
pub async fn get_notifications(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    // Fetch notifications
    let rows = sqlx::query(
        r#"
        SELECT 
            id,
            user_id,
            type,
            title,
            message,
            action_url,
            action_label,
            is_read,
            read_at,
            priority,
            related_file_id,
            related_user_id,
            created_at,
            expires_at
        FROM notifications
        WHERE user_id = ?
        AND (expires_at IS NULL OR expires_at > datetime('now'))
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .bind(&user_info.id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch notifications: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let notifications: Vec<Notification> = rows
        .into_iter()
        .map(|row| Notification {
            id: row.get("id").unwrap_or_default(),
            user_id: row.get("user_id").unwrap_or_default(),
            notification_type: row.get("type").unwrap_or_else(|_| "info".to_string()),
            title: row.get("title").unwrap_or_default(),
            message: row.get("message").unwrap_or_default(),
            action_url: row.get("action_url").ok(),
            action_label: row.get("action_label").ok(),
            is_read: row.try_get::<i64, _>("is_read").unwrap_or(0) != 0,
            read_at: row.get("read_at").ok(),
            priority: row.get("priority").unwrap_or_else(|_| "normal".to_string()),
            related_file_id: row.get("related_file_id").ok(),
            related_user_id: row.get("related_user_id").ok(),
            created_at: row.get("created_at").unwrap_or_default(),
            expires_at: row.get("expires_at").ok(),
        })
        .collect();

    // Get unread count
    let unread_count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM notifications
        WHERE user_id = ? AND is_read = 0
        AND (expires_at IS NULL OR expires_at > datetime('now'))
        "#
    )
    .bind(&user_info.id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_count = notifications.len() as i64;

    Ok(Json(NotificationList {
        notifications,
        unread_count: unread_count.0,
        total_count,
    }))
}

/// POST /api/notifications/:id/read - Mark notification as read
pub async fn mark_notification_read(
    State(state): State<AppState>,
    user_info: UserInfo,
    Path(notification_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();

    let result = sqlx::query(
        "UPDATE notifications SET is_read = 1, read_at = ? WHERE id = ? AND user_id = ?"
    )
    .bind(&now)
    .bind(&notification_id)
    .bind(&user_info.id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

/// POST /api/notifications/read-all - Mark all as read
pub async fn mark_all_notifications_read(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE notifications SET is_read = 1, read_at = ? WHERE user_id = ? AND is_read = 0"
    )
    .bind(&now)
    .bind(&user_info.id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

/// DELETE /api/notifications/:id - Delete notification
pub async fn delete_notification(
    State(state): State<AppState>,
    user_info: UserInfo,
    Path(notification_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM notifications WHERE id = ? AND user_id = ?"
    )
    .bind(&notification_id)
    .bind(&user_info.id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

/// DELETE /api/notifications - Delete all notifications
pub async fn delete_all_notifications(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query(
        "DELETE FROM notifications WHERE user_id = ?"
    )
    .bind(&user_info.id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// ==================== HELPER FUNCTIONS ====================

/// Create a notification for a user
pub async fn create_notification(
    pool: &SqlitePool,
    user_id: &str,
    notification_type: &str,
    title: &str,
    message: &str,
    priority: &str,
    action_url: Option<&str>,
    action_label: Option<&str>,
    related_file_id: Option<&str>,
    related_user_id: Option<&str>,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO notifications (
            id, user_id, type, title, message,
            action_url, action_label, is_read, priority,
            related_file_id, related_user_id, created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(user_id)
    .bind(notification_type)
    .bind(title)
    .bind(message)
    .bind(action_url)
    .bind(action_label)
    .bind(priority)
    .bind(related_file_id)
    .bind(related_user_id)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(id)
}


