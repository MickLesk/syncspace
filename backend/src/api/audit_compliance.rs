//! Enhanced Audit & Compliance API
//! Provides comprehensive audit logging, compliance reports, and retention policies

use crate::auth::UserInfo;
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLogEntry {
    pub id: String,
    pub user_id: String,
    pub username: Option<String>,
    pub action: String,
    pub action_category: Option<String>,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub resource_name: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub request_method: Option<String>,
    pub request_path: Option<String>,
    pub response_status: Option<i32>,
    pub metadata: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub severity: Option<String>,
    pub is_sensitive: Option<bool>,
    pub is_compliance_relevant: Option<bool>,
    pub retention_until: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RetentionPolicy {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub retention_days: i32,
    pub auto_delete: bool,
    pub archive_before_delete: Option<bool>,
    pub notify_before_delete: Option<bool>,
    pub notify_days_before: Option<i32>,
    pub is_active: Option<bool>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub last_applied_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ComplianceReport {
    pub id: String,
    pub report_type: String,
    pub report_name: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub filters: Option<String>,
    pub total_records: Option<i32>,
    pub file_path: Option<String>,
    pub file_format: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub checksum: Option<String>,
    pub generated_by: String,
    pub generated_at: Option<String>,
    pub expires_at: Option<String>,
    pub download_count: Option<i32>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ComplianceTemplate {
    pub id: String,
    pub name: String,
    pub report_type: String,
    pub description: Option<String>,
    pub filters: Option<String>,
    pub columns: Option<String>,
    pub is_default: Option<bool>,
    pub is_public: Option<bool>,
    pub created_by: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditArchive {
    pub id: String,
    pub archive_name: String,
    pub start_date: String,
    pub end_date: String,
    pub record_count: i32,
    pub file_path: String,
    pub file_size_bytes: Option<i64>,
    pub checksum: Option<String>,
    pub compression_type: Option<String>,
    pub is_encrypted: Option<bool>,
    pub created_by: Option<String>,
    pub created_at: String,
}

// ============================================================================
// Query Parameters
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct AuditLogQuery {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub action_category: Option<String>,
    pub resource_type: Option<String>,
    pub severity: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub compliance_only: Option<bool>,
    pub search: Option<String>,
}

fn default_limit() -> i32 {
    100
}

#[derive(Debug, Deserialize)]
pub struct CreateAuditLogRequest {
    pub action: String,
    pub action_category: Option<String>,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub resource_name: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub severity: Option<String>,
    pub is_sensitive: Option<bool>,
    pub is_compliance_relevant: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRetentionPolicyRequest {
    pub name: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub retention_days: i32,
    pub auto_delete: Option<bool>,
    pub archive_before_delete: Option<bool>,
    pub notify_before_delete: Option<bool>,
    pub notify_days_before: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRetentionPolicyRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub retention_days: Option<i32>,
    pub auto_delete: Option<bool>,
    pub archive_before_delete: Option<bool>,
    pub notify_before_delete: Option<bool>,
    pub notify_days_before: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateReportRequest {
    pub report_type: String,
    pub report_name: String,
    pub description: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub file_format: Option<String>,
    pub filters: Option<serde_json::Value>,
    pub template_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuditStats {
    pub total_logs: i64,
    pub logs_today: i64,
    pub logs_this_week: i64,
    pub logs_this_month: i64,
    pub critical_events: i64,
    pub error_events: i64,
    pub warning_events: i64,
    pub compliance_events: i64,
    pub by_category: Vec<CategoryCount>,
    pub by_action: Vec<ActionCount>,
    pub top_users: Vec<UserAuditCount>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CategoryCount {
    pub category: Option<String>,
    pub count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ActionCount {
    pub action: String,
    pub count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserAuditCount {
    pub user_id: String,
    pub username: Option<String>,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct RetentionResult {
    pub policy_id: String,
    pub policy_name: String,
    pub records_deleted: u64,
    pub records_archived: u64,
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        // Audit Logs
        .route("/audit/logs", get(list_audit_logs))
        .route("/audit/logs", post(create_audit_log))
        .route("/audit/logs/:id", get(get_audit_log))
        .route("/audit/logs/export", get(export_audit_logs))
        .route("/audit/stats", get(get_audit_stats))
        // Retention Policies
        .route("/audit/retention-policies", get(list_retention_policies))
        .route("/audit/retention-policies", post(create_retention_policy))
        .route("/audit/retention-policies/:id", get(get_retention_policy))
        .route(
            "/audit/retention-policies/:id",
            put(update_retention_policy),
        )
        .route(
            "/audit/retention-policies/:id",
            delete(delete_retention_policy),
        )
        .route(
            "/audit/retention-policies/:id/apply",
            post(apply_retention_policy),
        )
        .route(
            "/audit/retention/apply-all",
            post(apply_all_retention_policies),
        )
        // Compliance Reports
        .route("/audit/reports", get(list_compliance_reports))
        .route("/audit/reports", post(generate_compliance_report))
        .route("/audit/reports/:id", get(get_compliance_report))
        .route("/audit/reports/:id", delete(delete_compliance_report))
        .route(
            "/audit/reports/:id/download",
            get(download_compliance_report),
        )
        // Templates
        .route("/audit/templates", get(list_compliance_templates))
        .route("/audit/templates/:id", get(get_compliance_template))
        // Archives
        .route("/audit/archives", get(list_audit_archives))
        .route("/audit/archives", post(create_audit_archive))
        .route("/audit/archives/:id", delete(delete_audit_archive))
}

// ============================================================================
// Audit Log Handlers
// ============================================================================

async fn list_audit_logs(
    State(state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLogEntry>>, StatusCode> {
    let mut sql = String::from("SELECT * FROM audit_logs WHERE 1=1");

    if query.user_id.is_some() {
        sql.push_str(" AND user_id = ?");
    }
    if query.action.is_some() {
        sql.push_str(" AND action = ?");
    }
    if query.action_category.is_some() {
        sql.push_str(" AND action_category = ?");
    }
    if query.resource_type.is_some() {
        sql.push_str(" AND resource_type = ?");
    }
    if query.severity.is_some() {
        sql.push_str(" AND severity = ?");
    }
    if query.start_date.is_some() {
        sql.push_str(" AND created_at >= ?");
    }
    if query.end_date.is_some() {
        sql.push_str(" AND created_at <= ?");
    }
    if query.compliance_only == Some(true) {
        sql.push_str(" AND is_compliance_relevant = 1");
    }
    if query.search.is_some() {
        sql.push_str(" AND (action LIKE ? OR resource_name LIKE ? OR metadata LIKE ?)");
    }

    sql.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    // Build query dynamically
    let mut q = sqlx::query_as::<_, AuditLogEntry>(&sql);

    if let Some(ref uid) = query.user_id {
        q = q.bind(uid);
    }
    if let Some(ref action) = query.action {
        q = q.bind(action);
    }
    if let Some(ref cat) = query.action_category {
        q = q.bind(cat);
    }
    if let Some(ref rt) = query.resource_type {
        q = q.bind(rt);
    }
    if let Some(ref sev) = query.severity {
        q = q.bind(sev);
    }
    if let Some(ref sd) = query.start_date {
        q = q.bind(sd);
    }
    if let Some(ref ed) = query.end_date {
        q = q.bind(ed);
    }
    if let Some(ref search) = query.search {
        let pattern = format!("%{}%", search);
        q = q.bind(&pattern).bind(&pattern).bind(&pattern);
    }

    q = q.bind(query.limit).bind(query.offset);

    let logs = q
        .fetch_all(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(logs))
}

async fn get_audit_log(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<AuditLogEntry>, StatusCode> {
    let log: AuditLogEntry = sqlx::query_as("SELECT * FROM audit_logs WHERE id = ?")
        .bind(&id)
        .fetch_optional(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(log))
}

async fn create_audit_log(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateAuditLogRequest>,
) -> Result<Json<AuditLogEntry>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let metadata_json = req
        .metadata
        .map(|m| serde_json::to_string(&m).unwrap_or_default());
    let old_value_json = req
        .old_value
        .map(|v| serde_json::to_string(&v).unwrap_or_default());
    let new_value_json = req
        .new_value
        .map(|v| serde_json::to_string(&v).unwrap_or_default());

    sqlx::query(
        "INSERT INTO audit_logs (id, user_id, username, action, action_category, resource_type, 
         resource_id, resource_name, metadata, old_value, new_value, severity, 
         is_sensitive, is_compliance_relevant, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&user.user_id)
    .bind(&user.username)
    .bind(&req.action)
    .bind(&req.action_category.unwrap_or_else(|| "general".to_string()))
    .bind(&req.resource_type)
    .bind(&req.resource_id)
    .bind(&req.resource_name)
    .bind(&metadata_json)
    .bind(&old_value_json)
    .bind(&new_value_json)
    .bind(&req.severity.unwrap_or_else(|| "info".to_string()))
    .bind(req.is_sensitive.unwrap_or(false))
    .bind(req.is_compliance_relevant.unwrap_or(false))
    .bind(&now)
    .execute(&*state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let log: AuditLogEntry = sqlx::query_as("SELECT * FROM audit_logs WHERE id = ?")
        .bind(&id)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(log))
}

async fn export_audit_logs(
    State(state): State<AppState>,
    _user: UserInfo,
    Query(query): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLogEntry>>, StatusCode> {
    let mut sql = String::from("SELECT * FROM audit_logs WHERE 1=1");

    if query.start_date.is_some() {
        sql.push_str(" AND created_at >= ?");
    }
    if query.end_date.is_some() {
        sql.push_str(" AND created_at <= ?");
    }
    if query.action_category.is_some() {
        sql.push_str(" AND action_category = ?");
    }

    sql.push_str(" ORDER BY created_at DESC");

    let mut q = sqlx::query_as::<_, AuditLogEntry>(&sql);

    if let Some(ref sd) = query.start_date {
        q = q.bind(sd);
    }
    if let Some(ref ed) = query.end_date {
        q = q.bind(ed);
    }
    if let Some(ref cat) = query.action_category {
        q = q.bind(cat);
    }

    let logs = q
        .fetch_all(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(logs))
}

async fn get_audit_stats(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<AuditStats>, StatusCode> {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let week_ago = (Utc::now() - Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    let month_ago = (Utc::now() - Duration::days(30))
        .format("%Y-%m-%d")
        .to_string();

    // Total counts
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_logs")
        .fetch_one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let today_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE date(created_at) = date(?)")
            .bind(&today)
            .fetch_one(&*state.db)
            .await
            .unwrap_or((0,));

    let week_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE created_at >= ?")
            .bind(&week_ago)
            .fetch_one(&*state.db)
            .await
            .unwrap_or((0,));

    let month_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE created_at >= ?")
            .bind(&month_ago)
            .fetch_one(&*state.db)
            .await
            .unwrap_or((0,));

    // Severity counts
    let critical: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE severity = 'critical'")
            .fetch_one(&*state.db)
            .await
            .unwrap_or((0,));

    let error: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE severity = 'error'")
        .fetch_one(&*state.db)
        .await
        .unwrap_or((0,));

    let warning: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE severity = 'warning'")
            .fetch_one(&*state.db)
            .await
            .unwrap_or((0,));

    let compliance: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE is_compliance_relevant = 1")
            .fetch_one(&*state.db)
            .await
            .unwrap_or((0,));

    // By category
    let by_category: Vec<CategoryCount> = sqlx::query_as(
        "SELECT action_category as category, COUNT(*) as count FROM audit_logs 
         GROUP BY action_category ORDER BY count DESC LIMIT 10",
    )
    .fetch_all(&*state.db)
    .await
    .unwrap_or_default();

    // By action
    let by_action: Vec<ActionCount> = sqlx::query_as(
        "SELECT action, COUNT(*) as count FROM audit_logs 
         GROUP BY action ORDER BY count DESC LIMIT 10",
    )
    .fetch_all(&*state.db)
    .await
    .unwrap_or_default();

    // Top users
    let top_users: Vec<UserAuditCount> = sqlx::query_as(
        "SELECT user_id, username, COUNT(*) as count FROM audit_logs 
         GROUP BY user_id ORDER BY count DESC LIMIT 10",
    )
    .fetch_all(&*state.db)
    .await
    .unwrap_or_default();

    Ok(Json(AuditStats {
        total_logs: total.0,
        logs_today: today_count.0,
        logs_this_week: week_count.0,
        logs_this_month: month_count.0,
        critical_events: critical.0,
        error_events: error.0,
        warning_events: warning.0,
        compliance_events: compliance.0,
        by_category,
        by_action,
        top_users,
    }))
}

// ============================================================================
// Retention Policy Handlers
// ============================================================================

async fn list_retention_policies(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<RetentionPolicy>>, StatusCode> {
    let policies: Vec<RetentionPolicy> =
        sqlx::query_as("SELECT * FROM data_retention_policies ORDER BY name")
            .fetch_all(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(policies))
}

async fn get_retention_policy(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<RetentionPolicy>, StatusCode> {
    let policy: RetentionPolicy =
        sqlx::query_as("SELECT * FROM data_retention_policies WHERE id = ?")
            .bind(&id)
            .fetch_optional(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(policy))
}

async fn create_retention_policy(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateRetentionPolicyRequest>,
) -> Result<Json<RetentionPolicy>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO data_retention_policies 
         (id, name, description, resource_type, retention_days, auto_delete, 
          archive_before_delete, notify_before_delete, notify_days_before, created_by, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.resource_type)
    .bind(req.retention_days)
    .bind(req.auto_delete.unwrap_or(false))
    .bind(req.archive_before_delete.unwrap_or(true))
    .bind(req.notify_before_delete.unwrap_or(true))
    .bind(req.notify_days_before.unwrap_or(7))
    .bind(&user.user_id)
    .bind(&now)
    .execute(&*state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let policy: RetentionPolicy =
        sqlx::query_as("SELECT * FROM data_retention_policies WHERE id = ?")
            .bind(&id)
            .fetch_one(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(policy))
}

async fn update_retention_policy(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
    Json(req): Json<UpdateRetentionPolicyRequest>,
) -> Result<Json<RetentionPolicy>, StatusCode> {
    let now = Utc::now().to_rfc3339();

    // Check exists
    let _: RetentionPolicy = sqlx::query_as("SELECT * FROM data_retention_policies WHERE id = ?")
        .bind(&id)
        .fetch_optional(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Build dynamic update
    let mut updates = vec!["updated_at = ?".to_string()];
    if req.name.is_some() {
        updates.push("name = ?".to_string());
    }
    if req.description.is_some() {
        updates.push("description = ?".to_string());
    }
    if req.retention_days.is_some() {
        updates.push("retention_days = ?".to_string());
    }
    if req.auto_delete.is_some() {
        updates.push("auto_delete = ?".to_string());
    }
    if req.archive_before_delete.is_some() {
        updates.push("archive_before_delete = ?".to_string());
    }
    if req.notify_before_delete.is_some() {
        updates.push("notify_before_delete = ?".to_string());
    }
    if req.notify_days_before.is_some() {
        updates.push("notify_days_before = ?".to_string());
    }
    if req.is_active.is_some() {
        updates.push("is_active = ?".to_string());
    }

    let sql = format!(
        "UPDATE data_retention_policies SET {} WHERE id = ?",
        updates.join(", ")
    );

    let mut q = sqlx::query(&sql).bind(&now);

    if let Some(ref name) = req.name {
        q = q.bind(name);
    }
    if let Some(ref desc) = req.description {
        q = q.bind(desc);
    }
    if let Some(days) = req.retention_days {
        q = q.bind(days);
    }
    if let Some(auto) = req.auto_delete {
        q = q.bind(auto);
    }
    if let Some(archive) = req.archive_before_delete {
        q = q.bind(archive);
    }
    if let Some(notify) = req.notify_before_delete {
        q = q.bind(notify);
    }
    if let Some(days) = req.notify_days_before {
        q = q.bind(days);
    }
    if let Some(active) = req.is_active {
        q = q.bind(active);
    }

    q.bind(&id)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let policy: RetentionPolicy =
        sqlx::query_as("SELECT * FROM data_retention_policies WHERE id = ?")
            .bind(&id)
            .fetch_one(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(policy))
}

async fn delete_retention_policy(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM data_retention_policies WHERE id = ?")
        .bind(&id)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn apply_retention_policy(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<RetentionResult>, StatusCode> {
    let policy: RetentionPolicy =
        sqlx::query_as("SELECT * FROM data_retention_policies WHERE id = ?")
            .bind(&id)
            .fetch_optional(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

    let cutoff = (Utc::now() - Duration::days(policy.retention_days as i64)).to_rfc3339();
    let deleted = match policy.resource_type.as_str() {
        "audit_logs" => sqlx::query("DELETE FROM audit_logs WHERE created_at < ?")
            .bind(&cutoff)
            .execute(&*state.db)
            .await
            .map(|r| r.rows_affected())
            .unwrap_or(0),
        "trash" => sqlx::query("DELETE FROM trash WHERE deleted_at < ?")
            .bind(&cutoff)
            .execute(&*state.db)
            .await
            .map(|r| r.rows_affected())
            .unwrap_or(0),
        _ => 0,
    };

    // Update last applied
    sqlx::query("UPDATE data_retention_policies SET last_applied_at = ? WHERE id = ?")
        .bind(Utc::now().to_rfc3339())
        .bind(&id)
        .execute(&*state.db)
        .await
        .ok();

    Ok(Json(RetentionResult {
        policy_id: policy.id,
        policy_name: policy.name,
        records_deleted: deleted,
        records_archived: 0,
    }))
}

async fn apply_all_retention_policies(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<RetentionResult>>, StatusCode> {
    let policies: Vec<RetentionPolicy> = sqlx::query_as(
        "SELECT * FROM data_retention_policies WHERE is_active = 1 AND auto_delete = 1",
    )
    .fetch_all(&*state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut results = Vec::new();

    for policy in policies {
        let cutoff = (Utc::now() - Duration::days(policy.retention_days as i64)).to_rfc3339();
        let deleted = match policy.resource_type.as_str() {
            "audit_logs" => sqlx::query("DELETE FROM audit_logs WHERE created_at < ?")
                .bind(&cutoff)
                .execute(&*state.db)
                .await
                .map(|r| r.rows_affected())
                .unwrap_or(0),
            "trash" => sqlx::query("DELETE FROM trash WHERE deleted_at < ?")
                .bind(&cutoff)
                .execute(&*state.db)
                .await
                .map(|r| r.rows_affected())
                .unwrap_or(0),
            _ => 0,
        };

        sqlx::query("UPDATE data_retention_policies SET last_applied_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(&policy.id)
            .execute(&*state.db)
            .await
            .ok();

        results.push(RetentionResult {
            policy_id: policy.id,
            policy_name: policy.name,
            records_deleted: deleted,
            records_archived: 0,
        });
    }

    Ok(Json(results))
}

// ============================================================================
// Compliance Report Handlers
// ============================================================================

async fn list_compliance_reports(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<ComplianceReport>>, StatusCode> {
    let reports: Vec<ComplianceReport> =
        sqlx::query_as("SELECT * FROM compliance_reports ORDER BY created_at DESC")
            .fetch_all(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(reports))
}

async fn get_compliance_report(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<ComplianceReport>, StatusCode> {
    let report: ComplianceReport = sqlx::query_as("SELECT * FROM compliance_reports WHERE id = ?")
        .bind(&id)
        .fetch_optional(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(report))
}

async fn generate_compliance_report(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<GenerateReportRequest>,
) -> Result<Json<ComplianceReport>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let filters_json = req
        .filters
        .map(|f| serde_json::to_string(&f).unwrap_or_default());

    // Count matching records
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM audit_logs WHERE created_at >= ? AND created_at <= ?")
            .bind(&req.start_date)
            .bind(&req.end_date)
            .fetch_one(&*state.db)
            .await
            .unwrap_or((0,));

    // Create report entry
    sqlx::query(
        "INSERT INTO compliance_reports 
         (id, report_type, report_name, description, status, start_date, end_date, 
          filters, total_records, file_format, generated_by, generated_at, created_at)
         VALUES (?, ?, ?, ?, 'completed', ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&req.report_type)
    .bind(&req.report_name)
    .bind(&req.description)
    .bind(&req.start_date)
    .bind(&req.end_date)
    .bind(&filters_json)
    .bind(count.0 as i32)
    .bind(req.file_format.unwrap_or_else(|| "json".to_string()))
    .bind(&user.user_id)
    .bind(&now)
    .bind(&now)
    .execute(&*state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let report: ComplianceReport = sqlx::query_as("SELECT * FROM compliance_reports WHERE id = ?")
        .bind(&id)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(report))
}

async fn delete_compliance_report(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM compliance_reports WHERE id = ?")
        .bind(&id)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn download_compliance_report(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<Vec<AuditLogEntry>>, StatusCode> {
    let report: ComplianceReport = sqlx::query_as("SELECT * FROM compliance_reports WHERE id = ?")
        .bind(&id)
        .fetch_optional(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Fetch logs for report date range
    let logs: Vec<AuditLogEntry> = sqlx::query_as(
        "SELECT * FROM audit_logs WHERE created_at >= ? AND created_at <= ? ORDER BY created_at",
    )
    .bind(&report.start_date)
    .bind(&report.end_date)
    .fetch_all(&*state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update download count
    sqlx::query("UPDATE compliance_reports SET download_count = download_count + 1 WHERE id = ?")
        .bind(&id)
        .execute(&*state.db)
        .await
        .ok();

    Ok(Json(logs))
}

// ============================================================================
// Template Handlers
// ============================================================================

async fn list_compliance_templates(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<ComplianceTemplate>>, StatusCode> {
    let templates: Vec<ComplianceTemplate> =
        sqlx::query_as("SELECT * FROM compliance_templates ORDER BY name")
            .fetch_all(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(templates))
}

async fn get_compliance_template(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<Json<ComplianceTemplate>, StatusCode> {
    let template: ComplianceTemplate =
        sqlx::query_as("SELECT * FROM compliance_templates WHERE id = ?")
            .bind(&id)
            .fetch_optional(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(template))
}

// ============================================================================
// Archive Handlers
// ============================================================================

async fn list_audit_archives(
    State(state): State<AppState>,
    _user: UserInfo,
) -> Result<Json<Vec<AuditArchive>>, StatusCode> {
    let archives: Vec<AuditArchive> =
        sqlx::query_as("SELECT * FROM audit_log_archives ORDER BY created_at DESC")
            .fetch_all(&*state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(archives))
}

async fn create_audit_archive(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<AuditArchive>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let archive_name = format!("audit_archive_{}", now.format("%Y%m%d_%H%M%S"));

    // Get date range of logs
    let dates: (Option<String>, Option<String>) =
        sqlx::query_as("SELECT MIN(created_at), MAX(created_at) FROM audit_logs")
            .fetch_one(&*state.db)
            .await
            .unwrap_or((None, None));

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_logs")
        .fetch_one(&*state.db)
        .await
        .unwrap_or((0,));

    let file_path = format!("./data/archives/{}.json.gz", archive_name);

    sqlx::query(
        "INSERT INTO audit_log_archives 
         (id, archive_name, start_date, end_date, record_count, file_path, 
          compression_type, created_by, created_at)
         VALUES (?, ?, ?, ?, ?, ?, 'gzip', ?, ?)",
    )
    .bind(&id)
    .bind(&archive_name)
    .bind(dates.0.unwrap_or_else(|| now.to_rfc3339()))
    .bind(dates.1.unwrap_or_else(|| now.to_rfc3339()))
    .bind(count.0 as i32)
    .bind(&file_path)
    .bind(&user.user_id)
    .bind(now.to_rfc3339())
    .execute(&*state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let archive: AuditArchive = sqlx::query_as("SELECT * FROM audit_log_archives WHERE id = ?")
        .bind(&id)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(archive))
}

async fn delete_audit_archive(
    State(state): State<AppState>,
    _user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM audit_log_archives WHERE id = ?")
        .bind(&id)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
