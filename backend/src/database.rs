//! Database module for SyncSpace
//! 
//! Provides SQLite database connection and models for all tables.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const DB_FILENAME: &str = "syncspace.db";
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

    // Simple SQLite URL without protocol prefix - just the file path
    let db_url = format!("sqlite:{}", db_path.display());
    
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

/// Run SQL migrations from files
async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("🔄 Running database migrations...");

    // Check if migrations were already run by checking for a table
    let table_exists: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'"
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some((count,)) = table_exists {
        if count > 0 {
            println!("✅ Database already initialized, skipping migrations");
            return Ok(());
        }
    }

    // Read and execute migration file
    let migration_sql = include_str!("../migrations/001_initial_schema.sql");
    
    sqlx::query(migration_sql)
        .execute(pool)
        .await?;

    println!("✅ Migrations completed");
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
    
    path.push(DB_FILENAME);
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
