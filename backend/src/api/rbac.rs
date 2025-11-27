use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/roles", get(list_roles).post(create_role))
        .route(
            "/roles/:id",
            get(get_role).put(update_role).delete(delete_role),
        )
        .route("/roles/:id/permissions", get(get_role_permissions))
        .route(
            "/users/:user_id/roles",
            get(get_user_roles).post(assign_user_role),
        )
        .route("/users/:user_id/roles/:role_id", delete(revoke_user_role))
        .route("/users/:user_id/permissions", get(get_user_permissions))
        .route("/permissions/available", get(list_available_permissions))
        .route("/permissions/audit", get(get_permission_audit))
}

#[derive(Debug, Serialize, FromRow)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub permissions: String, // JSON array
    pub is_system: i64,
    pub is_default: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserRole {
    pub user_id: String,
    pub role_id: String,
    pub granted_by: Option<String>,
    pub granted_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserRoleWithDetails {
    pub user_id: String,
    pub role_id: String,
    pub role_name: String,
    pub role_display_name: String,
    pub permissions: String,
    pub granted_by: Option<String>,
    pub granted_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PermissionAudit {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub target_user_id: Option<String>,
    pub role_id: Option<String>,
    pub role_name: Option<String>,
    pub permissions_before: Option<String>,
    pub permissions_after: Option<String>,
    pub performed_by: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListRolesQuery {
    pub include_system: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub limit: Option<i64>,
}

// List all roles
async fn list_roles(
    State(state): State<AppState>,
    UserInfo { id: _user_id, .. }: UserInfo,
    Query(query): Query<ListRolesQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let include_system = query.include_system.unwrap_or(true);

    let roles: Vec<Role> = if include_system {
        sqlx::query_as("SELECT * FROM roles ORDER BY is_system DESC, name ASC")
            .fetch_all(&state.db_pool)
            .await
    } else {
        sqlx::query_as("SELECT * FROM roles WHERE is_system = 0 ORDER BY name ASC")
            .fetch_all(&state.db_pool)
            .await
    }
    .map_err(|e| {
        eprintln!("Database error listing roles: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(roles))
}

// Get a specific role
async fn get_role(
    State(state): State<AppState>,
    UserInfo { id: _user_id, .. }: UserInfo,
    Path(role_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let role: Role = sqlx::query_as("SELECT * FROM roles WHERE id = ?")
        .bind(&role_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(role))
}

// Create a new role (admin only)
async fn create_role(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Json(req): Json<CreateRoleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if user has role.create permission
    if !has_permission(&state, &user_id, "role.create").await {
        return Err(StatusCode::FORBIDDEN);
    }

    let role_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let permissions_json = serde_json::to_string(&req.permissions).unwrap_or_default();

    sqlx::query(
        "INSERT INTO roles (id, name, display_name, description, permissions, is_system, is_default, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, 0, 0, ?, ?)"
    )
    .bind(&role_id)
    .bind(&req.name)
    .bind(&req.display_name)
    .bind(&req.description)
    .bind(&permissions_json)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error creating role: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Audit log
    log_permission_audit(
        &state,
        &user_id,
        "create_role",
        None,
        Some(&role_id),
        Some(&req.name),
        None,
        Some(&permissions_json),
        &user_id,
    )
    .await;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({ "id": role_id })),
    ))
}

// Update a role (admin only, system roles protected)
async fn update_role(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(role_id): Path<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check permission
    if !has_permission(&state, &user_id, "role.manage").await {
        return Err(StatusCode::FORBIDDEN);
    }

    // Get existing role
    let role: Role = sqlx::query_as("SELECT * FROM roles WHERE id = ?")
        .bind(&role_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Prevent modification of system roles
    if role.is_system == 1 {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now().to_rfc3339();
    let permissions_json = req
        .permissions
        .as_ref()
        .map(|p| serde_json::to_string(p).unwrap_or_default());

    sqlx::query(
        "UPDATE roles SET display_name = COALESCE(?, display_name), description = COALESCE(?, description),
         permissions = COALESCE(?, permissions), updated_at = ?
         WHERE id = ?"
    )
    .bind(&req.display_name)
    .bind(&req.description)
    .bind(&permissions_json)
    .bind(&now)
    .bind(&role_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Audit log
    log_permission_audit(
        &state,
        &user_id,
        "update_role",
        None,
        Some(&role_id),
        Some(&role.name),
        Some(&role.permissions),
        permissions_json.as_deref(),
        &user_id,
    )
    .await;

    Ok(StatusCode::OK)
}

// Delete a role (admin only, system roles protected)
async fn delete_role(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(role_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check permission
    if !has_permission(&state, &user_id, "role.delete").await {
        return Err(StatusCode::FORBIDDEN);
    }

    // Get role
    let role: Role = sqlx::query_as("SELECT * FROM roles WHERE id = ?")
        .bind(&role_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Prevent deletion of system roles
    if role.is_system == 1 {
        return Err(StatusCode::FORBIDDEN);
    }

    sqlx::query("DELETE FROM roles WHERE id = ?")
        .bind(&role_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Audit log
    log_permission_audit(
        &state,
        &user_id,
        "delete_role",
        None,
        Some(&role_id),
        Some(&role.name),
        Some(&role.permissions),
        None,
        &user_id,
    )
    .await;

    Ok(StatusCode::NO_CONTENT)
}

// Get role permissions
async fn get_role_permissions(
    State(state): State<AppState>,
    UserInfo { id: _user_id, .. }: UserInfo,
    Path(role_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let role: Role = sqlx::query_as("SELECT * FROM roles WHERE id = ?")
        .bind(&role_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let permissions: Vec<String> = serde_json::from_str(&role.permissions).unwrap_or_default();
    Ok(Json(permissions))
}

// Get user's roles
async fn get_user_roles(
    State(state): State<AppState>,
    UserInfo {
        id: requesting_user,
        ..
    }: UserInfo,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Users can view their own roles, admins can view any
    if user_id != requesting_user && !has_permission(&state, &requesting_user, "user.manage").await
    {
        return Err(StatusCode::FORBIDDEN);
    }

    let user_roles: Vec<UserRoleWithDetails> = sqlx::query_as(
        "SELECT ur.user_id, ur.role_id, r.name as role_name, r.display_name as role_display_name,
         r.permissions, ur.granted_by, ur.granted_at, ur.expires_at
         FROM user_roles ur
         JOIN roles r ON ur.role_id = r.id
         WHERE ur.user_id = ?
         ORDER BY r.is_system DESC, r.name ASC",
    )
    .bind(&user_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user_roles))
}

// Assign role to user
async fn assign_user_role(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(target_user_id): Path<String>,
    Json(req): Json<AssignRoleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check permission
    if !has_permission(&state, &user_id, "role.assign").await {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT OR REPLACE INTO user_roles (user_id, role_id, granted_by, granted_at, expires_at)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&target_user_id)
    .bind(&req.role_id)
    .bind(&user_id)
    .bind(&now)
    .bind(&req.expires_at)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get role name for audit
    let role: Role = sqlx::query_as("SELECT * FROM roles WHERE id = ?")
        .bind(&req.role_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Audit log
    log_permission_audit(
        &state,
        &target_user_id,
        "grant_role",
        Some(&target_user_id),
        Some(&req.role_id),
        Some(&role.name),
        None,
        Some(&role.permissions),
        &user_id,
    )
    .await;

    Ok(StatusCode::CREATED)
}

// Revoke role from user
async fn revoke_user_role(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path((target_user_id, role_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check permission
    if !has_permission(&state, &user_id, "role.assign").await {
        return Err(StatusCode::FORBIDDEN);
    }

    // Get role for audit
    let role: Option<Role> = sqlx::query_as("SELECT * FROM roles WHERE id = ?")
        .bind(&role_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("DELETE FROM user_roles WHERE user_id = ? AND role_id = ?")
        .bind(&target_user_id)
        .bind(&role_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Audit log
    if let Some(r) = role {
        log_permission_audit(
            &state,
            &target_user_id,
            "revoke_role",
            Some(&target_user_id),
            Some(&role_id),
            Some(&r.name),
            Some(&r.permissions),
            None,
            &user_id,
        )
        .await;
    }

    Ok(StatusCode::NO_CONTENT)
}

// Get user's effective permissions
async fn get_user_permissions(
    State(state): State<AppState>,
    UserInfo {
        id: requesting_user,
        ..
    }: UserInfo,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Users can view their own permissions, admins can view any
    if user_id != requesting_user && !has_permission(&state, &requesting_user, "user.manage").await
    {
        return Err(StatusCode::FORBIDDEN);
    }

    let permissions = get_user_all_permissions(&state, &user_id).await;
    Ok(Json(permissions))
}

// List available permissions
async fn list_available_permissions(
    State(_state): State<AppState>,
    UserInfo { id: _user_id, .. }: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let permissions = vec![
        // System permissions
        "system.admin",
        "system.config",
        // User management
        "user.manage",
        "user.create",
        "user.delete",
        "user.view",
        // Role management
        "role.manage",
        "role.create",
        "role.delete",
        "role.assign",
        // File operations
        "file.read",
        "file.write",
        "file.delete",
        "file.share",
        "file.download",
        // Share management
        "share.manage",
        "share.create",
        "share.delete",
        // Backup operations
        "backup.manage",
        "backup.create",
        "backup.restore",
        "backup.view",
        // Audit and settings
        "audit.view",
        "settings.manage",
        "settings.view",
    ];

    Ok(Json(permissions))
}

// Get permission audit log
async fn get_permission_audit(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Query(query): Query<AuditQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check permission
    if !has_permission(&state, &user_id, "audit.view").await {
        return Err(StatusCode::FORBIDDEN);
    }

    let limit = query.limit.unwrap_or(100);

    let mut sql = String::from("SELECT * FROM permission_audit WHERE 1=1");

    if let Some(uid) = &query.user_id {
        sql.push_str(&format!(" AND user_id = '{}'", uid));
    }

    if let Some(act) = &query.action {
        sql.push_str(&format!(" AND action = '{}'", act));
    }

    sql.push_str(&format!(" ORDER BY created_at DESC LIMIT {}", limit));

    let audit_logs: Vec<PermissionAudit> = sqlx::query_as(&sql)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(audit_logs))
}

// Helper: Check if user has permission
async fn has_permission(state: &AppState, user_id: &str, permission: &str) -> bool {
    let permissions = get_user_all_permissions(state, user_id).await;
    permissions.contains(&permission.to_string())
}

// Helper: Get all user permissions from all roles
async fn get_user_all_permissions(state: &AppState, user_id: &str) -> Vec<String> {
    let user_roles: Vec<UserRoleWithDetails> = sqlx::query_as(
        "SELECT ur.user_id, ur.role_id, r.name as role_name, r.display_name as role_display_name,
         r.permissions, ur.granted_by, ur.granted_at, ur.expires_at
         FROM user_roles ur
         JOIN roles r ON ur.role_id = r.id
         WHERE ur.user_id = ?",
    )
    .bind(user_id)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    let mut all_permissions = std::collections::HashSet::new();

    for user_role in user_roles {
        if let Ok(perms) = serde_json::from_str::<Vec<String>>(&user_role.permissions) {
            for perm in perms {
                all_permissions.insert(perm);
            }
        }
    }

    all_permissions.into_iter().collect()
}

// Helper: Log permission audit
async fn log_permission_audit(
    state: &AppState,
    user_id: &str,
    action: &str,
    target_user_id: Option<&str>,
    role_id: Option<&str>,
    role_name: Option<&str>,
    permissions_before: Option<&str>,
    permissions_after: Option<&str>,
    performed_by: &str,
) {
    let audit_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let _ = sqlx::query(
        "INSERT INTO permission_audit (id, user_id, action, target_user_id, role_id, role_name,
         permissions_before, permissions_after, performed_by, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&audit_id)
    .bind(user_id)
    .bind(action)
    .bind(target_user_id)
    .bind(role_id)
    .bind(role_name)
    .bind(permissions_before)
    .bind(permissions_after)
    .bind(performed_by)
    .bind(&now)
    .execute(&state.db_pool)
    .await;
}
