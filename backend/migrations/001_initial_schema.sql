-- SyncSpace Database Schema
-- Version: 0.2.0

-- ==================== USERS ====================
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    email TEXT,
    display_name TEXT,
    avatar_base64 TEXT,
    
    -- Security
    totp_secret TEXT,
    totp_enabled BOOLEAN NOT NULL DEFAULT 0,
    
    -- Quota & Limits
    storage_quota_bytes INTEGER DEFAULT 10737418240, -- 10GB default
    storage_used_bytes INTEGER NOT NULL DEFAULT 0,
    
    -- Preferences
    default_view TEXT DEFAULT 'grid', -- grid or list
    language TEXT DEFAULT 'de',
    theme TEXT DEFAULT 'light',
    
    -- Timestamps
    created_at TEXT NOT NULL,
    last_login TEXT,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);


-- ==================== FOLDERS ====================
CREATE TABLE IF NOT EXISTS folders (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE, -- Full path from root, e.g. "/Documents/Projects"
    parent_id TEXT, -- NULL for root folders
    
    -- Ownership
    owner_id TEXT NOT NULL,
    
    -- Metadata
    color TEXT, -- Optional folder color for UI
    icon TEXT, -- Optional custom icon name
    is_shared BOOLEAN NOT NULL DEFAULT 0,
    is_favorite BOOLEAN NOT NULL DEFAULT 0,
    
    -- Soft delete
    is_deleted BOOLEAN NOT NULL DEFAULT 0,
    deleted_at TEXT,
    deleted_by TEXT,
    
    -- Timestamps
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES folders(id) ON DELETE CASCADE,
    FOREIGN KEY (deleted_by) REFERENCES users(id)
);

CREATE INDEX idx_folders_parent ON folders(parent_id);
CREATE INDEX idx_folders_owner ON folders(owner_id);
CREATE INDEX idx_folders_path ON folders(path);
CREATE INDEX idx_folders_deleted ON folders(is_deleted);


-- ==================== FILES ====================
CREATE TABLE IF NOT EXISTS files (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    path TEXT NOT NULL, -- Full path including filename
    folder_id TEXT, -- NULL for root files
    
    -- Ownership
    owner_id TEXT NOT NULL,
    
    -- File metadata
    size_bytes INTEGER NOT NULL,
    mime_type TEXT,
    checksum_sha256 TEXT, -- For deduplication and integrity
    
    -- Physical storage
    storage_path TEXT NOT NULL, -- Actual path on disk (can be deduplicated)
    
    -- Flags
    is_encrypted BOOLEAN NOT NULL DEFAULT 0,
    is_shared BOOLEAN NOT NULL DEFAULT 0,
    is_favorite BOOLEAN NOT NULL DEFAULT 0,
    
    -- Soft delete with revision safety
    is_deleted BOOLEAN NOT NULL DEFAULT 0,
    deleted_at TEXT,
    deleted_by TEXT,
    
    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    previous_version_id TEXT, -- Link to previous version (if versioning enabled)
    
    -- Timestamps
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_accessed_at TEXT,
    
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE,
    FOREIGN KEY (deleted_by) REFERENCES users(id),
    FOREIGN KEY (previous_version_id) REFERENCES files(id)
);

CREATE INDEX idx_files_folder ON files(folder_id);
CREATE INDEX idx_files_owner ON files(owner_id);
CREATE INDEX idx_files_path ON files(path);
CREATE INDEX idx_files_checksum ON files(checksum_sha256);
CREATE INDEX idx_files_deleted ON files(is_deleted);
CREATE INDEX idx_files_name ON files(name);


-- ==================== FILE HISTORY ====================
CREATE TABLE IF NOT EXISTS file_history (
    id TEXT PRIMARY KEY NOT NULL,
    file_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    
    -- Action type
    action TEXT NOT NULL, -- created, renamed, moved, modified, deleted, restored, shared, unshared
    
    -- Change details (JSON)
    old_value TEXT, -- JSON with old state (name, path, size, etc)
    new_value TEXT, -- JSON with new state
    
    -- Additional context
    ip_address TEXT,
    user_agent TEXT,
    
    -- Timestamp
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_history_file ON file_history(file_id);
CREATE INDEX idx_history_user ON file_history(user_id);
CREATE INDEX idx_history_action ON file_history(action);
CREATE INDEX idx_history_created ON file_history(created_at);


-- ==================== TRASH ====================
CREATE TABLE IF NOT EXISTS trash (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Reference to original item
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    item_id TEXT NOT NULL, -- file_id or folder_id
    
    -- Original location (for restore)
    original_path TEXT NOT NULL,
    original_name TEXT NOT NULL,
    original_parent_id TEXT, -- NULL if was in root
    
    -- Deletion info
    deleted_by TEXT NOT NULL,
    deleted_at TEXT NOT NULL,
    
    -- Auto-cleanup
    auto_delete_at TEXT, -- NULL = never, otherwise date when to permanently delete
    
    -- Size for quota calculation
    size_bytes INTEGER NOT NULL DEFAULT 0,
    
    FOREIGN KEY (deleted_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_trash_item ON trash(item_type, item_id);
CREATE INDEX idx_trash_deleted_by ON trash(deleted_by);
CREATE INDEX idx_trash_auto_delete ON trash(auto_delete_at);


-- ==================== FAVORITES ====================
CREATE TABLE IF NOT EXISTS favorites (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    
    -- Reference to favorited item
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    item_id TEXT NOT NULL,
    
    -- Ordering (for user-defined sort)
    sort_order INTEGER NOT NULL DEFAULT 0,
    
    -- Timestamp
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    
    UNIQUE(user_id, item_type, item_id)
);

CREATE INDEX idx_favorites_user ON favorites(user_id);
CREATE INDEX idx_favorites_item ON favorites(item_type, item_id);


-- ==================== SHARED LINKS ====================
CREATE TABLE IF NOT EXISTS shared_links (
    id TEXT PRIMARY KEY NOT NULL, -- UUID for the public link
    
    -- Reference to shared item
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    item_id TEXT NOT NULL,
    
    -- Access control
    created_by TEXT NOT NULL,
    password_hash TEXT, -- Optional password protection (bcrypt)
    
    -- Link settings
    is_public BOOLEAN NOT NULL DEFAULT 1, -- Public or requires login
    allow_download BOOLEAN NOT NULL DEFAULT 1,
    allow_upload BOOLEAN NOT NULL DEFAULT 0, -- For folder shares
    
    -- Expiry
    expires_at TEXT, -- NULL = never expires
    max_downloads INTEGER, -- NULL = unlimited
    download_count INTEGER NOT NULL DEFAULT 0,
    
    -- Timestamps
    created_at TEXT NOT NULL,
    last_accessed_at TEXT,
    
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_shared_links_item ON shared_links(item_type, item_id);
CREATE INDEX idx_shared_links_creator ON shared_links(created_by);
CREATE INDEX idx_shared_links_expires ON shared_links(expires_at);


-- ==================== SETTINGS ====================
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    value_type TEXT NOT NULL, -- 'string', 'integer', 'boolean', 'json'
    description TEXT,
    category TEXT NOT NULL, -- 'general', 'security', 'storage', 'features'
    updated_at TEXT NOT NULL,
    updated_by TEXT,
    
    FOREIGN KEY (updated_by) REFERENCES users(id)
);

-- Default settings
INSERT OR IGNORE INTO settings (key, value, value_type, description, category, updated_at) VALUES
    ('revision_safety_enabled', 'true', 'boolean', 'Keep deleted files for recovery', 'storage', datetime('now')),
    ('auto_trash_cleanup_days', '30', 'integer', 'Days before auto-deleting from trash (0 = never)', 'storage', datetime('now')),
    ('max_upload_size_mb', '1024', 'integer', 'Maximum file upload size in MB', 'storage', datetime('now')),
    ('enable_file_versioning', 'false', 'boolean', 'Keep previous versions of modified files', 'features', datetime('now')),
    ('enable_deduplication', 'true', 'boolean', 'Store identical files only once', 'storage', datetime('now')),
    ('max_file_versions', '5', 'integer', 'Maximum versions to keep per file', 'features', datetime('now')),
    ('require_2fa', 'false', 'boolean', 'Require 2FA for all users', 'security', datetime('now')),
    ('session_timeout_hours', '24', 'integer', 'JWT token expiration in hours', 'security', datetime('now')),
    ('enable_shared_links', 'true', 'boolean', 'Allow users to create public share links', 'features', datetime('now')),
    ('default_user_quota_gb', '10', 'integer', 'Default storage quota per user in GB', 'storage', datetime('now'));

CREATE INDEX idx_settings_category ON settings(category);
