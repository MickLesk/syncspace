//! Database module for SyncSpace
//! 
//! Provides SQLite database connection and models for all tables.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;
use std::path::PathBuf;
use uuid::Uuid;

const MAX_CONNECTIONS: u32 = 10;

/// Initialize database pool and run migrations
pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Get database path - always use ./data/syncspace.db relative to current working directory
    let db_path = PathBuf::from("./data/syncspace.db");
    
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("⚠️ Failed to create data directory: {}", e);
            return Err(sqlx::Error::Configuration(format!("Cannot create data directory: {}", e).into()));
        }
    }
    
    if !db_path.exists() {
        println!("📦 Creating new database: {}", db_path.display());
    } else {
        println!("🔄 Using existing database: {}", db_path.display());
    }

    // Convert to absolute path for better debugging
    let abs_path = std::fs::canonicalize(&db_path)
        .unwrap_or_else(|_| db_path.clone());
    
    println!("📂 Absolute path: {}", abs_path.display());

    // SQLite URL with create flag - this creates the file if it doesn't exist
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    println!("🔗 Connecting to database...");

    // Create connection pool with pragma settings for performance
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                // Enable WAL mode for better concurrency
                sqlx::query("PRAGMA journal_mode=WAL;").execute(&mut *conn).await?;
                // Enable foreign keys
                sqlx::query("PRAGMA foreign_keys=ON;").execute(&mut *conn).await?;
                // Optimize for performance
                sqlx::query("PRAGMA synchronous=NORMAL;").execute(&mut *conn).await?;
                sqlx::query("PRAGMA temp_store=MEMORY;").execute(&mut *conn).await?;
                sqlx::query("PRAGMA cache_size=-64000;").execute(&mut *conn).await?; // 64MB cache
                Ok(())
            })
        })
        .connect(&db_url)
        .await?;

    println!("✅ Database pool created");

    // Run migrations
    run_migrations(&pool).await?;
    
    // Ensure admin user exists
    ensure_admin_user(&pool).await?;

    Ok(pool)
}

/// Run SQL migrations from files - SIMPLIFIED VERSION for core tables only
async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("🔄 Running database migrations...");

    // Check if migrations were already run by checking for a table
    let table_exists: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'"
    )
    .fetch_optional(pool)
    .await?;
    
    let migrations_needed = if let Some((count,)) = table_exists {
        count == 0
    } else {
        true
    };

    if migrations_needed {
        // Run initial schema migration
        println!("📋 Running migration: 001_initial_schema.sql");
        let migration_sql = include_str!("../migrations/001_initial_schema.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 002_add_comments_tags.sql");
        let migration_sql = include_str!("../migrations/002_add_comments_tags.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 003_add_backups.sql");
        let migration_sql = include_str!("../migrations/003_add_backups.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 004_add_notifications_and_preferences.sql");
        let migration_sql = include_str!("../migrations/004_add_notifications_and_preferences.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 005_add_notifications.sql");
        let migration_sql = include_str!("../migrations/005_add_notifications.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 006_add_webhooks.sql");
        let migration_sql = include_str!("../migrations/006_add_webhooks.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 007_add_encryption.sql");
        let migration_sql = include_str!("../migrations/007_add_encryption.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 008_add_locking.sql");
        let migration_sql = include_str!("../migrations/008_add_locking.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 009_add_permissions.sql");
        let migration_sql = include_str!("../migrations/009_add_permissions.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 010_add_additional_features.sql");
        let migration_sql = include_str!("../migrations/010_add_additional_features.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 011_add_integration_features.sql");
        let migration_sql = include_str!("../migrations/011_add_integration_features.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 012_add_oauth.sql");
        let migration_sql = include_str!("../migrations/012_add_oauth.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 013_add_collaboration.sql");
        let migration_sql = include_str!("../migrations/013_add_collaboration.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 014_add_file_versioning.sql");
        let migration_sql = include_str!("../migrations/014_add_file_versioning.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 015_add_user_preferences.sql");
        let migration_sql = include_str!("../migrations/015_add_user_preferences.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        
        println!("📋 Running migration: 016_add_backup_scheduling.sql");
        let migration_sql = include_str!("../migrations/016_add_backup_scheduling.sql");
        sqlx::query(migration_sql).execute(pool).await?;
    }

    println!("✅ All migrations completed (001-016)");
    Ok(())
}


/// Ensure admin user exists (username: admin, password: admin)
/// This user is never overwritten and always available for initial access
async fn ensure_admin_user(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Check if admin user already exists
    let existing: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM users WHERE username = 'admin'"
    )
    .fetch_optional(pool)
    .await?;
    
    if existing.is_some() {
        println!("✅ Admin user already exists");
        return Ok(());
    }
    
    println!("👤 Creating default admin user (username: admin, password: admin)");
    
    // Hash password "admin" with argon2
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2
    };
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password("admin".as_bytes(), &salt)
        .map_err(|e| sqlx::Error::Configuration(format!("Password hashing failed: {}", e).into()))?
        .to_string();
    
    // Create admin user with UUID
    let admin_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    sqlx::query(
        "INSERT INTO users (
            id, username, password_hash, email, display_name,
            storage_quota_bytes, storage_used_bytes,
            default_view, language, theme,
            totp_enabled, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&admin_id)
    .bind("admin")
    .bind(&password_hash)
    .bind("admin@syncspace.local")
    .bind("Administrator")
    .bind(10737418240i64) // 10 GB default quota
    .bind(0i64)
    .bind("grid")
    .bind("de")
    .bind("light")
    .bind(false)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    
    println!("✅ Admin user created successfully");
    Ok(())
}

/// Get database path (in ./data directory like the file system)
fn get_db_path() -> PathBuf {
    // Try to use DATA_DIR constant from main if available, otherwise fallback to ./data
    let mut path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    
    // Go up to backend root if we're in target/debug or target/release
    if path.ends_with("debug") || path.ends_with("release") {
        path.pop(); // Remove debug/release
        path.pop(); // Remove target
    }
    
    path.push("data");
    
    // Create data directory if it doesn't exist
    if !path.exists() {
        std::fs::create_dir_all(&path).ok();
    }
    
    path.push("syncspace.db");
    path
}

// ==================== MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_base64: Option<String>,
    
    // Security
    pub totp_secret: Option<String>,
    pub totp_enabled: bool,
    
    // Quota
    pub storage_quota_bytes: i64,
    pub storage_used_bytes: i64,
    
    // Preferences
    pub default_view: String,
    pub language: String,
    pub theme: String,
    
    // Timestamps
    pub created_at: String,
    pub last_login: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub path: String,
    pub parent_id: Option<String>,
    pub owner_id: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<String>,
    pub deleted_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: String,
    pub name: String,
    pub path: String,
    pub folder_id: Option<String>,
    pub owner_id: String,
    pub size_bytes: i64,
    pub mime_type: Option<String>,
    pub checksum_sha256: Option<String>,
    pub storage_path: String,
    pub is_encrypted: bool,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<String>,
    pub deleted_by: Option<String>,
    pub version: i32,
    pub previous_version_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_accessed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileHistory {
    pub id: String,
    pub file_id: String,
    pub file_path: String,
    pub user_id: String,
    pub action: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TrashItem {
    pub id: String,
    pub item_type: String,
    pub item_id: String,
    pub original_path: String,
    pub original_name: String,
    pub original_parent_id: Option<String>,
    pub deleted_by: String,
    pub deleted_at: String,
    pub auto_delete_at: Option<String>,
    pub size_bytes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Favorite {
    pub id: String,
    pub user_id: String,
    pub item_type: String,
    pub item_id: String,
    pub sort_order: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SharedLink {
    pub id: String,
    pub item_type: String,
    pub item_id: String,
    pub created_by: String,
    pub password_hash: Option<String>,
    pub is_public: bool,
    pub allow_download: bool,
    pub allow_upload: bool,
    pub expires_at: Option<String>,
    pub max_downloads: Option<i32>,
    pub download_count: i32,
    pub created_at: String,
    pub last_accessed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreferences {
    pub id: String,
    pub user_id: String,
    pub view_mode: String,
    pub sort_by: String,
    pub sort_order: String,
    pub items_per_page: i32,
    pub sidebar_collapsed: bool,
    pub show_hidden_files: bool,
    pub recent_searches: Option<String>,
    pub search_filters: Option<String>,
    pub notification_email: bool,
    pub notification_browser: bool,
    pub notification_sound: bool,
    pub activity_visible: bool,
    pub show_online_status: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileVersionNew {
    pub id: String,
    pub file_id: String,
    pub version_number: i32,
    pub size_bytes: i64,
    pub checksum_sha256: String,
    pub storage_path: String,
    pub created_by: String,
    pub change_description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecentFile {
    pub id: String,
    pub user_id: String,
    pub file_id: String,
    pub access_count: i32,
    pub last_accessed_at: String,
    pub access_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FilePermission {
    pub id: String,
    pub item_type: String,
    pub item_id: String,
    pub user_id: String,
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
    pub can_share: bool,
    pub granted_by: String,
    pub granted_at: String,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileThumbnail {
    pub id: String,
    pub file_id: String,
    pub thumbnail_path: String,
    pub thumbnail_size_bytes: i32,
    pub width: i32,
    pub height: i32,
    pub format: String,
    pub generation_status: String,
    pub generation_error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SharedLinkAccessLog {
    pub id: String,
    pub shared_link_id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub user_id: Option<String>,
    pub action: String,
    pub accessed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: String,
    pub item_type: String,
    pub item_id: String,
    pub file_path: String,
    pub author_id: String,
    pub text: String,
    pub is_resolved: bool,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
    pub edited_at: Option<String>,
    pub edited_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub owner_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileTag {
    pub id: String,
    pub file_id: String,
    pub tag_id: String,
    pub item_type: String,
    pub file_path: String,
    pub tagged_by: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileVersion {
    pub id: String,
    pub file_id: String,
    pub version_number: i32,
    pub content_hash: String,
    pub size_bytes: i64,
    pub created_at: String,
    pub created_by: String,
    pub comment: Option<String>,
    pub is_current: bool,
    pub storage_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VersionMetadata {
    pub id: String,
    pub version_id: String,
    pub metadata_key: String,
    pub metadata_value: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VersionDiff {
    pub id: String,
    pub from_version_id: String,
    pub to_version_id: String,
    pub diff_type: String,
    pub diff_content: Option<String>,
    pub added_lines: i32,
    pub removed_lines: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VersionRestoration {
    pub id: String,
    pub file_id: String,
    pub restored_from_version_id: String,
    pub restored_to_version_id: String,
    pub restored_by: String,
    pub restored_at: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub value_type: String,
    pub description: Option<String>,
    pub category: String,
    pub updated_at: String,
    pub updated_by: Option<String>,
}

// ==================== DATABASE OPERATIONS ====================

impl User {
    /// Get user by ID
    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Get user by username
    pub async fn get_by_username(pool: &SqlitePool, username: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await
    }

    /// Create new user
    pub async fn create(
        pool: &SqlitePool,
        username: String,
        password_hash: String,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO users (id, username, password_hash, totp_enabled, storage_quota_bytes, storage_used_bytes, default_view, language, theme, created_at, updated_at)
             VALUES (?, ?, ?, 0, 10737418240, 0, 'grid', 'de', 'light', ?, ?)"
        )
        .bind(&id)
        .bind(&username)
        .bind(&password_hash)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        Self::get_by_id(pool, &id).await.map(|u| u.unwrap())
    }

    /// Update last login timestamp
    pub async fn update_last_login(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE users SET last_login = ?, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(&now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

// ============================================================================
// Collaboration Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileLock {
    pub id: String,
    pub file_id: String,
    pub file_path: String,
    pub locked_by: String,
    pub locked_at: String,
    pub expires_at: String,
    pub lock_type: String,
    pub last_heartbeat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPresence {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub file_path: Option<String>,
    pub activity_type: String,
    pub last_seen: String,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollaborationActivity {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub file_path: String,
    pub activity_type: String,
    pub details: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EditConflict {
    pub id: String,
    pub file_id: String,
    pub file_path: String,
    pub conflict_type: String,
    pub user1_id: String,
    pub user2_id: Option<String>,
    pub detected_at: String,
    pub resolved_at: Option<String>,
    pub resolution_strategy: Option<String>,
    pub status: String,
    pub details: Option<String>,
}

impl FileLock {
    /// Check if lock is still valid
    pub fn is_valid(&self) -> bool {
        if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(&self.expires_at) {
            expires > chrono::Utc::now()
        } else {
            false
        }
    }
}

impl FileHistory {
    /// Log a file action
    pub async fn log_action(
        pool: &SqlitePool,
        file_id: &str,
        user_id: &str,
        action: &str,
        old_value: Option<String>,
        new_value: Option<String>,
    ) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO file_history (id, file_id, user_id, action, old_value, new_value, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(file_id)
        .bind(user_id)
        .bind(action)
        .bind(old_value)
        .bind(new_value)
        .bind(&now)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get history for a file
    pub async fn get_for_file(pool: &SqlitePool, file_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, FileHistory>(
            "SELECT * FROM file_history WHERE file_id = ? ORDER BY created_at DESC"
        )
        .bind(file_id)
        .fetch_all(pool)
        .await
    }
}

impl Setting {
    /// Get setting value
    pub async fn get(pool: &SqlitePool, key: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Setting>("SELECT * FROM settings WHERE key = ?")
            .bind(key)
            .fetch_optional(pool)
            .await
    }

    /// Get boolean setting
    pub async fn get_bool(pool: &SqlitePool, key: &str) -> Result<bool, sqlx::Error> {
        let setting = Self::get(pool, key).await?;
        Ok(setting.map(|s| s.value == "true").unwrap_or(false))
    }

    /// Get integer setting
    pub async fn get_int(pool: &SqlitePool, key: &str) -> Result<i64, sqlx::Error> {
        let setting = Self::get(pool, key).await?;
        Ok(setting.and_then(|s| s.value.parse().ok()).unwrap_or(0))
    }

    /// Update setting
    pub async fn set(
        pool: &SqlitePool,
        key: &str,
        value: &str,
        updated_by: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE settings SET value = ?, updated_at = ?, updated_by = ? WHERE key = ?"
        )
        .bind(value)
        .bind(&now)
        .bind(updated_by)
        .bind(key)
        .execute(pool)
        .await?;
        Ok(())
    }
}

// ==================== BACKUP SCHEDULING MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BackupSchedule {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cron_expression: String,
    pub backup_type: String,
    pub enabled: i64,
    pub destination_type: String,
    pub destination_config: Option<String>,
    pub retention_days: Option<i64>,
    pub created_by: String,
    pub created_at: String,
    pub last_run_at: Option<String>,
    pub next_run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BackupVerification {
    pub id: String,
    pub backup_id: String,
    pub verification_type: String,
    pub status: String,
    pub details: Option<String>,
    pub verified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BackupFile {
    pub id: String,
    pub backup_id: String,
    pub file_path: String,
    pub file_size: i64,
    pub checksum: String,
    pub compressed_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EnhancedBackup {
    pub id: String,
    pub backup_type: String,
    pub size_bytes: i64,
    pub file_count: Option<i64>,
    pub storage_path: String,
    pub created_by: String,
    pub created_at: String,
    pub status: String,
    pub error_message: Option<String>,
    pub checksum: Option<String>,
    pub compression_type: Option<String>,
    pub schedule_id: Option<String>,
    pub destination_type: Option<String>,
    pub metadata: Option<String>,
}
