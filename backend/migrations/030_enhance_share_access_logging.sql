-- Migration 030: Enhance Share Access Logging and Token Management
-- Version: Enhanced Share Tracking with Token Regeneration

-- Check if shared_link_access_log exists, if not create it
-- (It may already exist from migration 004)
CREATE TABLE IF NOT EXISTS shared_link_access_log (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Reference to shared link
    share_id TEXT NOT NULL,
    
    -- Access details
    accessed_by_ip TEXT,
    accessed_by_user_id TEXT, -- NULL if anonymous
    accessed_at TEXT NOT NULL,
    
    -- Activity
    action TEXT NOT NULL DEFAULT 'view', -- 'view', 'download', 'upload'
    status TEXT NOT NULL DEFAULT 'success', -- 'success', 'failed'
    error_message TEXT,
    
    -- Client info
    user_agent TEXT,
    referer TEXT,
    
    FOREIGN KEY (share_id) REFERENCES shared_links(id) ON DELETE CASCADE,
    FOREIGN KEY (accessed_by_user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_share_access_log_share ON shared_link_access_log(share_id);
CREATE INDEX IF NOT EXISTS idx_share_access_log_accessed_at ON shared_link_access_log(accessed_at);
CREATE INDEX IF NOT EXISTS idx_share_access_log_user ON shared_link_access_log(accessed_by_user_id);
CREATE INDEX IF NOT EXISTS idx_share_access_log_action ON shared_link_access_log(action);

-- Add token_version column to shared_links for tracking token regenerations
ALTER TABLE shared_links ADD COLUMN token_version INTEGER DEFAULT 1;
ALTER TABLE shared_links ADD COLUMN regenerated_at TEXT;
ALTER TABLE shared_links ADD COLUMN regenerated_by TEXT;

-- Create index for token version (for invalidating old tokens)
CREATE INDEX IF NOT EXISTS idx_shared_links_token_version ON shared_links(token_version);
