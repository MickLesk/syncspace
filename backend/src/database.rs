//! Database module for SyncSpace
//!
//! Provides SQLite database connection and models for all tables.
//! Many models are part of the database API and may not be actively used yet.
#![allow(dead_code)]

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

// Connection pool settings
const MAX_CONNECTIONS: u32 = 20; // Increased from 10 for better concurrency
const MIN_CONNECTIONS: u32 = 2; // Keep minimum connections alive
const ACQUIRE_TIMEOUT_SECS: u64 = 30; // Timeout when acquiring connection from pool
const IDLE_TIMEOUT_SECS: u64 = 600; // Close idle connections after 10 minutes
const MAX_LIFETIME_SECS: u64 = 3600; // Recycle connections after 1 hour

/// Macro to automatically discover and include all migration files at compile time
/// Simply add a new .sql file to migrations/ and it will be automatically picked up!
/// The build.rs script generates this list automatically.
macro_rules! discover_migrations {
    () => {{
        // This is generated at compile-time by build.rs
        include!(concat!(env!("OUT_DIR"), "/migrations.rs"))
    }};
}

/// Initialize database pool and run migrations
pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Get database path - always use ./data/syncspace.db relative to current working directory
    let db_path = PathBuf::from("./data/syncspace.db");

    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("⚠️ Failed to create data directory: {}", e);
            return Err(sqlx::Error::Configuration(
                format!("Cannot create data directory: {}", e).into(),
            ));
        }
    }

    if !db_path.exists() {
        println!("📦 Creating new database: {}", db_path.display());
    } else {
        println!("🔄 Using existing database: {}", db_path.display());
    }

    // Convert to absolute path for better debugging
    let abs_path = std::fs::canonicalize(&db_path).unwrap_or_else(|_| db_path.clone());

    println!("📂 Absolute path: {}", abs_path.display());

    // SQLite URL with create flag - this creates the file if it doesn't exist
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    println!("🔗 Connecting to database...");

    // Create optimized connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .min_connections(MIN_CONNECTIONS)
        .acquire_timeout(Duration::from_secs(ACQUIRE_TIMEOUT_SECS))
        .idle_timeout(Some(Duration::from_secs(IDLE_TIMEOUT_SECS)))
        .max_lifetime(Some(Duration::from_secs(MAX_LIFETIME_SECS)))
        .test_before_acquire(true) // Test connection health before use
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                // Enable WAL mode for better concurrency
                sqlx::query("PRAGMA journal_mode=WAL;")
                    .execute(&mut *conn)
                    .await?;

                // TEMPORARILY DISABLE foreign keys due to InMemory vs SQLite user_id mismatch
                sqlx::query("PRAGMA foreign_keys=OFF;")
                    .execute(&mut *conn)
                    .await?;

                // Performance optimizations
                sqlx::query("PRAGMA synchronous=NORMAL;")
                    .execute(&mut *conn)
                    .await?; // Faster than FULL, still safe with WAL
                sqlx::query("PRAGMA temp_store=MEMORY;")
                    .execute(&mut *conn)
                    .await?; // Store temp tables in memory
                sqlx::query("PRAGMA cache_size=-64000;")
                    .execute(&mut *conn)
                    .await?; // 64MB cache (negative = KB)
                sqlx::query("PRAGMA mmap_size=268435456;")
                    .execute(&mut *conn)
                    .await?; // 256MB memory-mapped I/O
                sqlx::query("PRAGMA page_size=4096;")
                    .execute(&mut *conn)
                    .await?; // Optimal page size

                // WAL checkpointing settings
                sqlx::query("PRAGMA wal_autocheckpoint=1000;")
                    .execute(&mut *conn)
                    .await?; // Checkpoint every 1000 pages
                sqlx::query("PRAGMA busy_timeout=5000;")
                    .execute(&mut *conn)
                    .await?; // 5 second busy timeout

                println!("✅ Connection configured with optimized settings");
                Ok(())
            })
        })
        .connect(&db_url)
        .await?;

    println!(
        "✅ Database pool created (max: {}, min: {}, timeout: {}s)",
        MAX_CONNECTIONS, MIN_CONNECTIONS, ACQUIRE_TIMEOUT_SECS
    );

    // Run migrations
    run_migrations(&pool).await?;

    // Ensure admin user exists
    ensure_admin_user(&pool).await?;

    Ok(pool)
}

/// Helper function to execute multi-statement SQL migrations
async fn execute_migration_statements(
    pool: &SqlitePool,
    migration_sql: &str,
) -> Result<(), sqlx::Error> {
    // Remove SQL comments (both -- line comments and inline comments)
    // and join lines into a cleaned SQL string
    let cleaned_sql: String = migration_sql
        .lines()
        .map(|line| {
            // Remove inline comments (everything after --)
            if let Some(pos) = line.find("--") {
                &line[..pos]
            } else {
                line
            }
        })
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    // Split into individual statements by semicolon
    let statements: Vec<String> = cleaned_sql
        .split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Start a transaction for all statements in this migration
    let mut tx = pool.begin().await?;

    // Execute each statement individually within the transaction
    for (i, statement) in statements.iter().enumerate() {
        println!(
            "  ↳ Executing statement {}/{}: {}...",
            i + 1,
            statements.len(),
            if statement.len() > 60 {
                &statement[..60]
            } else {
                statement
            }
        );

        if let Err(e) = sqlx::query(statement).execute(&mut *tx).await {
            // Check if error is "duplicate column" or "already exists" - these are safe to ignore
            let error_msg = format!("{:?}", e);
            let is_safe_error = error_msg.contains("duplicate column")
                || error_msg.contains("already exists")
                || error_msg.contains("UNIQUE constraint failed");

            if is_safe_error {
                println!(
                    "  ⚠️  Statement already applied (safe to skip): {}",
                    if statement.len() > 50 {
                        &statement[..50]
                    } else {
                        statement
                    }
                );
            } else {
                eprintln!("❌ Failed to execute statement {}:", i + 1);
                eprintln!("   SQL: {}", statement);
                eprintln!("   Error: {:?}", e);
                return Err(e);
            }
        }
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(())
}

/// Run SQL migrations from files - DYNAMICALLY discovers and runs all migrations
async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("🔄 Running database migrations...");

    // Create migrations_tracker table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS migrations_tracker (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            migration_file TEXT NOT NULL UNIQUE,
            executed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await?;

    // Check if this is an existing database (check for users table)
    let is_existing_db: Option<(i64,)> =
        sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'")
            .fetch_optional(pool)
            .await?;

    let db_exists = if let Some((count,)) = is_existing_db {
        count > 0
    } else {
        false
    };

    // If database exists but migrations_tracker is empty, mark all existing migrations as executed
    // This prevents re-running migrations on existing databases
    if db_exists {
        let tracker_count: Option<(i64,)> =
            sqlx::query_as("SELECT COUNT(*) FROM migrations_tracker")
                .fetch_optional(pool)
                .await?;

        if let Some((count,)) = tracker_count {
            if count == 0 {
                println!("📌 Existing database detected. Marking all discovered migrations as executed...");

                // Get all migrations from auto-discovery
                let all_migrations = discover_migrations!();

                for (filename, _) in all_migrations {
                    sqlx::query(
                        "INSERT OR IGNORE INTO migrations_tracker (migration_file) VALUES (?)",
                    )
                    .bind(filename)
                    .execute(pool)
                    .await?;
                }
                println!("✅ Historical migrations marked");
            }
        }
    }

    // AUTO-DISCOVER all migrations at compile time using macro
    // Simply add new .sql files to migrations/ directory and they'll be picked up automatically!
    let migrations = discover_migrations!();

    let mut executed_count = 0;
    let mut skipped_count = 0;

    for (filename, migration_sql) in migrations {
        // Check if this migration was already executed
        let already_run: Option<(i64,)> =
            sqlx::query_as("SELECT COUNT(*) FROM migrations_tracker WHERE migration_file = ?")
                .bind(filename)
                .fetch_optional(pool)
                .await?;

        if let Some((count,)) = already_run {
            if count > 0 {
                skipped_count += 1;
                continue; // Skip already executed migrations
            }
        }

        // Execute the migration
        println!("📋 Running migration: {}", filename);
        execute_migration_statements(pool, migration_sql).await?;

        // Record successful execution
        sqlx::query("INSERT INTO migrations_tracker (migration_file) VALUES (?)")
            .bind(filename)
            .execute(pool)
            .await?;

        executed_count += 1;
    }

    if executed_count > 0 {
        println!(
            "✅ Executed {} new migrations successfully!",
            executed_count
        );
    }
    if skipped_count > 0 {
        println!("⏭️  Skipped {} already executed migrations", skipped_count);
    }

    Ok(())
}

/// Ensure admin user exists (username: admin, password: admin)
/// ONLY creates default admin if setup has NOT been completed
/// This provides backwards compatibility for existing installations
async fn ensure_admin_user(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Check if setup has been completed
    let setup_completed: Option<(bool,)> =
        sqlx::query_as("SELECT setup_completed FROM system_settings WHERE id = 1")
            .fetch_optional(pool)
            .await?;

    if setup_completed.map(|(c,)| c).unwrap_or(false) {
        println!("✅ Setup completed - skipping default admin creation");
        return Ok(());
    }

    // Check if admin user already exists
    let existing: Option<(String,)> =
        sqlx::query_as("SELECT id FROM users WHERE username = 'admin'")
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
        Argon2,
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
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
#[allow(dead_code)]
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
    pub bio: Option<String>,
    pub avatar_base64: Option<String>,

    // Security
    pub totp_secret: Option<String>,
    pub totp_enabled: bool,
    pub token_version: i32, // For global token invalidation

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

/// Refresh token for JWT authentication with rotation and revocation support
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: String,                   // UUID for token identifier
    pub user_id: String,              // Foreign key to users table
    pub token_hash: String,           // SHA256 hash of refresh token
    pub token_version: i32,           // Version number for token rotation
    pub expires_at: String,           // ISO 8601 timestamp
    pub created_at: String,           // ISO 8601 timestamp
    pub last_used_at: Option<String>, // ISO 8601 timestamp
    pub revoked_at: Option<String>,   // ISO 8601 timestamp (NULL if active)
    pub user_agent: Option<String>,   // User agent for security tracking
    pub ip_address: Option<String>,   // IP address for security tracking
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
    pub token_version: i32,
    pub regenerated_at: Option<String>,
    pub regenerated_by: Option<String>,
    pub allow_external: Option<bool>, // New: Toggle external sharing
    pub share_type: Option<String>,   // New: 'public', 'user', 'mixed'
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ShareUser {
    pub id: String,
    pub share_id: String,
    pub user_id: String,
    pub permission: String, // 'read', 'write', 'admin'
    pub created_at: String,
    pub created_by: String,
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
pub struct UserFavorite {
    pub id: String,
    pub user_id: String,
    pub item_id: String,
    pub item_type: String, // 'file' or 'folder'
    pub item_path: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteItem {
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub item_path: String,
    pub name: String,
    pub size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub is_directory: bool,
    pub created_at: String,
    pub favorited_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommentReaction {
    pub id: String,
    pub comment_id: String,
    pub user_id: String,
    pub emoji: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCommentWithDetails {
    pub id: String,
    pub file_path: String,
    pub user_id: String,
    pub user_display_name: String,
    pub user_avatar_base64: Option<String>,
    pub parent_comment_id: Option<String>,
    pub content: String,
    pub mentions: Vec<String>, // Parsed from JSON
    pub reactions: Vec<CommentReactionSummary>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub is_edited: bool,
    pub reply_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReactionSummary {
    pub emoji: String,
    pub count: i32,
    pub users: Vec<String>, // Display names of users who reacted
    pub current_user_reacted: bool,
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
    // New fields from migration 034
    pub parent_comment_id: Option<String>, // Threading support
    pub mentions: Option<String>, // JSON array of user_ids
    pub is_deleted: i32, // Soft delete flag
    pub deleted_at: Option<String>, // Soft delete timestamp
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
    pub async fn get_by_username(
        pool: &SqlitePool,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
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
            "SELECT * FROM file_history WHERE file_id = ? ORDER BY created_at DESC",
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
        sqlx::query("UPDATE settings SET value = ?, updated_at = ?, updated_by = ? WHERE key = ?")
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

// ==================== AUTHENTICATION SECURITY MODELS ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoginAttempt {
    pub id: String,
    pub username: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub success: i64, // 0=failed, 1=success
    pub failure_reason: Option<String>,
    pub attempted_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    pub id: String,
    pub user_id: String,
    pub session_token: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub created_at: String,
    pub last_active_at: String,
    pub expires_at: String,
    pub revoked: i64, // 0=active, 1=revoked
    pub revoked_at: Option<String>,
    pub revoked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountLockout {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub locked_at: String,
    pub locked_until: String,
    pub reason: String,
    pub failed_attempts_count: i64,
    pub unlocked_at: Option<String>,
    pub unlocked_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordHistory {
    pub id: String,
    pub user_id: String,
    pub password_hash: String,
    pub changed_at: String,
}

impl LoginAttempt {
    /// Log a login attempt
    pub async fn create(
        pool: &SqlitePool,
        username: &str,
        ip_address: &str,
        user_agent: Option<&str>,
        success: bool,
        failure_reason: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query_as::<_, LoginAttempt>(
            "INSERT INTO login_attempts (id, username, ip_address, user_agent, success, failure_reason, attempted_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING *"
        )
        .bind(&id)
        .bind(username)
        .bind(ip_address)
        .bind(user_agent)
        .bind(if success { 1 } else { 0 })
        .bind(failure_reason)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await
    }

    /// Get recent failed attempts for a username
    pub async fn get_recent_failures(
        pool: &SqlitePool,
        username: &str,
        minutes: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let cutoff = Utc::now() - chrono::Duration::minutes(minutes);
        sqlx::query_as::<_, LoginAttempt>(
            "SELECT * FROM login_attempts 
             WHERE username = ? AND success = 0 AND attempted_at >= ?
             ORDER BY attempted_at DESC",
        )
        .bind(username)
        .bind(cutoff.to_rfc3339())
        .fetch_all(pool)
        .await
    }
}

impl UserSession {
    /// Create a new session
    pub async fn create(
        pool: &SqlitePool,
        user_id: &str,
        session_token: &str,
        ip_address: &str,
        user_agent: Option<&str>,
        expires_at: chrono::DateTime<Utc>,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query_as::<_, UserSession>(
            "INSERT INTO user_sessions (id, user_id, session_token, ip_address, user_agent, created_at, last_active_at, expires_at, revoked)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0) RETURNING *"
        )
        .bind(&id)
        .bind(user_id)
        .bind(session_token)
        .bind(ip_address)
        .bind(user_agent)
        .bind(&now)
        .bind(&now)
        .bind(expires_at.to_rfc3339())
        .fetch_one(pool)
        .await
    }

    /// Get active sessions for a user
    pub async fn get_user_sessions(
        pool: &SqlitePool,
        user_id: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, UserSession>(
            "SELECT * FROM user_sessions 
             WHERE user_id = ? AND revoked = 0 AND expires_at > datetime('now')
             ORDER BY last_active_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Revoke a session
    pub async fn revoke(
        pool: &SqlitePool,
        session_id: &str,
        reason: &str,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE user_sessions SET revoked = 1, revoked_at = ?, revoked_reason = ?
             WHERE id = ?",
        )
        .bind(&now)
        .bind(reason)
        .bind(session_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Update last active timestamp
    pub async fn update_activity(
        pool: &SqlitePool,
        session_token: &str,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE user_sessions SET last_active_at = ? WHERE session_token = ?")
            .bind(&now)
            .bind(session_token)
            .execute(pool)
            .await?;
        Ok(())
    }
}

impl AccountLockout {
    /// Create an account lockout
    pub async fn create(
        pool: &SqlitePool,
        user_id: &str,
        username: &str,
        failed_attempts: i64,
        lock_duration_minutes: i64,
    ) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let locked_until = now + chrono::Duration::minutes(lock_duration_minutes);

        sqlx::query_as::<_, AccountLockout>(
            "INSERT INTO account_lockouts (id, user_id, username, locked_at, locked_until, reason, failed_attempts_count)
             VALUES (?, ?, ?, ?, ?, 'too_many_failed_attempts', ?) RETURNING *"
        )
        .bind(&id)
        .bind(user_id)
        .bind(username)
        .bind(now.to_rfc3339())
        .bind(locked_until.to_rfc3339())
        .bind(failed_attempts)
        .fetch_one(pool)
        .await
    }

    /// Check if user is currently locked
    pub async fn is_locked(pool: &SqlitePool, username: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, AccountLockout>(
            "SELECT * FROM account_lockouts 
             WHERE username = ? AND locked_until > datetime('now') AND unlocked_at IS NULL
             ORDER BY locked_at DESC LIMIT 1",
        )
        .bind(username)
        .fetch_optional(pool)
        .await
    }

    /// Unlock an account (admin action)
    pub async fn unlock(
        pool: &SqlitePool,
        lockout_id: &str,
        unlocked_by: &str,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE account_lockouts SET unlocked_at = ?, unlocked_by = ?
             WHERE id = ?",
        )
        .bind(&now)
        .bind(unlocked_by)
        .bind(lockout_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

impl PasswordHistory {
    /// Add password to history
    pub async fn add(
        pool: &SqlitePool,
        user_id: &str,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO password_history (id, user_id, password_hash, changed_at)
             VALUES (?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(user_id)
        .bind(password_hash)
        .bind(&now)
        .execute(pool)
        .await?;

        // Keep only last 5 passwords
        sqlx::query(
            "DELETE FROM password_history 
             WHERE user_id = ? AND id NOT IN (
                 SELECT id FROM password_history 
                 WHERE user_id = ? 
                 ORDER BY changed_at DESC 
                 LIMIT 5
             )",
        )
        .bind(user_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Check if password was used recently
    pub async fn was_used_recently(
        pool: &SqlitePool,
        user_id: &str,
        password_hash: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM password_history 
             WHERE user_id = ? AND password_hash = ?",
        )
        .bind(user_id)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        Ok(result > 0)
    }
}
