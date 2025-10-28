-- Add Notifications, User Preferences, File Versions, Recent Files
-- Version: 0.2.2

-- ==================== NOTIFICATIONS ====================
CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    
    -- Notification content
    type TEXT NOT NULL, -- 'file_shared', 'file_modified', 'comment_added', 'storage_warning', 'system'
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    
    -- Action/Link
    action_url TEXT, -- e.g., "/files/12345" for "File shared with you"
    action_label TEXT, -- e.g., "View File"
    
    -- Status
    is_read BOOLEAN NOT NULL DEFAULT 0,
    read_at TEXT,
    
    -- Priority
    priority TEXT NOT NULL DEFAULT 'normal', -- 'low', 'normal', 'high', 'urgent'
    
    -- Related entities (for grouping/filtering)
    related_file_id TEXT,
    related_user_id TEXT,
    
    -- Timestamps
    created_at TEXT NOT NULL,
    expires_at TEXT, -- Auto-delete after this date
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (related_file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (related_user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_notifications_user ON notifications(user_id);
CREATE INDEX idx_notifications_read ON notifications(is_read);
CREATE INDEX idx_notifications_type ON notifications(type);
CREATE INDEX idx_notifications_created ON notifications(created_at);
CREATE INDEX idx_notifications_priority ON notifications(priority);


-- ==================== USER PREFERENCES ====================
CREATE TABLE IF NOT EXISTS user_preferences (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL UNIQUE,
    
    -- View preferences
    view_mode TEXT NOT NULL DEFAULT 'grid', -- 'grid', 'list', 'compact'
    sort_by TEXT NOT NULL DEFAULT 'name', -- 'name', 'date', 'size', 'type'
    sort_order TEXT NOT NULL DEFAULT 'asc', -- 'asc', 'desc'
    items_per_page INTEGER NOT NULL DEFAULT 50,
    
    -- UI state
    sidebar_collapsed BOOLEAN NOT NULL DEFAULT 0,
    show_hidden_files BOOLEAN NOT NULL DEFAULT 0,
    
    -- Search preferences
    recent_searches TEXT, -- JSON array of recent search queries
    search_filters TEXT, -- JSON object with saved filter presets
    
    -- Notifications
    notification_email BOOLEAN NOT NULL DEFAULT 1,
    notification_browser BOOLEAN NOT NULL DEFAULT 1,
    notification_sound BOOLEAN NOT NULL DEFAULT 0,
    
    -- Privacy
    activity_visible BOOLEAN NOT NULL DEFAULT 1,
    show_online_status BOOLEAN NOT NULL DEFAULT 1,
    
    -- Timestamps
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_prefs_user ON user_preferences(user_id);


-- ==================== FILE VERSIONS ====================
CREATE TABLE IF NOT EXISTS file_versions (
    id TEXT PRIMARY KEY NOT NULL,
    file_id TEXT NOT NULL, -- Reference to current file
    
    -- Version metadata
    version_number INTEGER NOT NULL,
    size_bytes INTEGER NOT NULL,
    checksum_sha256 TEXT NOT NULL,
    
    -- Storage
    storage_path TEXT NOT NULL, -- Path to versioned file on disk
    
    -- Version info
    created_by TEXT NOT NULL,
    change_description TEXT, -- Optional description of changes
    
    -- Timestamps
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE,
    
    UNIQUE(file_id, version_number)
);

CREATE INDEX idx_file_versions_file ON file_versions(file_id);
CREATE INDEX idx_file_versions_created ON file_versions(created_at);
CREATE INDEX idx_file_versions_number ON file_versions(version_number);


-- ==================== RECENT FILES ====================
CREATE TABLE IF NOT EXISTS recent_files (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    file_id TEXT NOT NULL,
    
    -- Access info
    access_count INTEGER NOT NULL DEFAULT 1,
    last_accessed_at TEXT NOT NULL,
    
    -- Context
    access_type TEXT NOT NULL DEFAULT 'view', -- 'view', 'edit', 'download'
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    
    UNIQUE(user_id, file_id)
);

CREATE INDEX idx_recent_files_user ON recent_files(user_id);
CREATE INDEX idx_recent_files_file ON recent_files(file_id);
CREATE INDEX idx_recent_files_accessed ON recent_files(last_accessed_at);


-- ==================== FILE PERMISSIONS ====================
CREATE TABLE IF NOT EXISTS file_permissions (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Target
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    item_id TEXT NOT NULL,
    
    -- Grantee
    user_id TEXT NOT NULL,
    
    -- Permissions
    can_read BOOLEAN NOT NULL DEFAULT 1,
    can_write BOOLEAN NOT NULL DEFAULT 0,
    can_delete BOOLEAN NOT NULL DEFAULT 0,
    can_share BOOLEAN NOT NULL DEFAULT 0,
    
    -- Grant info
    granted_by TEXT NOT NULL,
    granted_at TEXT NOT NULL,
    expires_at TEXT, -- NULL = never expires
    
    -- Timestamps
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (granted_by) REFERENCES users(id) ON DELETE CASCADE,
    
    UNIQUE(item_type, item_id, user_id)
);

CREATE INDEX idx_file_perms_item ON file_permissions(item_type, item_id);
CREATE INDEX idx_file_perms_user ON file_permissions(user_id);
CREATE INDEX idx_file_perms_granted_by ON file_permissions(granted_by);
CREATE INDEX idx_file_perms_expires ON file_permissions(expires_at);


-- ==================== FILE THUMBNAILS ====================
CREATE TABLE IF NOT EXISTS file_thumbnails (
    id TEXT PRIMARY KEY NOT NULL,
    file_id TEXT NOT NULL UNIQUE,
    
    -- Thumbnail metadata
    thumbnail_path TEXT NOT NULL, -- Path to thumbnail on disk
    thumbnail_size_bytes INTEGER NOT NULL,
    
    -- Generation info
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    format TEXT NOT NULL, -- 'jpeg', 'png', 'webp'
    
    -- Status
    generation_status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'completed', 'failed'
    generation_error TEXT,
    
    -- Timestamps
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE INDEX idx_thumbnails_file ON file_thumbnails(file_id);
CREATE INDEX idx_thumbnails_status ON file_thumbnails(generation_status);


-- ==================== SHARED LINK ACCESS LOG ====================
CREATE TABLE IF NOT EXISTS shared_link_access_log (
    id TEXT PRIMARY KEY NOT NULL,
    shared_link_id TEXT NOT NULL,
    
    -- Access details
    ip_address TEXT,
    user_agent TEXT,
    referer TEXT,
    
    -- User (if logged in)
    user_id TEXT,
    
    -- Action
    action TEXT NOT NULL, -- 'view', 'download', 'upload'
    
    -- Timestamp
    accessed_at TEXT NOT NULL,
    
    FOREIGN KEY (shared_link_id) REFERENCES shared_links(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_shared_access_log_link ON shared_link_access_log(shared_link_id);
CREATE INDEX idx_shared_access_log_user ON shared_link_access_log(user_id);
CREATE INDEX idx_shared_access_log_accessed ON shared_link_access_log(accessed_at);


-- ==================== UPDATE SETTINGS ====================
INSERT OR IGNORE INTO settings (key, value, value_type, description, category, updated_at) VALUES
    ('enable_notifications', 'true', 'boolean', 'Enable notification system', 'features', datetime('now')),
    ('notification_retention_days', '90', 'integer', 'Days to keep notifications', 'general', datetime('now')),
    ('enable_file_versioning', 'true', 'boolean', 'Enable automatic file versioning', 'features', datetime('now')),
    ('max_file_versions', '10', 'integer', 'Maximum versions to keep per file', 'storage', datetime('now')),
    ('enable_thumbnails', 'true', 'boolean', 'Generate thumbnails for images/videos', 'features', datetime('now')),
    ('thumbnail_max_size', '256', 'integer', 'Maximum thumbnail dimension in pixels', 'storage', datetime('now')),
    ('recent_files_limit', '50', 'integer', 'Number of recent files to track per user', 'general', datetime('now'));
