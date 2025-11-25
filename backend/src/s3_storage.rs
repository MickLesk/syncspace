/// S3-compatible storage backend
/// Supports AWS S3, MinIO, Wasabi, Backblaze B2, DigitalOcean Spaces
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, primitives::ByteStream};
use aws_sdk_s3::config::{Credentials, SharedCredentialsProvider};
use std::path::Path;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct S3Config {
    pub id: String,
    pub name: String,
    pub provider: String, // "aws", "minio", "wasabi", "b2", "spaces"
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key_encrypted: String,
    pub use_path_style: bool, // For MinIO
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateS3ConfigRequest {
    pub name: String,
    pub provider: String,
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub use_path_style: bool,
}

/// Add S3 configuration
pub async fn add_s3_config(
    pool: &SqlitePool,
    req: CreateS3ConfigRequest,
) -> Result<S3Config, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    // Encrypt secret_key properly using base64 encoding
    // In production, should use actual encryption (AES-256) with KMS
    let secret_encrypted = BASE64.encode(&req.secret_key);
    
    sqlx::query(
        "INSERT INTO s3_configs 
         (id, name, provider, endpoint, region, bucket, access_key, secret_key_encrypted, use_path_style, is_default, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, datetime('now'))"
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.provider)
    .bind(&req.endpoint)
    .bind(&req.region)
    .bind(&req.bucket)
    .bind(&req.access_key)
    .bind(&secret_encrypted)
    .bind(if req.use_path_style { 1 } else { 0 })
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM s3_configs WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Upload file to S3
pub async fn upload_to_s3(
    config: &S3Config,
    file_path: &str,
    s3_key: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Decrypt secret key (for now just base64 decode)
    let secret_key = String::from_utf8(BASE64.decode(&config.secret_key_encrypted)?)?;
    
    // Create AWS credentials
    let credentials = Credentials::new(
        &config.access_key,
        &secret_key,
        None,
        None,
        "s3_storage",
    );
    
    // Build AWS config
    let mut aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(config.region.clone()))
        .credentials_provider(SharedCredentialsProvider::new(credentials));
    
    // Set custom endpoint if not AWS
    if !config.endpoint.is_empty() && !config.endpoint.contains("amazonaws.com") {
        aws_config = aws_config.endpoint_url(&config.endpoint);
    }
    
    let sdk_config = aws_config.load().await;
    let mut s3_config = aws_sdk_s3::config::Builder::from(&sdk_config);
    
    // Enable path-style for MinIO and others
    if config.use_path_style {
        s3_config = s3_config.force_path_style(true);
    }
    
    let client = Client::from_conf(s3_config.build());
    
    // Read file and upload
    let body = ByteStream::from_path(Path::new(file_path)).await?;
    
    client
        .put_object()
        .bucket(&config.bucket)
        .key(s3_key)
        .body(body)
        .send()
        .await?;
    
    Ok(format!("s3://{}/{}", config.bucket, s3_key))
}

/// Download file from S3
pub async fn download_from_s3(
    config: &S3Config,
    s3_key: &str,
    local_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Decrypt secret key
    let secret_key = String::from_utf8(base64::decode(&config.secret_key_encrypted)?)?;
    
    let credentials = Credentials::new(
        &config.access_key,
        &secret_key,
        None,
        None,
        "s3_storage",
    );
    
    let mut aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(config.region.clone()))
        .credentials_provider(SharedCredentialsProvider::new(credentials));
    
    if !config.endpoint.is_empty() && !config.endpoint.contains("amazonaws.com") {
        aws_config = aws_config.endpoint_url(&config.endpoint);
    }
    
    let sdk_config = aws_config.load().await;
    let mut s3_config = aws_sdk_s3::config::Builder::from(&sdk_config);
    
    if config.use_path_style {
        s3_config = s3_config.force_path_style(true);
    }
    
    let client = Client::from_conf(s3_config.build());
    
    // Get object
    let resp = client
        .get_object()
        .bucket(&config.bucket)
        .key(s3_key)
        .send()
        .await?;
    
    // Write to file
    let data = resp.body.collect().await?;
    tokio::fs::write(local_path, data.into_bytes()).await?;
    
    Ok(())
}

/// List S3 configurations
pub async fn list_s3_configs(pool: &SqlitePool) -> Result<Vec<S3Config>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM s3_configs ORDER BY is_default DESC, created_at DESC"
    )
    .fetch_all(pool)
    .await
}

/// Create new S3 configuration
pub async fn create_s3_config(
    pool: &SqlitePool,
    mut config: S3Config,
) -> Result<S3Config, sqlx::Error> {
    use uuid::Uuid;
    use chrono::Utc;
    
    // Generate ID if not provided
    if config.id.is_empty() {
        config.id = Uuid::new_v4().to_string();
    }
    
    // Set timestamps
    let now = Utc::now().to_rfc3339();
    config.created_at = now.clone();
    config.updated_at = now;
    
    // Insert into database
    sqlx::query(
        "INSERT INTO s3_configs (id, name, endpoint, region, bucket, access_key, secret_key_encrypted, is_default, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&config.id)
    .bind(&config.name)
    .bind(&config.endpoint)
    .bind(&config.region)
    .bind(&config.bucket)
    .bind(&config.access_key)
    .bind(&config.secret_key_encrypted)
    .bind(config.is_default)
    .bind(&config.created_at)
    .bind(&config.updated_at)
    .execute(pool)
    .await?;
    
    Ok(config)
}

/// Get default S3 config
pub async fn get_default_s3_config(pool: &SqlitePool) -> Result<Option<S3Config>, sqlx::Error> {
    sqlx::query_as(
        "SELECT * FROM s3_configs WHERE is_default = 1 LIMIT 1"
    )
    .fetch_optional(pool)
    .await
}

/// Set default S3 config
pub async fn set_default_s3_config(
    pool: &SqlitePool,
    config_id: &str,
) -> Result<(), sqlx::Error> {
    // Unset all
    sqlx::query("UPDATE s3_configs SET is_default = 0")
        .execute(pool)
        .await?;
    
    // Set new default
    sqlx::query("UPDATE s3_configs SET is_default = 1 WHERE id = ?")
        .bind(config_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Delete S3 config
pub async fn delete_s3_config(
    pool: &SqlitePool,
    config_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM s3_configs WHERE id = ?")
        .bind(config_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// Test S3 connection
pub async fn test_s3_connection(
    config: &S3Config,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    // Decrypt secret key
    let secret_key = String::from_utf8(base64::decode(&config.secret_key_encrypted)?)?;
    
    let credentials = Credentials::new(
        &config.access_key,
        &secret_key,
        None,
        None,
        "s3_storage",
    );
    
    let mut aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(config.region.clone()))
        .credentials_provider(SharedCredentialsProvider::new(credentials));
    
    if !config.endpoint.is_empty() && !config.endpoint.contains("amazonaws.com") {
        aws_config = aws_config.endpoint_url(&config.endpoint);
    }
    
    let sdk_config = aws_config.load().await;
    let mut s3_config = aws_sdk_s3::config::Builder::from(&sdk_config);
    
    if config.use_path_style {
        s3_config = s3_config.force_path_style(true);
    }
    
    let client = Client::from_conf(s3_config.build());
    
    // Try to list bucket to test connection
    match client
        .list_objects_v2()
        .bucket(&config.bucket)
        .max_keys(1)
        .send()
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => {
            tracing::warn!("S3 connection test failed: {}", e);
            Ok(false)
        }
    }
}
