//! Activity log and audit trail handlers

use crate::auth;
use crate::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct ActivityStats {
    pub total_actions: i64,
    pub unique_users: i64,
    pub actions_by_type: HashMap<String, i64>,
    pub recent_activity: Vec<ActivityLog>,
}

/// List activity logs
pub async fn list_activity_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<ActivityLog>>, StatusCode> {
    let limit: i64 = params.get("limit")
        .and_then(|l| l.parse().ok())
        .unwrap_or(50);
    
    let logs: Vec<(String, String, String, String, String, Option<String>, Option<String>, Option<String>, Option<String>, String)> = sqlx::query_as(
        "SELECT al.id, al.user_id, u.username, al.action, al.resource_type, al.resource_id, 
                al.details, al.ip_address, al.user_agent, al.timestamp
         FROM activity_log al
         LEFT JOIN users u ON al.user_id = u.id
         ORDER BY al.timestamp DESC
         LIMIT ?"
    )
    .bind(limit)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let activities = logs.into_iter().map(|(id, user_id, username, action, resource_type, resource_id, details, ip, ua, timestamp)| {
        ActivityLog {
            id,
            user_id,
            username,
            action,
            resource_type,
            resource_id,
            details,
            ip_address: ip,
            user_agent: ua,
            timestamp,
        }
    }).collect();
    
    Ok(Json(activities))
}

/// Get activity statistics
pub async fn activity_stats_handler(
    State(state): State<AppState>,
    _user: auth::User,
) -> Result<Json<ActivityStats>, StatusCode> {
    let total_actions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM activity_log")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let unique_users: (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT user_id) FROM activity_log")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let by_type: Vec<(String, i64)> = sqlx::query_as(
        "SELECT action, COUNT(*) as count FROM activity_log GROUP BY action"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut actions_by_type = HashMap::new();
    for (action, count) in by_type {
        actions_by_type.insert(action, count);
    }
    
    let recent: Vec<(String, String, String, String, String, Option<String>, Option<String>, Option<String>, Option<String>, String)> = sqlx::query_as(
        "SELECT al.id, al.user_id, u.username, al.action, al.resource_type, al.resource_id,
                al.details, al.ip_address, al.user_agent, al.timestamp
         FROM activity_log al
         LEFT JOIN users u ON al.user_id = u.id
         ORDER BY al.timestamp DESC
         LIMIT 10"
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let recent_activity = recent.into_iter().map(|(id, user_id, username, action, resource_type, resource_id, details, ip, ua, timestamp)| {
        ActivityLog {
            id,
            user_id,
            username,
            action,
            resource_type,
            resource_id,
            details,
            ip_address: ip,
            user_agent: ua,
            timestamp,
        }
    }).collect();
    
    Ok(Json(ActivityStats {
        total_actions: total_actions.0,
        unique_users: unique_users.0,
        actions_by_type,
        recent_activity,
    }))
}
