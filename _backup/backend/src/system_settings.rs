/// System settings management
/// Includes revision-safe delete option and other global settings
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SystemSetting {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    pub revision_safe_delete: bool, // Never physically delete files
    pub auto_thumbnail: bool,
    pub virus_scan_on_upload: bool,
    pub max_file_size_mb: i64,
    pub enable_encryption: bool,
    pub enable_versioning: bool,
    pub retention_days: i32,
    pub enable_audit_log: bool,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            revision_safe_delete: false,
            auto_thumbnail: true,
            virus_scan_on_upload: false,
            max_file_size_mb: 1024,
            enable_encryption: false,
            enable_versioning: true,
            retention_days: 365,
            enable_audit_log: true,
        }
    }
}

/// Get system setting
pub async fn get_setting(
    pool: &SqlitePool,
    key: &str,
) -> Result<Option<String>, sqlx::Error> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM system_settings WHERE key = ?"
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;
    
    Ok(result.map(|(v,)| v))
}

/// Set system setting
pub async fn set_setting(
    pool: &SqlitePool,
    key: &str,
    value: &str,
    description: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO system_settings (key, value, description, updated_at)
         VALUES (?, ?, ?, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = ?, description = ?, updated_at = datetime('now')"
    )
    .bind(key)
    .bind(value)
    .bind(description)
    .bind(value)
    .bind(description)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Get all settings as structured object
pub async fn get_all_settings(pool: &SqlitePool) -> Result<SystemSettings, sqlx::Error> {
    let mut settings = SystemSettings::default();
    
    if let Some(val) = get_setting(pool, "revision_safe_delete").await? {
        settings.revision_safe_delete = val == "true" || val == "1";
    }
    if let Some(val) = get_setting(pool, "auto_thumbnail").await? {
        settings.auto_thumbnail = val == "true" || val == "1";
    }
    if let Some(val) = get_setting(pool, "virus_scan_on_upload").await? {
        settings.virus_scan_on_upload = val == "true" || val == "1";
    }
    if let Some(val) = get_setting(pool, "max_file_size_mb").await? {
        settings.max_file_size_mb = val.parse().unwrap_or(1024);
    }
    if let Some(val) = get_setting(pool, "enable_encryption").await? {
        settings.enable_encryption = val == "true" || val == "1";
    }
    if let Some(val) = get_setting(pool, "enable_versioning").await? {
        settings.enable_versioning = val == "true" || val == "1";
    }
    if let Some(val) = get_setting(pool, "retention_days").await? {
        settings.retention_days = val.parse().unwrap_or(365);
    }
    if let Some(val) = get_setting(pool, "enable_audit_log").await? {
        settings.enable_audit_log = val == "true" || val == "1";
    }
    
    Ok(settings)
}

/// Update multiple settings
pub async fn update_settings(
    pool: &SqlitePool,
    settings: SystemSettings,
) -> Result<(), sqlx::Error> {
    set_setting(pool, "revision_safe_delete", &settings.revision_safe_delete.to_string(), Some("Never physically delete files")).await?;
    set_setting(pool, "auto_thumbnail", &settings.auto_thumbnail.to_string(), Some("Auto-generate thumbnails")).await?;
    set_setting(pool, "virus_scan_on_upload", &settings.virus_scan_on_upload.to_string(), Some("Scan files on upload")).await?;
    set_setting(pool, "max_file_size_mb", &settings.max_file_size_mb.to_string(), Some("Max file size in MB")).await?;
    set_setting(pool, "enable_encryption", &settings.enable_encryption.to_string(), Some("Enable file encryption")).await?;
    set_setting(pool, "enable_versioning", &settings.enable_versioning.to_string(), Some("Enable file versioning")).await?;
    set_setting(pool, "retention_days", &settings.retention_days.to_string(), Some("Data retention period")).await?;
    set_setting(pool, "enable_audit_log", &settings.enable_audit_log.to_string(), Some("Enable audit logging")).await?;
    
    Ok(())
}

/// Check if revision-safe delete is enabled
pub async fn is_revision_safe_delete_enabled(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let val = get_setting(pool, "revision_safe_delete").await?;
    Ok(val.map(|v| v == "true" || v == "1").unwrap_or(false))
}

/// Initialize default settings
pub async fn init_default_settings(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let defaults = SystemSettings::default();
    
    // Only set if not exists
    for (key, value, desc) in [
        ("revision_safe_delete", defaults.revision_safe_delete.to_string(), "Never physically delete files"),
        ("auto_thumbnail", defaults.auto_thumbnail.to_string(), "Auto-generate thumbnails"),
        ("virus_scan_on_upload", defaults.virus_scan_on_upload.to_string(), "Scan files on upload"),
        ("max_file_size_mb", defaults.max_file_size_mb.to_string(), "Max file size in MB"),
        ("enable_encryption", defaults.enable_encryption.to_string(), "Enable file encryption"),
        ("enable_versioning", defaults.enable_versioning.to_string(), "Enable file versioning"),
        ("retention_days", defaults.retention_days.to_string(), "Data retention period"),
        ("enable_audit_log", defaults.enable_audit_log.to_string(), "Enable audit logging"),
    ] {
        if get_setting(pool, key).await?.is_none() {
            set_setting(pool, key, &value, Some(desc)).await?;
        }
    }
    
    Ok(())
}
