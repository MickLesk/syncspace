//! Authentication security service
//! Handles password policies, account lockout, login tracking, session management

use crate::database::{LoginAttempt, UserSession, AccountLockout, PasswordHistory};
use sqlx::SqlitePool;
use chrono::Utc;

// Configuration constants
const MAX_FAILED_ATTEMPTS: i64 = 5;
const LOCKOUT_DURATION_MINUTES: i64 = 15;
const FAILED_ATTEMPT_WINDOW_MINUTES: i64 = 15;

// Password policy constants
const MIN_PASSWORD_LENGTH: usize = 8;
const REQUIRE_UPPERCASE: bool = true;
const REQUIRE_LOWERCASE: bool = true;
const REQUIRE_NUMBER: bool = true;
const REQUIRE_SPECIAL_CHAR: bool = true;

/// Password policy violation errors
#[derive(Debug)]
#[allow(dead_code)] // RecentlyUsed variant prepared for password history feature
pub enum PasswordPolicyError {
    TooShort,
    NoUppercase,
    NoLowercase,
    NoNumber,
    NoSpecialChar,
    TooCommon,
    RecentlyUsed,
}

impl std::fmt::Display for PasswordPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooShort => write!(f, "Password must be at least {} characters", MIN_PASSWORD_LENGTH),
            Self::NoUppercase => write!(f, "Password must contain at least one uppercase letter"),
            Self::NoLowercase => write!(f, "Password must contain at least one lowercase letter"),
            Self::NoNumber => write!(f, "Password must contain at least one number"),
            Self::NoSpecialChar => write!(f, "Password must contain at least one special character"),
            Self::TooCommon => write!(f, "Password is too common, please choose a stronger password"),
            Self::RecentlyUsed => write!(f, "Password was used recently, please choose a different password"),
        }
    }
}

impl std::error::Error for PasswordPolicyError {}

/// Validate password against policy
pub fn validate_password_policy(password: &str) -> Result<(), PasswordPolicyError> {
    // Check minimum length
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(PasswordPolicyError::TooShort);
    }
    
    // Check uppercase
    if REQUIRE_UPPERCASE && !password.chars().any(|c| c.is_uppercase()) {
        return Err(PasswordPolicyError::NoUppercase);
    }
    
    // Check lowercase
    if REQUIRE_LOWERCASE && !password.chars().any(|c| c.is_lowercase()) {
        return Err(PasswordPolicyError::NoLowercase);
    }
    
    // Check number
    if REQUIRE_NUMBER && !password.chars().any(|c| c.is_numeric()) {
        return Err(PasswordPolicyError::NoNumber);
    }
    
    // Check special character
    if REQUIRE_SPECIAL_CHAR && !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
        return Err(PasswordPolicyError::NoSpecialChar);
    }
    
    // Check against common passwords
    if is_common_password(password) {
        return Err(PasswordPolicyError::TooCommon);
    }
    
    Ok(())
}

/// Check if password is in common password list
fn is_common_password(password: &str) -> bool {
    const COMMON_PASSWORDS: &[&str] = &[
        "password", "123456", "123456789", "12345678", "12345", "1234567",
        "password1", "qwerty", "abc123", "111111", "123123", "admin",
        "letmein", "welcome", "monkey", "dragon", "master", "sunshine",
        "princess", "football", "baseball", "iloveyou", "trustno1",
    ];
    
    COMMON_PASSWORDS.contains(&password.to_lowercase().as_str())
}

/// Log a login attempt
pub async fn log_login_attempt(
    pool: &SqlitePool,
    username: &str,
    ip_address: &str,
    user_agent: Option<&str>,
    success: bool,
    failure_reason: Option<&str>,
) -> Result<(), sqlx::Error> {
    LoginAttempt::create(pool, username, ip_address, user_agent, success, failure_reason).await?;
    Ok(())
}

/// Check if account should be locked (too many failed attempts)
pub async fn check_and_lock_account(
    pool: &SqlitePool,
    username: &str,
) -> Result<bool, sqlx::Error> {
    // Check if already locked
    if let Some(_lockout) = AccountLockout::is_locked(pool, username).await? {
        return Ok(true); // Already locked
    }
    
    // Get recent failed attempts
    let failed_attempts = LoginAttempt::get_recent_failures(
        pool,
        username,
        FAILED_ATTEMPT_WINDOW_MINUTES,
    ).await?;
    
    let failed_count = failed_attempts.len() as i64;
    
    if failed_count >= MAX_FAILED_ATTEMPTS {
        // Get user_id from username
        let user_id = match sqlx::query_scalar::<_, String>(
            "SELECT id FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(pool)
        .await? {
            Some(id) => id,
            None => return Ok(false), // User doesn't exist
        };
        
        // Lock the account
        AccountLockout::create(
            pool,
            &user_id,
            username,
            failed_count,
            LOCKOUT_DURATION_MINUTES,
        ).await?;
        
        // Update user record
        sqlx::query(
            "UPDATE users SET account_locked_until = ?, failed_login_attempts = ?
             WHERE id = ?"
        )
        .bind((Utc::now() + chrono::Duration::minutes(LOCKOUT_DURATION_MINUTES)).to_rfc3339())
        .bind(failed_count)
        .bind(&user_id)
        .execute(pool)
        .await?;
        
        return Ok(true); // Account locked
    }
    
    Ok(false)
}

/// Check if account is currently locked
pub async fn is_account_locked(
    pool: &SqlitePool,
    username: &str,
) -> Result<Option<AccountLockout>, sqlx::Error> {
    AccountLockout::is_locked(pool, username).await
}

/// Reset failed login attempts (after successful login)
pub async fn reset_failed_attempts(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET failed_login_attempts = 0, last_failed_login_at = NULL, account_locked_until = NULL
         WHERE id = ?"
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Create a new session
pub async fn create_session(
    pool: &SqlitePool,
    user_id: &str,
    session_token: &str,
    ip_address: &str,
    user_agent: Option<&str>,
    expires_at: chrono::DateTime<Utc>,
) -> Result<UserSession, sqlx::Error> {
    UserSession::create(pool, user_id, session_token, ip_address, user_agent, expires_at).await
}

/// Get user's active sessions
pub async fn get_user_sessions(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<UserSession>, sqlx::Error> {
    UserSession::get_user_sessions(pool, user_id).await
}

/// Revoke a session
pub async fn revoke_session(
    pool: &SqlitePool,
    session_id: &str,
    reason: &str,
) -> Result<(), sqlx::Error> {
    UserSession::revoke(pool, session_id, reason).await
}

/// Revoke all user sessions (e.g., on password change)
pub async fn revoke_all_user_sessions(
    pool: &SqlitePool,
    user_id: &str,
    reason: &str,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE user_sessions SET revoked = 1, revoked_at = ?, revoked_reason = ?
         WHERE user_id = ? AND revoked = 0"
    )
    .bind(&now)
    .bind(reason)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Validate password change against history
pub async fn validate_password_change(
    pool: &SqlitePool,
    user_id: &str,
    new_password: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate against policy
    validate_password_policy(new_password)?;
    
    // Hash the password using argon2 to check against history
    use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2};
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(new_password.as_bytes(), &salt)
        .map_err(|e| format!("Password hashing failed: {}", e))?
        .to_string();
    
    // Check if used recently
    if PasswordHistory::was_used_recently(pool, user_id, &password_hash).await? {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Password was used recently"
        )));
    }
    
    Ok(())
}

/// Update password and history
pub async fn update_password_with_history(
    pool: &SqlitePool,
    user_id: &str,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    // Update user password
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE users SET password_hash = ?, password_last_changed_at = ?
         WHERE id = ?"
    )
    .bind(password_hash)
    .bind(&now)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    // Add to history
    PasswordHistory::add(pool, user_id, password_hash).await?;
    
    Ok(())
}

/// Clean up expired sessions (background job)
pub async fn cleanup_expired_sessions(pool: &SqlitePool) -> Result<usize, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE user_sessions 
         SET revoked = 1, revoked_at = datetime('now'), revoked_reason = 'expired'
         WHERE expires_at <= datetime('now') AND revoked = 0"
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() as usize)
}

/// Clean up old login attempts (keep last 90 days)
pub async fn cleanup_old_login_attempts(pool: &SqlitePool) -> Result<usize, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM login_attempts 
         WHERE attempted_at < datetime('now', '-90 days')"
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() as usize)
}
