// Workflow Automation API
// Provides endpoints for creating, managing, and executing workflow rules

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::auth::UserInfo;
use crate::AppState;

// ============ Models ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowRule {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub trigger_type: String,
    pub trigger_config: String,   // JSON
    pub condition_config: String, // JSON
    pub action_type: String,
    pub action_config: String, // JSON
    pub is_active: bool,
    pub priority: i32,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowExecution {
    pub id: String,
    pub rule_id: String,
    pub triggered_by: Option<String>,
    pub trigger_event: String, // JSON
    pub condition_met: bool,
    pub condition_result: Option<String>, // JSON
    pub action_executed: bool,
    pub action_result: Option<String>, // JSON
    pub status: String,
    pub error_message: Option<String>,
    pub execution_time_ms: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowRuleWithStats {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub trigger_type: String,
    pub trigger_config: String,
    pub condition_config: String,
    pub action_type: String,
    pub action_config: String,
    pub is_active: bool,
    pub priority: i32,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub execution_count: i64,
    pub success_count: i64,
    pub failed_count: i64,
    pub last_execution: Option<String>,
}

// ============ Request/Response DTOs ============

#[derive(Debug, Deserialize)]
pub struct CreateWorkflowRuleRequest {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub trigger_type: String,
    pub trigger_config: serde_json::Value,
    pub condition_config: Option<serde_json::Value>,
    pub action_type: String,
    pub action_config: serde_json::Value,
    pub is_active: Option<bool>,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkflowRuleRequest {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub trigger_config: Option<serde_json::Value>,
    pub condition_config: Option<serde_json::Value>,
    pub action_config: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListWorkflowsQuery {
    pub trigger_type: Option<String>,
    pub action_type: Option<String>,
    pub is_active: Option<bool>,
    pub include_stats: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionHistoryQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ManualExecuteRequest {
    pub file_path: String,
    pub trigger_context: Option<serde_json::Value>,
}

// ============ Router Setup ============

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/workflows", post(create_workflow_rule))
        .route("/workflows", get(list_workflow_rules))
        .route("/workflows/:id", get(get_workflow_rule))
        .route("/workflows/:id", put(update_workflow_rule))
        .route("/workflows/:id", delete(delete_workflow_rule))
        .route("/workflows/:id/toggle", post(toggle_workflow_rule))
        .route("/workflows/:id/execute", post(execute_workflow_manually))
        .route("/workflows/:id/executions", get(get_execution_history))
        .route("/workflows/executions/recent", get(get_recent_executions))
        .route("/workflows/trigger-types", get(list_trigger_types))
        .route("/workflows/action-types", get(list_action_types))
}

// ============ API Handlers ============

/// Create a new workflow rule
async fn create_workflow_rule(
    State(state): State<AppState>,
    UserInfo { id, .. }: UserInfo,
    Json(req): Json<CreateWorkflowRuleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate trigger and action types
    if !is_valid_trigger_type(&req.trigger_type) {
        return Err(StatusCode::BAD_REQUEST);
    }
    if !is_valid_action_type(&req.action_type) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Serialize JSON configs
    let trigger_config =
        serde_json::to_string(&req.trigger_config).map_err(|_| StatusCode::BAD_REQUEST)?;
    let condition_config = serde_json::to_string(&req.condition_config.unwrap_or(json!({})))
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let action_config =
        serde_json::to_string(&req.action_config).map_err(|_| StatusCode::BAD_REQUEST)?;

    let is_active = req.is_active.unwrap_or(true);
    let priority = req.priority.unwrap_or(100);

    let rule = sqlx::query_as::<_, WorkflowRule>(
        r#"
        INSERT INTO workflow_rules (
            name, display_name, description, trigger_type, trigger_config,
            condition_config, action_type, action_config, is_active, priority,
            created_by
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&req.name)
    .bind(&req.display_name)
    .bind(&req.description)
    .bind(&req.trigger_type)
    .bind(&trigger_config)
    .bind(&condition_config)
    .bind(&req.action_type)
    .bind(&action_config)
    .bind(is_active)
    .bind(priority)
    .bind(&id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rule))
}

/// List workflow rules with optional filters
async fn list_workflow_rules(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Query(query): Query<ListWorkflowsQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let include_stats = query.include_stats.unwrap_or(false);

    if include_stats {
        // Return rules with execution statistics
        let mut sql = String::from(
            r#"
            SELECT 
                wr.*,
                COUNT(DISTINCT we.id) as execution_count,
                COUNT(DISTINCT CASE WHEN we.status = 'success' THEN we.id END) as success_count,
                COUNT(DISTINCT CASE WHEN we.status = 'failed' THEN we.id END) as failed_count,
                MAX(we.created_at) as last_execution
            FROM workflow_rules wr
            LEFT JOIN workflow_executions we ON wr.id = we.rule_id
            WHERE 1=1
            "#,
        );

        let mut params: Vec<String> = vec![];

        if let Some(trigger_type) = &query.trigger_type {
            sql.push_str(" AND wr.trigger_type = ?");
            params.push(trigger_type.clone());
        }
        if let Some(action_type) = &query.action_type {
            sql.push_str(" AND wr.action_type = ?");
            params.push(action_type.clone());
        }
        if let Some(is_active) = query.is_active {
            sql.push_str(" AND wr.is_active = ?");
            params.push(if is_active { "1" } else { "0" }.to_string());
        }

        sql.push_str(" GROUP BY wr.id ORDER BY wr.priority DESC, wr.created_at DESC");

        let mut query_builder = sqlx::query_as::<_, WorkflowRuleWithStats>(&sql);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rules = query_builder
            .fetch_all(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(serde_json::to_value(rules).unwrap()))
    } else {
        // Return basic rules without stats
        let mut sql = String::from("SELECT * FROM workflow_rules WHERE 1=1");

        let mut params: Vec<String> = vec![];

        if let Some(trigger_type) = &query.trigger_type {
            sql.push_str(" AND trigger_type = ?");
            params.push(trigger_type.clone());
        }
        if let Some(action_type) = &query.action_type {
            sql.push_str(" AND action_type = ?");
            params.push(action_type.clone());
        }
        if let Some(is_active) = query.is_active {
            sql.push_str(" AND is_active = ?");
            params.push(if is_active { "1" } else { "0" }.to_string());
        }

        sql.push_str(" ORDER BY priority DESC, created_at DESC");

        let mut query_builder = sqlx::query_as::<_, WorkflowRule>(&sql);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rules = query_builder
            .fetch_all(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(serde_json::to_value(rules).unwrap()))
    }
}

/// Get a specific workflow rule by ID
async fn get_workflow_rule(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let rule = sqlx::query_as::<_, WorkflowRule>("SELECT * FROM workflow_rules WHERE id = ?")
        .bind(&id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(rule))
}

/// Update an existing workflow rule
async fn update_workflow_rule(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Path(id): Path<String>,
    Json(req): Json<UpdateWorkflowRuleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if rule exists
    let existing = sqlx::query_as::<_, WorkflowRule>("SELECT * FROM workflow_rules WHERE id = ?")
        .bind(&id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Build update query dynamically
    let mut updates = vec![];
    let mut params: Vec<String> = vec![];

    if let Some(display_name) = &req.display_name {
        updates.push("display_name = ?");
        params.push(display_name.clone());
    }
    if let Some(description) = &req.description {
        updates.push("description = ?");
        params.push(description.clone());
    }
    if let Some(trigger_config) = &req.trigger_config {
        let json_str =
            serde_json::to_string(trigger_config).map_err(|_| StatusCode::BAD_REQUEST)?;
        updates.push("trigger_config = ?");
        params.push(json_str);
    }
    if let Some(condition_config) = &req.condition_config {
        let json_str =
            serde_json::to_string(condition_config).map_err(|_| StatusCode::BAD_REQUEST)?;
        updates.push("condition_config = ?");
        params.push(json_str);
    }
    if let Some(action_config) = &req.action_config {
        let json_str = serde_json::to_string(action_config).map_err(|_| StatusCode::BAD_REQUEST)?;
        updates.push("action_config = ?");
        params.push(json_str);
    }
    if let Some(is_active) = req.is_active {
        updates.push("is_active = ?");
        params.push(if is_active { "1" } else { "0" }.to_string());
    }
    if let Some(priority) = req.priority {
        updates.push("priority = ?");
        params.push(priority.to_string());
    }

    if updates.is_empty() {
        return Ok(Json(existing));
    }

    updates.push("updated_at = CURRENT_TIMESTAMP");

    let sql = format!(
        "UPDATE workflow_rules SET {} WHERE id = ? RETURNING *",
        updates.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, WorkflowRule>(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(&id);

    let updated_rule = query_builder
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_rule))
}

/// Delete a workflow rule
async fn delete_workflow_rule(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query("DELETE FROM workflow_rules WHERE id = ?")
        .bind(&id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(
        json!({"success": true, "message": "Workflow rule deleted"}),
    ))
}

/// Toggle workflow rule active status
async fn toggle_workflow_rule(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let rule = sqlx::query_as::<_, WorkflowRule>(
        "UPDATE workflow_rules SET is_active = NOT is_active, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING *"
    )
    .bind(&id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(rule))
}

/// Manually execute a workflow rule for a specific file
async fn execute_workflow_manually(
    State(state): State<AppState>,
    UserInfo { id, .. }: UserInfo,
    Path(rule_id): Path<String>,
    Json(req): Json<ManualExecuteRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get the workflow rule
    let rule = sqlx::query_as::<_, WorkflowRule>("SELECT * FROM workflow_rules WHERE id = ?")
        .bind(&rule_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Create trigger event
    let trigger_event = json!({
        "event_type": "manual_execution",
        "file_path": req.file_path,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "context": req.trigger_context.unwrap_or(json!({}))
    });

    // For now, just log the execution (actual workflow engine would evaluate and execute)
    let execution = sqlx::query_as::<_, WorkflowExecution>(
        r#"
        INSERT INTO workflow_executions (
            rule_id, triggered_by, trigger_event, condition_met,
            action_executed, status
        )
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&rule.id)
    .bind(&id)
    .bind(serde_json::to_string(&trigger_event).unwrap())
    .bind(true) // Assume condition met for manual execution
    .bind(false) // Action not yet executed (would be done by workflow engine)
    .bind("success")
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": true,
        "execution": execution,
        "message": "Workflow execution started"
    })))
}

/// Get execution history for a specific workflow rule
async fn get_execution_history(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Path(id): Path<String>,
    Query(query): Query<ExecutionHistoryQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let limit = query.limit.unwrap_or(50).min(200);
    let offset = query.offset.unwrap_or(0);

    let mut sql = String::from("SELECT * FROM workflow_executions WHERE rule_id = ?");
    let mut params: Vec<String> = vec![id.clone()];

    if let Some(status) = &query.status {
        sql.push_str(" AND status = ?");
        params.push(status.clone());
    }

    sql.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    params.push(limit.to_string());
    params.push(offset.to_string());

    let mut query_builder = sqlx::query_as::<_, WorkflowExecution>(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    let executions = query_builder
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(executions))
}

/// Get recent workflow executions across all rules
async fn get_recent_executions(
    State(state): State<AppState>,
    UserInfo { .. }: UserInfo,
    Query(query): Query<ExecutionHistoryQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let limit = query.limit.unwrap_or(50).min(200);
    let offset = query.offset.unwrap_or(0);

    let mut sql = String::from("SELECT * FROM workflow_executions WHERE 1=1");
    let mut params: Vec<String> = vec![];

    if let Some(status) = &query.status {
        sql.push_str(" AND status = ?");
        params.push(status.clone());
    }

    sql.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    params.push(limit.to_string());
    params.push(offset.to_string());

    let mut query_builder = sqlx::query_as::<_, WorkflowExecution>(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    let executions = query_builder
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(executions))
}

/// List available trigger types
async fn list_trigger_types(UserInfo { .. }: UserInfo) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(json!({
        "trigger_types": [
            {"value": "file_upload", "label": "File Upload", "description": "Triggered when a file is uploaded"},
            {"value": "file_delete", "label": "File Delete", "description": "Triggered when a file is deleted"},
            {"value": "file_move", "label": "File Move", "description": "Triggered when a file is moved"},
            {"value": "file_rename", "label": "File Rename", "description": "Triggered when a file is renamed"},
            {"value": "file_share", "label": "File Share", "description": "Triggered when a file is shared"},
            {"value": "file_tag", "label": "File Tag", "description": "Triggered when a file is tagged"},
            {"value": "file_version", "label": "File Version", "description": "Triggered when a new file version is created"},
            {"value": "scheduled", "label": "Scheduled", "description": "Triggered on a schedule (cron expression)"},
            {"value": "webhook", "label": "Webhook", "description": "Triggered by external webhook"},
            {"value": "manual", "label": "Manual", "description": "Triggered manually by user"}
        ]
    })))
}

/// List available action types
async fn list_action_types(UserInfo { .. }: UserInfo) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(json!({
        "action_types": [
            {"value": "convert_file", "label": "Convert File", "description": "Convert file to another format"},
            {"value": "compress_file", "label": "Compress File", "description": "Compress/optimize file"},
            {"value": "send_notification", "label": "Send Notification", "description": "Send in-app notification"},
            {"value": "add_tag", "label": "Add Tag", "description": "Add tag to file"},
            {"value": "move_file", "label": "Move File", "description": "Move file to another location"},
            {"value": "copy_file", "label": "Copy File", "description": "Copy file to another location"},
            {"value": "delete_file", "label": "Delete File", "description": "Delete the file"},
            {"value": "send_webhook", "label": "Send Webhook", "description": "Send HTTP webhook to external service"},
            {"value": "send_email", "label": "Send Email", "description": "Send email notification"},
            {"value": "run_script", "label": "Run Script", "description": "Execute custom script"}
        ]
    })))
}

// ============ Helper Functions ============

fn is_valid_trigger_type(trigger: &str) -> bool {
    matches!(
        trigger,
        "file_upload"
            | "file_delete"
            | "file_move"
            | "file_rename"
            | "file_share"
            | "file_tag"
            | "file_version"
            | "scheduled"
            | "webhook"
            | "manual"
    )
}

fn is_valid_action_type(action: &str) -> bool {
    matches!(
        action,
        "convert_file"
            | "compress_file"
            | "send_notification"
            | "add_tag"
            | "move_file"
            | "copy_file"
            | "delete_file"
            | "send_webhook"
            | "send_email"
            | "run_script"
    )
}
