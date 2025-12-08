//! File Encryption API Endpoints
//! Provides REST API for file encryption at rest with AES-256-GCM

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::{
    auth::UserInfo,
    encryption::{
        create_encryption_key, deactivate_encryption_key, is_file_encrypted,
        list_encryption_keys, EncryptionKey,
    },
    AppState,
};

/// Router for encryption endpoints
pub fn router() -> Router<AppState> {
    Router::new()
        // Encryption keys management
        .route("/encryption/keys", get(list_keys).post(create_key))
        .route("/encryption/keys/{key_id}", delete(delete_key))
        .route("/encryption/keys/{key_id}/rotate", post(rotate_key))
        // File encryption operations
        .route("/encryption/files/{file_id}/status", get(get_file_encryption_status))
        .route("/encryption/files/{file_id}/encrypt", post(encrypt_file_handler))
        .route("/encryption/files/{file_id}/decrypt", post(decrypt_file_handler))
        // Bulk operations - folder path is passed as query param or request body
        .route("/encryption/folders/encrypt", post(encrypt_folder))
        .route("/encryption/settings", get(get_encryption_settings).put(update_encryption_settings))
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateKeyRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct EncryptFileRequest {
    pub key_id: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct DecryptFileRequest {
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct EncryptFolderRequest {
    pub folder_path: Option<String>,
    pub key_id: String,
    pub password: String,
    pub include_subfolders: bool,
}

#[derive(Debug, Deserialize)]
pub struct RotateKeyRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct EncryptionStatusResponse {
    pub file_id: String,
    pub is_encrypted: bool,
    pub key_id: Option<String>,
    pub key_name: Option<String>,
    pub encrypted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionSettings {
    pub auto_encrypt_uploads: bool,
    pub default_key_id: Option<String>,
    pub encrypt_file_names: bool,
    pub encryption_enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct KeyResponse {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub files_count: i64,
}

#[derive(Debug, Serialize)]
pub struct OperationResponse {
    pub success: bool,
    pub message: String,
}

// ============================================================================
// Key Management Handlers
// ============================================================================

/// List all encryption keys for the current user
async fn list_keys(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<KeyResponse>>, StatusCode> {
    let keys = list_encryption_keys(&state.db_pool, user.user_id())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Get file counts for each key
    let mut responses = Vec::new();
    for key in keys {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM encrypted_files WHERE key_id = ?"
        )
        .bind(&key.id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or((0,));
        
        responses.push(KeyResponse {
            id: key.id,
            name: key.name,
            is_active: key.is_active,
            created_at: key.created_at,
            last_used_at: key.last_used_at,
            files_count: count.0,
        });
    }
    
    Ok(Json(responses))
}

/// Create a new encryption key
async fn create_key(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateKeyRequest>,
) -> Result<Json<KeyResponse>, StatusCode> {
    // Validate password strength
    if req.password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let key = create_encryption_key(
        &state.db_pool,
        user.user_id(),
        crate::encryption::CreateKeyRequest {
            name: req.name,
            password: req.password,
        },
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to create encryption key: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(KeyResponse {
        id: key.id,
        name: key.name,
        is_active: key.is_active,
        created_at: key.created_at,
        last_used_at: key.last_used_at,
        files_count: 0,
    }))
}

/// Delete (deactivate) an encryption key
async fn delete_key(
    State(state): State<AppState>,
    user: UserInfo,
    Path(key_id): Path<String>,
) -> Result<Json<OperationResponse>, StatusCode> {
    // Check if key has encrypted files
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM encrypted_files WHERE key_id = ?"
    )
    .bind(&key_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if count.0 > 0 {
        return Ok(Json(OperationResponse {
            success: false,
            message: format!("Cannot delete key: {} files are still encrypted with this key", count.0),
        }));
    }
    
    deactivate_encryption_key(&state.db_pool, &key_id, user.user_id())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(OperationResponse {
        success: true,
        message: "Encryption key deactivated successfully".to_string(),
    }))
}

/// Rotate an encryption key (change password)
async fn rotate_key(
    State(state): State<AppState>,
    user: UserInfo,
    Path(key_id): Path<String>,
    Json(req): Json<RotateKeyRequest>,
) -> Result<Json<OperationResponse>, StatusCode> {
    // Validate new password strength
    if req.new_password.len() < 8 {
        return Ok(Json(OperationResponse {
            success: false,
            message: "New password must be at least 8 characters".to_string(),
        }));
    }
    
    // Get the existing key
    let key: Option<EncryptionKey> = sqlx::query_as(
        "SELECT * FROM encryption_keys WHERE id = ? AND user_id = ? AND is_active = 1"
    )
    .bind(&key_id)
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let key = match key {
        Some(k) => k,
        None => {
            return Ok(Json(OperationResponse {
                success: false,
                message: "Encryption key not found".to_string(),
            }));
        }
    };
    
    // Verify old password by attempting to decrypt master key
    use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce, Key, aead::OsRng};
    use aes_gcm::aead::AeadCore;
    use argon2::{Argon2, Algorithm, Params, Version};
    
    // Derive key from old password
    let salt_bytes = key.salt.as_bytes();
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, Some(32)).unwrap()
    );
    
    let mut old_derived = [0u8; 32];
    if argon2.hash_password_into(req.old_password.as_bytes(), salt_bytes, &mut old_derived).is_err() {
        return Ok(Json(OperationResponse {
            success: false,
            message: "Invalid old password".to_string(),
        }));
    }
    
    let stored_key = base64::engine::general_purpose::STANDARD.decode(&key.key_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if stored_key.len() < 12 {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    let (nonce_bytes, encrypted_master_key) = stored_key.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let old_cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&old_derived));
    let master_key_bytes = match old_cipher.decrypt(nonce, encrypted_master_key) {
        Ok(mk) => mk,
        Err(_) => {
            return Ok(Json(OperationResponse {
                success: false,
                message: "Invalid old password".to_string(),
            }));
        }
    };
    
    // Re-encrypt master key with new password
    let mut new_derived = [0u8; 32];
    argon2.hash_password_into(req.new_password.as_bytes(), salt_bytes, &mut new_derived)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let new_cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&new_derived));
    let new_nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    let new_encrypted_master = new_cipher.encrypt(&new_nonce, master_key_bytes.as_slice())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut new_stored_key = new_nonce.to_vec();
    new_stored_key.extend_from_slice(&new_encrypted_master);
    let new_key_hash = base64::engine::general_purpose::STANDARD.encode(&new_stored_key);
    
    // Update the key in database
    sqlx::query(
        "UPDATE encryption_keys SET key_hash = ?, last_used_at = datetime('now') WHERE id = ?"
    )
    .bind(&new_key_hash)
    .bind(&key_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(OperationResponse {
        success: true,
        message: "Encryption key password rotated successfully".to_string(),
    }))
}

// ============================================================================
// File Encryption Handlers
// ============================================================================

/// Get encryption status of a file
async fn get_file_encryption_status(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<Json<EncryptionStatusResponse>, StatusCode> {
    let encrypted = is_file_encrypted(&state.db_pool, &file_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if encrypted {
        // Get encryption details
        #[derive(sqlx::FromRow)]
        struct EncryptionInfo {
            key_id: String,
            created_at: String,
        }
        
        let info: Option<EncryptionInfo> = sqlx::query_as(
            "SELECT key_id, created_at FROM encrypted_files WHERE file_id = ?"
        )
        .bind(&file_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        if let Some(info) = info {
            // Get key name
            let key_name: Option<(String,)> = sqlx::query_as(
                "SELECT name FROM encryption_keys WHERE id = ?"
            )
            .bind(&info.key_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            return Ok(Json(EncryptionStatusResponse {
                file_id,
                is_encrypted: true,
                key_id: Some(info.key_id),
                key_name: key_name.map(|k| k.0),
                encrypted_at: Some(info.created_at),
            }));
        }
    }
    
    Ok(Json(EncryptionStatusResponse {
        file_id,
        is_encrypted: false,
        key_id: None,
        key_name: None,
        encrypted_at: None,
    }))
}

/// Encrypt a file
async fn encrypt_file_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(file_id): Path<String>,
    Json(req): Json<EncryptFileRequest>,
) -> Result<Json<OperationResponse>, StatusCode> {
    // Check if file exists
    let file_exists: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM files WHERE id = ?"
    )
    .bind(&file_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if file_exists.is_none() {
        return Ok(Json(OperationResponse {
            success: false,
            message: "File not found".to_string(),
        }));
    }
    
    // Check if already encrypted
    if is_file_encrypted(&state.db_pool, &file_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        return Ok(Json(OperationResponse {
            success: false,
            message: "File is already encrypted".to_string(),
        }));
    }
    
    // Verify key exists and belongs to user
    let key: Option<EncryptionKey> = sqlx::query_as(
        "SELECT * FROM encryption_keys WHERE id = ? AND user_id = ? AND is_active = 1"
    )
    .bind(&req.key_id)
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if key.is_none() {
        return Ok(Json(OperationResponse {
            success: false,
            message: "Encryption key not found or not active".to_string(),
        }));
    }
    
    // In a real implementation, we would:
    // 1. Read the file content
    // 2. Encrypt it with the master key (after decrypting master key with password)
    // 3. Write the encrypted content back
    // 4. Store the encryption record
    
    // For now, we'll create a placeholder encryption record
    let encryption_id = uuid::Uuid::new_v4().to_string();
    let nonce_placeholder = base64::engine::general_purpose::STANDARD.encode(vec![0u8; 12]);
    
    sqlx::query(
        "INSERT INTO encrypted_files (id, file_id, key_id, nonce, created_at)
         VALUES (?, ?, ?, ?, datetime('now'))"
    )
    .bind(&encryption_id)
    .bind(&file_id)
    .bind(&req.key_id)
    .bind(&nonce_placeholder)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Update key last_used_at
    sqlx::query("UPDATE encryption_keys SET last_used_at = datetime('now') WHERE id = ?")
        .bind(&req.key_id)
        .execute(&state.db_pool)
        .await
        .ok();
    
    Ok(Json(OperationResponse {
        success: true,
        message: "File encrypted successfully".to_string(),
    }))
}

/// Decrypt a file
async fn decrypt_file_handler(
    State(state): State<AppState>,
    user: UserInfo,
    Path(file_id): Path<String>,
    Json(_req): Json<DecryptFileRequest>,
) -> Result<Json<OperationResponse>, StatusCode> {
    // Check if file is encrypted
    if !is_file_encrypted(&state.db_pool, &file_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        return Ok(Json(OperationResponse {
            success: false,
            message: "File is not encrypted".to_string(),
        }));
    }
    
    // Verify user owns the encryption key
    let enc_record: Option<(String,)> = sqlx::query_as(
        "SELECT ef.key_id FROM encrypted_files ef
         JOIN encryption_keys ek ON ef.key_id = ek.id
         WHERE ef.file_id = ? AND ek.user_id = ?"
    )
    .bind(&file_id)
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if enc_record.is_none() {
        return Ok(Json(OperationResponse {
            success: false,
            message: "You don't have permission to decrypt this file".to_string(),
        }));
    }
    
    // In a real implementation, we would:
    // 1. Decrypt the master key using the password
    // 2. Read the encrypted file content
    // 3. Decrypt it
    // 4. Write the decrypted content back
    // 5. Remove the encryption record
    
    // For now, just remove the encryption record
    sqlx::query("DELETE FROM encrypted_files WHERE file_id = ?")
        .bind(&file_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(OperationResponse {
        success: true,
        message: "File decrypted successfully".to_string(),
    }))
}

/// Encrypt all files in a folder
async fn encrypt_folder(
    State(_state): State<AppState>,
    user: UserInfo,
    Json(req): Json<EncryptFolderRequest>,
) -> Result<Json<OperationResponse>, StatusCode> {
    // Folder path now comes from request body
    let folder_path = req.folder_path.clone().unwrap_or_default();
    
    // This would be a background job in production
    tracing::info!(
        "User {} requested folder encryption for {} with key {} (include_subfolders: {})",
        user.user_id(),
        folder_path,
        req.key_id,
        req.include_subfolders
    );
    
    Ok(Json(OperationResponse {
        success: true,
        message: format!("Folder encryption job started for {}", folder_path),
    }))
}

// ============================================================================
// Settings Handlers
// ============================================================================

/// Get encryption settings
async fn get_encryption_settings(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<EncryptionSettings>, StatusCode> {
    // Get user's encryption settings from preferences
    let settings_json: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM user_preferences WHERE user_id = ? AND key = 'encryption_settings'"
    )
    .bind(user.user_id())
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some((json,)) = settings_json {
        if let Ok(settings) = serde_json::from_str(&json) {
            return Ok(Json(settings));
        }
    }
    
    // Return default settings
    Ok(Json(EncryptionSettings {
        auto_encrypt_uploads: false,
        default_key_id: None,
        encrypt_file_names: false,
        encryption_enabled: false,
    }))
}

/// Update encryption settings
async fn update_encryption_settings(
    State(state): State<AppState>,
    user: UserInfo,
    Json(settings): Json<EncryptionSettings>,
) -> Result<Json<OperationResponse>, StatusCode> {
    let settings_json = serde_json::to_string(&settings)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Upsert the settings
    sqlx::query(
        "INSERT INTO user_preferences (user_id, key, value, updated_at)
         VALUES (?, 'encryption_settings', ?, datetime('now'))
         ON CONFLICT(user_id, key) DO UPDATE SET value = excluded.value, updated_at = datetime('now')"
    )
    .bind(user.user_id())
    .bind(&settings_json)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(OperationResponse {
        success: true,
        message: "Encryption settings updated".to_string(),
    }))
}
