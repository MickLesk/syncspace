-- Migration 042: Version Tags and Enhanced Version Management

-- Version tags table for semantic versioning labels (v1.0, production, draft, etc.)
CREATE TABLE IF NOT EXISTS version_tags (
    id TEXT PRIMARY KEY,
    version_id TEXT NOT NULL,
    tag_name TEXT NOT NULL,
    tag_color TEXT DEFAULT '#3b82f6',
    description TEXT,
    created_by TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (version_id) REFERENCES file_versions(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id)
);

-- Unique constraint: one tag name per version
CREATE UNIQUE INDEX IF NOT EXISTS idx_version_tags_unique ON version_tags(version_id, tag_name);
CREATE INDEX IF NOT EXISTS idx_version_tags_version_id ON version_tags(version_id);
CREATE INDEX IF NOT EXISTS idx_version_tags_tag_name ON version_tags(tag_name);

-- Predefined tag templates for quick access
CREATE TABLE IF NOT EXISTS version_tag_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#3b82f6',
    description TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert default tag templates
INSERT OR IGNORE INTO version_tag_templates (id, name, color, description, is_system) VALUES
    ('tpl_production', 'production', '#22c55e', 'Production-ready version', 1),
    ('tpl_staging', 'staging', '#f59e0b', 'Staging/testing version', 1),
    ('tpl_draft', 'draft', '#6b7280', 'Work in progress draft', 1),
    ('tpl_approved', 'approved', '#3b82f6', 'Approved by reviewer', 1),
    ('tpl_deprecated', 'deprecated', '#ef4444', 'Deprecated, do not use', 1),
    ('tpl_backup', 'backup', '#8b5cf6', 'Backup checkpoint', 1);

-- Add pinned and starred columns to file_versions if not exists
-- (SQLite doesn't support IF NOT EXISTS for columns, so we use a workaround)
ALTER TABLE file_versions ADD COLUMN is_pinned INTEGER NOT NULL DEFAULT 0;
ALTER TABLE file_versions ADD COLUMN is_starred INTEGER NOT NULL DEFAULT 0;

-- Version statistics view (for storage usage reporting)
CREATE VIEW IF NOT EXISTS version_storage_stats AS
SELECT 
    f.id as file_id,
    f.filename,
    f.file_path,
    COUNT(fv.id) as version_count,
    SUM(fv.original_size) as total_original_size,
    SUM(fv.compressed_size) as total_compressed_size,
    MAX(fv.created_at) as last_version_at,
    MIN(fv.created_at) as first_version_at
FROM files f
LEFT JOIN file_versions fv ON f.id = fv.file_id
GROUP BY f.id, f.filename, f.file_path;

-- Index for version cleanup queries
CREATE INDEX IF NOT EXISTS idx_file_versions_is_pinned ON file_versions(is_pinned);
CREATE INDEX IF NOT EXISTS idx_file_versions_is_starred ON file_versions(is_starred);
