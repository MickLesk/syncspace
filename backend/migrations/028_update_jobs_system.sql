-- Migration: Update jobs system schema
-- Created: 2025-11-07
-- Description: Update jobs table to match new job system

-- Create new jobs table with updated schema
CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY,
    job_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    payload TEXT NOT NULL,
    result TEXT,
    error TEXT,
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    created_at TEXT NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    scheduled_for TEXT,
    created_by TEXT
);

-- Indexes for efficient job queue operations
CREATE INDEX IF NOT EXISTS idx_jobs_status_scheduled ON jobs(status, scheduled_for);
CREATE INDEX IF NOT EXISTS idx_jobs_type ON jobs(job_type);
CREATE INDEX IF NOT EXISTS idx_jobs_created ON jobs(created_at);
CREATE INDEX IF NOT EXISTS idx_jobs_completed ON jobs(completed_at);

-- Migrate data from background_jobs if it exists
INSERT OR IGNORE INTO jobs (id, job_type, status, payload, result, error, attempts, max_attempts, created_at, started_at, completed_at, scheduled_for, created_by)
SELECT 
    id,
    job_type,
    status,
    payload,
    result,
    error,
    attempts,
    max_attempts,
    created_at,
    started_at,
    completed_at,
    scheduled_at,
    NULL
FROM background_jobs
WHERE EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='background_jobs');
