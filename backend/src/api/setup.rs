//! First-Start Setup Wizard API

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct SetupStatus {
    pub setup_completed: bool,
    pub has_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct CompleteSetupRequest {
    // Admin Account
    pub admin_username: String,
    pub admin_password: String,
    pub admin_email: String,
    pub admin_display_name: String,
    
    // Server Info
    pub server_name: String,
    pub server_description: String,
    
    // Settings
    pub default_language: String,
    pub default_quota_gb: i32,
    pub allow_registration: bool,
    pub require_email_verification: bool,
    
    // Security
    pub enable_2fa_requirement: bool,
    pub password_min_length: i32,
    pub password_require_uppercase: bool,
    pub password_require_lowercase: bool,
    pub password_require_numbers: bool,
    pub password_require_special: bool,
    pub session_timeout_minutes: i32,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/setup/status", get(get_setup_status))
        .route("/setup/complete", post(complete_setup))
}

/// Check if setup is needed
async fn get_setup_status(State(state): State<AppState>) -> Result<Json<SetupStatus>, StatusCode> {
    // Check if setup has been completed
    let setup_completed: Option<(bool,)> = sqlx::query_as(
        "SELECT setup_completed FROM system_settings WHERE id = 1"
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let completed = setup_completed.map(|(c,)| c).unwrap_or(false);
    
    // Check if any admin user exists
    let admin_count: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE username = 'admin' OR id IN (SELECT user_id FROM user_system_roles WHERE role = 'admin')"
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let has_admin = admin_count.map(|(c,)| c > 0).unwrap_or(false);
    
    Ok(Json(SetupStatus {
        setup_completed: completed,
        has_admin,
    }))
}

/// Complete the initial setup
async fn complete_setup(
    State(state): State<AppState>,
    Json(req): Json<CompleteSetupRequest>,
) -> Result<StatusCode, StatusCode> {
    // Validate that setup hasn't been completed yet
    let setup_completed: Option<(bool,)> = sqlx::query_as(
        "SELECT setup_completed FROM system_settings WHERE id = 1"
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if setup_completed.map(|(c,)| c).unwrap_or(false) {
        return Err(StatusCode::CONFLICT); // Setup already completed
    }
    
    // Validate input
    if req.admin_username.is_empty() || req.admin_password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Hash admin password with argon2
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2
    };
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.admin_password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    
    // Create admin user
    let admin_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT INTO users (
            id, username, password_hash, email, display_name,
            storage_quota_bytes, storage_used_bytes,
            default_view, language, theme,
            totp_enabled, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&admin_id)
    .bind(&req.admin_username)
    .bind(&password_hash)
    .bind(&req.admin_email)
    .bind(&req.admin_display_name)
    .bind((req.default_quota_gb as i64) * 1024 * 1024 * 1024) // GB to bytes
    .bind(0i64)
    .bind("grid")
    .bind(&req.default_language)
    .bind("light")
    .bind(false)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Update system settings
    sqlx::query(
        "UPDATE system_settings SET
            setup_completed = 1,
            server_name = ?,
            server_description = ?,
            default_language = ?,
            allow_registration = ?,
            require_email_verification = ?,
            default_quota_gb = ?,
            enable_2fa_requirement = ?,
            password_min_length = ?,
            password_require_uppercase = ?,
            password_require_lowercase = ?,
            password_require_numbers = ?,
            password_require_special = ?,
            session_timeout_minutes = ?,
            updated_at = ?
        WHERE id = 1"
    )
    .bind(&req.server_name)
    .bind(&req.server_description)
    .bind(&req.default_language)
    .bind(req.allow_registration)
    .bind(req.require_email_verification)
    .bind(req.default_quota_gb)
    .bind(req.enable_2fa_requirement)
    .bind(req.password_min_length)
    .bind(req.password_require_uppercase)
    .bind(req.password_require_lowercase)
    .bind(req.password_require_numbers)
    .bind(req.password_require_special)
    .bind(req.session_timeout_minutes)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    println!("🎉 Setup completed! Admin user '{}' created", req.admin_username);
    
    Ok(StatusCode::OK)
}
