-- Migration 030: Enhance Share Access Logging and Token Management
-- Version: Enhanced Share Tracking with Token Regeneration

-- Drop old table if it exists to recreate with correct schema
DROP TABLE IF EXISTS shared_link_access_log;

-- Create new table with correct column names matching database.rs
CREATE TABLE shared_link_access_log (
    id TEXT PRIMARY KEY NOT NULL,
    shared_link_id TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    referer TEXT,
    user_id TEXT,
    action TEXT NOT NULL DEFAULT 'view',
    accessed_at TEXT NOT NULL,
    
    FOREIGN KEY (shared_link_id) REFERENCES shared_links(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_share_access_log_share ON shared_link_access_log(shared_link_id);
CREATE INDEX IF NOT EXISTS idx_share_access_log_accessed_at ON shared_link_access_log(accessed_at);
CREATE INDEX IF NOT EXISTS idx_share_access_log_user ON shared_link_access_log(user_id);
CREATE INDEX IF NOT EXISTS idx_share_access_log_action ON shared_link_access_log(action);

-- Add token_version column to shared_links for tracking token regenerations
ALTER TABLE shared_links ADD COLUMN token_version INTEGER DEFAULT 1;
ALTER TABLE shared_links ADD COLUMN regenerated_at TEXT;
ALTER TABLE shared_links ADD COLUMN regenerated_by TEXT;

-- Create index for token version (for invalidating old tokens)
CREATE INDEX IF NOT EXISTS idx_shared_links_token_version ON shared_links(token_version);
