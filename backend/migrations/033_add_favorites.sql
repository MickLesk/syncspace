-- Migration: Add Favorites System
-- Created: 2025-11-26
-- Description: User favorites for quick file access

-- Create user_favorites table
CREATE TABLE IF NOT EXISTS user_favorites (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    item_type TEXT NOT NULL DEFAULT 'file', -- 'file' or 'folder'
    item_path TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    -- Foreign keys
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    
    -- Prevent duplicate favorites
    UNIQUE(user_id, item_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_favorites_user_id ON user_favorites(user_id);
CREATE INDEX IF NOT EXISTS idx_favorites_created_at ON user_favorites(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_favorites_item_type ON user_favorites(item_type);

-- Add is_favorite flag to files table for quick lookups (optional optimization)
ALTER TABLE files ADD COLUMN is_favorite INTEGER DEFAULT 0;

-- Add is_favorite flag to folders table for quick lookups (optional optimization)
ALTER TABLE folders ADD COLUMN is_favorite INTEGER DEFAULT 0;
