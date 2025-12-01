use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::UserInfo,
    services::smart_folders_service::{RuleCondition, SmartFolderRule},
    AppState,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSmartFolderRequest {
    pub name: String,
    pub description: Option<String>,
    pub conditions: Vec<RuleCondition>,
    pub logic: String, // "AND" or "OR"
    pub icon: Option<String>,
    pub color: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSmartFolderRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub conditions: Option<Vec<RuleCondition>>,
    pub logic: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmartFolderResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub sort_by: String,
    pub sort_order: String,
    pub conditions_count: usize,
    pub logic: String,
    pub is_active: bool,
}

// GET /api/smart-folders - List all smart folders for user
pub async fn list_smart_folders(
    State(state): State<AppState>,
    user: UserInfo,
) -> Result<Json<Vec<SmartFolderResponse>>, StatusCode> {
    let query = "SELECT id, name, description, icon, color, sort_by, sort_order, conditions, logic, is_active FROM smart_folder_rules WHERE user_id = ?";

    let rules: Vec<SmartFolderRule> = sqlx::query_as(query)
        .bind(&user.id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = rules
        .into_iter()
        .map(|rule| {
            let conditions_count = serde_json::from_str::<Vec<RuleCondition>>(&rule.conditions)
                .map(|c| c.len())
                .unwrap_or(0);

            SmartFolderResponse {
                id: rule.id,
                name: rule.name,
                description: rule.description,
                icon: rule.icon,
                color: rule.color,
                sort_by: rule.sort_by,
                sort_order: rule.sort_order,
                conditions_count,
                logic: rule.logic,
                is_active: rule.is_active,
            }
        })
        .collect();

    Ok(Json(response))
}

// POST /api/smart-folders - Create new smart folder
pub async fn create_smart_folder(
    State(state): State<AppState>,
    user: UserInfo,
    Json(req): Json<CreateSmartFolderRequest>,
) -> Result<(StatusCode, Json<SmartFolderResponse>), StatusCode> {
    let id = Uuid::new_v4().to_string();
    let conditions_json =
        serde_json::to_string(&req.conditions).map_err(|_| StatusCode::BAD_REQUEST)?;

    let query = "INSERT INTO smart_folder_rules (id, user_id, name, description, conditions, logic, icon, color, sort_by, sort_order, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

    sqlx::query(query)
        .bind(&id)
        .bind(&user.id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&conditions_json)
        .bind(&req.logic)
        .bind(&req.icon)
        .bind(&req.color)
        .bind(req.sort_by.unwrap_or_else(|| "name".to_string()))
        .bind(req.sort_order.unwrap_or_else(|| "asc".to_string()))
        .bind(true)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(SmartFolderResponse {
            id,
            name: req.name,
            description: req.description,
            icon: req.icon,
            color: req.color,
            sort_by: "name".to_string(),
            sort_order: "asc".to_string(),
            conditions_count: req.conditions.len(),
            logic: req.logic,
            is_active: true,
        }),
    ))
}

// PUT /api/smart-folders/:id - Update smart folder
pub async fn update_smart_folder(
    State(state): State<AppState>,
    user: UserInfo,
    Path(id): Path<String>,
    Json(req): Json<UpdateSmartFolderRequest>,
) -> Result<Json<SmartFolderResponse>, StatusCode> {
    // Verify ownership
    let existing: SmartFolderRule = sqlx::query_as(
        "SELECT id, user_id, name, description, conditions, logic, icon, color, sort_by, sort_order, is_active, created_at, updated_at FROM smart_folder_rules WHERE id = ? AND user_id = ?"
    )
    .bind(&id)
    .bind(&user.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let name = req.name.unwrap_or(existing.name.clone());
    let description = req.description.or(existing.description.clone());
    let conditions_json = if let Some(conds) = req.conditions {
        serde_json::to_string(&conds).map_err(|_| StatusCode::BAD_REQUEST)?
    } else {
        existing.conditions.clone()
    };
    let logic = req.logic.unwrap_or(existing.logic.clone());
    let icon = req.icon.or(existing.icon.clone());
    let color = req.color.or(existing.color.clone());
    let sort_by = req.sort_by.unwrap_or(existing.sort_by.clone());
    let sort_order = req.sort_order.unwrap_or(existing.sort_order.clone());
    let is_active = req.is_active.unwrap_or(existing.is_active);

    sqlx::query("UPDATE smart_folder_rules SET name = ?, description = ?, conditions = ?, logic = ?, icon = ?, color = ?, sort_by = ?, sort_order = ?, is_active = ?, updated_at = ? WHERE id = ?")
        .bind(&name)
        .bind(&description)
        .bind(&conditions_json)
        .bind(&logic)
        .bind(&icon)
        .bind(&color)
        .bind(&sort_by)
        .bind(&sort_order)
        .bind(is_active)
        .bind(Utc::now())
        .bind(&id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let conditions_count = serde_json::from_str::<Vec<RuleCondition>>(&conditions_json)
        .map(|c| c.len())
        .unwrap_or(0);

    Ok(Json(SmartFolderResponse {
        id,
        name,
        description,
        icon,
        color,
        sort_by,
        sort_order,
        conditions_count,
        logic,
        is_active,
    }))
}

// DELETE /api/smart-folders/:id - Delete smart folder
pub async fn delete_smart_folder(
    State(state): State<AppState>,
    user: UserInfo,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM smart_folder_rules WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user.id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

// POST /api/smart-folders/:id/preview - Preview files matching the rule
#[derive(Serialize)]
pub struct PreviewResponse {
    pub total_count: usize,
    pub matching_files: Vec<String>,
}

pub async fn preview_smart_folder(
    State(_state): State<AppState>,
    _user: UserInfo,
    Path(_id): Path<String>,
) -> Result<Json<PreviewResponse>, StatusCode> {
    // For now, return a placeholder
    // In production, this would scan files and return matches
    Ok(Json(PreviewResponse {
        total_count: 0,
        matching_files: vec![],
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/smart-folders",
            get(list_smart_folders).post(create_smart_folder),
        )
        .route(
            "/smart-folders/:id",
            put(update_smart_folder).delete(delete_smart_folder),
        )
        .route("/smart-folders/:id/preview", post(preview_smart_folder))
}
