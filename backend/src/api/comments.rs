//! Comments API Routes

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::AppState;
use crate::auth::UserInfo;

/// Comment model
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub file_path: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ListCommentsQuery {
    pub file_path: String,
}

/// Create a new comment
async fn create_comment(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(req): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let comment_id = Uuid::new_v4();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO comments (id, file_path, user_id, username, content, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(comment_id.to_string())
    .bind(&req.file_path)
    .bind(&user_info.id)
    .bind(&user_info.username)
    .bind(&req.content)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "id": comment_id,
        "message": "Comment created successfully"
    }))))
}

/// List comments for a file
async fn list_comments(
    State(state): State<AppState>,
    Query(query): Query<ListCommentsQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let comments = sqlx::query_as::<_, Comment>(
        r#"
        SELECT id, file_path, user_id, username, content, 
               created_at, updated_at
        FROM comments
        WHERE file_path = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(&query.file_path)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(comments))
}

/// Delete a comment
async fn delete_comment(
    State(state): State<AppState>,
    Path(comment_id): Path<Uuid>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query(
        r#"
        DELETE FROM comments
        WHERE id = ? AND user_id = ?
        "#,
    )
    .bind(comment_id.to_string())
    .bind(&user_info.id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    
    Ok(StatusCode::NO_CONTENT)
}

/// Build comments router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/comments", post(create_comment).get(list_comments))
        .route("/comments/{comment_id}", delete(delete_comment))
}
