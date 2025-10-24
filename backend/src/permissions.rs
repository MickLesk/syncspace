/// Advanced permissions system with ACL, roles, and inheritance
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: String, // JSON array of permission strings
    pub is_system: bool, // System roles cannot be deleted
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub scope: String, // "global", "folder:{id}", "file:{id}"
    pub granted_by: String,
    pub granted_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccessControlEntry {
    pub id: String,
    pub resource_type: String, // "file", "folder"
    pub resource_id: String,
    pub principal_type: String, // "user", "role", "group"
    pub principal_id: String,
    pub permissions: String, // JSON array: ["read", "write", "delete", "share"]
    pub inherited: bool,
    pub created_at: String,
}

/// Check if user has permission on resource
pub async fn has_permission(
    pool: &SqlitePool,
    user_id: &str,
    resource_type: &str,
    resource_id: &str,
    permission: &str,
) -> Result<bool, sqlx::Error> {
    // Check direct ACL entry
    let direct_acl: Vec<AccessControlEntry> = sqlx::query_as(
        "SELECT * FROM access_control WHERE resource_type = ? AND resource_id = ? 
         AND principal_type = 'user' AND principal_id = ?"
    )
    .bind(resource_type)
    .bind(resource_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    
    for acl in direct_acl {
        let perms: Vec<String> = serde_json::from_str(&acl.permissions).unwrap_or_default();
        if perms.contains(&permission.to_string()) || perms.contains(&"*".to_string()) {
            return Ok(true);
        }
    }
    
    // Check role-based permissions
    let user_roles: Vec<UserRole> = sqlx::query_as(
        "SELECT * FROM user_roles WHERE user_id = ? AND (scope = 'global' OR scope = ?)"
    )
    .bind(user_id)
    .bind(format!("{}:{}", resource_type, resource_id))
    .fetch_all(pool)
    .await?;
    
    for user_role in user_roles {
        let role: Role = sqlx::query_as("SELECT * FROM roles WHERE id = ?")
            .bind(&user_role.role_id)
            .fetch_one(pool)
            .await?;
        
        let perms: Vec<String> = serde_json::from_str(&role.permissions).unwrap_or_default();
        if perms.contains(&permission.to_string()) || perms.contains(&"*".to_string()) {
            return Ok(true);
        }
    }
    
    Ok(false)
}

/// Grant permission via ACL
pub async fn grant_permission(
    pool: &SqlitePool,
    resource_type: &str,
    resource_id: &str,
    principal_type: &str,
    principal_id: &str,
    permissions: Vec<String>,
) -> Result<AccessControlEntry, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let perms_json = serde_json::to_string(&permissions).unwrap_or_else(|_| "[]".to_string());
    
    sqlx::query(
        "INSERT INTO access_control (id, resource_type, resource_id, principal_type, principal_id, permissions, inherited, created_at)
         VALUES (?, ?, ?, ?, ?, ?, 0, datetime('now'))"
    )
    .bind(&id)
    .bind(resource_type)
    .bind(resource_id)
    .bind(principal_type)
    .bind(principal_id)
    .bind(&perms_json)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM access_control WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Create a new role
pub async fn create_role(
    pool: &SqlitePool,
    name: &str,
    description: Option<String>,
    permissions: Vec<String>,
) -> Result<Role, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let perms_json = serde_json::to_string(&permissions).unwrap_or_else(|_| "[]".to_string());
    
    sqlx::query(
        "INSERT INTO roles (id, name, description, permissions, is_system, created_at)
         VALUES (?, ?, ?, ?, 0, datetime('now'))"
    )
    .bind(&id)
    .bind(name)
    .bind(description)
    .bind(&perms_json)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM roles WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Assign role to user
pub async fn assign_role(
    pool: &SqlitePool,
    user_id: &str,
    role_id: &str,
    scope: &str,
    granted_by: &str,
) -> Result<UserRole, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO user_roles (id, user_id, role_id, scope, granted_by, granted_at)
         VALUES (?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(user_id)
    .bind(role_id)
    .bind(scope)
    .bind(granted_by)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM user_roles WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Initialize default roles
pub async fn init_default_roles(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let roles = vec![
        ("admin", "Full system access", vec!["*"]),
        ("editor", "Edit files", vec!["read", "write", "delete"]),
        ("viewer", "View files only", vec!["read"]),
        ("contributor", "Upload and edit own files", vec!["read", "write_own", "delete_own"]),
    ];
    
    for (name, desc, perms) in roles {
        let existing: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM roles WHERE name = ? AND is_system = 1"
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        
        if existing.is_none() {
            let id = Uuid::new_v4().to_string();
            let perms_json = serde_json::to_string(&perms).unwrap();
            
            sqlx::query(
                "INSERT INTO roles (id, name, description, permissions, is_system, created_at)
                 VALUES (?, ?, ?, ?, 1, datetime('now'))"
            )
            .bind(&id)
            .bind(name)
            .bind(desc)
            .bind(&perms_json)
            .execute(pool)
            .await?;
        }
    }
    
    Ok(())
}
