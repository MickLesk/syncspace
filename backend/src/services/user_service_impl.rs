//! User service - Full implementation
use crate::{api::users::*, auth::UserInfo, AppState, database::UserPreferences};
use anyhow::{anyhow, Result};
use chrono::Utc;
use uuid::Uuid;

pub async fn get_profile(state: &AppState, user: &UserInfo) -> Result<serde_json::Value> {
    let db_user: Option<crate::database::User> = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&user.id).fetch_optional(&state.db_pool).await?;
    
    match db_user {
        Some(u) => Ok(serde_json::json!({
            "id": u.id, "username": u.username, "email": u.email, "display_name": u.display_name,
            "avatar_base64": u.avatar_base64, "storage_quota_bytes": u.storage_quota_bytes,
            "storage_used_bytes": u.storage_used_bytes, "created_at": u.created_at,
        })),
        None => Err(anyhow!("User not found")),
    }
}

pub async fn update_profile(state: &AppState, user: &UserInfo, req: UpdateProfileRequest) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    if let Some(display_name) = req.display_name {
        sqlx::query("UPDATE users SET display_name = ?, updated_at = ? WHERE id = ?")
            .bind(display_name).bind(&now).bind(&user.id).execute(&state.db_pool).await?;
    }
    if let Some(email) = req.email {
        sqlx::query("UPDATE users SET email = ?, updated_at = ? WHERE id = ?")
            .bind(email).bind(&now).bind(&user.id).execute(&state.db_pool).await?;
    }
    if let Some(avatar_base64) = req.avatar_base64 {
        sqlx::query("UPDATE users SET avatar_base64 = ?, updated_at = ? WHERE id = ?")
            .bind(avatar_base64).bind(&now).bind(&user.id).execute(&state.db_pool).await?;
    }
    Ok(())
}

pub async fn get_settings(state: &AppState, user: &UserInfo) -> Result<serde_json::Value> {
    let db_user: Option<crate::database::User> = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&user.id).fetch_optional(&state.db_pool).await?;
    
    match db_user {
        Some(u) => Ok(serde_json::json!({
            "default_view": u.default_view, "language": u.language, "theme": u.theme,
        })),
        None => Err(anyhow!("User not found")),
    }
}

pub async fn update_settings(state: &AppState, user: &UserInfo, req: UpdateSettingsRequest) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    if let Some(language) = req.language {
        sqlx::query("UPDATE users SET language = ?, updated_at = ? WHERE id = ?")
            .bind(language).bind(&now).bind(&user.id).execute(&state.db_pool).await?;
    }
    if let Some(theme) = req.theme {
        sqlx::query("UPDATE users SET theme = ?, updated_at = ? WHERE id = ?")
            .bind(theme).bind(&now).bind(&user.id).execute(&state.db_pool).await?;
    }
    if let Some(default_view) = req.default_view {
        sqlx::query("UPDATE users SET default_view = ?, updated_at = ? WHERE id = ?")
            .bind(default_view).bind(&now).bind(&user.id).execute(&state.db_pool).await?;
    }
    Ok(())
}

pub async fn get_preferences(state: &AppState, user: &UserInfo) -> Result<serde_json::Value> {
    let prefs: Option<UserPreferences> = sqlx::query_as("SELECT * FROM user_preferences WHERE user_id = ?")
        .bind(&user.id).fetch_optional(&state.db_pool).await?;
    
    match prefs {
        Some(p) => Ok(serde_json::json!({
            "view_mode": p.view_mode, "sort_by": p.sort_by, "sort_order": p.sort_order,
            "sidebar_collapsed": p.sidebar_collapsed, "recent_searches": p.recent_searches,
        })),
        None => Ok(serde_json::json!({"view_mode": "grid", "sort_by": "name", "sort_order": "asc"})),
    }
}

pub async fn update_preferences(state: &AppState, user: &UserInfo, prefs: serde_json::Value) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM user_preferences WHERE user_id = ?")
        .bind(&user.id).fetch_optional(&state.db_pool).await?;
    
    if exists.is_none() {
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO user_preferences (id, user_id, view_mode, sort_by, sort_order, created_at, updated_at) VALUES (?, ?, 'grid', 'name', 'asc', ?, ?)")
            .bind(&id).bind(&user.id).bind(&now).bind(&now).execute(&state.db_pool).await?;
    }
    
    if let Some(view_mode) = prefs.get("view_mode").and_then(|v| v.as_str()) {
        sqlx::query("UPDATE user_preferences SET view_mode = ?, updated_at = ? WHERE user_id = ?")
            .bind(view_mode).bind(&now).bind(&user.id).execute(&state.db_pool).await?;
    }
    if let Some(sort_by) = prefs.get("sort_by").and_then(|v| v.as_str()) {
        sqlx::query("UPDATE user_preferences SET sort_by = ?, updated_at = ? WHERE user_id = ?")
            .bind(sort_by).bind(&now).bind(&user.id).execute(&state.db_pool).await?;
    }
    Ok(())
}
