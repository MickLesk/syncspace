/// LDAP/Active Directory authentication integration
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};
use ldap3::{LdapConn, Scope, SearchEntry};
use base64::{Engine as _, engine::general_purpose};
use ldap3::{LdapConn, Scope, SearchEntry};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LdapConfig {
    pub id: String,
    pub name: String,
    pub server_url: String,
    pub base_dn: String,
    pub bind_dn: String,
    pub bind_password_encrypted: String,
    pub user_filter: String, // e.g., "(uid={username})"
    pub group_filter: String, // e.g., "(member={user_dn})"
    pub enabled: bool,
    pub auto_create_users: bool,
    pub default_role: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLdapConfigRequest {
    pub name: String,
    pub server_url: String,
    pub base_dn: String,
    pub bind_dn: String,
    pub bind_password: String,
    pub user_filter: String,
    pub group_filter: String,
    pub enabled: bool,
    pub auto_create_users: bool,
    pub default_role: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LdapUser {
    pub dn: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub groups: Vec<String>,
}

/// Add LDAP configuration
pub async fn add_ldap_config(
    pool: &SqlitePool,
    req: CreateLdapConfigRequest,
) -> Result<LdapConfig, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let password_encrypted = general_purpose::STANDARD.encode(&req.bind_password);
    
    sqlx::query(
        "INSERT INTO ldap_configs 
         (id, name, server_url, base_dn, bind_dn, bind_password_encrypted, 
          user_filter, group_filter, enabled, auto_create_users, default_role,
          created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.server_url)
    .bind(&req.base_dn)
    .bind(&req.bind_dn)
    .bind(&password_encrypted)
    .bind(&req.user_filter)
    .bind(&req.group_filter)
    .bind(if req.enabled { 1 } else { 0 })
    .bind(if req.auto_create_users { 1 } else { 0 })
    .bind(&req.default_role)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM ldap_configs WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Authenticate user via LDAP
pub async fn ldap_authenticate(
    config: &LdapConfig,
    username: &str,
    password: &str,
) -> Result<LdapUser, Box<dyn std::error::Error + Send + Sync>> {
    // Decode bind password
    let bind_password_bytes = general_purpose::STANDARD.decode(&config.bind_password_encrypted)?;
    let bind_password = String::from_utf8(bind_password_bytes)?;
    
    // Connect and bind with service account
    let mut ldap = LdapConn::new(&config.server_url)?;
    ldap.simple_bind(&config.bind_dn, &bind_password)?.success()?;
    
    // Search for user
    let filter = config.user_filter.replace("{username}", username);
    let (rs, _res) = ldap.search(
        &config.base_dn,
        Scope::Subtree,
        &filter,
        vec!["dn", "uid", "mail", "cn"]
    )?.success()?;
    
    if rs.is_empty() {
        return Err("User not found in LDAP directory".into());
    }
    
    let entry = SearchEntry::construct(rs[0].clone());
    let user_dn = entry.dn.clone();
    
    // Close admin connection
    drop(ldap);
    
    // Authenticate user by binding with their credentials
    let mut ldap = LdapConn::new(&config.server_url)?;
    match ldap.simple_bind(&user_dn, password) {
        Ok(res) => {
            res.success()?;
            
            // Search for user's groups
            let group_filter = config.group_filter.replace("{user_dn}", &user_dn);
            let (groups_rs, _) = ldap.search(
                &config.base_dn,
                Scope::Subtree,
                &group_filter,
                vec!["cn"]
            )?.success()?;
            
            let groups: Vec<String> = groups_rs.iter()
                .map(|g| SearchEntry::construct(g.clone()))
                .filter_map(|e| e.attrs.get("cn").and_then(|v| v.first().cloned()))
                .collect();
            
            Ok(LdapUser {
                dn: user_dn,
                username: entry.attrs.get("uid")
                    .and_then(|v| v.first().cloned())
                    .unwrap_or_else(|| username.to_string()),
                email: entry.attrs.get("mail")
                    .and_then(|v| v.first().cloned())
                    .unwrap_or_default(),
                full_name: entry.attrs.get("cn")
                    .and_then(|v| v.first().cloned())
                    .unwrap_or_default(),
                groups,
            })
        },
        Err(e) => Err(format!("LDAP authentication failed: {}", e).into())
    }
}

/// List LDAP configurations
pub async fn list_ldap_configs(
    pool: &SqlitePool,
) -> Result<Vec<LdapConfig>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM ldap_configs ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
}

/// Get active LDAP configuration
pub async fn get_active_ldap_config(
    pool: &SqlitePool,
) -> Result<Option<LdapConfig>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM ldap_configs WHERE enabled = 1 ORDER BY created_at DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await
}

/// Update LDAP configuration
pub async fn update_ldap_config(
    pool: &SqlitePool,
    config_id: &str,
    req: CreateLdapConfigRequest,
) -> Result<(), sqlx::Error> {
    let password_encrypted = general_purpose::STANDARD.encode(&req.bind_password);
    
    sqlx::query(
        "UPDATE ldap_configs 
         SET name = ?, server_url = ?, base_dn = ?, bind_dn = ?, 
             bind_password_encrypted = ?, user_filter = ?, group_filter = ?,
             enabled = ?, auto_create_users = ?, default_role = ?,
             updated_at = datetime('now')
         WHERE id = ?"
    )
    .bind(&req.name)
    .bind(&req.server_url)
    .bind(&req.base_dn)
    .bind(&req.bind_dn)
    .bind(&password_encrypted)
    .bind(&req.user_filter)
    .bind(&req.group_filter)
    .bind(if req.enabled { 1 } else { 0 })
    .bind(if req.auto_create_users { 1 } else { 0 })
    .bind(&req.default_role)
    .bind(config_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Delete LDAP configuration
pub async fn delete_ldap_config(
    pool: &SqlitePool,
    config_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM ldap_configs WHERE id = ?")
        .bind(config_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Test LDAP connection
pub async fn test_ldap_connection(
    config: &LdapConfig,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Decode bind password
    let bind_password_bytes = general_purpose::STANDARD.decode(&config.bind_password_encrypted)?;
    let bind_password = String::from_utf8(bind_password_bytes)?;
    
    // Attempt connection and bind
    let mut ldap = LdapConn::new(&config.server_url)?;
    ldap.simple_bind(&config.bind_dn, &bind_password)?.success()?;
    
    // Connection successful, close it
    let _ = ldap.unbind();
    
    Ok(format!("Successfully connected to LDAP server at {}", config.server_url))
}
