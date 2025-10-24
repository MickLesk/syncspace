-- Migration: Add file locking and collaboration tables
-- Created: 2025-10-24

-- File locks table (pessimistic locking)
CREATE TABLE IF NOT EXISTS file_locks (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    lock_type TEXT NOT NULL CHECK(lock_type IN ('read', 'write')),
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_file_locks_file ON file_locks(file_id);
CREATE INDEX idx_file_locks_user ON file_locks(user_id);
CREATE INDEX idx_file_locks_expires ON file_locks(expires_at);

-- File presence table (real-time collaboration)
CREATE TABLE IF NOT EXISTS file_presence (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    user_name TEXT NOT NULL,
    cursor_position INTEGER,
    last_seen TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_file_presence_file ON file_presence(file_id);
CREATE INDEX idx_file_presence_user ON file_presence(user_id);
CREATE INDEX idx_file_presence_last_seen ON file_presence(last_seen);
CREATE UNIQUE INDEX idx_file_presence_unique ON file_presence(file_id, user_id);

-- Conflict resolution table
CREATE TABLE IF NOT EXISTS conflict_resolutions (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    version_a_id TEXT NOT NULL,
    version_b_id TEXT NOT NULL,
    resolved_version_id TEXT,
    resolution_strategy TEXT NOT NULL CHECK(resolution_strategy IN ('manual', 'theirs', 'ours', 'merge')),
    resolved_by TEXT,
    resolved_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (resolved_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_conflicts_file ON conflict_resolutions(file_id);
CREATE INDEX idx_conflicts_resolved ON conflict_resolutions(resolved_at);
