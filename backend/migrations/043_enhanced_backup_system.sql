-- Migration 043: Enhanced Backup System with Encryption and Remote Storage

-- Add encryption support to backups table
ALTER TABLE backups ADD COLUMN is_encrypted INTEGER NOT NULL DEFAULT 0;
ALTER TABLE backups ADD COLUMN encryption_key_hash TEXT;
ALTER TABLE backups ADD COLUMN compressed_size_bytes INTEGER;
ALTER TABLE backups ADD COLUMN description TEXT;
ALTER TABLE backups ADD COLUMN restore_point TEXT; -- JSON with restore metadata

-- Add restore history table
CREATE TABLE IF NOT EXISTS backup_restores (
    id TEXT PRIMARY KEY,
    backup_id TEXT NOT NULL,
    restored_by TEXT NOT NULL,
    restored_at TEXT NOT NULL DEFAULT (datetime('now')),
    restore_type TEXT NOT NULL DEFAULT 'full', -- 'full', 'partial', 'selective'
    restore_path TEXT, -- Target path for restoration
    files_restored INTEGER DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'in_progress', -- 'in_progress', 'completed', 'failed'
    error_message TEXT,
    FOREIGN KEY (backup_id) REFERENCES backups(id) ON DELETE CASCADE,
    FOREIGN KEY (restored_by) REFERENCES users(id)
);

-- Add remote storage configurations
CREATE TABLE IF NOT EXISTS backup_destinations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    destination_type TEXT NOT NULL, -- 'local', 's3', 'sftp', 'webdav', 'azure', 'gcs'
    config TEXT NOT NULL, -- JSON encrypted configuration
    is_default INTEGER NOT NULL DEFAULT 0,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_verified_at TEXT,
    status TEXT NOT NULL DEFAULT 'active', -- 'active', 'inactive', 'error'
    FOREIGN KEY (created_by) REFERENCES users(id)
);

-- Update backup_schedules with more options
ALTER TABLE backup_schedules ADD COLUMN destination_id TEXT;
ALTER TABLE backup_schedules ADD COLUMN encryption_enabled INTEGER NOT NULL DEFAULT 0;
ALTER TABLE backup_schedules ADD COLUMN include_versions INTEGER NOT NULL DEFAULT 1;
ALTER TABLE backup_schedules ADD COLUMN include_database INTEGER NOT NULL DEFAULT 1;
ALTER TABLE backup_schedules ADD COLUMN exclude_patterns TEXT; -- JSON array of glob patterns
ALTER TABLE backup_schedules ADD COLUMN notify_on_success INTEGER NOT NULL DEFAULT 0;
ALTER TABLE backup_schedules ADD COLUMN notify_on_failure INTEGER NOT NULL DEFAULT 1;
ALTER TABLE backup_schedules ADD COLUMN max_backups INTEGER DEFAULT 10; -- Auto-cleanup old backups

-- Add backup metrics table for dashboard
CREATE TABLE IF NOT EXISTS backup_metrics (
    id TEXT PRIMARY KEY,
    date TEXT NOT NULL,
    total_backups INTEGER NOT NULL DEFAULT 0,
    successful_backups INTEGER NOT NULL DEFAULT 0,
    failed_backups INTEGER NOT NULL DEFAULT 0,
    total_size_bytes INTEGER NOT NULL DEFAULT 0,
    total_files INTEGER NOT NULL DEFAULT 0,
    avg_duration_seconds INTEGER DEFAULT 0,
    UNIQUE(date)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_backup_restores_backup_id ON backup_restores(backup_id);
CREATE INDEX IF NOT EXISTS idx_backup_restores_restored_by ON backup_restores(restored_by);
CREATE INDEX IF NOT EXISTS idx_backup_destinations_type ON backup_destinations(destination_type);
CREATE INDEX IF NOT EXISTS idx_backup_metrics_date ON backup_metrics(date);

-- Insert default local destination
INSERT OR IGNORE INTO backup_destinations (id, name, destination_type, config, is_default, created_by, created_at, status)
VALUES ('dest_local_default', 'Local Storage', 'local', '{"path": "./data/backups"}', 1, 'system', datetime('now'), 'active');
