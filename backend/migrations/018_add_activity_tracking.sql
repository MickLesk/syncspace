-- Activity Tracking System
-- Logs all file operations for audit trail and activity timeline

CREATE TABLE IF NOT EXISTS activity_log (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT NOT NULL,
    action TEXT NOT NULL, -- 'upload', 'download', 'delete', 'rename', 'move', 'create', 'share', 'favorite'
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER,
    old_path TEXT, -- for rename/move operations
    ip_address TEXT,
    user_agent TEXT,
    status TEXT DEFAULT 'success', -- 'success', 'failed', 'pending'
    error_message TEXT,
    metadata TEXT, -- JSON for additional data
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for fast queries
CREATE INDEX IF NOT EXISTS idx_activity_user_id ON activity_log(user_id);
CREATE INDEX IF NOT EXISTS idx_activity_created_at ON activity_log(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_activity_action ON activity_log(action);
CREATE INDEX IF NOT EXISTS idx_activity_file_path ON activity_log(file_path);
CREATE INDEX IF NOT EXISTS idx_activity_status ON activity_log(status);

-- Combined index for common queries
CREATE INDEX IF NOT EXISTS idx_activity_user_date ON activity_log(user_id, created_at DESC);
