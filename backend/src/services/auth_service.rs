//! Authentication service - business logic for auth operations

use crate::{api::auth::*, auth::{self, UserInfo}, AppState};
use anyhow::anyhow;
use chrono::Utc;
use uuid::Uuid;

pub struct AuthService;

pub async fn register(state: &AppState, username: String, password: String) -> Result<AuthResponse, anyhow::Error> {
    // Check if registration is enabled
    let registration_enabled: Option<(i32,)> = sqlx::query_as(
        "SELECT allow_registration FROM system_settings WHERE id = 1"
    )
    .fetch_optional(&state.db_pool)
    .await
    .ok()
    .flatten();
    
    if registration_enabled.map(|(e,)| e == 0).unwrap_or(true) {
        return Err(anyhow!("User registration is currently disabled"));
    }
    
    if username.is_empty() || password.is_empty() { return Err(anyhow!("Username and password required")); }
    if username.len() < 3 { return Err(anyhow!("Username must be at least 3 characters")); }
    if password.len() < 6 { return Err(anyhow!("Password must be at least 6 characters")); }
    
    let user = state.user_db.create_user(username.clone(), password.clone()).map_err(|e| anyhow!("Registration failed: {}", e))?;
    
    let password_hash = {
        use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2};
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default().hash_password(password.as_bytes(), &salt).map_err(|e| anyhow!("Password hashing failed: {}", e))?.to_string()
    };
    
    let now = Utc::now().to_rfc3339();
    let _ = sqlx::query("INSERT INTO users (id, username, password_hash, totp_enabled, storage_quota_bytes, storage_used_bytes, default_view, language, theme, created_at, updated_at) VALUES (?, ?, ?, 0, 10737418240, 0, 'grid', 'de', 'light', ?, ?)")
        .bind(user.id.to_string()).bind(&username).bind(&password_hash).bind(&now).bind(&now).execute(&state.db_pool).await;
    
    let token = auth::generate_token(&user).map_err(|e| anyhow!("Token generation failed: {}", e))?;
    let refresh_token = auth::generate_refresh_token(&user).map_err(|e| anyhow!("Refresh token generation failed: {}", e))?;
    let csrf_token = crate::security::generate_csrf_token();
    
    Ok(AuthResponse { token, refresh_token: Some(refresh_token), user: UserInfo { id: user.id.to_string(), username: user.username, totp_enabled: user.totp_enabled }, requires_2fa: false, csrf_token })
}

pub async fn login(state: &AppState, username: String, password: String, totp_code: Option<String>) -> Result<AuthResponse, anyhow::Error> {
    if !state.rate_limiter.check_rate_limit(&username, 5, 60) { return Err(anyhow!("Too many login attempts. Please try again later.")); }
    
    let mut user = state.user_db.verify_password(&username, &password).map_err(|e| anyhow!("Invalid credentials: {}", e))?;
    
    if user.totp_enabled {
        if let Some(code) = totp_code {
            if let Some(ref secret) = user.totp_secret {
                if !auth::verify_totp_code(secret, &code) { return Err(anyhow!("Invalid 2FA code")); }
            } else { return Err(anyhow!("2FA enabled but no secret configured")); }
        } else { return Err(anyhow!("2FA code required")); }
    }
    
    user.last_login = Some(Utc::now());
    state.user_db.update_user(user.clone());
    
    let now = Utc::now().to_rfc3339();
    let _ = sqlx::query("UPDATE users SET last_login = ?, updated_at = ? WHERE id = ?").bind(&now).bind(&now).bind(user.id.to_string()).execute(&state.db_pool).await;
    
    let token = auth::generate_token(&user).map_err(|e| anyhow!("Token generation failed: {}", e))?;
    let refresh_token = auth::generate_refresh_token(&user).map_err(|e| anyhow!("Refresh token generation failed: {}", e))?;
    let csrf_token = crate::security::generate_csrf_token();
    
    Ok(AuthResponse { token, refresh_token: Some(refresh_token), user: UserInfo { id: user.id.to_string(), username: user.username.clone(), totp_enabled: user.totp_enabled }, requires_2fa: false, csrf_token })
}

pub async fn change_password(state: &AppState, user: &UserInfo, old_password: String, new_password: String) -> Result<(), anyhow::Error> {
    if new_password.len() < 6 { return Err(anyhow!("Password must be at least 6 characters")); }
    
    let user_id = Uuid::parse_str(&user.id).map_err(|_| anyhow!("Invalid user ID"))?;
    state.user_db.change_password(user_id, &old_password, &new_password).map_err(|e| anyhow!("Password change failed: {}", e))?;
    
    let db_user = state.user_db.get_by_id(&user_id).ok_or_else(|| anyhow!("User not found"))?;
    let now = Utc::now().to_rfc3339();
    let _ = sqlx::query("UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?").bind(&db_user.password_hash).bind(&now).bind(user_id.to_string()).execute(&state.db_pool).await;
    
    Ok(())
}

pub async fn setup_2fa(state: &AppState, user: &UserInfo) -> Result<Setup2FAResponse, anyhow::Error> {
    let secret = auth::generate_totp_secret();
    let qr_url = format!("otpauth://totp/SyncSpace:{}?secret={}&issuer=SyncSpace", user.username, secret);
    Ok(Setup2FAResponse { 
        secret: secret.clone(), 
        qr_code_url: qr_url.clone(), 
        qr_url 
    })
}

pub async fn enable_2fa(state: &AppState, user: &UserInfo, totp_code: String) -> Result<(), anyhow::Error> {
    let user_id = Uuid::parse_str(&user.id).map_err(|_| anyhow!("Invalid user ID"))?;
    let mut db_user = state.user_db.get_by_id(&user_id).ok_or_else(|| anyhow!("User not found"))?;
    
    let secret = db_user.totp_secret.clone().unwrap_or_else(auth::generate_totp_secret);
    if !auth::verify_totp_code(&secret, &totp_code) { return Err(anyhow!("Invalid TOTP code")); }
    
    db_user.totp_secret = Some(secret.clone());
    db_user.totp_enabled = true;
    state.user_db.update_user(db_user.clone());
    
    let now = Utc::now().to_rfc3339();
    let _ = sqlx::query("UPDATE users SET totp_secret = ?, totp_enabled = 1, updated_at = ? WHERE id = ?").bind(&secret).bind(&now).bind(user_id.to_string()).execute(&state.db_pool).await;
    
    Ok(())
}

pub async fn disable_2fa(state: &AppState, user: &UserInfo, password: String) -> Result<(), anyhow::Error> {
    let user_id = Uuid::parse_str(&user.id).map_err(|_| anyhow!("Invalid user ID"))?;
    let mut db_user = state.user_db.get_by_id(&user_id).ok_or_else(|| anyhow!("User not found"))?;
    
    state.user_db.verify_password(&user.username, &password).map_err(|e| anyhow!("Invalid password: {}", e))?;
    
    db_user.totp_enabled = false;
    db_user.totp_secret = None;
    state.user_db.update_user(db_user.clone());
    
    let now = Utc::now().to_rfc3339();
    let _ = sqlx::query("UPDATE users SET totp_enabled = 0, totp_secret = NULL, updated_at = ? WHERE id = ?").bind(&now).bind(user_id.to_string()).execute(&state.db_pool).await;
    
    Ok(())
}

pub async fn refresh_token(state: &AppState, user: &UserInfo) -> Result<AuthResponse, anyhow::Error> {
    let user_id = Uuid::parse_str(&user.id).map_err(|_| anyhow!("Invalid user ID"))?;
    let db_user = state.user_db.get_by_id(&user_id).ok_or_else(|| anyhow!("User not found"))?;
    
    let token = auth::generate_token(&db_user).map_err(|e| anyhow!("Token generation failed: {}", e))?;
    let new_refresh_token = auth::generate_refresh_token(&db_user).map_err(|e| anyhow!("Refresh token generation failed: {}", e))?;
    let csrf_token = crate::security::generate_csrf_token();
    
    Ok(AuthResponse { token, refresh_token: Some(new_refresh_token), user: UserInfo { id: db_user.id.to_string(), username: db_user.username.clone(), totp_enabled: db_user.totp_enabled }, requires_2fa: false, csrf_token })
}
