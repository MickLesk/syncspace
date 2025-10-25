-- Enhanced backup system with scheduling and verification
-- Migration 013

-- Add backup schedules table
CREATE TABLE IF NOT EXISTS backup_schedules (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    cron_expression TEXT NOT NULL, -- e.g., '0 2 * * *' for daily at 2am
    backup_type TEXT NOT NULL, -- 'full', 'incremental', 'database_only', 'files_only'
    enabled INTEGER NOT NULL DEFAULT 1,
    destination_type TEXT NOT NULL, -- 'local', 's3', 'webdav'
    destination_config TEXT, -- JSON config for destination
    retention_days INTEGER DEFAULT 30,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    last_run_at TEXT,
    next_run_at TEXT,
    
    FOREIGN KEY (created_by) REFERENCES users(id)
);

-- Add backup verification table
CREATE TABLE IF NOT EXISTS backup_verifications (
    id TEXT PRIMARY KEY NOT NULL,
    backup_id TEXT NOT NULL,
    verification_type TEXT NOT NULL, -- 'checksum', 'restore_test', 'file_count'
    status TEXT NOT NULL, -- 'passed', 'failed', 'in_progress'
    details TEXT, -- JSON with verification details
    verified_at TEXT NOT NULL,
    
    FOREIGN KEY (backup_id) REFERENCES backups(id) ON DELETE CASCADE
);

-- Add backup files tracking (for incremental backups)
CREATE TABLE IF NOT EXISTS backup_files (
    id TEXT PRIMARY KEY NOT NULL,
    backup_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    checksum TEXT NOT NULL, -- SHA256 checksum
    compressed_size INTEGER,
    
    FOREIGN KEY (backup_id) REFERENCES backups(id) ON DELETE CASCADE
);

-- Enhance backups table
ALTER TABLE backups ADD COLUMN checksum TEXT;
ALTER TABLE backups ADD COLUMN compression_type TEXT DEFAULT 'zip'; -- 'zip', 'tar.gz', 'tar.bz2'
ALTER TABLE backups ADD COLUMN schedule_id TEXT;
ALTER TABLE backups ADD COLUMN destination_type TEXT DEFAULT 'local';
ALTER TABLE backups ADD COLUMN metadata TEXT; -- JSON for additional info

-- Create indexes
CREATE INDEX idx_backup_schedules_enabled ON backup_schedules(enabled);
CREATE INDEX idx_backup_schedules_next_run ON backup_schedules(next_run_at);
CREATE INDEX idx_backup_verifications_backup_id ON backup_verifications(backup_id);
CREATE INDEX idx_backup_files_backup_id ON backup_files(backup_id);
CREATE INDEX idx_backups_schedule_id ON backups(schedule_id);
