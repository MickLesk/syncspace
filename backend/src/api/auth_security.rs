//! Authentication security endpoints
//! Handles session management, login attempt history, password policy

use crate::auth::UserInfo;
use crate::services::auth_security_service;
use crate::database::UserSession;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/sessions", get(list_sessions))
        .route("/auth/sessions/{session_id}", delete(revoke_session))
        .route("/auth/sessions/revoke-all", post(revoke_all_sessions))
        .route("/auth/login-attempts", get(list_login_attempts))
        .route("/auth/password-policy", get(get_password_policy))
        .route("/auth/validate-password", post(validate_password))
}

// ==================== SESSION MANAGEMENT ====================

/// GET /api/auth/sessions - List active sessions for current user
async fn list_sessions(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<SessionResponse>>, StatusCode> {
    let sessions = auth_security_service::get_user_sessions(&state.db_pool, &user.id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get sessions: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let response: Vec<SessionResponse> = sessions.into_iter().map(|s| SessionResponse {
        id: s.id,
        ip_address: s.ip_address,
        user_agent: s.user_agent,
        created_at: s.created_at,
        last_active_at: s.last_active_at,
        expires_at: s.expires_at,
        is_current: false, // TODO: Compare with current session token
    }).collect();
    
    Ok(Json(response))
}

/// DELETE /api/auth/sessions/:session_id - Revoke a specific session
async fn revoke_session(
    State(state): State<AppState>,
    user: UserInfo,
    Path(session_id): Path<String>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    // Verify session belongs to user
    let sessions = auth_security_service::get_user_sessions(&state.db_pool, &user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !sessions.iter().any(|s| s.id == session_id) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    auth_security_service::revoke_session(&state.db_pool, &session_id, "user_logout")
        .await
        .map_err(|e| {
            eprintln!("Failed to revoke session: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(SuccessResponse {
        success: true,
        message: "Session revoked successfully".to_string(),
    }))
}

/// POST /api/auth/sessions/revoke-all - Revoke all sessions (except current)
async fn revoke_all_sessions(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<SuccessResponse>, StatusCode> {
    auth_security_service::revoke_all_user_sessions(&state.db_pool, &user.id, "user_revoke_all")
        .await
        .map_err(|e| {
            eprintln!("Failed to revoke all sessions: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(SuccessResponse {
        success: true,
        message: "All sessions revoked successfully".to_string(),
    }))
}

// ==================== LOGIN ATTEMPTS ====================

/// GET /api/auth/login-attempts - List recent login attempts for current user
async fn list_login_attempts(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<LoginAttemptResponse>>, StatusCode> {
    let username = sqlx::query_scalar::<_, String>(
        "SELECT username FROM users WHERE id = ?"
    )
    .bind(&user.id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let attempts = sqlx::query_as::<_, crate::database::LoginAttempt>(
        "SELECT * FROM login_attempts WHERE username = ? ORDER BY attempted_at DESC LIMIT 50"
    )
    .bind(&username)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get login attempts: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let response: Vec<LoginAttemptResponse> = attempts.into_iter().map(|a| LoginAttemptResponse {
        ip_address: a.ip_address,
        user_agent: a.user_agent,
        success: a.success == 1,
        failure_reason: a.failure_reason,
        attempted_at: a.attempted_at,
    }).collect();
    
    Ok(Json(response))
}

// ==================== PASSWORD POLICY ====================

/// GET /api/auth/password-policy - Get password policy requirements
async fn get_password_policy() -> Json<PasswordPolicyResponse> {
    Json(PasswordPolicyResponse {
        min_length: 8,
        require_uppercase: true,
        require_lowercase: true,
        require_number: true,
        require_special_char: true,
        disallow_common: true,
        max_password_history: 5,
    })
}

/// POST /api/auth/validate-password - Validate password against policy
async fn validate_password(
    Json(req): Json<ValidatePasswordRequest>,
) -> Result<Json<ValidatePasswordResponse>, StatusCode> {
    match auth_security_service::validate_password_policy(&req.password) {
        Ok(_) => Ok(Json(ValidatePasswordResponse {
            valid: true,
            errors: vec![],
        })),
        Err(e) => Ok(Json(ValidatePasswordResponse {
            valid: false,
            errors: vec![e.to_string()],
        })),
    }
}

// ==================== RESPONSE TYPES ====================

#[derive(Debug, Serialize)]
struct SessionResponse {
    id: String,
    ip_address: String,
    user_agent: Option<String>,
    created_at: String,
    last_active_at: String,
    expires_at: String,
    is_current: bool,
}

#[derive(Debug, Serialize)]
struct LoginAttemptResponse {
    ip_address: String,
    user_agent: Option<String>,
    success: bool,
    failure_reason: Option<String>,
    attempted_at: String,
}

#[derive(Debug, Serialize)]
struct PasswordPolicyResponse {
    min_length: usize,
    require_uppercase: bool,
    require_lowercase: bool,
    require_number: bool,
    require_special_char: bool,
    disallow_common: bool,
    max_password_history: usize,
}

#[derive(Debug, Deserialize)]
struct ValidatePasswordRequest {
    password: String,
}

#[derive(Debug, Serialize)]
struct ValidatePasswordResponse {
    valid: bool,
    errors: Vec<String>,
}

#[derive(Debug, Serialize)]
struct SuccessResponse {
    success: bool,
    message: String,
}
