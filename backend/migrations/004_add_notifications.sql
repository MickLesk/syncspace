-- Add notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    notification_type TEXT NOT NULL,  -- 'file_shared', 'comment_added', 'quota_warning', 'backup_complete', etc.
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    link TEXT,  -- Deep link to relevant resource
    icon TEXT,  -- Icon name for UI
    is_read INTEGER NOT NULL DEFAULT 0,
    metadata TEXT,  -- JSON metadata
    created_at DATETIME NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_is_read ON notifications(is_read);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);
CREATE INDEX IF NOT EXISTS idx_notifications_user_unread ON notifications(user_id, is_read) WHERE is_read = 0;
