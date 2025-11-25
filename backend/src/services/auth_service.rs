//! Authentication service - business logic for auth operations

use crate::{
    api::auth::*,
    auth::{self, UserInfo},
    AppState,
};
use anyhow::anyhow;
use chrono::Utc;

#[allow(dead_code)]
pub struct AuthService;

pub async fn register(
    state: &AppState,
    username: String,
    password: String,
) -> Result<AuthResponse, anyhow::Error> {
    // Check if registration is enabled
    let registration_enabled: Option<(i32,)> =
        sqlx::query_as("SELECT allow_registration FROM system_settings WHERE id = 1")
            .fetch_optional(&state.db_pool)
            .await
            .ok()
            .flatten();

    if registration_enabled.map(|(e,)| e == 0).unwrap_or(true) {
        return Err(anyhow!("User registration is currently disabled"));
    }

    // Validate inputs
    if username.is_empty() || password.is_empty() {
        return Err(anyhow!("Username and password required"));
    }
    if username.len() < 3 {
        return Err(anyhow!("Username must be at least 3 characters"));
    }

    // Validate username format
    crate::security::validate_username(&username).map_err(|_| {
        anyhow!("Invalid username format (3-32 alphanumeric, underscore, hyphen only)")
    })?;

    // Validate password strength
    crate::security::validate_password_strength(&password)
        .map_err(|e| anyhow!("Weak password: {}", e))?;

    // Check if username already exists
    if auth::get_user_by_username(&state.db_pool, &username)
        .await?
        .is_some()
    {
        return Err(anyhow!("Username already exists"));
    }

    // Hash password
    let password_hash = {
        use argon2::{
            password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
            Argon2,
        };
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Password hashing failed: {}", e))?
            .to_string()
    };

    // Create user in database using helper
    let user = crate::database::User::create(&state.db_pool, username.clone(), password_hash)
        .await
        .map_err(|e| anyhow!("Failed to create user: {}", e))?;

    // Generate tokens
    let token =
        auth::generate_token(&user).map_err(|e| anyhow!("Token generation failed: {}", e))?;
    let refresh_token = auth::generate_refresh_token(&user)
        .map_err(|e| anyhow!("Refresh token generation failed: {}", e))?;
    let csrf_token = crate::security::generate_csrf_token();

    // Store refresh token in database
    auth::store_refresh_token(&state.db_pool, &user.id, &refresh_token, None, None)
        .await
        .map_err(|e| anyhow!("Failed to store refresh token: {}", e))?;

    Ok(AuthResponse {
        token,
        refresh_token: Some(refresh_token),
        user: UserInfo {
            id: user.id.clone(),
            username: user.username.clone(),
            totp_enabled: user.totp_enabled,
        },
        requires_2fa: false,
        csrf_token,
    })
}

pub async fn login(
    state: &AppState,
    username: String,
    password: String,
    totp_code: Option<String>,
) -> Result<AuthResponse, anyhow::Error> {
    // Rate limiting
    if !state.rate_limiter.check_rate_limit(&username, 5, 60) {
        return Err(anyhow!("Too many login attempts. Please try again later."));
    }

    // Check if account is locked
    if let Some(lockout) =
        crate::services::auth_security_service::is_account_locked(&state.db_pool, &username).await?
    {
        // Log failed attempt with lockout reason
        let _ = crate::services::auth_security_service::log_login_attempt(
            &state.db_pool,
            &username,
            "127.0.0.1", // IP passed via API layer - needs ConnectInfo extractor
            None,
            false,
            Some("account_locked"),
        )
        .await;

        return Err(anyhow!(
            "Account is locked due to too many failed login attempts. Please try again after {}",
            lockout.locked_until
        ));
    }

    // Verify password against SQLite database
    let user = match auth::verify_password(&state.db_pool, &username, &password).await {
        Ok(user) => user,
        Err(e) => {
            // Log failed login attempt
            let _ = crate::services::auth_security_service::log_login_attempt(
                &state.db_pool,
                &username,
                "127.0.0.1", // IP passed via API layer - needs ConnectInfo extractor
                None,
                false,
                Some("invalid_password"),
            )
            .await;

            // Check if account should be locked
            let _ = crate::services::auth_security_service::check_and_lock_account(
                &state.db_pool,
                &username,
            )
            .await;

            return Err(anyhow!("Invalid credentials: {}", e));
        }
    };

    // Check 2FA if enabled
    if user.totp_enabled {
        if let Some(code) = totp_code {
            if let Some(ref secret) = user.totp_secret {
                if !auth::verify_totp_code(secret, &code) {
                    // Log failed 2FA attempt
                    let _ = crate::services::auth_security_service::log_login_attempt(
                        &state.db_pool,
                        &username,
                        "127.0.0.1", // IP passed via API layer - needs ConnectInfo extractor
                        None,
                        false,
                        Some("2fa_failed"),
                    )
                    .await;

                    return Err(anyhow!("Invalid 2FA code"));
                }
            } else {
                return Err(anyhow!("2FA enabled but no secret configured"));
            }
        } else {
            return Err(anyhow!("2FA code required"));
        }
    }

    // Login successful - log it
    let _ = crate::services::auth_security_service::log_login_attempt(
        &state.db_pool,
        &username,
        "127.0.0.1", // IP passed via API layer - needs ConnectInfo extractor
        None,
        true,
        None,
    )
    .await;

    // Reset failed login attempts counter
    let _ = crate::services::auth_security_service::reset_failed_attempts(&state.db_pool, &user.id)
        .await;

    // Update last_login in database using helper
    crate::database::User::update_last_login(&state.db_pool, &user.id)
        .await
        .map_err(|e| anyhow!("Failed to update last login: {}", e))?;

    // Generate tokens
    let token =
        auth::generate_token(&user).map_err(|e| anyhow!("Token generation failed: {}", e))?;
    let refresh_token = auth::generate_refresh_token(&user)
        .map_err(|e| anyhow!("Refresh token generation failed: {}", e))?;
    let csrf_token = crate::security::generate_csrf_token();

    // Store refresh token in database
    auth::store_refresh_token(&state.db_pool, &user.id, &refresh_token, None, None)
        .await
        .map_err(|e| anyhow!("Failed to store refresh token: {}", e))?;

    // Create session record
    let expires_at = Utc::now() + chrono::Duration::days(7);
    let _ = crate::services::auth_security_service::create_session(
        &state.db_pool,
        &user.id,
        &token,    // Using JWT token as session token
        "127.0.0.1", // IP passed via API layer - needs ConnectInfo extractor
        None,      // TODO: Extract user agent
        expires_at,
    )
    .await;

    Ok(AuthResponse {
        token,
        refresh_token: Some(refresh_token),
        user: UserInfo {
            id: user.id.clone(),
            username: user.username.clone(),
            totp_enabled: user.totp_enabled,
        },
        requires_2fa: false,
        csrf_token,
    })
}

pub async fn change_password(
    state: &AppState,
    user: &UserInfo,
    old_password: String,
    new_password: String,
) -> Result<(), anyhow::Error> {
    if new_password.len() < 6 {
        return Err(anyhow!("Password must be at least 6 characters"));
    }

    // Validate password strength using security module
    crate::security::validate_password_strength(&new_password)
        .map_err(|e| anyhow!("Weak password: {}", e))?;

    // Validate against password policy and history
    crate::services::auth_security_service::validate_password_change(
        &state.db_pool,
        &user.id,
        &new_password,
    )
    .await
    .map_err(|e| anyhow!("Password validation failed: {}", e))?;

    // Change password in SQLite database (this also hashes the password internally)
    auth::change_user_password(&state.db_pool, &user.id, &old_password, &new_password)
        .await
        .map_err(|e| anyhow!("Password change failed: {}", e))?;

    // Get the newly hashed password from database for history tracking
    let updated_user = auth::get_user_by_id(&state.db_pool, &user.id)
        .await
        .map_err(|e| anyhow!("Failed to fetch updated user: {}", e))?
        .ok_or_else(|| anyhow!("User not found after password change"))?;

    // Add to password history
    crate::services::auth_security_service::update_password_with_history(
        &state.db_pool,
        &user.id,
        &updated_user.password_hash,
    )
    .await
    .map_err(|e| anyhow!("Failed to update password history: {}", e))?;

    // Revoke all user sessions on password change (security best practice)
    crate::services::auth_security_service::revoke_all_user_sessions(
        &state.db_pool,
        &user.id,
        "password_changed",
    )
    .await
    .map_err(|e| anyhow!("Failed to revoke sessions: {}", e))?;

    // Revoke all refresh tokens on password change (security best practice)
    auth::revoke_all_user_tokens(&state.db_pool, &user.id)
        .await
        .map_err(|e| anyhow!("Failed to revoke tokens: {}", e))?;

    Ok(())
}

pub async fn setup_2fa(
    state: &AppState,
    user: &UserInfo,
) -> Result<Setup2FAResponse, anyhow::Error> {
    let secret = auth::generate_totp_secret();
    let qr_url = format!(
        "otpauth://totp/SyncSpace:{}?secret={}&issuer=SyncSpace",
        user.username, secret
    );

    // Log 2FA setup attempt
    let _ = sqlx::query(
        "INSERT INTO activity_log (id, user_id, action, status, created_at) VALUES (?, ?, 'setup_2fa_initiated', 'pending', datetime('now'))"
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(&user.id)
    .execute(&state.db_pool)
    .await;

    Ok(Setup2FAResponse {
        secret: secret.clone(),
        qr_code_url: qr_url.clone(),
        qr_url,
    })
}

pub async fn enable_2fa(
    state: &AppState,
    user: &UserInfo,
    totp_code: String,
) -> Result<(), anyhow::Error> {
    // Get user from SQLite
    let db_user = auth::get_user_by_id(&state.db_pool, &user.id)
        .await
        .map_err(|e| anyhow!("Database error: {}", e))?
        .ok_or_else(|| anyhow!("User not found"))?;

    let secret = db_user
        .totp_secret
        .clone()
        .unwrap_or_else(auth::generate_totp_secret);

    // Verify TOTP code
    if !auth::verify_totp_code(&secret, &totp_code) {
        return Err(anyhow!("Invalid TOTP code"));
    }

    // Update in SQLite database
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE users SET totp_secret = ?, totp_enabled = 1, updated_at = ? WHERE id = ?")
        .bind(&secret)
        .bind(&now)
        .bind(&user.id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| anyhow!("Failed to enable 2FA: {}", e))?;

    Ok(())
}

pub async fn disable_2fa(
    state: &AppState,
    user: &UserInfo,
    password: String,
) -> Result<(), anyhow::Error> {
    // Verify password before disabling 2FA
    auth::verify_password(&state.db_pool, &user.username, &password)
        .await
        .map_err(|e| anyhow!("Invalid password: {}", e))?;

    // Update in SQLite database
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE users SET totp_enabled = 0, totp_secret = NULL, updated_at = ? WHERE id = ?",
    )
    .bind(&now)
    .bind(&user.id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| anyhow!("Failed to disable 2FA: {}", e))?;

    Ok(())
}

pub async fn refresh_token(
    state: &AppState,
    user: &UserInfo,
) -> Result<AuthResponse, anyhow::Error> {
    // Note: This endpoint currently uses access token for authentication
    // In production, you should pass refresh_token in request body instead

    // Get user from SQLite to ensure latest data and token_version
    let db_user = auth::get_user_by_id(&state.db_pool, &user.id)
        .await
        .map_err(|e| anyhow!("Database error: {}", e))?
        .ok_or_else(|| anyhow!("User not found"))?;

    // Generate new tokens
    let token =
        auth::generate_token(&db_user).map_err(|e| anyhow!("Token generation failed: {}", e))?;
    let new_refresh_token = auth::generate_refresh_token(&db_user)
        .map_err(|e| anyhow!("Refresh token generation failed: {}", e))?;
    let csrf_token = crate::security::generate_csrf_token();

    // Store new refresh token in database (token rotation)
    auth::store_refresh_token(&state.db_pool, &db_user.id, &new_refresh_token, None, None)
        .await
        .map_err(|e| anyhow!("Failed to store refresh token: {}", e))?;

    // Note: In production, you'd also revoke the old refresh token using:
    // auth::revoke_refresh_token(&state.db_pool, &old_refresh_token).await?;

    Ok(AuthResponse {
        token,
        refresh_token: Some(new_refresh_token),
        user: UserInfo {
            id: db_user.id.clone(),
            username: db_user.username.clone(),
            totp_enabled: db_user.totp_enabled,
        },
        requires_2fa: false,
        csrf_token,
    })
}

pub async fn logout(state: &AppState, user: &UserInfo) -> Result<(), anyhow::Error> {
    // Revoke all refresh tokens for this user (logs out from all devices)
    auth::revoke_all_user_tokens(&state.db_pool, &user.id)
        .await
        .map_err(|e| anyhow!("Failed to revoke tokens: {}", e))?;

    Ok(())
}

