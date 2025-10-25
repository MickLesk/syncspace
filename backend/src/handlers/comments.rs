//! Comments and tags handlers for file annotations

use crate::auth;
use crate::AppState;
use axum::{
    extract::{Path as AxumPath, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: String,
    pub item_type: String,
    pub item_id: String,
    pub file_path: String,
    pub author_id: String,
    pub text: String,
    pub is_resolved: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub item_type: String, // 'file' or 'folder'
    pub item_id: String,
    pub file_path: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub owner_id: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileTagRequest {
    pub item_type: String, // 'file' or 'folder'
    pub file_id: String,
    pub file_path: String,
    pub tag_ids: Vec<String>,
}

/// Create a comment on a file or folder
pub async fn create_comment_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>), StatusCode> {
    let pool = &state.db_pool;
    let comment_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let user_id = user.id.to_string();
    
    sqlx::query(
        "INSERT INTO comments (id, item_type, item_id, file_path, author_id, text, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&comment_id)
    .bind(&req.item_type)
    .bind(&req.item_id)
    .bind(&req.file_path)
    .bind(&user_id)
    .bind(&req.text)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((
        StatusCode::CREATED,
        Json(Comment {
            id: comment_id,
            item_type: req.item_type,
            item_id: req.item_id,
            file_path: req.file_path,
            author_id: user_id,
            text: req.text,
            is_resolved: false,
            created_at: now.clone(),
            updated_at: now,
        }),
    ))
}

/// List comments for a file/folder
pub async fn list_comments_handler(
    State(state): State<AppState>,
    _user: auth::User,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Comment>>, StatusCode> {
    let pool = &state.db_pool;
    let file_path = params.get("file_path").ok_or(StatusCode::BAD_REQUEST)?;
    
    let comments = sqlx::query_as::<_, Comment>(
        "SELECT id, item_type, item_id, file_path, author_id, text, is_resolved, created_at, updated_at
         FROM comments WHERE file_path = ? ORDER BY created_at DESC"
    )
    .bind(file_path)
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(comments))
}

/// Delete a comment
pub async fn delete_comment_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(comment_id): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    // Verify ownership
    sqlx::query("DELETE FROM comments WHERE id = ? AND author_id = ?")
        .bind(&comment_id)
        .bind(&user_id)
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::OK, "deleted"))
}

/// Create a new tag for the user
pub async fn create_tag_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<CreateTagRequest>,
) -> Result<(StatusCode, Json<Tag>), StatusCode> {
    let pool = &state.db_pool;
    let tag_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let user_id = user.id.to_string();
    
    sqlx::query(
        "INSERT INTO tags (id, name, color, owner_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&tag_id)
    .bind(&req.name)
    .bind(&req.color)
    .bind(&user_id)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((
        StatusCode::CREATED,
        Json(Tag {
            id: tag_id,
            name: req.name,
            color: req.color,
            owner_id: user_id,
            created_at: now.clone(),
        }),
    ))
}

/// List all tags for the user
pub async fn list_tags_handler(
    State(state): State<AppState>,
    user: auth::User,
) -> Result<Json<Vec<Tag>>, StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    let tags = sqlx::query_as::<_, Tag>(
        "SELECT id, name, color, owner_id, created_at FROM tags WHERE owner_id = ? ORDER BY name ASC"
    )
    .bind(&user_id)
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(tags))
}

/// Delete a tag
pub async fn delete_tag_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(tag_id): AxumPath<String>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    // Verify ownership and delete
    sqlx::query("DELETE FROM tags WHERE id = ? AND owner_id = ?")
        .bind(&tag_id)
        .bind(&user_id)
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::OK, "deleted"))
}

/// Add tags to a file
pub async fn tag_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    Json(req): Json<FileTagRequest>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    
    // Add each tag
    for tag_id in req.tag_ids {
        let file_tag_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        let _ = sqlx::query(
            "INSERT OR IGNORE INTO file_tags (id, file_id, tag_id, item_type, file_path, tagged_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&file_tag_id)
        .bind(&req.file_id)
        .bind(&tag_id)
        .bind(&req.item_type)
        .bind(&req.file_path)
        .bind(&user_id)
        .bind(&now)
        .execute(pool)
        .await;
    }
    
    Ok((StatusCode::OK, "tagged"))
}

/// Remove tags from a file
pub async fn untag_file_handler(
    State(state): State<AppState>,
    user: auth::User,
    AxumPath(file_id): AxumPath<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<(StatusCode, &'static str), StatusCode> {
    let pool = &state.db_pool;
    let user_id = user.id.to_string();
    let tag_id = params.get("tag_id").ok_or(StatusCode::BAD_REQUEST)?;
    
    sqlx::query(
        "DELETE FROM file_tags WHERE file_id = ? AND tag_id = ? AND tagged_by = ?"
    )
    .bind(&file_id)
    .bind(tag_id)
    .bind(&user_id)
    .execute(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::OK, "untagged"))
}
