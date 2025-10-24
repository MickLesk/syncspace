/// OAuth2 authentication support
/// Supports Google, GitHub, Microsoft providers
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

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
    let access_token_encrypted = base64::encode(access_token);
    let refresh_token_encrypted = refresh_token.map(|t| base64::encode(&t));
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

/// Refresh OAuth token (placeholder - requires oauth2 crate)
pub async fn refresh_oauth_token(
    _pool: &SqlitePool,
    _provider: &OAuthProvider,
    _refresh_token: &str,
) -> Result<OAuthTokenResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Placeholder implementation
    // In production: use `oauth2` crate
    /*
    Example with oauth2 crate:
    
    use oauth2::{
        AuthUrl, ClientId, ClientSecret, RefreshToken, TokenUrl,
        basic::BasicClient, reqwest::async_http_client,
    };
    
    let client = BasicClient::new(
        ClientId::new(provider.client_id.clone()),
        Some(ClientSecret::new(decrypt_secret(&provider.client_secret_encrypted)?)),
        AuthUrl::new(get_auth_url(&provider.provider))?,
        Some(TokenUrl::new(get_token_url(&provider.provider))?)
    );
    
    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
        .request_async(async_http_client)
        .await?;
    
    Ok(OAuthTokenResponse {
        access_token: token_result.access_token().secret().clone(),
        refresh_token: token_result.refresh_token().map(|t| t.secret().clone()),
        expires_in: token_result.expires_in()
            .map(|d| d.as_secs())
            .unwrap_or(3600),
        token_type: "Bearer".to_string(),
    })
    */
    
    Err("OAuth token refresh not implemented - requires oauth2 crate".into())
}

/// Exchange authorization code for access token (placeholder)
pub async fn exchange_code_for_token(
    _provider: &OAuthProvider,
    _code: &str,
) -> Result<OAuthTokenResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Placeholder implementation
    // In production: use `oauth2` crate
    /*
    Example:
    
    let client = BasicClient::new(
        ClientId::new(provider.client_id.clone()),
        Some(ClientSecret::new(decrypt_secret(&provider.client_secret_encrypted)?)),
        AuthUrl::new(get_auth_url(&provider.provider))?,
        Some(TokenUrl::new(get_token_url(&provider.provider))?)
    ).set_redirect_uri(RedirectUrl::new(provider.redirect_uri.clone())?);
    
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await?;
    
    Ok(OAuthTokenResponse {
        access_token: token_result.access_token().secret().clone(),
        refresh_token: token_result.refresh_token().map(|t| t.secret().clone()),
        expires_in: token_result.expires_in().map(|d| d.as_secs()).unwrap_or(3600),
        token_type: "Bearer".to_string(),
    })
    */
    
    Err("OAuth code exchange not implemented - requires oauth2 crate".into())
}

/// Get user info from OAuth provider (placeholder)
pub async fn get_oauth_user_info(
    provider: &str,
    _access_token: &str,
) -> Result<OAuthUserInfo, Box<dyn std::error::Error + Send + Sync>> {
    // Placeholder - would make API calls to provider
    /*
    match provider {
        "google" => {
            // GET https://www.googleapis.com/oauth2/v2/userinfo
        },
        "github" => {
            // GET https://api.github.com/user
        },
        "microsoft" => {
            // GET https://graph.microsoft.com/v1.0/me
        },
        _ => return Err("Unknown provider".into())
    }
    */
    
    Err(format!("OAuth user info not implemented for provider: {}", provider).into())
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
