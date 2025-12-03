-- Backup & Restore System - Compact Version
-- Migration: 043_backup_system_v2
-- Description: Comprehensive backup and restore system

-- Backup Schedules Table
CREATE TABLE IF NOT EXISTS backup_schedules (id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))), name TEXT NOT NULL, description TEXT, frequency TEXT NOT NULL CHECK(frequency IN ('manual', 'hourly', 'daily', 'weekly', 'monthly')), schedule_time TEXT, day_of_week INTEGER, day_of_month INTEGER CHECK(day_of_month BETWEEN 1 AND 31), backup_type TEXT NOT NULL DEFAULT 'incremental' CHECK(backup_type IN ('full', 'incremental', 'differential')), include_paths TEXT NOT NULL, exclude_patterns TEXT, enable_encryption BOOLEAN NOT NULL DEFAULT 0, encryption_key_id TEXT, compression_level INTEGER DEFAULT 6 CHECK(compression_level BETWEEN 0 AND 9), storage_backend_id TEXT, backup_path TEXT NOT NULL, max_backups INTEGER DEFAULT 10, retention_days INTEGER DEFAULT 30, retention_full_backups INTEGER DEFAULT 4, is_active BOOLEAN NOT NULL DEFAULT 1, last_run_at TIMESTAMP, next_run_at TIMESTAMP, last_status TEXT CHECK(last_status IN ('success', 'failed', 'running', 'pending')), last_error TEXT, created_by TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE);

-- Backup Jobs Table
CREATE TABLE IF NOT EXISTS backup_jobs (id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))), schedule_id TEXT, job_type TEXT NOT NULL CHECK(job_type IN ('full', 'incremental', 'differential')), status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'running', 'completed', 'failed', 'cancelled')), total_files INTEGER DEFAULT 0, processed_files INTEGER DEFAULT 0, total_bytes BIGINT DEFAULT 0, processed_bytes BIGINT DEFAULT 0, compressed_bytes BIGINT DEFAULT 0, started_at TIMESTAMP, completed_at TIMESTAMP, duration_seconds INTEGER, backup_path TEXT, backup_size BIGINT, files_added INTEGER DEFAULT 0, files_modified INTEGER DEFAULT 0, files_deleted INTEGER DEFAULT 0, dedup_enabled BOOLEAN DEFAULT 1, dedup_savings_bytes BIGINT DEFAULT 0, unique_blocks INTEGER DEFAULT 0, reused_blocks INTEGER DEFAULT 0, error_message TEXT, error_count INTEGER DEFAULT 0, warnings TEXT, created_by TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (schedule_id) REFERENCES backup_schedules(id) ON DELETE SET NULL, FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE);

-- Backup Files Table
CREATE TABLE IF NOT EXISTS backup_files (id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))), backup_job_id TEXT NOT NULL, file_path TEXT NOT NULL, file_hash TEXT, file_size BIGINT NOT NULL, compressed_size BIGINT, mime_type TEXT, modified_at TIMESTAMP, is_directory BOOLEAN DEFAULT 0, action TEXT CHECK(action IN ('added', 'modified', 'deleted', 'unchanged')), is_encrypted BOOLEAN DEFAULT 0, is_deduplicated BOOLEAN DEFAULT 0, block_refs TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (backup_job_id) REFERENCES backup_jobs(id) ON DELETE CASCADE);

-- Restore Jobs Table
CREATE TABLE IF NOT EXISTS restore_jobs (id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))), backup_job_id TEXT NOT NULL, restore_type TEXT NOT NULL CHECK(restore_type IN ('full', 'selective', 'point-in-time')), source_paths TEXT, destination_path TEXT NOT NULL, overwrite_existing BOOLEAN DEFAULT 0, status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'running', 'completed', 'failed', 'cancelled')), total_files INTEGER DEFAULT 0, restored_files INTEGER DEFAULT 0, total_bytes BIGINT DEFAULT 0, restored_bytes BIGINT DEFAULT 0, started_at TIMESTAMP, completed_at TIMESTAMP, duration_seconds INTEGER, files_restored INTEGER DEFAULT 0, files_skipped INTEGER DEFAULT 0, files_failed INTEGER DEFAULT 0, error_message TEXT, warnings TEXT, created_by TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (backup_job_id) REFERENCES backup_jobs(id) ON DELETE CASCADE, FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE);

-- Backup Deduplication Blocks
CREATE TABLE IF NOT EXISTS backup_dedup_blocks (block_hash TEXT PRIMARY KEY, block_size INTEGER NOT NULL, reference_count INTEGER DEFAULT 1, first_seen_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, last_accessed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, storage_path TEXT NOT NULL);

-- Backup Encryption Keys
CREATE TABLE IF NOT EXISTS backup_encryption_keys (id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))), key_name TEXT NOT NULL UNIQUE, key_hash TEXT NOT NULL, algorithm TEXT NOT NULL DEFAULT 'AES-256-GCM', created_by TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, last_used_at TIMESTAMP, is_active BOOLEAN DEFAULT 1, FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE);

-- Indexes
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

-- Default Schedules
INSERT OR IGNORE INTO backup_schedules (id, name, description, frequency, schedule_time, backup_type, include_paths, backup_path, compression_level, max_backups, retention_days, is_active, created_by) VALUES ('default-daily-backup', 'Daily Full Backup', 'Automatic daily backup of all files at 2 AM', 'daily', '02:00', 'full', '["/"]', './data/backups', 6, 7, 30, 0, 'system');

INSERT OR IGNORE INTO backup_schedules (id, name, description, frequency, schedule_time, day_of_week, backup_type, include_paths, backup_path, compression_level, max_backups, retention_days, is_active, created_by) VALUES ('default-weekly-backup', 'Weekly Full Backup', 'Weekly full backup every Sunday at 3 AM', 'weekly', '03:00', 0, 'full', '["/"]', './data/backups', 9, 4, 90, 0, 'system');
