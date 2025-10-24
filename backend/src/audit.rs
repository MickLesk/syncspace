/// Audit logging and compliance system
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<String>, // JSON
    pub severity: String, // "info", "warning", "critical"
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DataRetentionPolicy {
    pub id: String,
    pub name: String,
    pub resource_type: String,
    pub retention_days: i32,
    pub auto_delete: bool,
    pub created_at: String,
}

/// Log an audit event
pub async fn log_audit(
    pool: &SqlitePool,
    user_id: &str,
    action: &str,
    resource_type: &str,
    resource_id: &str,
    ip_address: Option<String>,
    metadata: Option<serde_json::Value>,
    severity: &str,
) -> Result<AuditLog, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let meta_json = metadata.map(|m| serde_json::to_string(&m).unwrap_or_default());
    
    sqlx::query(
        "INSERT INTO audit_logs (id, user_id, action, resource_type, resource_id, ip_address, metadata, severity, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(user_id)
    .bind(action)
    .bind(resource_type)
    .bind(resource_id)
    .bind(ip_address)
    .bind(meta_json)
    .bind(severity)
    .bind(Utc::now().to_rfc3339())
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM audit_logs WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}

/// Export audit logs as JSON
pub async fn export_audit_logs(
    pool: &SqlitePool,
    user_id: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Vec<AuditLog>, sqlx::Error> {
    let query = if let Some(uid) = user_id {
        sqlx::query_as(
            "SELECT * FROM audit_logs WHERE user_id = ? AND created_at BETWEEN ? AND ? ORDER BY created_at DESC"
        )
        .bind(uid)
        .bind(start_date.unwrap_or("1970-01-01"))
        .bind(end_date.unwrap_or("2099-12-31"))
    } else {
        sqlx::query_as(
            "SELECT * FROM audit_logs WHERE created_at BETWEEN ? AND ? ORDER BY created_at DESC"
        )
        .bind(start_date.unwrap_or("1970-01-01"))
        .bind(end_date.unwrap_or("2099-12-31"))
    };
    
    query.fetch_all(pool).await
}

/// Apply data retention policies
pub async fn apply_retention_policies(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let policies: Vec<DataRetentionPolicy> = sqlx::query_as(
        "SELECT * FROM data_retention_policies WHERE auto_delete = 1"
    )
    .fetch_all(pool)
    .await?;
    
    let mut total_deleted = 0u64;
    
    for policy in policies {
        let cutoff_date = Utc::now() - chrono::Duration::days(policy.retention_days as i64);
        
        let result = match policy.resource_type.as_str() {
            "audit_logs" => {
                sqlx::query("DELETE FROM audit_logs WHERE created_at < ?")
                    .bind(cutoff_date.to_rfc3339())
                    .execute(pool)
                    .await?
            }
            "trash" => {
                sqlx::query("DELETE FROM trash WHERE deleted_at < ?")
                    .bind(cutoff_date.to_rfc3339())
                    .execute(pool)
                    .await?
            }
            _ => continue,
        };
        
        total_deleted += result.rows_affected();
    }
    
    Ok(total_deleted)
}

/// Create retention policy
pub async fn create_retention_policy(
    pool: &SqlitePool,
    name: &str,
    resource_type: &str,
    retention_days: i32,
    auto_delete: bool,
) -> Result<DataRetentionPolicy, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query(
        "INSERT INTO data_retention_policies (id, name, resource_type, retention_days, auto_delete, created_at)
         VALUES (?, ?, ?, ?, ?, datetime('now'))"
    )
    .bind(&id)
    .bind(name)
    .bind(resource_type)
    .bind(retention_days)
    .bind(if auto_delete { 1 } else { 0 })
    .execute(pool)
    .await?;
    
    sqlx::query_as("SELECT * FROM data_retention_policies WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
}
