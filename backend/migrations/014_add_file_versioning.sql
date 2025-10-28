-- Migration 014: Enhanced File Versioning  
-- Note: file_versions table already exists from migration 004
-- This migration only adds complementary versioning features

-- Version diffs table for storing change information
CREATE TABLE IF NOT EXISTS version_diffs (
    id TEXT PRIMARY KEY,
    from_version_id TEXT NOT NULL,
    to_version_id TEXT NOT NULL,
    diff_type TEXT NOT NULL CHECK (diff_type IN ('binary', 'text', 'image')),
    diff_content TEXT,
    added_lines INTEGER DEFAULT 0,
    removed_lines INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (from_version_id) REFERENCES file_versions(id) ON DELETE CASCADE,
    FOREIGN KEY (to_version_id) REFERENCES file_versions(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_version_diffs_unique ON version_diffs(from_version_id, to_version_id);
CREATE INDEX IF NOT EXISTS idx_version_diffs_from ON version_diffs(from_version_id);
CREATE INDEX IF NOT EXISTS idx_version_diffs_to ON version_diffs(to_version_id);

-- Version restoration log for audit trail
CREATE TABLE IF NOT EXISTS version_restorations (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    restored_from_version_id TEXT NOT NULL,
    restored_to_version_id TEXT NOT NULL,
    restored_by TEXT NOT NULL,
    restored_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    reason TEXT,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (restored_from_version_id) REFERENCES file_versions(id),
    FOREIGN KEY (restored_to_version_id) REFERENCES file_versions(id),
    FOREIGN KEY (restored_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_version_restorations_file_id ON version_restorations(file_id);
CREATE INDEX IF NOT EXISTS idx_version_restorations_restored_at ON version_restorations(restored_at);
CREATE INDEX IF NOT EXISTS idx_version_restorations_restored_by ON version_restorations(restored_by);
