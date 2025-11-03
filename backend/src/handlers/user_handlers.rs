//! User Settings and Preferences Handlers
//!
//! Implements endpoints for:
//! - GET/PUT /api/users/settings - User settings (language, theme, default_view)
//! - GET/PUT /api/users/preferences - User preferences (view_mode, sort_by, sidebar state)

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

use crate::auth::UserInfo;
use crate::AppState;

// ==================== MODELS ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub language: String,
    pub theme: String,
    pub default_view: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub view_mode: String,
    pub sort_by: String,
    pub sort_order: String,
    pub items_per_page: i32,
    pub sidebar_collapsed: bool,
    pub show_hidden_files: bool,
    pub recent_searches: Vec<String>,
    pub notification_email: bool,
    pub notification_browser: bool,
    pub notification_sound: bool,
    pub activity_visible: bool,
    pub show_online_status: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserSettings {
    pub language: Option<String>,
    pub theme: Option<String>,
    pub default_view: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPreferences {
    pub view_mode: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub items_per_page: Option<i32>,
    pub sidebar_collapsed: Option<bool>,
    pub show_hidden_files: Option<bool>,
    pub recent_searches: Option<Vec<String>>,
    pub notification_email: Option<bool>,
    pub notification_browser: Option<bool>,
    pub notification_sound: Option<bool>,
    pub activity_visible: Option<bool>,
    pub show_online_status: Option<bool>,
}

// ==================== SETTINGS HANDLERS ====================

/// GET /api/users/settings - Get user settings
pub async fn get_user_settings(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<impl IntoResponse, StatusCode> {
    let row = sqlx::query(
        r#"
        SELECT language, theme, default_view
        FROM users
        WHERE id = ?
        "#
    )
    .bind(&user_info.id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch user settings: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let settings = UserSettings {
        language: row.try_get::<String, _>("language").unwrap_or_else(|_| "en".to_string()),
        theme: row.try_get::<String, _>("theme").unwrap_or_else(|_| "light".to_string()),
        default_view: row.try_get::<String, _>("default_view").unwrap_or_else(|_| "grid".to_string()),
    };

    Ok(Json(settings))
}

/// PUT /api/users/settings - Update user settings
pub async fn update_user_settings(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(payload): Json<UpdateUserSettings>,
) -> Result<impl IntoResponse, StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(lang) = &payload.language {
        updates.push("language = ?");
        params.push(lang.clone());
    }
    if let Some(theme) = &payload.theme {
        updates.push("theme = ?");
        params.push(theme.clone());
    }
    if let Some(view) = &payload.default_view {
        updates.push("default_view = ?");
        params.push(view.clone());
    }

    if updates.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    updates.push("updated_at = ?");
    params.push(now.clone());
    params.push(user_info.id.clone());

    let query = format!(
        "UPDATE users SET {} WHERE id = ?",
        updates.join(", ")
    );

    let mut query_builder = sqlx::query(&query);
    for param in &params {
        query_builder = query_builder.bind(param);
    }

    query_builder
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to update user settings: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch updated settings
    let row = sqlx::query(
        r#"
        SELECT language, theme, default_view
        FROM users
        WHERE id = ?
        "#
    )
    .bind(&user_info.id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let settings = UserSettings {
        language: row.try_get::<String, _>("language").unwrap_or_else(|_| "en".to_string()),
        theme: row.try_get::<String, _>("theme").unwrap_or_else(|_| "light".to_string()),
        default_view: row.try_get::<String, _>("default_view").unwrap_or_else(|_| "grid".to_string()),
    };

    Ok(Json(settings))
}

// ==================== PREFERENCES HANDLERS ====================

/// GET /api/users/preferences - Get user preferences
pub async fn get_user_preferences(
    State(state): State<AppState>,
    user_info: UserInfo,
) -> Result<Json<UserPreferences>, StatusCode> {
    // GRACEFUL DEGRADATION: If user_preferences table doesn't exist (migration disabled),
    // return default preferences instead of crashing
    let result = sqlx::query(
        r#"
        SELECT 
            view_mode, sort_by, sort_order, items_per_page,
            sidebar_collapsed, show_hidden_files, recent_searches,
            notification_email, notification_browser, notification_sound,
            activity_visible, show_online_status
        FROM user_preferences
        WHERE user_id = ?
        "#
    )
    .bind(&user_info.id)
    .fetch_optional(&state.db_pool)
    .await;

    let preferences = match result {
        Ok(Some(row)) => {
            let recent_searches_json: Option<String> = row.try_get::<String, _>("recent_searches").ok();
            let recent_searches: Vec<String> = recent_searches_json
                .and_then(|json: String| serde_json::from_str(&json).ok())
                .unwrap_or_default();

            UserPreferences {
                view_mode: row.try_get::<String, _>("view_mode").unwrap_or_else(|_| "grid".to_string()),
                sort_by: row.try_get::<String, _>("sort_by").unwrap_or_else(|_| "name".to_string()),
                sort_order: row.try_get::<String, _>("sort_order").unwrap_or_else(|_| "asc".to_string()),
                items_per_page: row.try_get::<i32, _>("items_per_page").unwrap_or(50),
                sidebar_collapsed: row.try_get::<bool, _>("sidebar_collapsed").unwrap_or(false),
                show_hidden_files: row.try_get::<bool, _>("show_hidden_files").unwrap_or(false),
                recent_searches,
                notification_email: row.try_get::<bool, _>("notification_email").unwrap_or(true),
                notification_browser: row.try_get::<bool, _>("notification_browser").unwrap_or(true),
                notification_sound: row.try_get::<bool, _>("notification_sound").unwrap_or(false),
                activity_visible: row.try_get::<bool, _>("activity_visible").unwrap_or(true),
                show_online_status: row.try_get::<bool, _>("show_online_status").unwrap_or(true),
            }
        },
        Ok(None) | Err(_) => {
            // Table doesn't exist or no preferences found - return defaults
            eprintln!("⚠️  user_preferences table not available or no data - using defaults");
            UserPreferences {
                view_mode: "grid".to_string(),
                sort_by: "name".to_string(),
                sort_order: "asc".to_string(),
                items_per_page: 50,
                sidebar_collapsed: false,
                show_hidden_files: false,
                recent_searches: vec![],
                notification_email: true,
                notification_browser: true,
                notification_sound: false,
                activity_visible: true,
                show_online_status: true,
            }
        }
    };

    Ok(Json(preferences))
}

/// PUT /api/users/preferences - Update user preferences
pub async fn update_user_preferences(
    State(state): State<AppState>,
    user_info: UserInfo,
    Json(payload): Json<UpdateUserPreferences>,
) -> Result<impl IntoResponse, StatusCode> {
    // GRACEFUL DEGRADATION: If table doesn't exist, return defaults instead of crashing
    let table_exists = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='user_preferences'")
        .fetch_optional(&state.db_pool)
        .await
        .ok()
        .flatten()
        .is_some();

    if !table_exists {
        eprintln!("⚠️  user_preferences table not available - ignoring update, returning defaults");
        // Return default preferences (updates are ignored but don't crash)
        let preferences = UserPreferences {
            view_mode: payload.view_mode.unwrap_or_else(|| "grid".to_string()),
            sort_by: payload.sort_by.unwrap_or_else(|| "name".to_string()),
            sort_order: payload.sort_order.unwrap_or_else(|| "asc".to_string()),
            items_per_page: payload.items_per_page.unwrap_or(50),
            sidebar_collapsed: payload.sidebar_collapsed.unwrap_or(false),
            show_hidden_files: payload.show_hidden_files.unwrap_or(false),
            recent_searches: payload.recent_searches.unwrap_or_default(),
            notification_email: payload.notification_email.unwrap_or(true),
            notification_browser: payload.notification_browser.unwrap_or(true),
            notification_sound: payload.notification_sound.unwrap_or(false),
            activity_visible: payload.activity_visible.unwrap_or(true),
            show_online_status: payload.show_online_status.unwrap_or(true),
        };
        return Ok(Json(preferences));
    }

    let now = chrono::Utc::now().to_rfc3339();

    // Ensure preferences exist
    let prefs_exist: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM user_preferences WHERE user_id = ?"
    )
    .bind(&user_info.id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((count,)) = prefs_exist {
        if count == 0 {
            create_default_preferences(&state.db_pool, &user_info.id).await?;
        }
    }

    // Build dynamic update
    let updates: Vec<String> = Vec::new();
    
    if let Some(view_mode) = &payload.view_mode {
        sqlx::query("UPDATE user_preferences SET view_mode = ? WHERE user_id = ?")
            .bind(view_mode)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(sort_by) = &payload.sort_by {
        sqlx::query("UPDATE user_preferences SET sort_by = ? WHERE user_id = ?")
            .bind(sort_by)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(sort_order) = &payload.sort_order {
        sqlx::query("UPDATE user_preferences SET sort_order = ? WHERE user_id = ?")
            .bind(sort_order)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(items_per_page) = payload.items_per_page {
        sqlx::query("UPDATE user_preferences SET items_per_page = ? WHERE user_id = ?")
            .bind(items_per_page)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(sidebar_collapsed) = payload.sidebar_collapsed {
        sqlx::query("UPDATE user_preferences SET sidebar_collapsed = ? WHERE user_id = ?")
            .bind(sidebar_collapsed)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(show_hidden_files) = payload.show_hidden_files {
        sqlx::query("UPDATE user_preferences SET show_hidden_files = ? WHERE user_id = ?")
            .bind(show_hidden_files)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(recent_searches) = &payload.recent_searches {
        let json = serde_json::to_string(recent_searches)
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        sqlx::query("UPDATE user_preferences SET recent_searches = ? WHERE user_id = ?")
            .bind(json)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(notification_email) = payload.notification_email {
        sqlx::query("UPDATE user_preferences SET notification_email = ? WHERE user_id = ?")
            .bind(notification_email)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(notification_browser) = payload.notification_browser {
        sqlx::query("UPDATE user_preferences SET notification_browser = ? WHERE user_id = ?")
            .bind(notification_browser)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(notification_sound) = payload.notification_sound {
        sqlx::query("UPDATE user_preferences SET notification_sound = ? WHERE user_id = ?")
            .bind(notification_sound)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(activity_visible) = payload.activity_visible {
        sqlx::query("UPDATE user_preferences SET activity_visible = ? WHERE user_id = ?")
            .bind(activity_visible)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    if let Some(show_online_status) = payload.show_online_status {
        sqlx::query("UPDATE user_preferences SET show_online_status = ? WHERE user_id = ?")
            .bind(show_online_status)
            .bind(&user_info.id)
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Update timestamp
    sqlx::query("UPDATE user_preferences SET updated_at = ? WHERE user_id = ?")
        .bind(&now)
        .bind(&user_info.id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch updated preferences
    get_user_preferences(State(state), user_info).await
}

// ==================== HELPER FUNCTIONS ====================

async fn create_default_preferences(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<(), StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let empty_searches = serde_json::to_string::<Vec<String>>(&vec![])
        .unwrap_or_else(|_| "[]".to_string());

    sqlx::query(
        r#"
        INSERT INTO user_preferences (
            id, user_id, view_mode, sort_by, sort_order, items_per_page,
            sidebar_collapsed, show_hidden_files, recent_searches,
            notification_email, notification_browser, notification_sound,
            activity_visible, show_online_status,
            created_at, updated_at
        ) VALUES (?, ?, 'grid', 'name', 'asc', 50, 0, 0, ?, 1, 1, 0, 1, 1, ?, ?)
        "#
    )
    .bind(&id)
    .bind(user_id)
    .bind(&empty_searches)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create default preferences: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}

