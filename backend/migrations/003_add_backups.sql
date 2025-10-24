-- Add backups table for backup management
-- Migration 003

CREATE TABLE IF NOT EXISTS backups (
    id TEXT PRIMARY KEY NOT NULL,
    backup_type TEXT NOT NULL, -- 'full', 'incremental', 'database', 'files'
    size_bytes INTEGER NOT NULL DEFAULT 0,
    file_count INTEGER,
    storage_path TEXT NOT NULL,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'in_progress', -- 'in_progress', 'completed', 'failed'
    error_message TEXT,
    
    FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE INDEX idx_backups_created_at ON backups(created_at);
CREATE INDEX idx_backups_status ON backups(status);
