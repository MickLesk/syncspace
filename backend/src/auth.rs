//! Authentication module for SyncSpace
//! 
//! Provides user management, JWT tokens, password hashing, and TOTP 2FA.
//! All data stored in SQLite database - NO JSON files.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use totp_lite::{totp_custom, Sha1};
use uuid::Uuid;

// TODO: Move to environment variable
const JWT_SECRET: &str = "your-secret-key-change-in-production";
const ACCESS_TOKEN_EXPIRATION_MINUTES: i64 = 15;  // Short-lived access token
const REFRESH_TOKEN_EXPIRATION_DAYS: i64 = 7;     // Long-lived refresh token

// ============================================================================
// SQLite-based Auth Functions (NO JSON files)
// ============================================================================

/// Get user by username from SQLite
pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<crate::database::User>, sqlx::Error> {
    sqlx::query_as::<_, crate::database::User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
}

/// Get user by ID from SQLite
pub async fn get_user_by_id(pool: &SqlitePool, user_id: &str) -> Result<Option<crate::database::User>, sqlx::Error> {
    sqlx::query_as::<_, crate::database::User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

/// Verify password for user
pub async fn verify_password(pool: &SqlitePool, username: &str, password: &str) -> Result<crate::database::User, String> {
    let user = get_user_by_username(pool, username)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("Invalid username or password")?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| "Invalid password hash".to_string())?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| "Invalid username or password".to_string())?;

    Ok(user)
}

/// Change user password in SQLite
pub async fn change_user_password(pool: &SqlitePool, user_id: &str, old_password: &str, new_password: &str) -> Result<(), String> {
    let user = get_user_by_id(pool, user_id)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("User not found")?;

    // Verify old password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| "Invalid password hash".to_string())?;

    Argon2::default()
        .verify_password(old_password.as_bytes(), &parsed_hash)
        .map_err(|_| "Current password is incorrect".to_string())?;

    // Hash new password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let new_hash = argon2
        .hash_password(new_password.as_bytes(), &salt)
        .map_err(|e| format!("Password hashing failed: {}", e))?
        .to_string();

    // Update in database
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?")
        .bind(&new_hash)
        .bind(&now)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update password: {}", e))?;

    Ok(())
}

/// Validate JWT token against SQLite database
pub async fn validate_token_against_db(pool: &SqlitePool, token: &str) -> Result<UserInfo, String> {
    let claims = verify_token(token)?;
    
    // Get user from SQLite to ensure they still exist
    let user = get_user_by_id(pool, &claims.sub)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("User not found")?;
    
    Ok(UserInfo {
        id: user.id,
        username: user.username,
        totp_enabled: user.totp_enabled,
    })
}

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub username: String, // Username for convenience
    pub exp: usize,       // Expiration time
    pub iat: usize,       // Issued at
}

/// Refresh Token Claims (longer expiration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,       // Subject (user ID)
    pub username: String,  // Username for convenience
    pub exp: usize,        // Expiration time (7 days)
    pub iat: usize,        // Issued at
    pub token_version: i32, // For token rotation/invalidation (changed from u32 to i32)
}

/// Generate JWT access token for authenticated user (15 minute expiration)
pub fn generate_token(user: &crate::database::User) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(ACCESS_TOKEN_EXPIRATION_MINUTES))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.clone(), // Already a String in database::User
        username: user.username.clone(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(|e| format!("Token generation failed: {}", e))
}

/// Generate refresh token (7 day expiration) with token version
pub fn generate_refresh_token(user: &crate::database::User) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(REFRESH_TOKEN_EXPIRATION_DAYS))
        .ok_or("Invalid timestamp")?
        .timestamp();

    let claims = RefreshTokenClaims {
        sub: user.id.clone(), // Already a String in database::User
        username: user.username.clone(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
        token_version: user.token_version, // Use user's current token version
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(|e| format!("Token generation failed: {}", e))
}

/// Verify JWT token and extract claims
pub fn verify_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Token verification failed: {}", e))
}

/// Verify refresh token and extract claims
pub fn verify_refresh_token(token: &str) -> Result<RefreshTokenClaims, String> {
    decode::<RefreshTokenClaims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Refresh token verification failed: {}", e))
}

/// TOTP 2FA Functions
pub fn generate_totp_secret() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..20).map(|_| rng.random()).collect();
    base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &bytes)
}

pub fn generate_totp_code(secret: &str, time_step: u64) -> Result<String, String> {
    let secret_bytes = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret)
        .ok_or("Invalid base32 secret")?;

    let code = totp_custom::<Sha1>(30, 6, &secret_bytes, time_step);
    Ok(format!("{:06}", code))
}

pub fn verify_totp_code(secret: &str, code: &str) -> bool {
    let time_step = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Check current time step and ±1 step (30s tolerance)
    for offset in [-1i64, 0, 1] {
        let check_time = (time_step as i64 + offset * 30) as u64;
        if let Ok(expected) = generate_totp_code(secret, check_time) {
            if expected == code {
                return true;
            }
        }
    }
    false
}

/// Warp filter to extract authenticated user from JWT token
/// Axum extractor for authenticated requests
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header"))?;

        // Verify Bearer token
        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid authorization header"));
        }

        let token = auth_header.trim_start_matches("Bearer ");
        let claims = verify_token(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID"))?;

        // Return a User wrapping UserInfo
        Ok(User(UserInfo {
            id: user_id.to_string(),
            username: claims.username,
            totp_enabled: false,
        }))
    }
}

/// Auth request/response structures
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserInfo,
    pub requires_2fa: bool,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub totp_enabled: bool,
}

impl UserInfo {
    /// Alias for backward compatibility with code expecting user_id
    pub fn user_id(&self) -> &str {
        &self.id
    }
}

// Axum extractor for authenticated user
impl<S> FromRequestParts<S> for UserInfo
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<User>()
            .map(|User(user_info)| user_info.clone())
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

// Tuple struct wrapper for extension storage
#[derive(Debug, Clone)]
pub struct User(pub UserInfo);

impl User {
    pub fn id(&self) -> &str {
        &self.0.id
    }
    
    pub fn username(&self) -> &str {
        &self.0.username
    }
}

// Provide direct field access for backward compatibility
impl std::ops::Deref for User {
    type Target = UserInfo;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct Enable2FARequest {
    pub totp_code: String,
}

#[derive(Debug, Serialize)]
pub struct Setup2FAResponse {
    pub secret: String,
    pub qr_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// ============================================================================
// Refresh Token Database Functions
// ============================================================================

/// Store refresh token in database
pub async fn store_refresh_token(
    pool: &SqlitePool,
    user_id: &str,
    refresh_token: &str,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<(), String> {
    use sha2::{Sha256, Digest};
    
    // Hash the refresh token for storage
    let mut hasher = Sha256::new();
    hasher.update(refresh_token.as_bytes());
    let token_hash = format!("{:x}", hasher.finalize());
    
    // Get user's token version
    let user = get_user_by_id(pool, user_id)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("User not found")?;
    
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(REFRESH_TOKEN_EXPIRATION_DAYS))
        .ok_or("Invalid timestamp")?
        .to_rfc3339();
    
    sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, token_version, expires_at, created_at, user_agent, ip_address) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(user_id)
    .bind(&token_hash)
    .bind(user.token_version)
    .bind(&expires_at)
    .bind(&now)
    .bind(user_agent)
    .bind(ip_address)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to store refresh token: {}", e))?;
    
    Ok(())
}

/// Validate refresh token against database
pub async fn validate_refresh_token(
    pool: &SqlitePool,
    refresh_token: &str,
) -> Result<crate::database::User, String> {
    use sha2::{Sha256, Digest};
    
    // Verify token signature and extract claims
    let claims = verify_refresh_token(refresh_token)?;
    
    // Hash the refresh token for lookup
    let mut hasher = Sha256::new();
    hasher.update(refresh_token.as_bytes());
    let token_hash = format!("{:x}", hasher.finalize());
    
    // Check if token exists in database and is not revoked
    let token_record: Option<crate::database::RefreshToken> = sqlx::query_as(
        "SELECT * FROM refresh_tokens 
         WHERE token_hash = ? AND user_id = ? AND revoked_at IS NULL 
         AND datetime(expires_at) > datetime('now')"
    )
    .bind(&token_hash)
    .bind(&claims.sub)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    let token_record = token_record.ok_or("Invalid or expired refresh token")?;
    
    // Get user and check token version
    let user = get_user_by_id(pool, &claims.sub)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("User not found")?;
    
    // Verify token version matches (for global invalidation)
    if token_record.token_version != user.token_version {
        return Err("Refresh token has been invalidated".to_string());
    }
    
    // Update last_used_at
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE refresh_tokens SET last_used_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&token_record.id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update token usage: {}", e))?;
    
    Ok(user)
}

/// Revoke specific refresh token
pub async fn revoke_refresh_token(
    pool: &SqlitePool,
    refresh_token: &str,
) -> Result<(), String> {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(refresh_token.as_bytes());
    let token_hash = format!("{:x}", hasher.finalize());
    
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE refresh_tokens SET revoked_at = ? WHERE token_hash = ?")
        .bind(&now)
        .bind(&token_hash)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to revoke token: {}", e))?;
    
    Ok(())
}

/// Revoke all refresh tokens for a user (e.g., on password change, logout all devices)
pub async fn revoke_all_user_tokens(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<(), String> {
    let now = Utc::now().to_rfc3339();
    
    // Revoke all tokens
    sqlx::query("UPDATE refresh_tokens SET revoked_at = ? WHERE user_id = ? AND revoked_at IS NULL")
        .bind(&now)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to revoke tokens: {}", e))?;
    
    // Increment user's token version to invalidate all tokens globally
    sqlx::query("UPDATE users SET token_version = token_version + 1, updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to increment token version: {}", e))?;
    
    Ok(())
}

/// Clean up expired refresh tokens (call this periodically)
pub async fn cleanup_expired_tokens(pool: &SqlitePool) -> Result<u64, String> {
    let result = sqlx::query("DELETE FROM refresh_tokens WHERE datetime(expires_at) < datetime('now')")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to clean up expired tokens: {}", e))?;
    
    Ok(result.rows_affected())
}

// Rate limiting structure (simple in-memory)
pub struct RateLimiter {
    attempts: Arc<Mutex<HashMap<String, Vec<DateTime<Utc>>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            attempts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn check_rate_limit(&self, key: &str, max_attempts: usize, window_secs: i64) -> bool {
        let mut attempts = self.attempts.lock().unwrap();
        let now = Utc::now();
        let cutoff = now - Duration::seconds(window_secs);

        let entry = attempts.entry(key.to_string()).or_insert_with(Vec::new);
        entry.retain(|&ts| ts > cutoff);

        if entry.len() >= max_attempts {
            return false;
        }

        entry.push(now);
        true
    }
}
