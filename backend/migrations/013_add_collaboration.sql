-- Collaborative File Locks (renamed from file_locks to avoid conflict with 008_add_locking.sql)
CREATE TABLE IF NOT EXISTS collaborative_locks (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    locked_by TEXT NOT NULL,
    locked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,
    lock_type TEXT NOT NULL DEFAULT 'exclusive', -- 'exclusive' or 'shared'
    last_heartbeat TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (locked_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_collaborative_locks_file_id ON collaborative_locks(file_id);
CREATE INDEX idx_collaborative_locks_file_path ON collaborative_locks(file_path);
CREATE INDEX idx_collaborative_locks_locked_by ON collaborative_locks(locked_by);
CREATE INDEX idx_collaborative_locks_expires_at ON collaborative_locks(expires_at);

-- User Presence tracking
CREATE TABLE IF NOT EXISTS user_presence (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    username TEXT NOT NULL,
    file_path TEXT,
    activity_type TEXT NOT NULL, -- 'viewing', 'editing', 'idle'
    last_seen TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT, -- JSON for cursor position, selection, etc.
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_presence_user_id ON user_presence(user_id);
CREATE INDEX idx_user_presence_file_path ON user_presence(file_path);
CREATE INDEX idx_user_presence_last_seen ON user_presence(last_seen);

-- Collaborative Activity Log
CREATE TABLE IF NOT EXISTS collaboration_activity (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    username TEXT NOT NULL,
    file_path TEXT NOT NULL,
    activity_type TEXT NOT NULL, -- 'lock_acquired', 'lock_released', 'edit_started', 'edit_saved', 'conflict_detected', 'conflict_resolved'
    details TEXT, -- JSON for additional context
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_collaboration_activity_user_id ON collaboration_activity(user_id);
CREATE INDEX idx_collaboration_activity_file_path ON collaboration_activity(file_path);
CREATE INDEX idx_collaboration_activity_created_at ON collaboration_activity(created_at);

-- Edit Conflicts tracking
CREATE TABLE IF NOT EXISTS edit_conflicts (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    conflict_type TEXT NOT NULL, -- 'concurrent_edit', 'version_mismatch', 'lock_timeout'
    user1_id TEXT NOT NULL,
    user2_id TEXT,
    detected_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMP,
    resolution_strategy TEXT, -- 'auto_merge', 'manual', 'keep_latest', 'keep_version'
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'resolved', 'ignored'
    details TEXT, -- JSON with conflict details
    FOREIGN KEY (user1_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (user2_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_edit_conflicts_file_id ON edit_conflicts(file_id);
CREATE INDEX idx_edit_conflicts_file_path ON edit_conflicts(file_path);
CREATE INDEX idx_edit_conflicts_status ON edit_conflicts(status);
CREATE INDEX idx_edit_conflicts_detected_at ON edit_conflicts(detected_at);
