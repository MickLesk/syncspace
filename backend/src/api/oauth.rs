/// OAuth2 API endpoints
/// 
/// Provides OAuth authentication flow:
/// - GET /api/oauth/providers - List available providers
/// - GET /api/oauth/{provider}/authorize - Start OAuth flow
/// - GET /api/oauth/{provider}/callback - Handle OAuth callback
/// - POST /api/oauth/{provider}/link - Link account (authenticated)
/// - DELETE /api/oauth/{provider}/unlink - Unlink account
/// - GET /api/oauth/linked - Get user's linked accounts
/// - POST /api/oauth/providers - Configure provider (admin)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::UserInfo,
    oauth::{self, LinkedAccount, OAuthError, OAuthProvider, UpsertOAuthProviderRequest},
    AppState,
};

/// Build OAuth router
pub fn router() -> Router<AppState> {
    Router::new()
        // Public routes (no auth required)
        .route("/oauth/providers", get(list_providers))
        .route("/oauth/{provider}/authorize", get(authorize))
        .route("/oauth/{provider}/callback", get(callback))
}

/// Build protected OAuth router (requires auth)
pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/oauth/linked", get(get_linked_accounts))
        .route("/oauth/{provider}/link", post(link_account))
        .route("/oauth/{provider}/unlink", delete(unlink_account))
        .route("/oauth/providers/config", post(configure_provider))
        .route("/oauth/providers/config/{provider}", delete(delete_provider))
}

// ==================== TYPES ====================

#[derive(Debug, Serialize)]
struct ProviderInfo {
    provider: String,
    enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AuthorizeParams {
    redirect_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

#[derive(Debug, Serialize)]
struct AuthResult {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<OAuthUserResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_url: Option<String>,
}

#[derive(Debug, Serialize)]
struct OAuthUserResponse {
    id: String,
    username: String,
    email: String,
}

// ==================== PUBLIC ENDPOINTS ====================

/// GET /api/oauth/providers - List available OAuth providers
async fn list_providers(State(state): State<AppState>) -> impl IntoResponse {
    match oauth::list_enabled_providers(&state.db_pool).await {
        Ok(providers) => {
            let infos: Vec<ProviderInfo> = providers.into_iter().map(|p| ProviderInfo {
                provider: p.provider,
                enabled: p.enabled,
                client_id: Some(p.client_id),
            }).collect();
            
            (StatusCode::OK, Json(infos)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list OAuth providers: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to list providers"
            }))).into_response()
        }
    }
}

/// GET /api/oauth/{provider}/authorize - Start OAuth flow
async fn authorize(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(params): Query<AuthorizeParams>,
) -> Response {
    // Get provider config
    let provider_config = match oauth::get_provider(&state.db_pool, &provider).await {
        Ok(Some(p)) if p.enabled => p,
        Ok(Some(_)) => {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Provider is disabled"
            }))).into_response();
        }
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Unknown OAuth provider"
            }))).into_response();
        }
        Err(e) => {
            tracing::error!("Failed to get OAuth provider: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Internal server error"
            }))).into_response();
        }
    };
    
    // Create state for CSRF protection
    let state_token = match oauth::create_state(
        &state.db_pool,
        &provider,
        None, // No user - this is for login
        params.redirect_url.as_deref(),
    ).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to create OAuth state: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to initiate OAuth flow"
            }))).into_response();
        }
    };
    
    // Generate authorization URL
    let auth_url = oauth::get_auth_url(&provider_config, &state_token);
    
    if auth_url.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Failed to generate authorization URL"
        }))).into_response();
    }
    
    // Return redirect or URL
    Redirect::temporary(&auth_url).into_response()
}

/// GET /api/oauth/{provider}/callback - Handle OAuth callback
async fn callback(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(params): Query<CallbackParams>,
) -> Response {
    // Check for OAuth error
    if let Some(error) = params.error {
        let description = params.error_description.unwrap_or_default();
        tracing::warn!("OAuth error: {} - {}", error, description);
        
        return Redirect::temporary(&format!(
            "/#/login?error=oauth_failed&message={}",
            urlencoding(&description)
        )).into_response();
    }
    
    // Validate state
    let oauth_state = match oauth::validate_state(&state.db_pool, &params.state).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return Redirect::temporary("/#/login?error=invalid_state").into_response();
        }
        Err(e) => {
            tracing::error!("Failed to validate OAuth state: {}", e);
            return Redirect::temporary("/#/login?error=internal_error").into_response();
        }
    };
    
    // Get provider config
    let provider_config = match oauth::get_provider(&state.db_pool, &provider).await {
        Ok(Some(p)) => p,
        _ => {
            return Redirect::temporary("/#/login?error=unknown_provider").into_response();
        }
    };
    
    // Exchange code for token
    let token_response = match oauth::exchange_code(&provider_config, &params.code).await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("OAuth token exchange failed: {}", e);
            return Redirect::temporary("/#/login?error=token_exchange_failed").into_response();
        }
    };
    
    // Get user info from provider
    let user_info = match oauth::get_user_info(&provider, &token_response.access_token).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to get OAuth user info: {}", e);
            return Redirect::temporary("/#/login?error=user_info_failed").into_response();
        }
    };
    
    // Check if this OAuth account is already linked to a user
    if let Ok(Some(existing_user_id)) = oauth::find_user_by_oauth(&state.db_pool, &provider, &user_info.id).await {
        // Login existing user
        match create_jwt_for_user(&state.db_pool, &existing_user_id).await {
            Ok(token) => {
                let redirect_url = oauth_state.redirect_url.unwrap_or_else(|| "/#/files".to_string());
                return Redirect::temporary(&format!("{}?token={}", redirect_url, token)).into_response();
            }
            Err(e) => {
                tracing::error!("Failed to create JWT: {}", e);
                return Redirect::temporary("/#/login?error=jwt_creation_failed").into_response();
            }
        }
    }
    
    // Check if user with this email already exists
    let existing_user: Option<crate::database::User> = sqlx::query_as(
        "SELECT * FROM users WHERE email = ?"
    )
    .bind(&user_info.email)
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(None);
    
    if let Some(user) = existing_user {
        // Link account and login
        let _ = oauth::store_token(&state.db_pool, &user.id, &provider, &token_response, &user_info).await;
        
        match create_jwt_for_user(&state.db_pool, &user.id).await {
            Ok(token) => {
                let redirect_url = oauth_state.redirect_url.unwrap_or_else(|| "/#/files".to_string());
                return Redirect::temporary(&format!("{}?token={}", redirect_url, token)).into_response();
            }
            Err(e) => {
                tracing::error!("Failed to create JWT: {}", e);
                return Redirect::temporary("/#/login?error=jwt_creation_failed").into_response();
            }
        }
    }
    
    // Create new user from OAuth info
    match create_user_from_oauth(&state.db_pool, &provider, &token_response, &user_info).await {
        Ok(user_id) => {
            match create_jwt_for_user(&state.db_pool, &user_id).await {
                Ok(token) => {
                    let redirect_url = oauth_state.redirect_url.unwrap_or_else(|| "/#/files".to_string());
                    Redirect::temporary(&format!("{}?token={}", redirect_url, token)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to create JWT: {}", e);
                    Redirect::temporary("/#/login?error=jwt_creation_failed").into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to create user from OAuth: {}", e);
            Redirect::temporary("/#/login?error=user_creation_failed").into_response()
        }
    }
}

// ==================== PROTECTED ENDPOINTS ====================

/// GET /api/oauth/linked - Get user's linked OAuth accounts
async fn get_linked_accounts(
    State(state): State<AppState>,
    user: UserInfo,
) -> impl IntoResponse {
    match oauth::get_linked_accounts(&state.db_pool, &user.id).await {
        Ok(accounts) => (StatusCode::OK, Json(accounts)),
        Err(e) => {
            tracing::error!("Failed to get linked accounts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![] as Vec<LinkedAccount>))
        }
    }
}

/// POST /api/oauth/{provider}/link - Link OAuth account to current user
async fn link_account(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    user: UserInfo,
    Query(params): Query<AuthorizeParams>,
) -> Response {
    // Get provider config
    let provider_config = match oauth::get_provider(&state.db_pool, &provider).await {
        Ok(Some(p)) if p.enabled => p,
        _ => {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Unknown or disabled OAuth provider"
            }))).into_response();
        }
    };
    
    // Create state with user ID (for linking)
    let state_token = match oauth::create_state(
        &state.db_pool,
        &provider,
        Some(&user.id),
        params.redirect_url.as_deref(),
    ).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to create OAuth state: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to initiate OAuth flow"
            }))).into_response();
        }
    };
    
    let auth_url = oauth::get_auth_url(&provider_config, &state_token);
    
    (StatusCode::OK, Json(serde_json::json!({
        "auth_url": auth_url
    }))).into_response()
}

/// DELETE /api/oauth/{provider}/unlink - Unlink OAuth account
async fn unlink_account(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    user: UserInfo,
) -> impl IntoResponse {
    match oauth::unlink_account(&state.db_pool, &user.id, &provider).await {
        Ok(true) => (StatusCode::OK, Json(serde_json::json!({
            "success": true,
            "message": format!("{} account unlinked", provider)
        }))),
        Ok(false) => (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "success": false,
            "error": "Account not linked"
        }))),
        Err(e) => {
            tracing::error!("Failed to unlink account: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "error": "Failed to unlink account"
            })))
        }
    }
}

/// POST /api/oauth/providers/config - Configure OAuth provider (admin only)
async fn configure_provider(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<UpsertOAuthProviderRequest>,
) -> impl IntoResponse {
    // Check admin role
    if user.role != "admin" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        })));
    }
    
    match oauth::upsert_provider(&state.db_pool, req).await {
        Ok(provider) => (StatusCode::OK, Json(serde_json::json!({
            "success": true,
            "provider": provider.provider,
            "enabled": provider.enabled
        }))),
        Err(e) => {
            tracing::error!("Failed to configure OAuth provider: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to configure provider"
            })))
        }
    }
}

/// DELETE /api/oauth/providers/config/{provider} - Delete OAuth provider (admin only)
async fn delete_provider(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    user: UserInfo,
) -> impl IntoResponse {
    if user.role != "admin" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Admin access required"
        })));
    }
    
    match oauth::delete_provider(&state.db_pool, &provider).await {
        Ok(true) => (StatusCode::OK, Json(serde_json::json!({
            "success": true,
            "message": format!("{} provider deleted", provider)
        }))),
        Ok(false) => (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": "Provider not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to delete OAuth provider: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to delete provider"
            })))
        }
    }
}

// ==================== HELPERS ====================

async fn create_jwt_for_user(pool: &sqlx::SqlitePool, user_id: &str) -> Result<String, String> {
    // Get user from database
    let user = sqlx::query_as::<_, crate::auth::User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;
    
    crate::auth::generate_token(&user)
        .map_err(|e| e.to_string())
}

async fn create_user_from_oauth(
    pool: &sqlx::SqlitePool,
    provider: &str,
    token_response: &oauth::OAuthTokenResponse,
    user_info: &oauth::OAuthUserInfo,
) -> Result<String, String> {
    let user_id = uuid::Uuid::new_v4().to_string();
    
    // Generate username from email or name
    let username = user_info.email.split('@').next()
        .unwrap_or(&user_info.name)
        .to_string();
    
    // Ensure unique username
    let unique_username = ensure_unique_username(pool, &username).await?;
    
    // Create user with random password (they'll auth via OAuth)
    let random_password = uuid::Uuid::new_v4().to_string();
    let password_hash = crate::auth::hash_password(&random_password)
        .map_err(|e| e.to_string())?;
    
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, email, display_name, role, created_at)
         VALUES (?, ?, ?, ?, ?, 'user', datetime('now'))"
    )
    .bind(&user_id)
    .bind(&unique_username)
    .bind(&password_hash)
    .bind(&user_info.email)
    .bind(&user_info.name)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;
    
    // Store OAuth token
    oauth::store_token(pool, &user_id, provider, token_response, user_info).await
        .map_err(|e| e.to_string())?;
    
    Ok(user_id)
}

async fn ensure_unique_username(pool: &sqlx::SqlitePool, base: &str) -> Result<String, String> {
    let mut username = base.to_string();
    let mut counter = 0;
    
    loop {
        let exists: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM users WHERE username = ?"
        )
        .bind(&username)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        if exists.is_none() {
            return Ok(username);
        }
        
        counter += 1;
        username = format!("{}{}", base, counter);
        
        if counter > 100 {
            return Err("Could not generate unique username".to_string());
        }
    }
}

fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            _ => {
                for b in c.to_string().bytes() {
                    result.push_str(&format!("%{:02X}", b));
                }
            }
        }
    }
    result
}
