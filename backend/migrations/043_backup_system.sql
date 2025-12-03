-- Backup & Restore System
-- Migration: 043
-- Description: Comprehensive backup and restore system with scheduling, encryption, and deduplication

-- Backup Schedules Table
CREATE TABLE IF NOT EXISTS backup_schedules (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    name TEXT NOT NULL,
    description TEXT,
    
    -- Schedule Configuration
    frequency TEXT NOT NULL CHECK(frequency IN ('manual', 'hourly', 'daily', 'weekly', 'monthly')),
    schedule_time TEXT, -- Format: HH:MM for daily, or cron expression
    day_of_week INTEGER, -- 0-6 for weekly (0 = Sunday)
    day_of_month INTEGER CHECK(day_of_month BETWEEN 1 AND 31), -- 1-31 for monthly
    
    -- Backup Settings
    backup_type TEXT NOT NULL DEFAULT 'incremental' CHECK(backup_type IN ('full', 'incremental', 'differential')),
    include_paths TEXT NOT NULL, -- JSON array of paths to backup
    exclude_patterns TEXT, -- JSON array of glob patterns to exclude
    
    -- Encryption & Compression
    enable_encryption BOOLEAN NOT NULL DEFAULT 0,
    encryption_key_id TEXT, -- Reference to encryption key
    compression_level INTEGER DEFAULT 6 CHECK(compression_level BETWEEN 0 AND 9), -- 0=none, 9=max
    
    -- Storage Settings
    storage_backend_id TEXT, -- Reference to cloud_storage_backends if using remote
    backup_path TEXT NOT NULL, -- Local or remote path for backups
    max_backups INTEGER DEFAULT 10, -- Keep last N backups
    
    -- Retention Policy
    retention_days INTEGER DEFAULT 30, -- Delete backups older than N days
    retention_full_backups INTEGER DEFAULT 4, -- Keep at least N full backups
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT 1,
    last_run_at TIMESTAMP,
    next_run_at TIMESTAMP,
    last_status TEXT CHECK(last_status IN ('success', 'failed', 'running', 'pending')),
    last_error TEXT,
    
    -- Metadata
    created_by TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

-- Backup Jobs Table (Tracks individual backup executions)
CREATE TABLE IF NOT EXISTS backup_jobs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    schedule_id TEXT, -- NULL for manual backups
    
    -- Job Info
    job_type TEXT NOT NULL CHECK(job_type IN ('full', 'incremental', 'differential')),
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'running', 'completed', 'failed', 'cancelled')),
    
    -- Progress Tracking
    total_files INTEGER DEFAULT 0,
    processed_files INTEGER DEFAULT 0,
    total_bytes BIGINT DEFAULT 0,
    processed_bytes BIGINT DEFAULT 0,
    compressed_bytes BIGINT DEFAULT 0,
    
    -- Timing
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    duration_seconds INTEGER,
    
    -- Results
    backup_path TEXT,
    backup_size BIGINT,
    files_added INTEGER DEFAULT 0,
    files_modified INTEGER DEFAULT 0,
    files_deleted INTEGER DEFAULT 0,
    
    -- Deduplication Stats
    dedup_enabled BOOLEAN DEFAULT 1,
    dedup_savings_bytes BIGINT DEFAULT 0,
    unique_blocks INTEGER DEFAULT 0,
    reused_blocks INTEGER DEFAULT 0,
    
    -- Error Handling
    error_message TEXT,
    error_count INTEGER DEFAULT 0,
    warnings TEXT, -- JSON array of warning messages
    
    -- Metadata
    created_by TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (schedule_id) REFERENCES backup_schedules(id) ON DELETE SET NULL,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

-- Backup Files Table (Tracks individual files in backups)
CREATE TABLE IF NOT EXISTS backup_files (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    backup_job_id TEXT NOT NULL,
    
    -- File Info
    file_path TEXT NOT NULL,
    file_hash TEXT, -- SHA256 for deduplication
    file_size BIGINT NOT NULL,
    compressed_size BIGINT,
    
    -- File Metadata
    mime_type TEXT,
    modified_at TIMESTAMP,
    is_directory BOOLEAN DEFAULT 0,
    
    -- Backup Info
    action TEXT CHECK(action IN ('added', 'modified', 'deleted', 'unchanged')),
    is_encrypted BOOLEAN DEFAULT 0,
    
    -- Deduplication
    is_deduplicated BOOLEAN DEFAULT 0,
    block_refs TEXT, -- JSON array of block references for dedup
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (backup_job_id) REFERENCES backup_jobs(id) ON DELETE CASCADE
);

-- Restore Jobs Table (Tracks restore operations)
CREATE TABLE IF NOT EXISTS restore_jobs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    backup_job_id TEXT NOT NULL,
    
    -- Restore Configuration
    restore_type TEXT NOT NULL CHECK(restore_type IN ('full', 'selective', 'point-in-time')),
    source_paths TEXT, -- JSON array of paths to restore (for selective)
    destination_path TEXT NOT NULL,
    overwrite_existing BOOLEAN DEFAULT 0,
    
    -- Progress
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'running', 'completed', 'failed', 'cancelled')),
    total_files INTEGER DEFAULT 0,
    restored_files INTEGER DEFAULT 0,
    total_bytes BIGINT DEFAULT 0,
    restored_bytes BIGINT DEFAULT 0,
    
    -- Timing
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    duration_seconds INTEGER,
    
    -- Results
    files_restored INTEGER DEFAULT 0,
    files_skipped INTEGER DEFAULT 0,
    files_failed INTEGER DEFAULT 0,
    
    -- Error Handling
    error_message TEXT,
    warnings TEXT, -- JSON array
    
    -- Metadata
    created_by TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (backup_job_id) REFERENCES backup_jobs(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

-- Backup Deduplication Blocks (For efficient storage)
CREATE TABLE IF NOT EXISTS backup_dedup_blocks (
    block_hash TEXT PRIMARY KEY, -- SHA256 of block
    block_size INTEGER NOT NULL,
    reference_count INTEGER DEFAULT 1,
    first_seen_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_accessed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    storage_path TEXT NOT NULL -- Physical location of block
);

-- Backup Encryption Keys (For encrypted backups)
CREATE TABLE IF NOT EXISTS backup_encryption_keys (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    key_name TEXT NOT NULL UNIQUE,
    key_hash TEXT NOT NULL, -- Hashed encryption key (never store plaintext)
    algorithm TEXT NOT NULL DEFAULT 'AES-256-GCM',
    created_by TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMP,
    is_active BOOLEAN DEFAULT 1,
    
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for Performance
CREATE INDEX IF NOT EXISTS idx_backup_schedules_active ON backup_schedules(is_active, next_run_at);
CREATE INDEX IF NOT EXISTS idx_backup_schedules_user ON backup_schedules(created_by);

CREATE INDEX IF NOT EXISTS idx_backup_jobs_schedule ON backup_jobs(schedule_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_backup_jobs_status ON backup_jobs(status);
CREATE INDEX IF NOT EXISTS idx_backup_jobs_created ON backup_jobs(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_backup_files_job ON backup_files(backup_job_id);
CREATE INDEX IF NOT EXISTS idx_backup_files_hash ON backup_files(file_hash);
CREATE INDEX IF NOT EXISTS idx_backup_files_path ON backup_files(file_path);

CREATE INDEX IF NOT EXISTS idx_restore_jobs_backup ON restore_jobs(backup_job_id);
CREATE INDEX IF NOT EXISTS idx_restore_jobs_status ON restore_jobs(status);
CREATE INDEX IF NOT EXISTS idx_restore_jobs_created ON restore_jobs(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_dedup_blocks_hash ON backup_dedup_blocks(block_hash);
CREATE INDEX IF NOT EXISTS idx_dedup_blocks_accessed ON backup_dedup_blocks(last_accessed_at);

CREATE INDEX IF NOT EXISTS idx_encryption_keys_active ON backup_encryption_keys(is_active);

-- Insert Default Backup Schedule (Daily backup)
INSERT OR IGNORE INTO backup_schedules (
    id, name, description, frequency, schedule_time,
    backup_type, include_paths, backup_path,
    compression_level, max_backups, retention_days,
    is_active, created_by
) VALUES (
    'default-daily-backup',
    'Daily Full Backup',
    'Automatic daily backup of all files at 2 AM',
    'daily',
    '02:00',
    'full',
    '["/"]',
    './data/backups',
    6,
    7,
    30,
    0, -- Disabled by default, user must enable
    'system'
);

-- Insert Weekly Incremental Schedule
INSERT OR IGNORE INTO backup_schedules (
    id, name, description, frequency, schedule_time, day_of_week,
    backup_type, include_paths, backup_path,
    compression_level, max_backups, retention_days,
    is_active, created_by
) VALUES (
    'default-weekly-backup',
    'Weekly Full Backup',
    'Weekly full backup every Sunday at 3 AM',
    'weekly',
    '03:00',
    0, -- Sunday
    'full',
    '["/"]',
    './data/backups',
    9,
    4,
    90,
    0, -- Disabled by default
    'system'
);
