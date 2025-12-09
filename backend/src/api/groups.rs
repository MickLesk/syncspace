use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserGroup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GroupMember {
    pub user_id: String,
    pub username: String,
    pub email: Option<String>,
    pub added_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role: String, // admin, moderator, user, guest
}

#[derive(Deserialize)]
struct SuspendRequest {
    reason: String,
    duration_days: Option<i64>,
}

/// List all groups
async fn list_groups(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let groups: Vec<UserGroup> = sqlx::query_as(
        "SELECT id, name, description, created_by, created_at, updated_at 
         FROM user_groups
         ORDER BY name ASC",
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(groups))
}

/// Create new group
async fn create_group(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateGroupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO user_groups (id, name, description, created_by, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&user.user_id())
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "id": id,
            "name": req.name,
            "message": "Group created successfully"
        })),
    ))
}

/// Get group details with members
async fn get_group(
    State(state): State<AppState>,
    Path(group_id): Path<String>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let group: Option<UserGroup> = sqlx::query_as(
        "SELECT id, name, description, created_by, created_at, updated_at 
         FROM user_groups WHERE id = ?",
    )
    .bind(&group_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let group = group.ok_or(StatusCode::NOT_FOUND)?;

    // Get members
    let members: Vec<GroupMember> = sqlx::query_as(
        "SELECT u.id as user_id, u.username, u.email, ugm.added_at
         FROM user_group_members ugm
         JOIN users u ON u.id = ugm.user_id
         WHERE ugm.group_id = ?
         ORDER BY u.username ASC",
    )
    .bind(&group_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "group": group,
        "members": members
    })))
}

/// Add member to group
async fn add_member(
    State(state): State<AppState>,
    Path(group_id): Path<String>,
    user: UserInfo,
    Json(req): Json<AddMemberRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO user_group_members (id, group_id, user_id, added_by, added_at)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&group_id)
    .bind(&req.user_id)
    .bind(&user.user_id())
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("UNIQUE constraint") {
            StatusCode::CONFLICT
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Member added successfully"
        })),
    ))
}

/// Remove member from group
async fn remove_member(
    State(state): State<AppState>,
    Path((group_id, user_id)): Path<(String, String)>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query("DELETE FROM user_group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group_id)
        .bind(&user_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Delete group
async fn delete_group(
    State(state): State<AppState>,
    Path(group_id): Path<String>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query("DELETE FROM user_groups WHERE id = ?")
        .bind(&group_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Assign role to user
async fn assign_role(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    user: UserInfo,
    Json(req): Json<AssignRoleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    // Validate role
    let valid_roles = ["admin", "moderator", "user", "guest", "readonly"];
    if !valid_roles.contains(&req.role.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Delete existing role assignment
    sqlx::query("DELETE FROM user_system_roles WHERE user_id = ? AND role = ?")
        .bind(&user_id)
        .bind(&req.role)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert new role
    sqlx::query(
        "INSERT INTO user_system_roles (id, user_id, role, assigned_by, assigned_at)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&user_id)
    .bind(&req.role)
    .bind(&user.user_id())
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": format!("Role '{}' assigned successfully", req.role)
        })),
    ))
}

/// Get user roles
async fn get_user_roles(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    _user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    #[derive(Serialize, FromRow)]
    struct RoleInfo {
        role: String,
        assigned_at: String,
    }

    let roles: Vec<RoleInfo> = sqlx::query_as(
        "SELECT role, assigned_at FROM user_system_roles WHERE user_id = ? ORDER BY assigned_at DESC"
    )
    .bind(&user_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(roles))
}

/// Suspend user
async fn suspend_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    user: UserInfo,
    Json(req): Json<SuspendRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();

    // Calculate expiration if duration provided
    let expires_at: Option<String> = req
        .duration_days
        .map(|days| (now + chrono::Duration::days(days)).to_rfc3339());

    sqlx::query(
        "INSERT INTO user_suspensions (id, user_id, reason, suspended_by, suspended_at, expires_at, is_active)
         VALUES (?, ?, ?, ?, ?, ?, 1)"
    )
    .bind(&id)
    .bind(&user_id)
    .bind(&req.reason)
    .bind(&user.user_id())
    .bind(&now_str)
    .bind(&expires_at)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update user status
    sqlx::query("UPDATE users SET status = 'suspended' WHERE id = ?")
        .bind(&user_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "User suspended successfully",
            "reason": req.reason,
            "expires_at": expires_at
        })),
    ))
}

/// Lift suspension
async fn unsuspend_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    user: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE user_suspensions SET is_active = 0, lifted_at = ?, lifted_by = ? 
         WHERE user_id = ? AND is_active = 1",
    )
    .bind(&now)
    .bind(&user.user_id())
    .bind(&user_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("UPDATE users SET status = 'active' WHERE id = ?")
        .bind(&user_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Suspension lifted successfully"
        })),
    ))
}

/// Router
pub fn router() -> Router<AppState> {
    Router::new()
        // Groups
        .route("/groups", get(list_groups).post(create_group))
        .route(
            "/groups/{group_id}",
            get(get_group).delete(delete_group),
        )
        .route("/groups/{group_id}/members", post(add_member))
        .route(
            "/groups/{group_id}/members/{user_id}",
            delete(remove_member),
        )
        // User group roles (different from RBAC roles)
        .route(
            "/groups/users/{user_id}/roles",
            get(get_user_roles).post(assign_role),
        )
        // Suspensions
        .route("/users/{user_id}/suspend", post(suspend_user))
        .route("/users/{user_id}/unsuspend", post(unsuspend_user))
}
