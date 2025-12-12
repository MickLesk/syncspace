-- Theme Customization Migration
-- Add table for custom theme settings per user

CREATE TABLE IF NOT EXISTS user_themes (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    theme_name TEXT NOT NULL, -- 'default', 'ocean', 'forest', etc.
    is_custom BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Color scheme
    primary_color TEXT,
    secondary_color TEXT, 
    accent_color TEXT,
    background_color TEXT,
    surface_color TEXT,
    text_color TEXT,
    
    -- Layout preferences  
    density TEXT DEFAULT 'medium', -- compact, medium, spacious
    font_size TEXT DEFAULT 'medium', -- small, medium, large
    border_radius INTEGER DEFAULT 8,
    shadow_intensity INTEGER DEFAULT 5,
    
    -- Advanced settings
    glass_effect BOOLEAN DEFAULT TRUE,
    animations BOOLEAN DEFAULT TRUE,
    high_contrast BOOLEAN DEFAULT FALSE,
    
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_user_themes_user_id ON user_themes(user_id);
CREATE INDEX IF NOT EXISTS idx_user_themes_theme_name ON user_themes(theme_name);

-- Insert default themes for existing users
INSERT INTO user_themes (user_id, theme_name, is_custom)
SELECT id, theme, FALSE 
FROM users 
WHERE NOT EXISTS (
    SELECT 1 FROM user_themes WHERE user_themes.user_id = users.id
);

-- Add active_theme_id to users table to reference current theme
ALTER TABLE users ADD COLUMN active_theme_id TEXT REFERENCES user_themes(id);

-- Set active theme for existing users
UPDATE users 
SET active_theme_id = (
    SELECT id FROM user_themes 
    WHERE user_themes.user_id = users.id 
    AND user_themes.theme_name = users.theme
    LIMIT 1
);