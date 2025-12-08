//! Authentication API endpoints
//! Handles login, registration, 2FA, password changes

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::auth::{User, UserInfo};
use crate::services;
use crate::AppState;

// ==================== REQUEST/RESPONSE TYPES ====================

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
    pub user: UserInfo,
    pub refresh_token: Option<String>,
    pub requires_2fa: bool,
    pub csrf_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct Enable2FARequest {
    pub totp_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Disable2FARequest {
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Setup2FAResponse {
    pub secret: String,
    pub qr_code_url: String,
    pub qr_url: String, // Alias for compatibility
}

// ==================== ROUTER ====================

/// Public authentication routes (no auth required)
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
}

/// Protected authentication routes (auth required)
pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/auth/me", get(me_handler))
        .route("/auth/change-password", post(change_password_handler))
        .route("/auth/logout", post(logout_handler))
        .route("/auth/2fa/setup", post(setup_2fa_handler))
        .route("/auth/2fa/enable", post(enable_2fa_handler))
        .route("/auth/2fa/disable", post(disable_2fa_handler))
        .route("/auth/refresh", post(refresh_token_handler))
}

/// Legacy function for backward compatibility (only public routes)
#[deprecated(note = "Use public_router() and protected_router() instead")]
#[allow(dead_code)]
pub fn router() -> Router<AppState> {
    public_router()
}

// ==================== HANDLERS ====================

/// Register a new user
#[tracing::instrument(skip(state, req), fields(username = %req.username))]
async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("User registration attempt");

    services::register(&state, req.username.clone(), req.password)
        .await
        .map(|response| {
            tracing::info!("User registered successfully: {}", req.username);
            Json(response)
        })
        .map_err(|e| {
            tracing::warn!("Registration failed for {}: {}", req.username, e);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        })
}

/// Login user
#[tracing::instrument(skip(state, req), fields(username = %req.username, has_2fa = req.totp_code.is_some()))]
async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("Login attempt");

    let username = req.username.clone();
    services::login(&state, req.username.clone(), req.password, req.totp_code)
        .await
        .map(|response| {
            tracing::info!("Login successful for: {}", username);
            // Log successful login
            let state_clone = state.clone();
            let user_id = response.user.id.clone();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::LOGIN,
                    "",
                    "",
                    None,
                    None,
                    "success",
                    None,
                    Some(serde_json::json!({ "method": "password" })),
                ).await;
            });
            Json(response)
        })
        .map_err(|e| {
            tracing::warn!("Login failed for {}: {}", username, e);
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": e.to_string()
                })),
            )
        })
}

/// Get current user info
#[tracing::instrument(skip(user), fields(user_id = %user.0.id))]
async fn me_handler(user: User) -> Json<UserInfo> {
    tracing::debug!("Getting current user info");
    Json(user.0)
}

/// Change password
#[tracing::instrument(skip(state, user, req), fields(user_id = %user.id))]
async fn change_password_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Password change attempt");

    let user_id = user.id.clone();
    services::change_password(&state, &user, req.old_password, req.new_password)
        .await
        .map(|_| {
            tracing::info!("Password changed successfully");
            // Log password change
            let state_clone = state.clone();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::PASSWORD_CHANGE,
                    "",
                    "",
                    None,
                    None,
                    "success",
                    None,
                    None,
                ).await;
            });
            StatusCode::OK
        })
        .map_err(|e| {
            tracing::warn!("Password change failed: {}", e);
            StatusCode::BAD_REQUEST
        })
}

/// Setup 2FA (generate secret)
async fn setup_2fa_handler(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Setup2FAResponse>, StatusCode> {
    services::setup_2fa(&state, &user)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Enable 2FA (verify TOTP code)
async fn enable_2fa_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<Enable2FARequest>,
) -> Result<StatusCode, StatusCode> {
    let user_id = user.id.clone();
    services::enable_2fa(&state, &user, req.totp_code)
        .await
        .map(|_| {
            // Log 2FA enable
            let state_clone = state.clone();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::TOTP_ENABLE,
                    "",
                    "",
                    None,
                    None,
                    "success",
                    None,
                    None,
                ).await;
            });
            StatusCode::OK
        })
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Disable 2FA
async fn disable_2fa_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<Disable2FARequest>,
) -> Result<StatusCode, StatusCode> {
    let user_id = user.id.clone();
    services::disable_2fa(&state, &user, req.password)
        .await
        .map(|_| {
            // Log 2FA disable
            let state_clone = state.clone();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::TOTP_DISABLE,
                    "",
                    "",
                    None,
                    None,
                    "success",
                    None,
                    None,
                ).await;
            });
            StatusCode::OK
        })
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

/// Refresh auth token
async fn refresh_token_handler(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<AuthResponse>, StatusCode> {
    services::refresh_token(&state, &user)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Logout user (revoke all refresh tokens)
#[tracing::instrument(skip(state, user), fields(user_id = %user.id))]
async fn logout_handler(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Logout attempt");

    let user_id = user.id.clone();
    services::logout(&state, &user)
        .await
        .map(|_| {
            tracing::info!("Logout successful");
            // Log logout
            let state_clone = state.clone();
            tokio::spawn(async move {
                let _ = crate::services::activity::log(
                    &state_clone,
                    &user_id,
                    crate::services::activity::actions::LOGOUT,
                    "",
                    "",
                    None,
                    None,
                    "success",
                    None,
                    None,
                ).await;
            });
            StatusCode::OK
        })
        .map_err(|e| {
            tracing::warn!("Logout failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
