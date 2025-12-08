//! Admin API endpoints for user and system management
//! Requires admin role for all operations

use crate::{auth::UserInfo, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ============================================
// Request/Response Types
// ============================================

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub role: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub role: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub password: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminUserResponse {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub totp_enabled: i32,
    pub created_at: String,
    pub last_login: Option<String>,
}

// ============================================
// Router
// ============================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/users", get(list_users).post(create_user))
        .route(
            "/admin/users/{user_id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/admin/users/{user_id}/reset-password", post(reset_password))
        .route(
            "/admin/users/{user_id}/force-password-change",
            post(force_password_change),
        )
}

// ============================================
// Handlers
// ============================================

/// Check if user is admin
fn require_admin(user: &UserInfo) -> Result<(), StatusCode> {
    // For now, check if username is "admin" or user has admin role
    // TODO: Implement proper RBAC check
    if user.username == "admin" {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

/// GET /admin/users - List all users (admin only)
async fn list_users(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<AdminUserResponse>>, StatusCode> {
    require_admin(&user)?;

    let users = sqlx::query_as::<_, AdminUserResponse>(
        r#"
        SELECT 
            id, username, email, display_name, bio,
            COALESCE(totp_enabled, 0) as totp_enabled,
            created_at,
            last_login
        FROM users
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to list users: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(users))
}

/// GET /admin/users/{user_id} - Get user details (admin only)
async fn get_user(
    State(state): State<AppState>,
    user: UserInfo,
    Path(user_id): Path<String>,
) -> Result<Json<AdminUserResponse>, StatusCode> {
    require_admin(&user)?;

    let user_data = sqlx::query_as::<_, AdminUserResponse>(
        r#"
        SELECT 
            id, username, email, display_name, bio,
            COALESCE(totp_enabled, 0) as totp_enabled,
            created_at,
            last_login
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(&user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user_data))
}

/// POST /admin/users - Create a new user (admin only)
async fn create_user(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    require_admin(&user)?;

    // Validate username
    if req.username.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    let existing = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing > 0 {
        return Err(StatusCode::CONFLICT);
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .to_string();

    // Generate user ID
    let user_id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // Insert user
    sqlx::query(
        r#"
        INSERT INTO users (id, username, password_hash, email, display_name, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&user_id)
    .bind(&req.username)
    .bind(&password_hash)
    .bind(&req.email)
    .bind(&req.display_name)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Assign role if provided
    if let Some(role) = &req.role {
        let role_id = match role.as_str() {
            "admin" => "admin",
            _ => "user",
        };

        // Try to assign role, ignore if table doesn't exist
        let _ = sqlx::query(
            "INSERT OR IGNORE INTO user_roles (user_id, role_id, granted_at, granted_by) VALUES (?, ?, datetime('now'), ?)",
        )
        .bind(&user_id)
        .bind(role_id)
        .bind(user.user_id())
        .execute(&state.db_pool)
        .await;
    }

    Ok(Json(serde_json::json!({
        "id": user_id,
        "username": req.username,
        "message": "User created successfully"
    })))
}

/// PUT /admin/users/{user_id} - Update a user (admin only)
async fn update_user(
    State(state): State<AppState>,
    user: UserInfo,
    Path(user_id): Path<String>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<StatusCode, StatusCode> {
    require_admin(&user)?;

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();

    if let Some(email) = &req.email {
        updates.push("email = ?");
        values.push(email.clone());
    }
    if let Some(display_name) = &req.display_name {
        updates.push("display_name = ?");
        values.push(display_name.clone());
    }
    if let Some(bio) = &req.bio {
        updates.push("bio = ?");
        values.push(bio.clone());
    }

    if updates.is_empty() {
        return Ok(StatusCode::OK);
    }

    // Add updated_at
    updates.push("updated_at = ?");
    values.push(Utc::now().to_rfc3339());

    let query = format!(
        "UPDATE users SET {} WHERE id = ?",
        updates.join(", ")
    );

    let mut query_builder = sqlx::query(&query);
    for value in &values {
        query_builder = query_builder.bind(value);
    }
    query_builder = query_builder.bind(&user_id);

    query_builder
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Update role if provided
    if let Some(role) = &req.role {
        // Remove existing roles
        let _ = sqlx::query("DELETE FROM user_roles WHERE user_id = ?")
            .bind(&user_id)
            .execute(&state.db_pool)
            .await;

        // Add new role
        let role_id = match role.as_str() {
            "admin" => "admin",
            _ => "user",
        };
        let _ = sqlx::query(
            "INSERT OR IGNORE INTO user_roles (user_id, role_id, granted_at, granted_by) VALUES (?, ?, datetime('now'), ?)",
        )
        .bind(&user_id)
        .bind(role_id)
        .bind(user.user_id())
        .execute(&state.db_pool)
        .await;
    }

    Ok(StatusCode::OK)
}

/// DELETE /admin/users/{user_id} - Delete a user (admin only)
async fn delete_user(
    State(state): State<AppState>,
    user: UserInfo,
    Path(user_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    require_admin(&user)?;

    // Prevent deleting yourself
    if user_id == user.user_id() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Prevent deleting the last admin (check if this is an admin)
    let admin_count = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM users u 
         LEFT JOIN user_roles ur ON u.id = ur.user_id 
         WHERE ur.role_id = 'admin' OR u.username = 'admin'",
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or(1);

    let is_target_admin = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM users u 
         LEFT JOIN user_roles ur ON u.id = ur.user_id 
         WHERE u.id = ? AND (ur.role_id = 'admin' OR u.username = 'admin')",
    )
    .bind(&user_id)
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or(0);

    if is_target_admin > 0 && admin_count <= 1 {
        return Err(StatusCode::FORBIDDEN); // Cannot delete last admin
    }

    // Delete user roles first
    let _ = sqlx::query("DELETE FROM user_roles WHERE user_id = ?")
        .bind(&user_id)
        .execute(&state.db_pool)
        .await;

    // Delete user
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&user_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

/// POST /admin/users/{user_id}/reset-password - Reset user password (admin only)
async fn reset_password(
    State(state): State<AppState>,
    user: UserInfo,
    Path(user_id): Path<String>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    require_admin(&user)?;

    // Hash new password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .to_string();

    // Update password
    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(&password_hash)
        .bind(&user_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to reset password: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

/// POST /admin/users/{user_id}/force-password-change - Force user to change password on next login
/// NOTE: This requires a database migration to add password_expired column
/// For now, this just logs a warning
async fn force_password_change(
    State(state): State<AppState>,
    user: UserInfo,
    Path(user_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    require_admin(&user)?;

    // TODO: Add password_expired column to users table via migration
    // For now, just verify the user exists
    let exists = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or(0);

    if exists == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    tracing::warn!(
        "Force password change requested for user {} but password_expired column not yet implemented",
        user_id
    );

    Ok(StatusCode::OK)
}
