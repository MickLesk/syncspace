-- Add Comments and Tags support to SyncSpace
-- Version: 0.2.1

-- ==================== COMMENTS ====================
CREATE TABLE IF NOT EXISTS comments (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Reference to commented item
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    item_id TEXT NOT NULL, -- file_id or folder_id
    file_path TEXT NOT NULL, -- Denormalized for easier querying
    
    -- Comment content
    author_id TEXT NOT NULL,
    text TEXT NOT NULL,
    
    -- Comment state
    is_resolved BOOLEAN NOT NULL DEFAULT 0,
    resolved_at TEXT,
    resolved_by TEXT,
    
    -- Editing
    edited_at TEXT,
    edited_by TEXT,
    
    -- Timestamps
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (resolved_by) REFERENCES users(id),
    FOREIGN KEY (edited_by) REFERENCES users(id)
);

CREATE INDEX idx_comments_item ON comments(item_type, item_id);
CREATE INDEX idx_comments_file_path ON comments(file_path);
CREATE INDEX idx_comments_author ON comments(author_id);
CREATE INDEX idx_comments_created ON comments(created_at);
CREATE INDEX idx_comments_resolved ON comments(is_resolved);


-- ==================== TAGS ====================
CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Tag metadata
    name TEXT NOT NULL,
    color TEXT, -- Hex color for UI display (e.g., #FF5733)
    
    -- Ownership
    owner_id TEXT NOT NULL,
    
    -- Timestamps
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    
    UNIQUE(owner_id, name) -- Tag names are unique per user
);

CREATE INDEX idx_tags_owner ON tags(owner_id);
CREATE INDEX idx_tags_name ON tags(name);


-- ==================== FILE TAGS (Junction Table) ====================
CREATE TABLE IF NOT EXISTS file_tags (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- References
    file_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    item_type TEXT NOT NULL, -- 'file' or 'folder'
    file_path TEXT NOT NULL, -- Denormalized for easier querying
    
    -- Metadata
    tagged_by TEXT NOT NULL, -- User who applied this tag
    
    -- Timestamps
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    FOREIGN KEY (tagged_by) REFERENCES users(id),
    
    UNIQUE(file_id, tag_id) -- Each tag only once per file
);

CREATE INDEX idx_file_tags_file ON file_tags(file_id);
CREATE INDEX idx_file_tags_tag ON file_tags(tag_id);
CREATE INDEX idx_file_tags_path ON file_tags(file_path);
CREATE INDEX idx_file_tags_tagged_by ON file_tags(tagged_by);


-- ==================== ACTIVITY LOG EXTENSIONS ====================
-- Add status column to file_history if not exists (for tracking success/failure)
ALTER TABLE file_history ADD COLUMN status TEXT DEFAULT 'success';
ALTER TABLE file_history ADD COLUMN error_message TEXT;

CREATE INDEX idx_history_status ON file_history(status);
