-- Migration 031: Fix user_sessions table - Add revoked columns
-- Date: 2025-11-10
-- Description: Adds missing revoked columns to user_sessions table for session management

-- Add revoked column if not exists
-- SQLite doesn't have ALTER TABLE IF NOT EXISTS for columns, so we use a workaround
-- Check if column exists by trying to query it

-- Add revoked column (0=active, 1=revoked)
ALTER TABLE user_sessions ADD COLUMN revoked INTEGER NOT NULL DEFAULT 0;

-- Add revoked_at column (timestamp when revoked)
ALTER TABLE user_sessions ADD COLUMN revoked_at TEXT;

-- Add revoked_reason column (reason for revocation)
ALTER TABLE user_sessions ADD COLUMN revoked_reason TEXT;

-- Create index for faster queries on revoked sessions
CREATE INDEX IF NOT EXISTS idx_user_sessions_revoked ON user_sessions(revoked);
