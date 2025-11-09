//! Database integration tests
//!
//! Tests SQLite operations with a temporary test database.

use sqlx::SqlitePool;
use tempfile::TempDir;
use uuid::Uuid;

// ============================================================================
// Test Database Setup
// ============================================================================

async fn create_test_database() -> (SqlitePool, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");

    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display()))
        .await
        .expect("Failed to connect to test database");

    // Run basic schema (simplified for testing)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            email TEXT,
            display_name TEXT,
            bio TEXT,
            avatar_base64 TEXT,
            theme TEXT DEFAULT 'light',
            language TEXT DEFAULT 'en',
            default_view TEXT DEFAULT 'grid',
            totp_secret TEXT,
            totp_enabled INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS files (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            file_path TEXT UNIQUE NOT NULL,
            size_bytes INTEGER NOT NULL,
            mime_type TEXT,
            owner_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (owner_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create files table");

    (pool, temp_dir)
}

// ============================================================================
// User CRUD Tests
// ============================================================================

#[tokio::test]
async fn test_create_user() {
    let (pool, _temp_dir) = create_test_database().await;

    let user_id = Uuid::new_v4().to_string();
    let username = "testuser";
    let password_hash = "hash123";
    let now = chrono::Utc::now().to_rfc3339();

    let result = sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(username)
    .bind(password_hash)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await;

    assert!(result.is_ok(), "Failed to insert user");
    assert_eq!(result.unwrap().rows_affected(), 1);
}

#[tokio::test]
async fn test_get_user_by_username() {
    let (pool, _temp_dir) = create_test_database().await;

    // Insert test user
    let user_id = Uuid::new_v4().to_string();
    let username = "testuser";
    let password_hash = "hash123";
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(username)
    .bind(password_hash)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    // Retrieve user
    #[derive(sqlx::FromRow)]
    struct User {
        id: String,
        username: String,
        password_hash: String,
    }

    let user: User =
        sqlx::query_as("SELECT id, username, password_hash FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch user");

    assert_eq!(user.id, user_id);
    assert_eq!(user.username, username);
    assert_eq!(user.password_hash, password_hash);
}

#[tokio::test]
async fn test_duplicate_username() {
    let (pool, _temp_dir) = create_test_database().await;

    let username = "testuser";
    let now = chrono::Utc::now().to_rfc3339();

    // Insert first user
    let result1 = sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(username)
    .bind("hash1")
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await;

    assert!(result1.is_ok());

    // Try to insert second user with same username
    let result2 = sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(username)
    .bind("hash2")
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await;

    assert!(result2.is_err(), "Duplicate username should fail");
}

#[tokio::test]
async fn test_update_user() {
    let (pool, _temp_dir) = create_test_database().await;

    // Insert user
    let user_id = Uuid::new_v4().to_string();
    let username = "testuser";
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(username)
    .bind("hash123")
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    // Update display name
    let new_display_name = "Test User";
    let result = sqlx::query("UPDATE users SET display_name = ?, updated_at = ? WHERE id = ?")
        .bind(new_display_name)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(&user_id)
        .execute(&pool)
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().rows_affected(), 1);

    // Verify update
    #[derive(sqlx::FromRow)]
    struct User {
        display_name: Option<String>,
    }

    let user: User = sqlx::query_as("SELECT display_name FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch user");

    assert_eq!(user.display_name.as_deref(), Some(new_display_name));
}

#[tokio::test]
async fn test_delete_user() {
    let (pool, _temp_dir) = create_test_database().await;

    // Insert user
    let user_id = Uuid::new_v4().to_string();
    let username = "testuser";
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(username)
    .bind("hash123")
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    // Delete user
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&user_id)
        .execute(&pool)
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().rows_affected(), 1);

    // Verify deletion
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to count users");

    assert_eq!(count.0, 0);
}

// ============================================================================
// File Operations Tests
// ============================================================================

#[tokio::test]
async fn test_create_file_record() {
    let (pool, _temp_dir) = create_test_database().await;

    // Create user first
    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind("testuser")
    .bind("hash123")
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    // Create file record
    let file_id = Uuid::new_v4().to_string();
    let filename = "test.txt";
    let file_path = "/test.txt";
    let size_bytes = 1024;

    let result = sqlx::query(
        "INSERT INTO files (id, filename, file_path, size_bytes, owner_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&file_id)
    .bind(filename)
    .bind(file_path)
    .bind(size_bytes)
    .bind(&user_id)
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().rows_affected(), 1);
}

#[tokio::test]
async fn test_get_files_by_owner() {
    let (pool, _temp_dir) = create_test_database().await;

    // Create user
    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind("testuser")
    .bind("hash123")
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    // Create multiple files
    for i in 0..3 {
        sqlx::query(
            "INSERT INTO files (id, filename, file_path, size_bytes, owner_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(format!("file{}.txt", i))
        .bind(format!("/file{}.txt", i))
        .bind(1024)
        .bind(&user_id)
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .expect("Failed to insert file");
    }

    // Query files by owner
    #[derive(sqlx::FromRow)]
    #[allow(dead_code)]
    struct File {
        filename: String,
    }

    let files: Vec<File> = sqlx::query_as("SELECT filename FROM files WHERE owner_id = ?")
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch files");

    assert_eq!(files.len(), 3);
}

// ============================================================================
// Connection Pool Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_queries() {
    let (pool, _temp_dir) = create_test_database().await;

    // Insert test user
    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind("testuser")
    .bind("hash123")
    .bind(&now)
    .bind(&now)
    .execute(&pool)
    .await
    .expect("Failed to insert user");

    // Run multiple concurrent queries
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let pool = pool.clone();
            let user_id = user_id.clone();
            tokio::spawn(async move {
                let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE id = ?")
                    .bind(&user_id)
                    .fetch_one(&pool)
                    .await
                    .expect("Failed to query");
            })
        })
        .collect();

    for handle in handles {
        handle.await.expect("Task panicked");
    }
}

#[tokio::test]
async fn test_transaction_rollback() {
    let (pool, _temp_dir) = create_test_database().await;

    let user_id = Uuid::new_v4().to_string();
    let username = "testuser";
    let now = chrono::Utc::now().to_rfc3339();

    // Start transaction
    let mut tx = pool.begin().await.expect("Failed to begin transaction");

    // Insert user
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(username)
    .bind("hash123")
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .expect("Failed to insert user");

    // Rollback transaction
    tx.rollback().await.expect("Failed to rollback");

    // Verify user was not inserted
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to count users");

    assert_eq!(count.0, 0, "User should not exist after rollback");
}

#[tokio::test]
async fn test_transaction_commit() {
    let (pool, _temp_dir) = create_test_database().await;

    let user_id = Uuid::new_v4().to_string();
    let username = "testuser";
    let now = chrono::Utc::now().to_rfc3339();

    // Start transaction
    let mut tx = pool.begin().await.expect("Failed to begin transaction");

    // Insert user
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(username)
    .bind("hash123")
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .expect("Failed to insert user");

    // Commit transaction
    tx.commit().await.expect("Failed to commit");

    // Verify user was inserted
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to count users");

    assert_eq!(count.0, 1, "User should exist after commit");
}
