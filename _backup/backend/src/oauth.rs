/// OAuth2 authentication support
/// Supports Google, GitHub, Microsoft providers
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthProvider {
    pub id: String,
    pub provider: String, // "google", "github", "microsoft"
    pub client_id: String,
    pub client_secret_encrypted: String,
    pub redirect_uri: String,
    pub scopes: String, // JSON array
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthToken {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub access_token_encrypted: String,
    pub refresh_token_encrypted: Option<String>,
    pub expires_at: String,
    pub scope: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub google: Option<OAuthProviderConfig>,
    pub github: Option<OAuthProviderConfig>,
    pub microsoft: Option<OAuthProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCallback {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
}

/// Get OAuth provider configuration
pub async fn get_oauth_provider(
    pool: &SqlitePool,
    provider: &str,
) -> Result<Option<OAuthProvider>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM oauth_providers WHERE provider = ? AND enabled = 1"
    )
    .bind(provider)
    .fetch_optional(pool)
    .await
}

/// Store OAuth token for user
pub async fn store_oauth_token(
    pool: &SqlitePool,
    user_id: &str,
    provider: &str,
    access_token: &str,
    refresh_token: Option<String>,
    expires_in: u64,
    scope: &str,
) -> Result<OAuthToken, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let access_token_encrypted = general_purpose::STANDARD.encode(access_token);
    let refresh_token_encrypted = refresh_token.map(|t| general_purpose::STANDARD.encode(&t));
    let expires_at = (Utc::now() + Duration::seconds(expires_in as i64)).to_rfc3339();
    
    // Delete old tokens for this provider
    sqlx::query("DELETE FROM oauth_tokens WHERE user_id = ? AND provider = ?")
        .bind(user_id)
        .bind(provider)
        .execute(pool)
        .await?;
    
    sqlx::query(
        "INSERT INTO oauth_tokens 
         (id, user_id, provider, access_token_encrypted, refresh_token_encrypted, 
          expires_at, scope, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
    )
    .bind(&id)
    .bind(user_id)
    .bind(provider)
    .bind(&access_token_encrypted)
    .bind(&refresh_token_encrypted)
    .bind(&expires_at)
    .bind(scope)
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM oauth_tokens WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Get OAuth token for user
pub async fn get_oauth_token(
    pool: &SqlitePool,
    user_id: &str,
    provider: &str,
) -> Result<Option<OAuthToken>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM oauth_tokens WHERE user_id = ? AND provider = ?"
    )
    .bind(user_id)
    .bind(provider)
    .fetch_optional(pool)
    .await
}

/// Refresh OAuth token
pub async fn refresh_oauth_token(
    pool: &SqlitePool,
    provider: &OAuthProvider,
    refresh_token: &str,
) -> Result<OAuthTokenResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Implementation note: This is a simplified version
    // For production OAuth2, uncomment the oauth2 crate implementation below
    
    let client = reqwest::Client::new();
    let token_url = get_token_url(&provider.provider)?;
    let client_secret = decrypt_secret(&provider.client_secret_encrypted)?;
    
    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", refresh_token);
    params.insert("client_id", &provider.client_id);
    params.insert("client_secret", &client_secret);
    
    let response = client
        .post(&token_url)
        .form(&params)
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("Token refresh failed: {}", response.status()).into());
    }
    
    let token_data: serde_json::Value = response.json().await?;
    
    Ok(OAuthTokenResponse {
        access_token: token_data["access_token"].as_str().unwrap_or("").to_string(),
        refresh_token: token_data["refresh_token"].as_str().map(|s| s.to_string()),
        expires_in: token_data["expires_in"].as_u64().unwrap_or(3600),
        token_type: token_data["token_type"].as_str().unwrap_or("Bearer").to_string(),
    })
}

/// Exchange authorization code for access token
pub async fn exchange_code_for_token(
    provider: &OAuthProvider,
    code: &str,
) -> Result<OAuthTokenResponse, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let token_url = get_token_url(&provider.provider)?;
    let client_secret = decrypt_secret(&provider.client_secret_encrypted)?;
    
    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", code);
    params.insert("client_id", &provider.client_id);
    params.insert("client_secret", &client_secret);
    params.insert("redirect_uri", &provider.redirect_uri);
    
    let response = client
        .post(&token_url)
        .form(&params)
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("Code exchange failed: {}", response.status()).into());
    }
    
    let token_data: serde_json::Value = response.json().await?;
    
    Ok(OAuthTokenResponse {
        access_token: token_data["access_token"].as_str().unwrap_or("").to_string(),
        refresh_token: token_data["refresh_token"].as_str().map(|s| s.to_string()),
        expires_in: token_data["expires_in"].as_u64().unwrap_or(3600),
        token_type: token_data["token_type"].as_str().unwrap_or("Bearer").to_string(),
    })
}

/// Get user info from OAuth provider
pub async fn get_oauth_user_info(
    provider: &str,
    access_token: &str,
) -> Result<OAuthUserInfo, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let user_info_url = get_user_info_url(provider);
    
    if user_info_url.is_empty() {
        return Err(format!("Unknown OAuth provider: {}", provider).into());
    }
    
    let response = client
        .get(&user_info_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "SyncSpace")
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to get user info: {}", response.status()).into());
    }
    
    let user_data: serde_json::Value = response.json().await?;
    
    // Parse provider-specific response
    let user_info = match provider {
        "google" => OAuthUserInfo {
            id: user_data["id"].as_str().unwrap_or("").to_string(),
            email: user_data["email"].as_str().unwrap_or("").to_string(),
            name: user_data["name"].as_str().unwrap_or("").to_string(),
            picture: user_data["picture"].as_str().map(|s| s.to_string()),
        },
        "github" => OAuthUserInfo {
            id: user_data["id"].to_string(),
            email: user_data["email"].as_str().unwrap_or("").to_string(),
            name: user_data["name"].as_str().unwrap_or(user_data["login"].as_str().unwrap_or("")).to_string(),
            picture: user_data["avatar_url"].as_str().map(|s| s.to_string()),
        },
        "microsoft" => OAuthUserInfo {
            id: user_data["id"].as_str().unwrap_or("").to_string(),
            email: user_data["userPrincipalName"].as_str().unwrap_or("").to_string(),
            name: user_data["displayName"].as_str().unwrap_or("").to_string(),
            picture: None, // Microsoft Graph requires separate photo endpoint
        },
        _ => return Err(format!("Unsupported provider: {}", provider).into()),
    };
    
    Ok(user_info)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

/// Initialize default OAuth providers
pub async fn init_oauth_providers(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Check if providers already exist
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM oauth_providers"
    )
    .fetch_one(pool)
    .await?;
    
    if count.0 == 0 {
        // Add placeholder providers (disabled by default)
        for provider in &["google", "github", "microsoft"] {
            let id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO oauth_providers 
                 (id, provider, client_id, client_secret_encrypted, redirect_uri, scopes, enabled, created_at)
                 VALUES (?, ?, 'CHANGE_ME', '', 'http://localhost:5173/auth/callback', '[]', 0, datetime('now'))"
            )
            .bind(&id)
            .bind(provider)
            .execute(pool)
            .await?;
        }
    }
    
    Ok(())
}

/// Provider-specific URLs
pub fn get_auth_url(provider: &str) -> String {
    match provider {
        "google" => "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
        "github" => "https://github.com/login/oauth/authorize".to_string(),
        "microsoft" => "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        _ => String::new(),
    }
}

pub fn get_token_url(provider: &str) -> String {
    match provider {
        "google" => "https://oauth2.googleapis.com/token".to_string(),
        "github" => "https://github.com/login/oauth/access_token".to_string(),
        "microsoft" => "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
        _ => String::new(),
    }
}

pub fn get_user_info_url(provider: &str) -> String {
    match provider {
        "google" => "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
        "github" => "https://api.github.com/user".to_string(),
        "microsoft" => "https://graph.microsoft.com/v1.0/me".to_string(),
        _ => String::new(),
    }
}
