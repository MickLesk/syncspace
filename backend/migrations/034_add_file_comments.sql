-- Enhanced File Comments with Threading and @Mentions
-- Migration: 034_add_file_comments_threading
-- Description: Adds thread support and @mentions to existing comments table

-- Add new columns to existing comments table for threading and mentions
ALTER TABLE comments ADD COLUMN parent_comment_id TEXT; -- NULL for top-level, ID for replies
ALTER TABLE comments ADD COLUMN mentions TEXT; -- JSON array of mentioned user_ids
ALTER TABLE comments ADD COLUMN is_deleted INTEGER NOT NULL DEFAULT 0; -- Soft delete flag
ALTER TABLE comments ADD COLUMN deleted_at TEXT; -- Soft delete timestamp

-- Add foreign key constraint for parent comments (threading)
CREATE INDEX IF NOT EXISTS idx_comments_parent_id ON comments(parent_comment_id);
CREATE INDEX IF NOT EXISTS idx_comments_is_deleted ON comments(is_deleted);

-- Migration complete: Comments table now supports threading, @mentions, and soft delete

