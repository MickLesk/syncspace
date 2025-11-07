-- Migration: Add background jobs table
-- Created: 2024-11-07
-- Description: Background job queue system for async task processing

CREATE TABLE IF NOT EXISTS background_jobs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    job_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    priority INTEGER NOT NULL DEFAULT 5,
    payload TEXT NOT NULL,
    result TEXT,
    error TEXT,
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    scheduled_at TEXT NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Indexes for efficient job queue operations
CREATE INDEX IF NOT EXISTS idx_jobs_status_priority ON background_jobs(status, priority DESC, scheduled_at ASC);
CREATE INDEX IF NOT EXISTS idx_jobs_type ON background_jobs(job_type);
CREATE INDEX IF NOT EXISTS idx_jobs_scheduled ON background_jobs(scheduled_at);
CREATE INDEX IF NOT EXISTS idx_jobs_created ON background_jobs(created_at);

-- Job history table for completed/failed jobs
CREATE TABLE IF NOT EXISTS job_history (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    job_type TEXT NOT NULL,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    result TEXT,
    error TEXT,
    attempts INTEGER NOT NULL,
    duration_ms INTEGER,
    completed_at TEXT NOT NULL,
    FOREIGN KEY (job_id) REFERENCES background_jobs(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_job_history_type ON job_history(job_type);
CREATE INDEX IF NOT EXISTS idx_job_history_status ON job_history(status);
CREATE INDEX IF NOT EXISTS idx_job_history_completed ON job_history(completed_at);

-- Cron jobs table for recurring tasks
CREATE TABLE IF NOT EXISTS cron_jobs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    name TEXT UNIQUE NOT NULL,
    job_type TEXT NOT NULL,
    cron_expression TEXT NOT NULL,
    payload TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    last_run_at TEXT,
    next_run_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_cron_enabled ON cron_jobs(enabled, next_run_at);
CREATE INDEX IF NOT EXISTS idx_cron_next_run ON cron_jobs(next_run_at);
