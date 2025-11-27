use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/templates", post(create_template).get(list_templates))
        .route(
            "/templates/:id",
            get(get_template)
                .put(update_template)
                .delete(delete_template),
        )
        .route("/templates/:id/use", post(use_template))
        .route("/templates/:id/favorite", post(toggle_favorite))
        .route("/template-categories", get(list_categories))
}

#[derive(Debug, Serialize, FromRow)]
pub struct FileTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub content: String,
    pub file_extension: Option<String>,
    pub variables: Option<String>, // JSON array
    pub is_public: i64,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub content: String,
    pub file_extension: Option<String>,
    pub variables: Option<Vec<String>>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub content: Option<String>,
    pub file_extension: Option<String>,
    pub variables: Option<Vec<String>>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UseTemplateRequest {
    pub filename: String,
    pub destination_path: String,
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct ListTemplatesQuery {
    pub category: Option<String>,
    pub is_public: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TemplateCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TemplateWithStats {
    // Template fields
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub content: String,
    pub file_extension: Option<String>,
    pub variables: Option<String>,
    pub is_public: i64,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    // Stats fields
    pub usage_count: i64,
    pub is_favorite: i64,
}

// Create a new template
async fn create_template(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let template_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let variables_json = req
        .variables
        .map(|v| serde_json::to_string(&v).unwrap_or_default());
    let is_public = req.is_public.unwrap_or(false) as i64;

    sqlx::query(
        "INSERT INTO file_templates (id, name, description, category, content, file_extension, variables, is_public, created_by, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&template_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.category)
    .bind(&req.content)
    .bind(&req.file_extension)
    .bind(&variables_json)
    .bind(is_public)
    .bind(&user_id)
    .bind(&now)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error creating template: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({ "id": template_id })),
    ))
}

// List templates with filters
async fn list_templates(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Query(query): Query<ListTemplatesQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut sql = String::from(
        "SELECT t.*, 
         COALESCE((SELECT COUNT(*) FROM template_usage WHERE template_id = t.id), 0) as usage_count,
         CASE WHEN EXISTS(SELECT 1 FROM user_template_favorites WHERE user_id = ? AND template_id = t.id) THEN 1 ELSE 0 END as is_favorite
         FROM file_templates t WHERE (t.created_by = ? OR t.is_public = 1)"
    );

    let mut conditions = Vec::new();

    if let Some(category) = &query.category {
        conditions.push(format!("t.category = '{}'", category));
    }

    if let Some(is_public) = query.is_public {
        conditions.push(format!("t.is_public = {}", is_public as i64));
    }

    if let Some(search) = &query.search {
        conditions.push(format!(
            "(t.name LIKE '%{}%' OR t.description LIKE '%{}%')",
            search, search
        ));
    }

    if !conditions.is_empty() {
        sql.push_str(" AND ");
        sql.push_str(&conditions.join(" AND "));
    }

    sql.push_str(" ORDER BY t.created_at DESC");

    let templates: Vec<TemplateWithStats> = sqlx::query_as(&sql)
        .bind(&user_id)
        .bind(&user_id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Database error listing templates: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(templates))
}

// Get a specific template
async fn get_template(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(template_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let template: FileTemplate = sqlx::query_as(
        "SELECT * FROM file_templates WHERE id = ? AND (created_by = ? OR is_public = 1)",
    )
    .bind(&template_id)
    .bind(&user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error getting template: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(template))
}

// Update a template (owner only)
async fn update_template(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(template_id): Path<String>,
    Json(req): Json<UpdateTemplateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check ownership
    let template: FileTemplate = sqlx::query_as("SELECT * FROM file_templates WHERE id = ?")
        .bind(&template_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if template.created_by != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now().to_rfc3339();
    let variables_json = req
        .variables
        .map(|v| serde_json::to_string(&v).unwrap_or_default());

    sqlx::query(
        "UPDATE file_templates SET name = COALESCE(?, name), description = COALESCE(?, description),
         category = COALESCE(?, category), content = COALESCE(?, content),
         file_extension = COALESCE(?, file_extension), variables = COALESCE(?, variables),
         is_public = COALESCE(?, is_public), updated_at = ?
         WHERE id = ?"
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.category)
    .bind(&req.content)
    .bind(&req.file_extension)
    .bind(&variables_json)
    .bind(req.is_public.map(|b| b as i64))
    .bind(&now)
    .bind(&template_id)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// Delete a template (owner only)
async fn delete_template(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(template_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let template: FileTemplate = sqlx::query_as("SELECT * FROM file_templates WHERE id = ?")
        .bind(&template_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if template.created_by != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    sqlx::query("DELETE FROM file_templates WHERE id = ?")
        .bind(&template_id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

// Use a template to create a file
async fn use_template(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(template_id): Path<String>,
    Json(req): Json<UseTemplateRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get template
    let template: FileTemplate = sqlx::query_as(
        "SELECT * FROM file_templates WHERE id = ? AND (created_by = ? OR is_public = 1)",
    )
    .bind(&template_id)
    .bind(&user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Perform variable substitution
    let mut content = template.content.clone();
    if let Some(vars) = req.variables {
        for (key, value) in vars.iter() {
            content = content.replace(&format!("{{{{{}}}}}", key), value);
        }
    }

    // Fill in default variables
    content = content.replace("{{user}}", &user_id);
    content = content.replace(
        "{{date}}",
        &chrono::Utc::now().format("%Y-%m-%d").to_string(),
    );
    content = content.replace(
        "{{time}}",
        &chrono::Utc::now().format("%H:%M:%S").to_string(),
    );
    content = content.replace("{{datetime}}", &chrono::Utc::now().to_rfc3339());

    // Create file path
    let file_path = if req.destination_path.ends_with('/') {
        format!("{}{}", req.destination_path, req.filename)
    } else {
        format!("{}/{}", req.destination_path, req.filename)
    };

    // Write file
    let full_path = format!("./data/{}", file_path);
    tokio::fs::write(&full_path, content.as_bytes())
        .await
        .map_err(|e| {
            eprintln!("Error writing file: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Log usage
    let usage_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let _ = sqlx::query(
        "INSERT INTO template_usage (id, template_id, user_id, used_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&usage_id)
    .bind(&template_id)
    .bind(&user_id)
    .bind(&now)
    .execute(&state.db_pool)
    .await;

    Ok(Json(serde_json::json!({ "path": file_path })))
}

// Toggle favorite status
async fn toggle_favorite(
    State(state): State<AppState>,
    UserInfo { id: user_id, .. }: UserInfo,
    Path(template_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if already favorited
    let exists: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM user_template_favorites WHERE user_id = ? AND template_id = ?",
    )
    .bind(&user_id)
    .bind(&template_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if exists.is_some() {
        // Remove favorite
        sqlx::query("DELETE FROM user_template_favorites WHERE user_id = ? AND template_id = ?")
            .bind(&user_id)
            .bind(&template_id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(serde_json::json!({ "is_favorite": false })))
    } else {
        // Add favorite
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO user_template_favorites (user_id, template_id, added_at) VALUES (?, ?, ?)",
        )
        .bind(&user_id)
        .bind(&template_id)
        .bind(&now)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(serde_json::json!({ "is_favorite": true })))
    }
}

// List template categories
async fn list_categories(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let categories: Vec<TemplateCategory> =
        sqlx::query_as("SELECT * FROM template_categories ORDER BY name")
            .fetch_all(&state.db_pool)
            .await
            .map_err(|e| {
                eprintln!("Database error listing categories: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    Ok(Json(categories))
}
