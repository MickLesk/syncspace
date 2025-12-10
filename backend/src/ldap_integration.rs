/// LDAP/Active Directory authentication integration
/// 
/// Provides enterprise SSO via LDAP directories:
/// - Multiple LDAP configurations support
/// - Automatic user creation from LDAP
/// - Group mapping for roles
/// - Connection testing

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};

/// LDAP configuration stored in database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LdapConfig {
    pub id: String,
    pub name: String,
    pub server_url: String,
    pub base_dn: String,
    pub bind_dn: String,
    #[serde(skip_serializing)]
    pub bind_password_encrypted: String,
    pub user_filter: String,
    pub group_filter: String,
    pub username_attribute: String,
    pub email_attribute: String,
    pub display_name_attribute: String,
    pub enabled: bool,
    pub auto_create_users: bool,
    pub default_role: String,
    pub group_role_mapping: String, // JSON: {"cn=admins": "admin", "cn=users": "user"}
    pub created_at: String,
    pub updated_at: Option<String>,
}

/// User info from LDAP directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapUser {
    pub dn: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub groups: Vec<String>,
    pub attributes: std::collections::HashMap<String, Vec<String>>,
}

/// LDAP user mapping to local account
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LdapUserMapping {
    pub id: String,
    pub user_id: String,
    pub ldap_config_id: String,
    pub ldap_dn: String,
    pub ldap_username: String,
    pub synced_at: String,
}

/// Request to create/update LDAP config
#[derive(Debug, Clone, Deserialize)]
pub struct UpsertLdapConfigRequest {
    pub name: String,
    pub server_url: String,
    pub base_dn: String,
    pub bind_dn: String,
    pub bind_password: String,
    pub user_filter: Option<String>,
    pub group_filter: Option<String>,
    pub username_attribute: Option<String>,
    pub email_attribute: Option<String>,
    pub display_name_attribute: Option<String>,
    pub enabled: bool,
    pub auto_create_users: bool,
    pub default_role: Option<String>,
    pub group_role_mapping: Option<std::collections::HashMap<String, String>>,
}

/// LDAP test result
#[derive(Debug, Clone, Serialize)]
pub struct LdapTestResult {
    pub success: bool,
    pub message: String,
    pub server_info: Option<LdapServerInfo>,
    pub user_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LdapServerInfo {
    pub server_type: String,
    pub base_dn: String,
    pub supports_tls: bool,
}

// ==================== DATABASE OPERATIONS ====================

/// List all LDAP configurations
pub async fn list_configs(pool: &SqlitePool) -> Result<Vec<LdapConfig>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM ldap_configs ORDER BY name"
    )
    .fetch_all(pool)
    .await
}

/// Get LDAP config by ID
pub async fn get_config(
    pool: &SqlitePool,
    config_id: &str,
) -> Result<Option<LdapConfig>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM ldap_configs WHERE id = ?"
    )
    .bind(config_id)
    .fetch_optional(pool)
    .await
}

/// Get active LDAP config (first enabled)
pub async fn get_active_config(pool: &SqlitePool) -> Result<Option<LdapConfig>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM ldap_configs WHERE enabled = 1 ORDER BY created_at LIMIT 1"
    )
    .fetch_optional(pool)
    .await
}

/// Create LDAP configuration
pub async fn create_config(
    pool: &SqlitePool,
    req: UpsertLdapConfigRequest,
) -> Result<LdapConfig, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let password_encrypted = encrypt_password(&req.bind_password);
    let group_mapping_json = req.group_role_mapping
        .map(|m| serde_json::to_string(&m).unwrap_or_else(|_| "{}".to_string()))
        .unwrap_or_else(|| "{}".to_string());
    
    sqlx::query(
        "INSERT INTO ldap_configs 
         (id, name, server_url, base_dn, bind_dn, bind_password_encrypted,
          user_filter, group_filter, username_attribute, email_attribute,
          display_name_attribute, enabled, auto_create_users, default_role,
          group_role_mapping, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.server_url)
    .bind(&req.base_dn)
    .bind(&req.bind_dn)
    .bind(&password_encrypted)
    .bind(req.user_filter.as_deref().unwrap_or("(uid={username})"))
    .bind(req.group_filter.as_deref().unwrap_or("(member={user_dn})"))
    .bind(req.username_attribute.as_deref().unwrap_or("uid"))
    .bind(req.email_attribute.as_deref().unwrap_or("mail"))
    .bind(req.display_name_attribute.as_deref().unwrap_or("cn"))
    .bind(req.enabled)
    .bind(req.auto_create_users)
    .bind(req.default_role.as_deref().unwrap_or("user"))
    .bind(&group_mapping_json)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM ldap_configs WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Update LDAP configuration
pub async fn update_config(
    pool: &SqlitePool,
    config_id: &str,
    req: UpsertLdapConfigRequest,
) -> Result<LdapConfig, sqlx::Error> {
    let password_encrypted = encrypt_password(&req.bind_password);
    let group_mapping_json = req.group_role_mapping
        .map(|m| serde_json::to_string(&m).unwrap_or_else(|_| "{}".to_string()))
        .unwrap_or_else(|| "{}".to_string());
    
    sqlx::query(
        "UPDATE ldap_configs 
         SET name = ?, server_url = ?, base_dn = ?, bind_dn = ?, 
             bind_password_encrypted = ?, user_filter = ?, group_filter = ?,
             username_attribute = ?, email_attribute = ?, display_name_attribute = ?,
             enabled = ?, auto_create_users = ?, default_role = ?,
             group_role_mapping = ?, updated_at = datetime('now')
         WHERE id = ?"
    )
    .bind(&req.name)
    .bind(&req.server_url)
    .bind(&req.base_dn)
    .bind(&req.bind_dn)
    .bind(&password_encrypted)
    .bind(req.user_filter.as_deref().unwrap_or("(uid={username})"))
    .bind(req.group_filter.as_deref().unwrap_or("(member={user_dn})"))
    .bind(req.username_attribute.as_deref().unwrap_or("uid"))
    .bind(req.email_attribute.as_deref().unwrap_or("mail"))
    .bind(req.display_name_attribute.as_deref().unwrap_or("cn"))
    .bind(req.enabled)
    .bind(req.auto_create_users)
    .bind(req.default_role.as_deref().unwrap_or("user"))
    .bind(&group_mapping_json)
    .bind(config_id)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM ldap_configs WHERE id = ?")
        .bind(config_id)
        .fetch_one(pool)
        .await
}

/// Delete LDAP configuration
pub async fn delete_config(pool: &SqlitePool, config_id: &str) -> Result<bool, sqlx::Error> {
    // Delete user mappings first
    sqlx::query("DELETE FROM ldap_user_mappings WHERE ldap_config_id = ?")
        .bind(config_id)
        .execute(pool)
        .await?;
    
    let result = sqlx::query("DELETE FROM ldap_configs WHERE id = ?")
        .bind(config_id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Get LDAP user mapping
pub async fn get_user_mapping(
    pool: &SqlitePool,
    ldap_config_id: &str,
    ldap_dn: &str,
) -> Result<Option<LdapUserMapping>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM ldap_user_mappings WHERE ldap_config_id = ? AND ldap_dn = ?"
    )
    .bind(ldap_config_id)
    .bind(ldap_dn)
    .fetch_optional(pool)
    .await
}

/// Create LDAP user mapping
pub async fn create_user_mapping(
    pool: &SqlitePool,
    user_id: &str,
    ldap_config_id: &str,
    ldap_user: &LdapUser,
) -> Result<LdapUserMapping, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO ldap_user_mappings 
         (id, user_id, ldap_config_id, ldap_dn, ldap_username, synced_at)
         VALUES (?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(user_id)
    .bind(ldap_config_id)
    .bind(&ldap_user.dn)
    .bind(&ldap_user.username)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM ldap_user_mappings WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Update sync time for user mapping
pub async fn update_user_mapping_sync(
    pool: &SqlitePool,
    mapping_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE ldap_user_mappings SET synced_at = datetime('now') WHERE id = ?"
    )
    .bind(mapping_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

// ==================== LDAP OPERATIONS ====================

/// Authenticate user via LDAP
pub async fn authenticate(
    config: &LdapConfig,
    username: &str,
    password: &str,
) -> Result<LdapUser, LdapError> {
    use ldap3::{LdapConnAsync, Scope, SearchEntry};
    
    // Decode bind password
    let bind_password = decrypt_password(&config.bind_password_encrypted)?;
    
    // Connect to LDAP server
    let (conn, mut ldap) = LdapConnAsync::new(&config.server_url).await
        .map_err(|e| LdapError::ConnectionFailed(e.to_string()))?;
    
    // Spawn connection handler
    ldap3::drive!(conn);
    
    // Bind with service account
    ldap.simple_bind(&config.bind_dn, &bind_password).await
        .map_err(|e| LdapError::BindFailed(e.to_string()))?
        .success()
        .map_err(|e| LdapError::BindFailed(e.to_string()))?;
    
    // Search for user
    let filter = config.user_filter.replace("{username}", username);
    let attrs = vec![
        config.username_attribute.as_str(),
        config.email_attribute.as_str(),
        config.display_name_attribute.as_str(),
        "memberOf",
    ];
    
    let (rs, _res) = ldap.search(
        &config.base_dn,
        Scope::Subtree,
        &filter,
        attrs,
    ).await
        .map_err(|e| LdapError::SearchFailed(e.to_string()))?
        .success()
        .map_err(|e| LdapError::SearchFailed(e.to_string()))?;
    
    if rs.is_empty() {
        return Err(LdapError::UserNotFound);
    }
    
    let entry = SearchEntry::construct(rs[0].clone());
    let user_dn = entry.dn.clone();
    
    // Close admin connection
    let _ = ldap.unbind().await;
    
    // Authenticate user with their credentials
    let (conn2, mut ldap2) = LdapConnAsync::new(&config.server_url).await
        .map_err(|e| LdapError::ConnectionFailed(e.to_string()))?;
    
    ldap3::drive!(conn2);
    
    ldap2.simple_bind(&user_dn, password).await
        .map_err(|_| LdapError::AuthenticationFailed)?
        .success()
        .map_err(|_| LdapError::AuthenticationFailed)?;
    
    // Get user's groups
    let groups = entry.attrs.get("memberOf")
        .map(|v| v.clone())
        .unwrap_or_default();
    
    let _ = ldap2.unbind().await;
    
    Ok(LdapUser {
        dn: user_dn,
        username: entry.attrs.get(&config.username_attribute)
            .and_then(|v| v.first().cloned())
            .unwrap_or_else(|| username.to_string()),
        email: entry.attrs.get(&config.email_attribute)
            .and_then(|v| v.first().cloned())
            .unwrap_or_default(),
        display_name: entry.attrs.get(&config.display_name_attribute)
            .and_then(|v| v.first().cloned())
            .unwrap_or_else(|| username.to_string()),
        groups,
        attributes: entry.attrs,
    })
}

/// Test LDAP connection
pub async fn test_connection(config: &LdapConfig) -> LdapTestResult {
    use ldap3::{LdapConnAsync, Scope};
    
    let bind_password = match decrypt_password(&config.bind_password_encrypted) {
        Ok(p) => p,
        Err(e) => return LdapTestResult {
            success: false,
            message: format!("Failed to decrypt password: {}", e),
            server_info: None,
            user_count: None,
        },
    };
    
    // Try to connect
    let (conn, mut ldap) = match LdapConnAsync::new(&config.server_url).await {
        Ok(c) => c,
        Err(e) => return LdapTestResult {
            success: false,
            message: format!("Connection failed: {}", e),
            server_info: None,
            user_count: None,
        },
    };
    
    ldap3::drive!(conn);
    
    // Try to bind
    if let Err(e) = ldap.simple_bind(&config.bind_dn, &bind_password).await {
        return LdapTestResult {
            success: false,
            message: format!("Bind failed: {}", e),
            server_info: None,
            user_count: None,
        };
    }
    
    // Count users
    let user_count = match ldap.search(
        &config.base_dn,
        Scope::Subtree,
        &config.user_filter.replace("{username}", "*"),
        vec!["dn"],
    ).await {
        Ok(result) => match result.success() {
            Ok((rs, _)) => Some(rs.len() as u64),
            Err(_) => None,
        },
        Err(_) => None,
    };
    
    let _ = ldap.unbind().await;
    
    LdapTestResult {
        success: true,
        message: format!("Successfully connected to {}", config.server_url),
        server_info: Some(LdapServerInfo {
            server_type: detect_server_type(&config.server_url),
            base_dn: config.base_dn.clone(),
            supports_tls: config.server_url.starts_with("ldaps://"),
        }),
        user_count,
    }
}

/// Sync users from LDAP directory
pub async fn sync_users(
    pool: &SqlitePool,
    config: &LdapConfig,
) -> Result<SyncResult, LdapError> {
    use ldap3::{LdapConnAsync, Scope, SearchEntry};
    
    let bind_password = decrypt_password(&config.bind_password_encrypted)?;
    
    let (conn, mut ldap) = LdapConnAsync::new(&config.server_url).await
        .map_err(|e| LdapError::ConnectionFailed(e.to_string()))?;
    
    ldap3::drive!(conn);
    
    ldap.simple_bind(&config.bind_dn, &bind_password).await
        .map_err(|e| LdapError::BindFailed(e.to_string()))?
        .success()
        .map_err(|e| LdapError::BindFailed(e.to_string()))?;
    
    // Search for all users
    let filter = config.user_filter.replace("{username}", "*");
    let attrs = vec![
        config.username_attribute.as_str(),
        config.email_attribute.as_str(),
        config.display_name_attribute.as_str(),
        "memberOf",
    ];
    
    let (rs, _res) = ldap.search(
        &config.base_dn,
        Scope::Subtree,
        &filter,
        attrs,
    ).await
        .map_err(|e| LdapError::SearchFailed(e.to_string()))?
        .success()
        .map_err(|e| LdapError::SearchFailed(e.to_string()))?;
    
    let _ = ldap.unbind().await;
    
    let mut created = 0;
    let mut updated = 0;
    let mut skipped = 0;
    
    for entry_raw in rs {
        let entry = SearchEntry::construct(entry_raw);
        
        let ldap_user = LdapUser {
            dn: entry.dn.clone(),
            username: entry.attrs.get(&config.username_attribute)
                .and_then(|v| v.first().cloned())
                .unwrap_or_default(),
            email: entry.attrs.get(&config.email_attribute)
                .and_then(|v| v.first().cloned())
                .unwrap_or_default(),
            display_name: entry.attrs.get(&config.display_name_attribute)
                .and_then(|v| v.first().cloned())
                .unwrap_or_default(),
            groups: entry.attrs.get("memberOf")
                .map(|v| v.clone())
                .unwrap_or_default(),
            attributes: entry.attrs.clone(),
        };
        
        if ldap_user.username.is_empty() {
            skipped += 1;
            continue;
        }
        
        // Check if mapping exists
        match get_user_mapping(pool, &config.id, &ldap_user.dn).await {
            Ok(Some(mapping)) => {
                // Update sync time
                let _ = update_user_mapping_sync(pool, &mapping.id).await;
                updated += 1;
            }
            Ok(None) if config.auto_create_users => {
                // Create user if auto-create is enabled
                if let Ok(user_id) = create_user_from_ldap(pool, config, &ldap_user).await {
                    let _ = create_user_mapping(pool, &user_id, &config.id, &ldap_user).await;
                    created += 1;
                } else {
                    skipped += 1;
                }
            }
            _ => {
                skipped += 1;
            }
        }
    }
    
    Ok(SyncResult {
        total_ldap_users: created + updated + skipped,
        created,
        updated,
        skipped,
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct SyncResult {
    pub total_ldap_users: u64,
    pub created: u64,
    pub updated: u64,
    pub skipped: u64,
}

/// Create local user from LDAP user
async fn create_user_from_ldap(
    pool: &SqlitePool,
    config: &LdapConfig,
    ldap_user: &LdapUser,
) -> Result<String, sqlx::Error> {
    let user_id = Uuid::new_v4().to_string();
    
    // Determine role from group mapping
    let role = determine_role_from_groups(config, &ldap_user.groups);
    
    // Create user with random password (they'll auth via LDAP)
    let random_password = Uuid::new_v4().to_string();
    let password_hash = crate::auth::hash_password(&random_password)
        .unwrap_or_else(|_| random_password);
    
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, email, display_name, role, created_at)
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&user_id)
    .bind(&ldap_user.username)
    .bind(&password_hash)
    .bind(&ldap_user.email)
    .bind(&ldap_user.display_name)
    .bind(&role)
    .execute(pool)
    .await?;
    
    Ok(user_id)
}

/// Determine user role from LDAP groups
fn determine_role_from_groups(config: &LdapConfig, groups: &[String]) -> String {
    let mapping: std::collections::HashMap<String, String> = 
        serde_json::from_str(&config.group_role_mapping).unwrap_or_default();
    
    for group in groups {
        // Check exact match
        if let Some(role) = mapping.get(group) {
            return role.clone();
        }
        
        // Check CN match
        if let Some(cn) = extract_cn(group) {
            if let Some(role) = mapping.get(&cn) {
                return role.clone();
            }
        }
    }
    
    config.default_role.clone()
}

fn extract_cn(dn: &str) -> Option<String> {
    dn.split(',')
        .find(|part| part.to_lowercase().starts_with("cn="))
        .map(|part| part[3..].to_string())
}

fn detect_server_type(url: &str) -> String {
    if url.contains("389") || url.contains("636") {
        "OpenLDAP/Standard LDAP".to_string()
    } else {
        "Unknown".to_string()
    }
}

// ==================== HELPERS ====================

fn encrypt_password(password: &str) -> String {
    general_purpose::STANDARD.encode(password)
}

fn decrypt_password(encrypted: &str) -> Result<String, LdapError> {
    let bytes = general_purpose::STANDARD.decode(encrypted)
        .map_err(|e| LdapError::DecryptionFailed(e.to_string()))?;
    String::from_utf8(bytes)
        .map_err(|e| LdapError::DecryptionFailed(e.to_string()))
}

// ==================== ERRORS ====================

#[derive(Debug, Clone)]
pub enum LdapError {
    ConnectionFailed(String),
    BindFailed(String),
    SearchFailed(String),
    AuthenticationFailed,
    UserNotFound,
    DecryptionFailed(String),
    ConfigNotFound,
}

impl std::fmt::Display for LdapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LdapError::ConnectionFailed(e) => write!(f, "LDAP connection failed: {}", e),
            LdapError::BindFailed(e) => write!(f, "LDAP bind failed: {}", e),
            LdapError::SearchFailed(e) => write!(f, "LDAP search failed: {}", e),
            LdapError::AuthenticationFailed => write!(f, "LDAP authentication failed"),
            LdapError::UserNotFound => write!(f, "User not found in LDAP directory"),
            LdapError::DecryptionFailed(e) => write!(f, "Password decryption failed: {}", e),
            LdapError::ConfigNotFound => write!(f, "LDAP configuration not found"),
        }
    }
}

impl std::error::Error for LdapError {}
