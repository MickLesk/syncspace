/// Notification System for real-time user notifications
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub notification_type: String, // "file_shared", "comment_added", "quota_warning", "backup_complete", etc.
    pub title: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>, // Deep link to relevant resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>, // Icon name for UI
    pub is_read: bool,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>, // JSON metadata
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNotificationRequest {
    pub user_id: String,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
    pub icon: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Create a new notification
pub async fn create_notification(
    pool: &SqlitePool,
    req: CreateNotificationRequest,
) -> Result<Notification, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let metadata_str = req.metadata.map(|m| m.to_string());
    
    sqlx::query(
        "INSERT INTO notifications (id, user_id, notification_type, title, message, link, icon, is_read, metadata, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(&req.user_id)
    .bind(&req.notification_type)
    .bind(&req.title)
    .bind(&req.message)
    .bind(&req.link)
    .bind(&req.icon)
    .bind(&metadata_str)
    .execute(pool)
    .await?;
    
    // Fetch the created notification
    let notification = sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool)
    .await?;
    
    Ok(notification)
}

/// Get all notifications for a user
pub async fn get_user_notifications(
    pool: &SqlitePool,
    user_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<Notification>, sqlx::Error> {
    sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications 
         WHERE user_id = ? 
         ORDER BY created_at DESC 
         LIMIT ? OFFSET ?"
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

/// Get unread notifications count
pub async fn get_unread_count(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<i64, sqlx::Error> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications WHERE user_id = ? AND is_read = 0"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    
    Ok(count.0)
}

/// Mark notification as read
pub async fn mark_as_read(
    pool: &SqlitePool,
    notification_id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE notifications SET is_read = 1 WHERE id = ? AND user_id = ?"
    )
    .bind(notification_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Mark all notifications as read for user
pub async fn mark_all_as_read(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE notifications SET is_read = 1 WHERE user_id = ? AND is_read = 0"
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Delete notification
pub async fn delete_notification(
    pool: &SqlitePool,
    notification_id: &str,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM notifications WHERE id = ? AND user_id = ?"
    )
    .bind(notification_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Delete all read notifications for user
pub async fn delete_read_notifications(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM notifications WHERE user_id = ? AND is_read = 1"
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected())
}

/// Helper functions for common notification types

pub async fn notify_file_shared(
    pool: &SqlitePool,
    user_id: &str,
    sharer_name: &str,
    file_name: &str,
    file_id: &str,
) -> Result<(), sqlx::Error> {
    create_notification(pool, CreateNotificationRequest {
        user_id: user_id.to_string(),
        notification_type: "file_shared".to_string(),
        title: format!("{} hat eine Datei geteilt", sharer_name),
        message: format!("\"{}\" wurde mit dir geteilt", file_name),
        link: Some(format!("/files/{}", file_id)),
        icon: Some("share".to_string()),
        metadata: None,
    }).await?;
    
    Ok(())
}

pub async fn notify_comment_added(
    pool: &SqlitePool,
    user_id: &str,
    commenter_name: &str,
    file_name: &str,
    file_id: &str,
) -> Result<(), sqlx::Error> {
    create_notification(pool, CreateNotificationRequest {
        user_id: user_id.to_string(),
        notification_type: "comment_added".to_string(),
        title: format!("{} hat kommentiert", commenter_name),
        message: format!("Neuer Kommentar bei \"{}\"", file_name),
        link: Some(format!("/files/{}", file_id)),
        icon: Some("message".to_string()),
        metadata: None,
    }).await?;
    
    Ok(())
}

pub async fn notify_quota_warning(
    pool: &SqlitePool,
    user_id: &str,
    percentage: f64,
) -> Result<(), sqlx::Error> {
    create_notification(pool, CreateNotificationRequest {
        user_id: user_id.to_string(),
        notification_type: "quota_warning".to_string(),
        title: "Speicherplatz wird knapp".to_string(),
        message: format!("Du hast {}% deines Speicherplatzes verwendet", percentage),
        link: Some("/settings/storage".to_string()),
        icon: Some("alert".to_string()),
        metadata: None,
    }).await?;
    
    Ok(())
}

pub async fn notify_backup_complete(
    pool: &SqlitePool,
    user_id: &str,
    backup_id: &str,
    backup_size: i64,
) -> Result<(), sqlx::Error> {
    let size_mb = backup_size / (1024 * 1024);
    create_notification(pool, CreateNotificationRequest {
        user_id: user_id.to_string(),
        notification_type: "backup_complete".to_string(),
        title: "Backup abgeschlossen".to_string(),
        message: format!("Backup erfolgreich erstellt ({} MB)", size_mb),
        link: Some(format!("/backups/{}", backup_id)),
        icon: Some("check".to_string()),
        metadata: None,
    }).await?;
    
    Ok(())
}
