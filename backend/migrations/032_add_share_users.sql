-- ==================== SHARE USERS ====================
-- Add support for user-to-user file sharing with permissions

CREATE TABLE IF NOT EXISTS share_users (
    id TEXT PRIMARY KEY NOT NULL, -- UUID
    share_id TEXT NOT NULL, -- Foreign key to shared_links
    user_id TEXT NOT NULL, -- User receiving the share
    permission TEXT NOT NULL DEFAULT 'read', -- 'read', 'write', 'admin'
    created_at TEXT NOT NULL,
    created_by TEXT NOT NULL, -- Who added this user to the share
    
    FOREIGN KEY (share_id) REFERENCES shared_links(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL,
    
    UNIQUE(share_id, user_id) -- A user can only be shared with once per item
);

CREATE INDEX idx_share_users_share ON share_users(share_id);
CREATE INDEX idx_share_users_user ON share_users(user_id);
CREATE INDEX idx_share_users_permission ON share_users(permission);

-- Add new columns to shared_links for external sharing toggle
ALTER TABLE shared_links ADD COLUMN allow_external BOOLEAN NOT NULL DEFAULT 1;
ALTER TABLE shared_links ADD COLUMN share_type TEXT NOT NULL DEFAULT 'public'; -- 'public', 'user', 'mixed'

-- Update existing shares to be 'public' type
UPDATE shared_links SET share_type = 'public' WHERE is_public = 1;
