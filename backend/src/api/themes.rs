use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::UserInfo,
    database::{UserTheme},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/themes", get(get_user_themes).post(create_theme))
        .route("/themes/{id}", get(get_theme).put(update_theme).delete(delete_theme))
        .route("/themes/{id}/activate", post(activate_theme))
        .route("/themes/presets", get(get_theme_presets))
        .route("/themes/export/{id}", get(export_theme))
        .route("/themes/import", post(import_theme))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateThemeRequest {
    pub theme_name: String,
    pub is_custom: bool,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>, 
    pub accent_color: Option<String>,
    pub background_color: Option<String>,
    pub surface_color: Option<String>,
    pub text_color: Option<String>,
    pub density: Option<String>,
    pub font_size: Option<String>,
    pub border_radius: Option<i32>,
    pub shadow_intensity: Option<i32>,
    pub glass_effect: Option<bool>,
    pub animations: Option<bool>,
    pub high_contrast: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateThemeRequest {
    pub theme_name: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>, 
    pub background_color: Option<String>,
    pub surface_color: Option<String>,
    pub text_color: Option<String>,
    pub density: Option<String>,
    pub font_size: Option<String>,
    pub border_radius: Option<i32>,
    pub shadow_intensity: Option<i32>,
    pub glass_effect: Option<bool>,
    pub animations: Option<bool>,
    pub high_contrast: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ThemePreset {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub category: String,
}

// Get all themes for current user
async fn get_user_themes(
    user: UserInfo,
    State(state): State<AppState>,
) -> Result<Json<Vec<UserTheme>>, StatusCode> {
    let themes = sqlx::query_as::<_, UserTheme>(
        "SELECT * FROM user_themes WHERE user_id = ? ORDER BY created_at ASC"
    )
    .bind(&user.user_id())
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to get user themes: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(themes))
}

// Get specific theme
async fn get_theme(
    user: UserInfo,
    Path(theme_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<UserTheme>, StatusCode> {
    let theme = sqlx::query_as::<_, UserTheme>(
        "SELECT * FROM user_themes WHERE id = ? AND user_id = ?"
    )
    .bind(&theme_id)
    .bind(&user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(theme))
}

// Create new theme
async fn create_theme(
    user: UserInfo,
    State(state): State<AppState>,
    Json(req): Json<CreateThemeRequest>,
) -> Result<Json<UserTheme>, StatusCode> {
    let theme_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let theme = sqlx::query_as::<_, UserTheme>(
        "INSERT INTO user_themes (
            id, user_id, theme_name, is_custom,
            primary_color, secondary_color, accent_color,
            background_color, surface_color, text_color,
            density, font_size, border_radius, shadow_intensity,
            glass_effect, animations, high_contrast,
            created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *"
    )
    .bind(&theme_id)
    .bind(&user.user_id())
    .bind(&req.theme_name)
    .bind(req.is_custom)
    .bind(&req.primary_color)
    .bind(&req.secondary_color)
    .bind(&req.accent_color)
    .bind(&req.background_color)
    .bind(&req.surface_color)
    .bind(&req.text_color)
    .bind(req.density.unwrap_or_else(|| "medium".to_string()))
    .bind(req.font_size.unwrap_or_else(|| "medium".to_string()))
    .bind(req.border_radius.unwrap_or(8))
    .bind(req.shadow_intensity.unwrap_or(5))
    .bind(req.glass_effect.unwrap_or(true))
    .bind(req.animations.unwrap_or(true))
    .bind(req.high_contrast.unwrap_or(false))
    .bind(&now)
    .bind(&now)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create theme: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(theme))
}

// Update theme
async fn update_theme(
    user: UserInfo,
    Path(theme_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateThemeRequest>,
) -> Result<Json<UserTheme>, StatusCode> {
    let now = chrono::Utc::now().to_rfc3339();

    // Simple update with all fields (using Option handling in SQL)
    sqlx::query(
        "UPDATE user_themes SET 
         updated_at = ?,
         theme_name = COALESCE(?, theme_name),
         primary_color = COALESCE(?, primary_color),
         secondary_color = COALESCE(?, secondary_color),
         accent_color = COALESCE(?, accent_color),
         background_color = COALESCE(?, background_color),
         surface_color = COALESCE(?, surface_color),
         text_color = COALESCE(?, text_color),
         density = COALESCE(?, density),
         font_size = COALESCE(?, font_size),
         border_radius = COALESCE(?, border_radius),
         shadow_intensity = COALESCE(?, shadow_intensity),
         glass_effect = COALESCE(?, glass_effect),
         animations = COALESCE(?, animations),
         high_contrast = COALESCE(?, high_contrast)
         WHERE id = ? AND user_id = ?"
    )
    .bind(&now)
    .bind(&req.theme_name)
    .bind(&req.primary_color)
    .bind(&req.secondary_color)
    .bind(&req.accent_color)
    .bind(&req.background_color)
    .bind(&req.surface_color)
    .bind(&req.text_color)
    .bind(&req.density)
    .bind(&req.font_size)
    .bind(req.border_radius)
    .bind(req.shadow_intensity)
    .bind(req.glass_effect)
    .bind(req.animations)
    .bind(req.high_contrast)
    .bind(&theme_id)
    .bind(&user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to update theme: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Return updated theme
    let theme = sqlx::query_as::<_, UserTheme>(
        "SELECT * FROM user_themes WHERE id = ? AND user_id = ?"
    )
    .bind(&theme_id)
    .bind(&user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(theme))
}

// Delete theme
async fn delete_theme(
    user: UserInfo,
    Path(theme_id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    // Don't allow deletion if this is the active theme
    let active_theme_check = sqlx::query(
        "SELECT id FROM users WHERE id = ? AND active_theme_id = ?"
    )
    .bind(&user.user_id())
    .bind(&theme_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if active_theme_check.is_some() {
        return Err(StatusCode::BAD_REQUEST); // Can't delete active theme
    }

    let result = sqlx::query(
        "DELETE FROM user_themes WHERE id = ? AND user_id = ? AND is_custom = TRUE"
    )
    .bind(&theme_id)
    .bind(&user.user_id())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

// Activate theme (set as active for user)
async fn activate_theme(
    user: UserInfo,
    Path(theme_id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    // Verify theme belongs to user
    let _theme = sqlx::query("SELECT id FROM user_themes WHERE id = ? AND user_id = ?")
        .bind(&theme_id)
        .bind(&user.user_id())
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Update user's active theme
    sqlx::query("UPDATE users SET active_theme_id = ? WHERE id = ?")
        .bind(&theme_id)
        .bind(&user.user_id())
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

// Get theme presets
async fn get_theme_presets() -> Json<Vec<ThemePreset>> {
    let presets = vec![
        ThemePreset {
            name: "default".to_string(),
            display_name: "Default".to_string(),
            description: "Moderne, saubere Farben".to_string(),
            primary_color: "#3b82f6".to_string(),
            secondary_color: "#1e40af".to_string(),
            accent_color: "#06b6d4".to_string(),
            category: "standard".to_string(),
        },
        ThemePreset {
            name: "ocean".to_string(),
            display_name: "Ocean".to_string(),
            description: "Ruhige Blautöne".to_string(),
            primary_color: "#0ea5e9".to_string(),
            secondary_color: "#0284c7".to_string(),
            accent_color: "#0891b2".to_string(),
            category: "nature".to_string(),
        },
        ThemePreset {
            name: "forest".to_string(),
            display_name: "Forest".to_string(),
            description: "Grüntöne der Natur".to_string(),
            primary_color: "#22c55e".to_string(),
            secondary_color: "#16a34a".to_string(),
            accent_color: "#15803d".to_string(),
            category: "nature".to_string(),
        },
        ThemePreset {
            name: "sunset".to_string(),
            display_name: "Sunset".to_string(),
            description: "Warme, energische Farben".to_string(),
            primary_color: "#f97316".to_string(),
            secondary_color: "#ea580c".to_string(),
            accent_color: "#dc2626".to_string(),
            category: "warm".to_string(),
        },
        ThemePreset {
            name: "cyberpunk".to_string(),
            display_name: "Cyberpunk".to_string(),
            description: "Neon-Vibrationen".to_string(),
            primary_color: "#a855f7".to_string(),
            secondary_color: "#9333ea".to_string(),
            accent_color: "#ec4899".to_string(),
            category: "vibrant".to_string(),
        },
        ThemePreset {
            name: "solarized_light".to_string(),
            display_name: "Solarized Light".to_string(),
            description: "Augenschonend, warm".to_string(),
            primary_color: "#268bd2".to_string(),
            secondary_color: "#2aa198".to_string(),
            accent_color: "#cb4b16".to_string(),
            category: "accessibility".to_string(),
        },
        ThemePreset {
            name: "solarized_dark".to_string(),
            display_name: "Solarized Dark".to_string(),
            description: "Augenschonend, dunkel".to_string(),
            primary_color: "#268bd2".to_string(),
            secondary_color: "#2aa198".to_string(),
            accent_color: "#dc322f".to_string(),
            category: "accessibility".to_string(),
        },
        ThemePreset {
            name: "nord".to_string(),
            display_name: "Nord".to_string(),
            description: "Arktische, kühle Töne".to_string(),
            primary_color: "#5e81ac".to_string(),
            secondary_color: "#81a1c1".to_string(),
            accent_color: "#88c0d0".to_string(),
            category: "cool".to_string(),
        },
        ThemePreset {
            name: "dracula".to_string(),
            display_name: "Dracula".to_string(),
            description: "Dunkle Vampir-Ästhetik".to_string(),
            primary_color: "#bd93f9".to_string(),
            secondary_color: "#50fa7b".to_string(),
            accent_color: "#ff79c6".to_string(),
            category: "dark".to_string(),
        },
        ThemePreset {
            name: "gruvbox".to_string(),
            display_name: "Gruvbox".to_string(),
            description: "Retro, warme Farben".to_string(),
            primary_color: "#fe8019".to_string(),
            secondary_color: "#b8bb26".to_string(),
            accent_color: "#fabd2f".to_string(),
            category: "retro".to_string(),
        },
        ThemePreset {
            name: "monokai".to_string(),
            display_name: "Monokai".to_string(),
            description: "Klassischer Editor-Look".to_string(),
            primary_color: "#66d9ef".to_string(),
            secondary_color: "#a6e22e".to_string(),
            accent_color: "#f92672".to_string(),
            category: "classic".to_string(),
        },
        ThemePreset {
            name: "catppuccin".to_string(),
            display_name: "Catppuccin".to_string(),
            description: "Sanfte Pastelltöne".to_string(),
            primary_color: "#89b4fa".to_string(),
            secondary_color: "#a6e3a1".to_string(),
            accent_color: "#f5c2e7".to_string(),
            category: "pastel".to_string(),
        },
        ThemePreset {
            name: "tokyo_night".to_string(),
            display_name: "Tokyo Night".to_string(),
            description: "Urbane Nachtszene".to_string(),
            primary_color: "#7aa2f7".to_string(),
            secondary_color: "#9ece6a".to_string(),
            accent_color: "#bb9af7".to_string(),
            category: "urban".to_string(),
        },
    ];

    Json(presets)
}

// Export theme as JSON
async fn export_theme(
    user: UserInfo,
    Path(theme_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<UserTheme>, StatusCode> {
    let theme = sqlx::query_as::<_, UserTheme>(
        "SELECT * FROM user_themes WHERE id = ? AND user_id = ?"
    )
    .bind(&theme_id)
    .bind(&user.user_id())
    .fetch_one(&state.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(theme))
}

// Import theme from JSON
async fn import_theme(
    user: UserInfo,
    State(state): State<AppState>,
    Json(theme_data): Json<CreateThemeRequest>,
) -> Result<Json<UserTheme>, StatusCode> {
    // Force it to be custom and generate new ID
    let mut req = theme_data;
    req.is_custom = true;
    
    create_theme(user, State(state), Json(req)).await
}
