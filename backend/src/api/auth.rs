//! Authentication API endpoints
//! Handles login, registration, 2FA, password changes

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
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
    pub qr_url: String,  // Alias for compatibility
}

// ==================== ROUTER ====================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/me", get(me_handler))
        .route("/auth/change-password", post(change_password_handler))
        .route("/auth/2fa/setup", post(setup_2fa_handler))
        .route("/auth/2fa/enable", post(enable_2fa_handler))
        .route("/auth/2fa/disable", post(disable_2fa_handler))
        .route("/auth/refresh", post(refresh_token_handler))
}

// ==================== HANDLERS ====================

/// Register a new user
async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    services::register(&state, req.username, req.password)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Login user
async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    services::login(&state, req.username, req.password, req.totp_code)
        .await
        .map(Json)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

/// Get current user info
async fn me_handler(user: User) -> Json<UserInfo> {
    Json(user.0)
}

/// Change password
async fn change_password_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    services::change_password(&state, &user, req.old_password, req.new_password)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
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
    services::enable_2fa(&state, &user, req.totp_code)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

/// Disable 2FA
async fn disable_2fa_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<Disable2FARequest>,
) -> Result<StatusCode, StatusCode> {
    services::disable_2fa(&state, &user, req.password)
        .await
        .map(|_| StatusCode::OK)
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
