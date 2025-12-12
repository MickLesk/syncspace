-- Add active_theme_id column to users table for theme system
-- Migration: 029_add_active_theme_id.sql

-- Add active_theme_id column to users table
ALTER TABLE users ADD COLUMN active_theme_id TEXT REFERENCES user_themes(id);

-- Add index for performance
CREATE INDEX IF NOT EXISTS idx_users_active_theme ON users(active_theme_id);