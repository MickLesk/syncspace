//! Personal Access Token (API Key) Management
//!
//! Allows users to create API tokens for programmatic access to the API.
//! Tokens can have scopes, expiration dates, and usage limits.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{auth::UserInfo, AppState};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiToken {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub token_prefix: String, // First 8 chars for display (e.g., "ssk_abc1...")
    pub token_hash: String,   // SHA-256 hash of full token
    pub scopes: String,       // Comma-separated: "read,write,admin"
    pub expires_at: Option<String>, // NULL = never expires
    pub last_used_at: Option<String>,
    pub usage_count: i64,
    pub max_uses: Option<i64>,        // NULL = unlimited
    pub ip_whitelist: Option<String>, // Comma-separated IPs, NULL = any
    pub is_active: bool,
    pub created_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiTokenResponse {
    pub id: String,
    pub name: String,
    pub token_prefix: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
    pub last_used_at: Option<String>,
    pub usage_count: i64,
    pub max_uses: Option<i64>,
    pub ip_whitelist: Option<Vec<String>>,
    pub is_active: bool,
    pub created_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTokenResponse {
    pub token: ApiTokenResponse,
    pub secret: String, // Only shown once on creation!
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    pub scopes: Vec<String>,
    #[serde(default)]
    pub expires_in_days: Option<i64>,
    #[serde(default)]
    pub max_uses: Option<i64>,
    #[serde(default)]
    pub ip_whitelist: Option<Vec<String>>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTokenRequest {
    pub name: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub ip_whitelist: Option<Vec<String>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTokensQuery {
    #[serde(default)]
    pub include_inactive: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenUsageStats {
    pub total_tokens: i64,
    pub active_tokens: i64,
    pub expired_tokens: i64,
    pub total_api_calls: i64,
    pub calls_today: i64,
    pub calls_this_week: i64,
    pub most_used_token: Option<String>,
    pub recent_activity: Vec<TokenActivity>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TokenActivity {
    pub token_name: String,
    pub endpoint: String,
    pub method: String,
    pub status_code: i32,
    pub ip_address: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AvailableScope {
    pub scope: String,
    pub description: String,
    pub category: String,
}

// ============================================================================
// ROUTER
// ============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api-tokens", get(list_tokens).post(create_token))
        .route("/api-tokens/scopes", get(list_scopes))
        .route("/api-tokens/stats", get(get_usage_stats))
        .route(
            "/api-tokens/{id}",
            get(get_token).put(update_token).delete(delete_token),
        )
        .route("/api-tokens/{id}/regenerate", post(regenerate_token))
        .route("/api-tokens/{id}/revoke", post(revoke_token))
        .route("/api-tokens/validate", post(validate_token))
}

// ============================================================================
// HANDLERS
// ============================================================================

/// List all API tokens for the current user
async fn list_tokens(
    State(state): State<AppState>,
    user: UserInfo,
    Query(query): Query<ListTokensQuery>,
) -> Result<Json<Vec<ApiTokenResponse>>, StatusCode> {
    let sql = if query.include_inactive {
        "SELECT * FROM api_tokens WHERE user_id = ? ORDER BY created_at DESC"
    } else {
        "SELECT * FROM api_tokens WHERE user_id = ? AND is_active = 1 ORDER BY created_at DESC"
    };

    let tokens: Vec<ApiToken> = sqlx::query_as(sql)
        .bind(user.user_id())
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<ApiTokenResponse> = tokens.into_iter().map(token_to_response).collect();
    Ok(Json(responses))
}

/// Get available scopes
async fn list_scopes() -> Json<Vec<AvailableScope>> {
    let scopes = vec![
        AvailableScope {
            scope: "read".to_string(),
            description: "Read access to files and folders".to_string(),
            category: "Files".to_string(),
        },
        AvailableScope {
            scope: "write".to_string(),
            description: "Create, update, and delete files and folders".to_string(),
            category: "Files".to_string(),
        },
        AvailableScope {
            scope: "share".to_string(),
            description: "Create and manage file shares".to_string(),
            category: "Sharing".to_string(),
        },
        AvailableScope {
            scope: "upload".to_string(),
            description: "Upload files".to_string(),
            category: "Files".to_string(),
        },
        AvailableScope {
            scope: "download".to_string(),
            description: "Download files".to_string(),
            category: "Files".to_string(),
        },
        AvailableScope {
            scope: "search".to_string(),
            description: "Search files and content".to_string(),
            category: "Search".to_string(),
        },
        AvailableScope {
            scope: "profile".to_string(),
            description: "Read and update user profile".to_string(),
            category: "User".to_string(),
        },
        AvailableScope {
            scope: "notifications".to_string(),
            description: "Access notifications".to_string(),
            category: "User".to_string(),
        },
        AvailableScope {
            scope: "webhooks".to_string(),
            description: "Manage webhooks".to_string(),
            category: "Integration".to_string(),
        },
        AvailableScope {
            scope: "admin".to_string(),
            description: "Administrative access (requires admin role)".to_string(),
            category: "Admin".to_string(),
        },
    ];

    Json(scopes)
}

/// Create a new API token
async fn create_token(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, (StatusCode, String)> {
    // Validate scopes
    let valid_scopes = [
        "read",
        "write",
        "share",
        "upload",
        "download",
        "search",
        "profile",
        "notifications",
        "webhooks",
        "admin",
    ];
    for scope in &req.scopes {
        if !valid_scopes.contains(&scope.as_str()) {
            return Err((StatusCode::BAD_REQUEST, format!("Invalid scope: {}", scope)));
        }
    }

    // Generate secure token
    let token_id = Uuid::new_v4().to_string();
    let raw_token = generate_api_token();
    let token_prefix = format!("ssk_{}", &raw_token[..8]);
    let token_hash = hash_token(&raw_token);

    // Calculate expiration
    let expires_at = req
        .expires_in_days
        .map(|days| (Utc::now() + Duration::days(days)).to_rfc3339());

    let scopes_str = req.scopes.join(",");
    let ip_whitelist_str = req.ip_whitelist.as_ref().map(|ips| ips.join(","));

    sqlx::query(
        "INSERT INTO api_tokens 
         (id, user_id, name, token_prefix, token_hash, scopes, expires_at, max_uses, ip_whitelist, is_active, created_at, description)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1, datetime('now'), ?)"
    )
    .bind(&token_id)
    .bind(user.user_id())
    .bind(&req.name)
    .bind(&token_prefix)
    .bind(&token_hash)
    .bind(&scopes_str)
    .bind(&expires_at)
    .bind(req.max_uses)
    .bind(&ip_whitelist_str)
    .bind(&req.description)
    .execute(&state.db_pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Fetch the created token
    let token: ApiToken = sqlx::query_as("SELECT * FROM api_tokens WHERE id = ?")
        .bind(&token_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Return with the secret (only time it's shown!)
    Ok(Json(CreateTokenResponse {
        token: token_to_response(token),
        secret: format!("ssk_{}", raw_token),
    }))
}

/// Get a specific token
async fn get_token(
    State(state): State<AppState>,
    user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<ApiTokenResponse>, StatusCode> {
    let token: ApiToken = sqlx::query_as("SELECT * FROM api_tokens WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(user.user_id())
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(token_to_response(token)))
}

/// Update a token
async fn update_token(
    State(state): State<AppState>,
    user: UserInfo,
    Path(id): Path<String>,
    Json(req): Json<UpdateTokenRequest>,
) -> Result<Json<ApiTokenResponse>, StatusCode> {
    // Verify ownership
    let existing: Option<ApiToken> =
        sqlx::query_as("SELECT * FROM api_tokens WHERE id = ? AND user_id = ?")
            .bind(&id)
            .bind(user.user_id())
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Build update query dynamically
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();

    if let Some(name) = &req.name {
        updates.push("name = ?");
        values.push(name.clone());
    }
    if let Some(scopes) = &req.scopes {
        updates.push("scopes = ?");
        values.push(scopes.join(","));
    }
    if let Some(is_active) = req.is_active {
        updates.push("is_active = ?");
        values.push(if is_active {
            "1".to_string()
        } else {
            "0".to_string()
        });
    }
    if let Some(ips) = &req.ip_whitelist {
        updates.push("ip_whitelist = ?");
        values.push(ips.join(","));
    }
    if let Some(desc) = &req.description {
        updates.push("description = ?");
        values.push(desc.clone());
    }

    if !updates.is_empty() {
        let sql = format!(
            "UPDATE api_tokens SET {} WHERE id = ? AND user_id = ?",
            updates.join(", ")
        );

        let mut query = sqlx::query(&sql);
        for val in values {
            query = query.bind(val);
        }
        query = query.bind(&id).bind(user.user_id());

        query
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Return updated token
    get_token(State(state), user, Path(id)).await
}

/// Delete a token
async fn delete_token(
    State(state): State<AppState>,
    user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM api_tokens WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(user.user_id())
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Regenerate a token (creates new secret)
async fn regenerate_token(
    State(state): State<AppState>,
    user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<CreateTokenResponse>, StatusCode> {
    // Verify ownership
    let existing: ApiToken =
        sqlx::query_as("SELECT * FROM api_tokens WHERE id = ? AND user_id = ?")
            .bind(&id)
            .bind(user.user_id())
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

    // Generate new token
    let raw_token = generate_api_token();
    let token_prefix = format!("ssk_{}", &raw_token[..8]);
    let token_hash = hash_token(&raw_token);

    sqlx::query(
        "UPDATE api_tokens SET token_prefix = ?, token_hash = ?, usage_count = 0, last_used_at = NULL WHERE id = ?"
    )
    .bind(&token_prefix)
    .bind(&token_hash)
    .bind(&id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch updated token
    let token: ApiToken = sqlx::query_as("SELECT * FROM api_tokens WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateTokenResponse {
        token: token_to_response(token),
        secret: format!("ssk_{}", raw_token),
    }))
}

/// Revoke (deactivate) a token
async fn revoke_token(
    State(state): State<AppState>,
    user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("UPDATE api_tokens SET is_active = 0 WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(user.user_id())
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

/// Validate a token (for testing)
#[derive(Debug, Deserialize)]
struct ValidateRequest {
    token: String,
}

#[derive(Debug, Serialize)]
struct ValidateResponse {
    valid: bool,
    token_name: Option<String>,
    scopes: Option<Vec<String>>,
    expires_at: Option<String>,
    message: String,
}

async fn validate_token(
    State(state): State<AppState>,
    Json(req): Json<ValidateRequest>,
) -> Json<ValidateResponse> {
    // Extract the raw token (remove prefix)
    let raw_token = req.token.strip_prefix("ssk_").unwrap_or(&req.token);
    let token_hash = hash_token(raw_token);

    let token: Option<ApiToken> = sqlx::query_as("SELECT * FROM api_tokens WHERE token_hash = ?")
        .bind(&token_hash)
        .fetch_optional(&state.db_pool)
        .await
        .ok()
        .flatten();

    match token {
        Some(t) => {
            // Check if active
            if !t.is_active {
                return Json(ValidateResponse {
                    valid: false,
                    token_name: Some(t.name),
                    scopes: None,
                    expires_at: None,
                    message: "Token has been revoked".to_string(),
                });
            }

            // Check expiration
            if let Some(exp) = &t.expires_at {
                if let Ok(exp_date) = exp.parse::<DateTime<Utc>>() {
                    if exp_date < Utc::now() {
                        return Json(ValidateResponse {
                            valid: false,
                            token_name: Some(t.name),
                            scopes: None,
                            expires_at: t.expires_at,
                            message: "Token has expired".to_string(),
                        });
                    }
                }
            }

            // Check usage limit
            if let Some(max) = t.max_uses {
                if t.usage_count >= max {
                    return Json(ValidateResponse {
                        valid: false,
                        token_name: Some(t.name),
                        scopes: None,
                        expires_at: None,
                        message: "Token usage limit exceeded".to_string(),
                    });
                }
            }

            Json(ValidateResponse {
                valid: true,
                token_name: Some(t.name),
                scopes: Some(t.scopes.split(',').map(String::from).collect()),
                expires_at: t.expires_at,
                message: "Token is valid".to_string(),
            })
        }
        None => Json(ValidateResponse {
            valid: false,
            token_name: None,
            scopes: None,
            expires_at: None,
            message: "Token not found".to_string(),
        }),
    }
}

/// Get usage statistics
async fn get_usage_stats(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<TokenUsageStats>, StatusCode> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM api_tokens WHERE user_id = ?")
        .bind(user.user_id())
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));

    let active: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM api_tokens WHERE user_id = ? AND is_active = 1")
            .bind(user.user_id())
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    let expired: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM api_tokens WHERE user_id = ? AND expires_at < datetime('now')",
    )
    .bind(user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .unwrap_or((0,));

    let total_calls: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(usage_count), 0) FROM api_tokens WHERE user_id = ?")
            .bind(user.user_id())
            .fetch_one(&state.db_pool)
            .await
            .unwrap_or((0,));

    // Get most used token
    let most_used: Option<(String,)> = sqlx::query_as(
        "SELECT name FROM api_tokens WHERE user_id = ? ORDER BY usage_count DESC LIMIT 1",
    )
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .ok()
    .flatten();

    Ok(Json(TokenUsageStats {
        total_tokens: total.0,
        active_tokens: active.0,
        expired_tokens: expired.0,
        total_api_calls: total_calls.0,
        calls_today: 0, // Would need activity log integration
        calls_this_week: 0,
        most_used_token: most_used.map(|t| t.0),
        recent_activity: vec![], // Would need activity log integration
    }))
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn generate_api_token() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();

    (0..48)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn token_to_response(token: ApiToken) -> ApiTokenResponse {
    ApiTokenResponse {
        id: token.id,
        name: token.name,
        token_prefix: token.token_prefix,
        scopes: token
            .scopes
            .split(',')
            .map(String::from)
            .filter(|s| !s.is_empty())
            .collect(),
        expires_at: token.expires_at,
        last_used_at: token.last_used_at,
        usage_count: token.usage_count,
        max_uses: token.max_uses,
        ip_whitelist: token
            .ip_whitelist
            .map(|s| s.split(',').map(String::from).collect()),
        is_active: token.is_active,
        created_at: token.created_at,
        description: token.description,
    }
}
