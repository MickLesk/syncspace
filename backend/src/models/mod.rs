//! Data models
//! Database entities and DTOs
//! Many models are part of the API and may not be actively constructed yet
#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ==================== USER MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub is_2fa_enabled: bool,
    pub totp_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSettings {
    pub user_id: Uuid,
    pub theme: String,
    pub language: String,
    pub default_view: String,
}

// ==================== FILE MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileInfo {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    #[serde(rename = "size_bytes")]
    pub size: i64,
    pub is_directory: bool,  // Changed from is_dir
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,  // Changed from Option<DateTime<Utc>>
    pub owner_id: Uuid,
    pub parent_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DirectoryInfo {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub parent_id: Option<Uuid>,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// ==================== SHARING MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Share {
    pub id: Uuid,
    pub file_path: String,
    pub token: String,
    pub permission: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub password_hash: Option<String>,
}

// ==================== ACTIVITY MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActivityLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

// ==================== TAG MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// File-Tag association model - part of tagging API (future implementation)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct FileTag {
    pub id: Uuid,
    pub file_path: String,
    pub tag_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// ==================== FAVORITE MODELS ====================

/// Favorite item model - part of favorites API (future implementation)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct Favorite {
    pub id: Uuid,
    pub user_id: Uuid,
    pub item_id: String,
    pub item_type: String,
    pub created_at: DateTime<Utc>,
}

// ==================== COLLABORATION MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileLock {
    pub id: Uuid,
    pub file_path: String,
    pub user_id: Uuid,
    pub lock_type: String,
    pub acquired_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPresence {
    pub user_id: Uuid,
    pub file_path: String,
    pub activity_type: String,
    pub last_seen: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

// ==================== BACKUP MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Backup {
    pub id: Uuid,
    pub backup_type: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub size_bytes: i64,
    pub status: String,
    pub file_path: String,
}
