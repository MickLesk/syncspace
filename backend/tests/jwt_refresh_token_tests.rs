// JWT Refresh Token System Tests
// Tests for token generation, validation, rotation, revocation, and expiration

use sqlx::SqlitePool;
use syncbackend::auth;
use syncbackend::database::{self, RefreshToken, User};

// Helper function to create test database pool
async fn create_test_pool() -> SqlitePool {
    SqlitePool::connect("sqlite::memory:").await.unwrap()
}

// Helper function to create test user
async fn create_test_user(pool: &SqlitePool) -> User {
    // Initialize tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            email TEXT,
            display_name TEXT,
            bio TEXT,
            avatar_base64 TEXT,
            totp_secret TEXT,
            totp_enabled INTEGER NOT NULL DEFAULT 0,
            token_version INTEGER NOT NULL DEFAULT 1,
            storage_quota_bytes INTEGER NOT NULL,
            storage_used_bytes INTEGER NOT NULL,
            default_view TEXT NOT NULL,
            language TEXT NOT NULL,
            theme TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_login TEXT,
            updated_at TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS refresh_tokens (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            token_hash TEXT NOT NULL UNIQUE,
            token_version INTEGER NOT NULL DEFAULT 1,
            expires_at TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_used_at TEXT,
            revoked_at TEXT,
            user_agent TEXT,
            ip_address TEXT,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await
    .unwrap();

    // Create test user
    let user_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT INTO users (
            id, username, password_hash, email, display_name,
            storage_quota_bytes, storage_used_bytes,
            default_view, language, theme, token_version,
            totp_enabled, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind("testuser")
    .bind("$argon2id$v=19$m=19456,t=2,p=1$test$test")
    .bind("test@example.com")
    .bind("Test User")
    .bind(10737418240i64)
    .bind(0i64)
    .bind("grid")
    .bind("en")
    .bind("light")
    .bind(1)
    .bind(false)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .unwrap();

    auth::get_user_by_id(pool, &user_id).await.unwrap().unwrap()
}

#[tokio::test]
async fn test_generate_access_and_refresh_tokens() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Generate access token (15 min expiration)
    let access_token = auth::generate_token(&user).expect("Failed to generate access token");
    assert!(!access_token.is_empty());

    // Generate refresh token (7 day expiration)
    let refresh_token = auth::generate_refresh_token(&user).expect("Failed to generate refresh token");
    assert!(!refresh_token.is_empty());

    // Verify access token
    let access_claims = auth::verify_token(&access_token).expect("Failed to verify access token");
    assert_eq!(access_claims.sub, user.id);
    assert_eq!(access_claims.username, user.username);

    // Verify refresh token
    let refresh_claims = auth::verify_refresh_token(&refresh_token).expect("Failed to verify refresh token");
    assert_eq!(refresh_claims.sub, user.id);
    assert_eq!(refresh_claims.username, user.username);
    assert_eq!(refresh_claims.token_version, user.token_version);
}

#[tokio::test]
async fn test_store_and_validate_refresh_token() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Generate refresh token
    let refresh_token = auth::generate_refresh_token(&user).expect("Failed to generate refresh token");

    // Store refresh token in database
    auth::store_refresh_token(&pool, &user.id, &refresh_token, Some("Test User Agent".to_string()), Some("127.0.0.1".to_string()))
        .await
        .expect("Failed to store refresh token");

    // Validate refresh token
    let validated_user = auth::validate_refresh_token(&pool, &refresh_token)
        .await
        .expect("Failed to validate refresh token");

    assert_eq!(validated_user.id, user.id);
    assert_eq!(validated_user.username, user.username);
}

#[tokio::test]
async fn test_token_rotation_on_refresh() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Generate and store first refresh token
    let refresh_token1 = auth::generate_refresh_token(&user).expect("Failed to generate refresh token");
    auth::store_refresh_token(&pool, &user.id, &refresh_token1, None, None)
        .await
        .expect("Failed to store refresh token");

    // Simulate token rotation (generate new token)
    let refresh_token2 = auth::generate_refresh_token(&user).expect("Failed to generate refresh token");
    auth::store_refresh_token(&pool, &user.id, &refresh_token2, None, None)
        .await
        .expect("Failed to store refresh token");

    // Both tokens should be valid (until old one is explicitly revoked)
    let user1 = auth::validate_refresh_token(&pool, &refresh_token1).await;
    let user2 = auth::validate_refresh_token(&pool, &refresh_token2).await;

    assert!(user1.is_ok());
    assert!(user2.is_ok());
}

#[tokio::test]
async fn test_revoke_specific_refresh_token() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Generate and store refresh token
    let refresh_token = auth::generate_refresh_token(&user).expect("Failed to generate refresh token");
    auth::store_refresh_token(&pool, &user.id, &refresh_token, None, None)
        .await
        .expect("Failed to store refresh token");

    // Validate token (should work)
    let result1 = auth::validate_refresh_token(&pool, &refresh_token).await;
    assert!(result1.is_ok());

    // Revoke token
    auth::revoke_refresh_token(&pool, &refresh_token)
        .await
        .expect("Failed to revoke refresh token");

    // Validate token again (should fail)
    let result2 = auth::validate_refresh_token(&pool, &refresh_token).await;
    assert!(result2.is_err());
}

#[tokio::test]
async fn test_revoke_all_user_tokens() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Generate and store multiple refresh tokens
    let token1 = auth::generate_refresh_token(&user).expect("Failed to generate token1");
    let token2 = auth::generate_refresh_token(&user).expect("Failed to generate token2");
    
    auth::store_refresh_token(&pool, &user.id, &token1, None, None).await.expect("Failed to store token1");
    auth::store_refresh_token(&pool, &user.id, &token2, None, None).await.expect("Failed to store token2");

    // Both tokens should be valid
    assert!(auth::validate_refresh_token(&pool, &token1).await.is_ok());
    assert!(auth::validate_refresh_token(&pool, &token2).await.is_ok());

    // Revoke all tokens for user
    auth::revoke_all_user_tokens(&pool, &user.id)
        .await
        .expect("Failed to revoke all tokens");

    // Both tokens should now be invalid (token_version incremented)
    assert!(auth::validate_refresh_token(&pool, &token1).await.is_err());
    assert!(auth::validate_refresh_token(&pool, &token2).await.is_err());
}

#[tokio::test]
async fn test_token_version_invalidation() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Generate and store refresh token
    let refresh_token = auth::generate_refresh_token(&user).expect("Failed to generate refresh token");
    auth::store_refresh_token(&pool, &user.id, &refresh_token, None, None)
        .await
        .expect("Failed to store refresh token");

    // Validate token (should work)
    assert!(auth::validate_refresh_token(&pool, &refresh_token).await.is_ok());

    // Increment user's token_version (simulating password change)
    sqlx::query("UPDATE users SET token_version = token_version + 1 WHERE id = ?")
        .bind(&user.id)
        .execute(&pool)
        .await
        .unwrap();

    // Validate token again (should fail due to version mismatch)
    let result = auth::validate_refresh_token(&pool, &refresh_token).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("invalidated"));
}

#[tokio::test]
async fn test_cleanup_expired_tokens() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Insert expired token manually
    let expired_token_id = uuid::Uuid::new_v4().to_string();
    let past_time = chrono::Utc::now() - chrono::Duration::days(10);
    
    sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, token_version, expires_at, created_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&expired_token_id)
    .bind(&user.id)
    .bind("expired_hash")
    .bind(1)
    .bind(past_time.to_rfc3339())
    .bind(past_time.to_rfc3339())
    .execute(&pool)
    .await
    .unwrap();

    // Insert valid token
    let refresh_token = auth::generate_refresh_token(&user).expect("Failed to generate refresh token");
    auth::store_refresh_token(&pool, &user.id, &refresh_token, None, None)
        .await
        .expect("Failed to store refresh token");

    // Count tokens before cleanup
    let count_before: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM refresh_tokens")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count_before.0, 2);

    // Run cleanup
    let deleted = auth::cleanup_expired_tokens(&pool)
        .await
        .expect("Failed to cleanup expired tokens");
    assert_eq!(deleted, 1);

    // Count tokens after cleanup
    let count_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM refresh_tokens")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count_after.0, 1);
}

#[tokio::test]
async fn test_access_token_short_expiration() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Generate access token
    let access_token = auth::generate_token(&user).expect("Failed to generate access token");
    let claims = auth::verify_token(&access_token).expect("Failed to verify token");

    // Check expiration is ~15 minutes (900 seconds) from now
    let now = chrono::Utc::now().timestamp() as usize;
    let exp_diff = claims.exp - now;
    
    // Allow some margin (14-16 minutes)
    assert!(exp_diff >= 840 && exp_diff <= 960, "Expected ~900s expiration, got {}", exp_diff);
}

#[tokio::test]
async fn test_refresh_token_long_expiration() {
    let pool = create_test_pool().await;
    let user = create_test_user(&pool).await;

    // Generate refresh token
    let refresh_token = auth::generate_refresh_token(&user).expect("Failed to generate refresh token");
    let claims = auth::verify_refresh_token(&refresh_token).expect("Failed to verify token");

    // Check expiration is ~7 days (604800 seconds) from now
    let now = chrono::Utc::now().timestamp() as usize;
    let exp_diff = claims.exp - now;
    
    // Allow some margin (6.9-7.1 days)
    assert!(exp_diff >= 595000 && exp_diff <= 614000, "Expected ~604800s expiration, got {}", exp_diff);
}
