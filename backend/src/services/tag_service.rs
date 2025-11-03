//! Tag Service - Business logic for tag management

use crate::{AppState, auth::UserInfo};
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub owner_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileTag {
    pub id: String,
    pub file_id: String,
    pub tag_id: String,
    pub item_type: String,
    pub file_path: String,
    pub tagged_by: String,
    pub created_at: String,
}

/// List all tags for a user
pub async fn list(state: &AppState, user: &UserInfo) -> Result<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>(
        r#"
        SELECT id, name, color, owner_id, created_at, updated_at
        FROM tags
        WHERE owner_id = ?
        ORDER BY name ASC
        "#,
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await?;
    
    Ok(tags)
}

/// Create a new tag
pub async fn create(state: &AppState, user: &UserInfo, name: &str, color: Option<String>) -> Result<Tag> {
    // Validate color format if provided (must be hex color)
    if let Some(ref c) = color {
        if !c.starts_with('#') || (c.len() != 7 && c.len() != 9) {
            bail!("Invalid color format. Use #RRGGBB or #RRGGBBAA");
        }
    }
    
    let tag_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO tags (id, name, color, owner_id, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&tag_id)
    .bind(name)
    .bind(&color)
    .bind(&user.id)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;
    
    Ok(Tag {
        id: tag_id,
        name: name.to_string(),
        color,
        owner_id: user.id.clone(),
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Delete a tag (only owner can delete)
pub async fn delete(state: &AppState, user: &UserInfo, tag_id: &str) -> Result<()> {
    let result = sqlx::query(
        r#"
        DELETE FROM tags
        WHERE id = ? AND owner_id = ?
        "#,
    )
    .bind(tag_id)
    .bind(&user.id)
    .execute(&state.db)
    .await?;
    
    if result.rows_affected() == 0 {
        bail!("Tag not found or access denied");
    }
    
    Ok(())
}

/// Update tag color
pub async fn update_color(state: &AppState, user: &UserInfo, tag_id: &str, color: String) -> Result<()> {
    // Validate color format
    if !color.starts_with('#') || (color.len() != 7 && color.len() != 9) {
        bail!("Invalid color format. Use #RRGGBB or #RRGGBBAA");
    }
    
    let now = Utc::now().to_rfc3339();
    
    let result = sqlx::query(
        r#"
        UPDATE tags
        SET color = ?, updated_at = ?
        WHERE id = ? AND owner_id = ?
        "#,
    )
    .bind(&color)
    .bind(&now)
    .bind(tag_id)
    .bind(&user.id)
    .execute(&state.db)
    .await?;
    
    if result.rows_affected() == 0 {
        bail!("Tag not found or access denied");
    }
    
    Ok(())
}

/// Tag a file (assign tag to file)
pub async fn tag_file(state: &AppState, user: &UserInfo, file_id: &str, tag_id: &str) -> Result<FileTag> {
    // Verify tag belongs to user
    let tag = sqlx::query_as::<_, Tag>(
        "SELECT id, name, color, owner_id, created_at, updated_at FROM tags WHERE id = ? AND owner_id = ?"
    )
    .bind(tag_id)
    .bind(&user.id)
    .fetch_optional(&state.db)
    .await?;
    
    if tag.is_none() {
        bail!("Tag not found or access denied");
    }
    
    // Get file path (assuming file_id is the path for now)
    let file_path = file_id;
    
    let file_tag_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"
        INSERT INTO file_tags (id, file_id, tag_id, item_type, file_path, tagged_by, created_at)
        VALUES (?, ?, ?, 'file', ?, ?, ?)
        "#,
    )
    .bind(&file_tag_id)
    .bind(file_id)
    .bind(tag_id)
    .bind(file_path)
    .bind(&user.id)
    .bind(&now)
    .execute(&state.db)
    .await?;
    
    Ok(FileTag {
        id: file_tag_id,
        file_id: file_id.to_string(),
        tag_id: tag_id.to_string(),
        item_type: "file".to_string(),
        file_path: file_path.to_string(),
        tagged_by: user.id.clone(),
        created_at: now,
    })
}

/// Remove tag from file
pub async fn untag_file(state: &AppState, user: &UserInfo, file_id: &str, tag_id: &str) -> Result<()> {
    let result = sqlx::query(
        r#"
        DELETE FROM file_tags
        WHERE file_id = ? AND tag_id = ? AND tagged_by = ?
        "#,
    )
    .bind(file_id)
    .bind(tag_id)
    .bind(&user.id)
    .execute(&state.db)
    .await?;
    
    if result.rows_affected() == 0 {
        bail!("File tag not found or access denied");
    }
    
    Ok(())
}

/// List tags for a specific file
pub async fn list_file_tags(state: &AppState, file_path: &str) -> Result<Vec<(Tag, FileTag)>> {
    let rows = sqlx::query(
        r#"
        SELECT 
            t.id as tag_id, t.name, t.color, t.owner_id, t.created_at as tag_created, t.updated_at,
            ft.id as file_tag_id, ft.file_id, ft.tag_id as ft_tag_id, ft.item_type, 
            ft.file_path, ft.tagged_by, ft.created_at as ft_created
        FROM file_tags ft
        JOIN tags t ON ft.tag_id = t.id
        WHERE ft.file_path = ?
        ORDER BY t.name ASC
        "#,
    )
    .bind(file_path)
    .fetch_all(&state.db)
    .await?;
    
    let mut results = Vec::new();
    for row in rows {
        let tag = Tag {
            id: row.get("tag_id"),
            name: row.get("name"),
            color: row.get("color"),
            owner_id: row.get("owner_id"),
            created_at: row.get("tag_created"),
            updated_at: row.get("updated_at"),
        };
        let file_tag = FileTag {
            id: row.get("file_tag_id"),
            file_id: row.get("file_id"),
            tag_id: row.get("ft_tag_id"),
            item_type: row.get("item_type"),
            file_path: row.get("file_path"),
            tagged_by: row.get("tagged_by"),
            created_at: row.get("ft_created"),
        };
        results.push((tag, file_tag));
    }
    
    Ok(results)
}
