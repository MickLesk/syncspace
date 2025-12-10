/// OAuth2 authentication support
/// Supports Google, GitHub, Microsoft providers
/// 
/// Usage:
/// 1. Configure providers in admin settings
/// 2. Users can link accounts via /api/oauth/{provider}/authorize
/// 3. Callback handles token exchange and account linking

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;

/// OAuth provider configuration stored in database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthProvider {
    pub id: String,
    pub provider: String, // "google", "github", "microsoft"
    pub client_id: String,
    #[serde(skip_serializing)]
    pub client_secret_encrypted: String,
    pub redirect_uri: String,
    pub scopes: String, // JSON array
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
}

/// OAuth token for linked account
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthToken {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    #[serde(skip_serializing)]
    pub access_token_encrypted: String,
    #[serde(skip_serializing)]
    pub refresh_token_encrypted: Option<String>,
    pub expires_at: String,
    pub scope: String,
    pub provider_user_id: Option<String>,
    pub provider_email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// OAuth state for CSRF protection
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthState {
    pub id: String,
    pub state: String,
    pub provider: String,
    pub user_id: Option<String>, // If linking to existing account
    pub redirect_url: Option<String>,
    pub expires_at: String,
    pub created_at: String,
}

/// Request to create/update OAuth provider
#[derive(Debug, Clone, Deserialize)]
pub struct UpsertOAuthProviderRequest {
    pub provider: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub enabled: bool,
}

/// OAuth callback parameters
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCallbackParams {
    pub code: String,
    pub state: String,
}

/// Token response from OAuth provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: Option<String>,
}

/// User info from OAuth provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub verified: bool,
}

/// Linked account info for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedAccount {
    pub provider: String,
    pub email: Option<String>,
    pub linked_at: String,
}

// ==================== DATABASE OPERATIONS ====================

/// Get all OAuth providers
pub async fn list_providers(pool: &SqlitePool) -> Result<Vec<OAuthProvider>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM oauth_providers ORDER BY provider"
    )
    .fetch_all(pool)
    .await
}

/// Get enabled OAuth providers (for login page)
pub async fn list_enabled_providers(pool: &SqlitePool) -> Result<Vec<OAuthProvider>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM oauth_providers WHERE enabled = 1 ORDER BY provider"
    )
    .fetch_all(pool)
    .await
}

/// Get OAuth provider by name
pub async fn get_provider(
    pool: &SqlitePool,
    provider: &str,
) -> Result<Option<OAuthProvider>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM oauth_providers WHERE provider = ?"
    )
    .bind(provider)
    .fetch_optional(pool)
    .await
}

/// Create or update OAuth provider
pub async fn upsert_provider(
    pool: &SqlitePool,
    req: UpsertOAuthProviderRequest,
) -> Result<OAuthProvider, sqlx::Error> {
    let existing = get_provider(pool, &req.provider).await?;
    let secret_encrypted = encrypt_secret(&req.client_secret);
    let scopes_json = serde_json::to_string(&req.scopes).unwrap_or_else(|_| "[]".to_string());
    
    if let Some(provider) = existing {
        // Update existing
        sqlx::query(
            "UPDATE oauth_providers 
             SET client_id = ?, client_secret_encrypted = ?, redirect_uri = ?,
                 scopes = ?, enabled = ?, updated_at = datetime('now')
             WHERE id = ?"
        )
        .bind(&req.client_id)
        .bind(&secret_encrypted)
        .bind(&req.redirect_uri)
        .bind(&scopes_json)
        .bind(req.enabled)
        .bind(&provider.id)
        .execute(pool)
        .await?;
        
        get_provider(pool, &req.provider).await.map(|p| p.unwrap())
    } else {
        // Create new
        let id = Uuid::new_v4().to_string();
        
        sqlx::query(
            "INSERT INTO oauth_providers 
             (id, provider, client_id, client_secret_encrypted, redirect_uri, scopes, enabled, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))"
        )
        .bind(&id)
        .bind(&req.provider)
        .bind(&req.client_id)
        .bind(&secret_encrypted)
        .bind(&req.redirect_uri)
        .bind(&scopes_json)
        .bind(req.enabled)
        .execute(pool)
        .await?;
        
        sqlx::query_as("SELECT * FROM oauth_providers WHERE id = ?")
            .bind(&id)
            .fetch_one(pool)
            .await
    }
}

/// Delete OAuth provider
pub async fn delete_provider(pool: &SqlitePool, provider: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM oauth_providers WHERE provider = ?")
        .bind(provider)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Create OAuth state for CSRF protection
pub async fn create_state(
    pool: &SqlitePool,
    provider: &str,
    user_id: Option<&str>,
    redirect_url: Option<&str>,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let state = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::minutes(10)).to_rfc3339();
    
    sqlx::query(
        "INSERT INTO oauth_states (id, state, provider, user_id, redirect_url, expires_at, created_at)
         VALUES (?, ?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(&state)
    .bind(provider)
    .bind(user_id)
    .bind(redirect_url)
    .bind(&expires_at)
    .execute(pool)
    .await?;
    
    Ok(state)
}

/// Validate and consume OAuth state
pub async fn validate_state(
    pool: &SqlitePool,
    state: &str,
) -> Result<Option<OAuthState>, sqlx::Error> {
    let oauth_state: Option<OAuthState> = sqlx::query_as(
        "SELECT * FROM oauth_states WHERE state = ? AND expires_at > datetime('now')"
    )
    .bind(state)
    .fetch_optional(pool)
    .await?;
    
    // Delete used state
    sqlx::query("DELETE FROM oauth_states WHERE state = ?")
        .bind(state)
        .execute(pool)
        .await?;
    
    Ok(oauth_state)
}

/// Store OAuth token for user
pub async fn store_token(
    pool: &SqlitePool,
    user_id: &str,
    provider: &str,
    token_response: &OAuthTokenResponse,
    user_info: &OAuthUserInfo,
) -> Result<OAuthToken, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let access_encrypted = encrypt_secret(&token_response.access_token);
    let refresh_encrypted = token_response.refresh_token.as_ref().map(|t| encrypt_secret(t));
    let expires_at = (Utc::now() + Duration::seconds(token_response.expires_in as i64)).to_rfc3339();
    let scope = token_response.scope.clone().unwrap_or_default();
    
    // Delete old tokens for this provider
    sqlx::query("DELETE FROM oauth_tokens WHERE user_id = ? AND provider = ?")
        .bind(user_id)
        .bind(provider)
        .execute(pool)
        .await?;
    
    sqlx::query(
        "INSERT INTO oauth_tokens 
         (id, user_id, provider, access_token_encrypted, refresh_token_encrypted,
          expires_at, scope, provider_user_id, provider_email, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
    )
    .bind(&id)
    .bind(user_id)
    .bind(provider)
    .bind(&access_encrypted)
    .bind(&refresh_encrypted)
    .bind(&expires_at)
    .bind(&scope)
    .bind(&user_info.id)
    .bind(&user_info.email)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM oauth_tokens WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Get user's linked OAuth accounts
pub async fn get_linked_accounts(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<LinkedAccount>, sqlx::Error> {
    let tokens: Vec<OAuthToken> = sqlx::query_as(
        "SELECT * FROM oauth_tokens WHERE user_id = ?"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    
    Ok(tokens.into_iter().map(|t| LinkedAccount {
        provider: t.provider,
        email: t.provider_email,
        linked_at: t.created_at,
    }).collect())
}

/// Unlink OAuth account
pub async fn unlink_account(
    pool: &SqlitePool,
    user_id: &str,
    provider: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM oauth_tokens WHERE user_id = ? AND provider = ?"
    )
    .bind(user_id)
    .bind(provider)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Find user by OAuth provider user ID
pub async fn find_user_by_oauth(
    pool: &SqlitePool,
    provider: &str,
    provider_user_id: &str,
) -> Result<Option<String>, sqlx::Error> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT user_id FROM oauth_tokens WHERE provider = ? AND provider_user_id = ?"
    )
    .bind(provider)
    .bind(provider_user_id)
    .fetch_optional(pool)
    .await?;
    
    Ok(result.map(|r| r.0))
}

// ==================== OAUTH FLOW ====================

/// Get authorization URL for OAuth provider
pub fn get_auth_url(provider: &OAuthProvider, state: &str) -> String {
    let base_url = match provider.provider.as_str() {
        "google" => "https://accounts.google.com/o/oauth2/v2/auth",
        "github" => "https://github.com/login/oauth/authorize",
        "microsoft" => "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
        _ => return String::new(),
    };
    
    let scopes: Vec<String> = serde_json::from_str(&provider.scopes)
        .unwrap_or_else(|_| get_default_scopes(&provider.provider));
    
    let mut params = vec![
        ("client_id", provider.client_id.clone()),
        ("redirect_uri", provider.redirect_uri.clone()),
        ("state", state.to_string()),
        ("scope", scopes.join(" ")),
    ];
    
    match provider.provider.as_str() {
        "google" => {
            params.push(("response_type", "code".to_string()));
            params.push(("access_type", "offline".to_string()));
            params.push(("prompt", "consent".to_string()));
        }
        "github" => {
            // GitHub uses different param names
        }
        "microsoft" => {
            params.push(("response_type", "code".to_string()));
            params.push(("response_mode", "query".to_string()));
        }
        _ => {}
    }
    
    let query = params.iter()
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    
    format!("{}?{}", base_url, query)
}

/// Exchange authorization code for access token
pub async fn exchange_code(
    provider: &OAuthProvider,
    code: &str,
) -> Result<OAuthTokenResponse, OAuthError> {
    let token_url = match provider.provider.as_str() {
        "google" => "https://oauth2.googleapis.com/token",
        "github" => "https://github.com/login/oauth/access_token",
        "microsoft" => "https://login.microsoftonline.com/common/oauth2/v2.0/token",
        _ => return Err(OAuthError::UnsupportedProvider(provider.provider.clone())),
    };
    
    let client_secret = decrypt_secret(&provider.client_secret_encrypted)?;
    
    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code".to_string());
    params.insert("code", code.to_string());
    params.insert("client_id", provider.client_id.clone());
    params.insert("client_secret", client_secret);
    params.insert("redirect_uri", provider.redirect_uri.clone());
    
    let client = reqwest::Client::new();
    let mut request = client.post(token_url).form(&params);
    
    // GitHub requires Accept header for JSON
    if provider.provider == "github" {
        request = request.header("Accept", "application/json");
    }
    
    let response = request.send().await
        .map_err(|e| OAuthError::NetworkError(e.to_string()))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(OAuthError::TokenExchangeFailed(error_text));
    }
    
    let token_data: serde_json::Value = response.json().await
        .map_err(|e| OAuthError::ParseError(e.to_string()))?;
    
    Ok(OAuthTokenResponse {
        access_token: token_data["access_token"].as_str().unwrap_or("").to_string(),
        refresh_token: token_data["refresh_token"].as_str().map(|s| s.to_string()),
        expires_in: token_data["expires_in"].as_u64().unwrap_or(3600),
        token_type: token_data["token_type"].as_str().unwrap_or("Bearer").to_string(),
        scope: token_data["scope"].as_str().map(|s| s.to_string()),
    })
}

/// Get user info from OAuth provider
pub async fn get_user_info(
    provider: &str,
    access_token: &str,
) -> Result<OAuthUserInfo, OAuthError> {
    let user_info_url = match provider {
        "google" => "https://www.googleapis.com/oauth2/v2/userinfo",
        "github" => "https://api.github.com/user",
        "microsoft" => "https://graph.microsoft.com/v1.0/me",
        _ => return Err(OAuthError::UnsupportedProvider(provider.to_string())),
    };
    
    let client = reqwest::Client::new();
    let response = client
        .get(user_info_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "SyncSpace/1.0")
        .send()
        .await
        .map_err(|e| OAuthError::NetworkError(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(OAuthError::UserInfoFailed(response.status().to_string()));
    }
    
    let user_data: serde_json::Value = response.json().await
        .map_err(|e| OAuthError::ParseError(e.to_string()))?;
    
    // Parse provider-specific response
    match provider {
        "google" => Ok(OAuthUserInfo {
            id: user_data["id"].as_str().unwrap_or("").to_string(),
            email: user_data["email"].as_str().unwrap_or("").to_string(),
            name: user_data["name"].as_str().unwrap_or("").to_string(),
            picture: user_data["picture"].as_str().map(|s| s.to_string()),
            verified: user_data["verified_email"].as_bool().unwrap_or(false),
        }),
        "github" => {
            // GitHub may need separate email request if email is private
            let email = user_data["email"].as_str().unwrap_or("").to_string();
            Ok(OAuthUserInfo {
                id: user_data["id"].to_string(),
                email,
                name: user_data["name"].as_str()
                    .or(user_data["login"].as_str())
                    .unwrap_or("")
                    .to_string(),
                picture: user_data["avatar_url"].as_str().map(|s| s.to_string()),
                verified: true, // GitHub emails are verified
            })
        }
        "microsoft" => Ok(OAuthUserInfo {
            id: user_data["id"].as_str().unwrap_or("").to_string(),
            email: user_data["userPrincipalName"].as_str()
                .or(user_data["mail"].as_str())
                .unwrap_or("")
                .to_string(),
            name: user_data["displayName"].as_str().unwrap_or("").to_string(),
            picture: None, // Microsoft Graph requires separate photo endpoint
            verified: true,
        }),
        _ => Err(OAuthError::UnsupportedProvider(provider.to_string())),
    }
}

// ==================== HELPERS ====================

fn get_default_scopes(provider: &str) -> Vec<String> {
    match provider {
        "google" => vec![
            "openid".to_string(),
            "email".to_string(),
            "profile".to_string(),
        ],
        "github" => vec![
            "read:user".to_string(),
            "user:email".to_string(),
        ],
        "microsoft" => vec![
            "openid".to_string(),
            "email".to_string(),
            "profile".to_string(),
            "User.Read".to_string(),
        ],
        _ => vec![],
    }
}

fn encrypt_secret(secret: &str) -> String {
    // Simple base64 encoding for now
    // TODO: Use proper encryption with AES-GCM
    general_purpose::STANDARD.encode(secret)
}

fn decrypt_secret(encrypted: &str) -> Result<String, OAuthError> {
    let bytes = general_purpose::STANDARD.decode(encrypted)
        .map_err(|e| OAuthError::DecryptionFailed(e.to_string()))?;
    String::from_utf8(bytes)
        .map_err(|e| OAuthError::DecryptionFailed(e.to_string()))
}

// ==================== ERRORS ====================

#[derive(Debug, Clone)]
pub enum OAuthError {
    UnsupportedProvider(String),
    NetworkError(String),
    TokenExchangeFailed(String),
    UserInfoFailed(String),
    ParseError(String),
    DecryptionFailed(String),
    InvalidState,
    AccountAlreadyLinked,
    UserNotFound,
}

impl std::fmt::Display for OAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthError::UnsupportedProvider(p) => write!(f, "Unsupported OAuth provider: {}", p),
            OAuthError::NetworkError(e) => write!(f, "Network error: {}", e),
            OAuthError::TokenExchangeFailed(e) => write!(f, "Token exchange failed: {}", e),
            OAuthError::UserInfoFailed(e) => write!(f, "Failed to get user info: {}", e),
            OAuthError::ParseError(e) => write!(f, "Parse error: {}", e),
            OAuthError::DecryptionFailed(e) => write!(f, "Decryption failed: {}", e),
            OAuthError::InvalidState => write!(f, "Invalid or expired OAuth state"),
            OAuthError::AccountAlreadyLinked => write!(f, "Account already linked to another user"),
            OAuthError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl std::error::Error for OAuthError {}

// Add urlencoding dependency or use this simple implementation
mod urlencoding {
    pub fn encode(s: &str) -> String {
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
}
