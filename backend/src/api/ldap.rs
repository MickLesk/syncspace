/// LDAP API endpoints
/// 
/// Provides enterprise LDAP/AD authentication:
/// - GET /api/ldap/configs - List LDAP configurations
/// - POST /api/ldap/configs - Create LDAP configuration
/// - PUT /api/ldap/configs/{id} - Update LDAP configuration
/// - DELETE /api/ldap/configs/{id} - Delete LDAP configuration
/// - POST /api/ldap/configs/{id}/test - Test LDAP connection
/// - POST /api/ldap/sync - Sync users from LDAP

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::UserInfo,
    ldap_integration::{self, LdapConfig, LdapTestResult, SyncResult, UpsertLdapConfigRequest},
    AppState,
};

/// Helper to check admin access
fn is_admin(user: &UserInfo) -> bool {
    user.is_admin || user.role.as_deref() == Some("admin")
}

/// Build LDAP router (all routes require admin)
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ldap/configs", get(list_configs))
        .route("/ldap/configs", post(create_config))
        .route("/ldap/configs/{id}", get(get_config))
        .route("/ldap/configs/{id}", put(update_config))
        .route("/ldap/configs/{id}", delete(delete_config))
        .route("/ldap/configs/{id}/test", post(test_connection))
        .route("/ldap/sync", post(sync_users))
        .route("/ldap/sync/{id}", post(sync_users_for_config))
}

// ==================== TYPES ====================

#[derive(Debug, Serialize)]
struct LdapConfigResponse {
    pub id: String,
    pub name: String,
    pub server_url: String,
    pub base_dn: String,
    pub bind_dn: String,
    pub user_filter: String,
    pub group_filter: String,
    pub enabled: bool,
    pub auto_create_users: bool,
    pub default_role: String,
    pub created_at: String,
}

impl From<LdapConfig> for LdapConfigResponse {
    fn from(config: LdapConfig) -> Self {
        Self {
            id: config.id,
            name: config.name,
            server_url: config.server_url,
            base_dn: config.base_dn,
            bind_dn: config.bind_dn,
            user_filter: config.user_filter,
            group_filter: config.group_filter,
            enabled: config.enabled,
            auto_create_users: config.auto_create_users,
            default_role: config.default_role,
            created_at: config.created_at,
        }
    }
}

// ==================== ENDPOINTS ====================

/// GET /api/ldap/configs - List all LDAP configurations
async fn list_configs(
    State(state): State<AppState>,
    user: UserInfo,
) -> impl IntoResponse {
    // Check admin role
    if !is_admin(&user) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        }))).into_response();
    }
    
    match ldap_integration::list_configs(&state.db_pool).await {
        Ok(configs) => {
            let responses: Vec<LdapConfigResponse> = configs.into_iter()
                .map(LdapConfigResponse::from)
                .collect();
            (StatusCode::OK, Json(responses)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list LDAP configs: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to list configurations"
            }))).into_response()
        }
    }
}

/// GET /api/ldap/configs/{id} - Get LDAP configuration by ID
async fn get_config(
    State(state): State<AppState>,
    Path(config_id): Path<String>,
    user: UserInfo,
) -> impl IntoResponse {
    if !is_admin(&user) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        }))).into_response();
    }
    
    match ldap_integration::get_config(&state.db_pool, &config_id).await {
        Ok(Some(config)) => {
            (StatusCode::OK, Json(LdapConfigResponse::from(config))).into_response()
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Configuration not found"
            }))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get LDAP config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get configuration"
            }))).into_response()
        }
    }
}

/// POST /api/ldap/configs - Create LDAP configuration
async fn create_config(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<UpsertLdapConfigRequest>,
) -> impl IntoResponse {
    if !is_admin(&user) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        }))).into_response();
    }
    
    // Validate required fields
    if req.name.is_empty() || req.server_url.is_empty() || req.base_dn.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Name, server URL, and base DN are required"
        }))).into_response();
    }
    
    match ldap_integration::create_config(&state.db_pool, req).await {
        Ok(config) => {
            tracing::info!("LDAP config created: {} by user {}", config.name, user.username);
            (StatusCode::CREATED, Json(LdapConfigResponse::from(config))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create LDAP config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to create configuration"
            }))).into_response()
        }
    }
}

/// PUT /api/ldap/configs/{id} - Update LDAP configuration
async fn update_config(
    State(state): State<AppState>,
    Path(config_id): Path<String>,
    user: UserInfo,
    Json(req): Json<UpsertLdapConfigRequest>,
) -> impl IntoResponse {
    if !is_admin(&user) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        }))).into_response();
    }
    
    // Check if config exists
    if ldap_integration::get_config(&state.db_pool, &config_id).await.ok().flatten().is_none() {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": "Configuration not found"
        }))).into_response();
    }
    
    match ldap_integration::update_config(&state.db_pool, &config_id, req).await {
        Ok(config) => {
            tracing::info!("LDAP config updated: {} by user {}", config.name, user.username);
            (StatusCode::OK, Json(LdapConfigResponse::from(config))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to update LDAP config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to update configuration"
            }))).into_response()
        }
    }
}

/// DELETE /api/ldap/configs/{id} - Delete LDAP configuration
async fn delete_config(
    State(state): State<AppState>,
    Path(config_id): Path<String>,
    user: UserInfo,
) -> impl IntoResponse {
    if !is_admin(&user) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        }))).into_response();
    }
    
    match ldap_integration::delete_config(&state.db_pool, &config_id).await {
        Ok(true) => {
            tracing::info!("LDAP config deleted: {} by user {}", config_id, user.username);
            (StatusCode::OK, Json(serde_json::json!({
                "success": true,
                "message": "Configuration deleted"
            }))).into_response()
        }
        Ok(false) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Configuration not found"
            }))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to delete LDAP config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to delete configuration"
            }))).into_response()
        }
    }
}

/// POST /api/ldap/configs/{id}/test - Test LDAP connection
async fn test_connection(
    State(state): State<AppState>,
    Path(config_id): Path<String>,
    user: UserInfo,
) -> impl IntoResponse {
    if !is_admin(&user) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        }))).into_response();
    }
    
    let config = match ldap_integration::get_config(&state.db_pool, &config_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Configuration not found"
            }))).into_response();
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Database error: {}", e)
            }))).into_response();
        }
    };
    
    let result = ldap_integration::test_connection(&config).await;
    
    if result.success {
        (StatusCode::OK, Json(result)).into_response()
    } else {
        (StatusCode::BAD_REQUEST, Json(result)).into_response()
    }
}

/// POST /api/ldap/sync - Sync users from all enabled LDAP directories
async fn sync_users(
    State(state): State<AppState>,
    user: UserInfo,
) -> impl IntoResponse {
    if !is_admin(&user) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        }))).into_response();
    }
    
    // Get all enabled configs
    let configs = match ldap_integration::list_configs(&state.db_pool).await {
        Ok(configs) => configs.into_iter().filter(|c| c.enabled).collect::<Vec<_>>(),
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Failed to list configurations: {}", e)
            }))).into_response();
        }
    };
    
    if configs.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "No enabled LDAP configurations found"
        }))).into_response();
    }
    
    let mut total_result = SyncResultResponse {
        configs_processed: 0,
        total_ldap_users: 0,
        created: 0,
        updated: 0,
        skipped: 0,
        errors: vec![],
    };
    
    for config in configs {
        match ldap_integration::sync_users(&state.db_pool, &config).await {
            Ok(result) => {
                total_result.configs_processed += 1;
                total_result.total_ldap_users += result.total_ldap_users;
                total_result.created += result.created;
                total_result.updated += result.updated;
                total_result.skipped += result.skipped;
            }
            Err(e) => {
                total_result.errors.push(format!("{}: {}", config.name, e));
            }
        }
    }
    
    tracing::info!(
        "LDAP sync completed by {}: {} configs, {} users synced",
        user.username,
        total_result.configs_processed,
        total_result.created + total_result.updated
    );
    
    (StatusCode::OK, Json(total_result)).into_response()
}

/// POST /api/ldap/sync/{id} - Sync users from specific LDAP directory
async fn sync_users_for_config(
    State(state): State<AppState>,
    Path(config_id): Path<String>,
    user: UserInfo,
) -> impl IntoResponse {
    if !is_admin(&user) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        }))).into_response();
    }
    
    let config = match ldap_integration::get_config(&state.db_pool, &config_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Configuration not found"
            }))).into_response();
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Database error: {}", e)
            }))).into_response();
        }
    };
    
    match ldap_integration::sync_users(&state.db_pool, &config).await {
        Ok(result) => {
            tracing::info!(
                "LDAP sync for {} completed by {}: {} users synced",
                config.name,
                user.username,
                result.created + result.updated
            );
            (StatusCode::OK, Json(result)).into_response()
        }
        Err(e) => {
            tracing::error!("LDAP sync failed for {}: {}", config.name, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Sync failed: {}", e)
            }))).into_response()
        }
    }
}

#[derive(Debug, Serialize)]
struct SyncResultResponse {
    configs_processed: u64,
    total_ldap_users: u64,
    created: u64,
    updated: u64,
    skipped: u64,
    errors: Vec<String>,
}
