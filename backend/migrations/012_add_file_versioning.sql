-- Migration 012: Add File Versioning System
-- Creates tables and indexes for comprehensive file version tracking

-- File versions table
CREATE TABLE file_versions (
    id TEXT PRIMARY KEY DEFAULT (uuid()),
    file_id TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    content_hash TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT NOT NULL,
    comment TEXT,
    is_current BOOLEAN DEFAULT FALSE,
    storage_path TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id),
    UNIQUE(file_id, version_number)
);

-- Version metadata table for additional properties
CREATE TABLE version_metadata (
    id TEXT PRIMARY KEY DEFAULT (uuid()),
    version_id TEXT NOT NULL,
    metadata_key TEXT NOT NULL,
    metadata_value TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (version_id) REFERENCES file_versions(id) ON DELETE CASCADE,
    UNIQUE(version_id, metadata_key)
);

-- Version diffs table for storing change information
CREATE TABLE version_diffs (
    id TEXT PRIMARY KEY DEFAULT (uuid()),
    from_version_id TEXT NOT NULL,
    to_version_id TEXT NOT NULL,
    diff_type TEXT NOT NULL CHECK (diff_type IN ('binary', 'text', 'image')),
    diff_content TEXT,
    added_lines INTEGER DEFAULT 0,
    removed_lines INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (from_version_id) REFERENCES file_versions(id) ON DELETE CASCADE,
    FOREIGN KEY (to_version_id) REFERENCES file_versions(id) ON DELETE CASCADE,
    UNIQUE(from_version_id, to_version_id)
);

-- Version restoration log for audit trail
CREATE TABLE version_restorations (
    id TEXT PRIMARY KEY DEFAULT (uuid()),
    file_id TEXT NOT NULL,
    restored_from_version_id TEXT NOT NULL,
    restored_to_version_id TEXT NOT NULL,
    restored_by TEXT NOT NULL,
    restored_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    reason TEXT,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (restored_from_version_id) REFERENCES file_versions(id),
    FOREIGN KEY (restored_to_version_id) REFERENCES file_versions(id),
    FOREIGN KEY (restored_by) REFERENCES users(id)
);

-- Indexes for performance
CREATE INDEX idx_file_versions_file_id ON file_versions(file_id);
CREATE INDEX idx_file_versions_created_at ON file_versions(created_at);
CREATE INDEX idx_file_versions_current ON file_versions(file_id, is_current);
CREATE INDEX idx_version_metadata_version_id ON version_metadata(version_id);
CREATE INDEX idx_version_diffs_versions ON version_diffs(from_version_id, to_version_id);
CREATE INDEX idx_version_restorations_file_id ON version_restorations(file_id);
CREATE INDEX idx_version_restorations_restored_at ON version_restorations(restored_at);

-- Trigger to ensure only one current version per file
CREATE TRIGGER ensure_single_current_version
AFTER INSERT ON file_versions
WHEN NEW.is_current = TRUE
BEGIN
    UPDATE file_versions 
    SET is_current = FALSE 
    WHERE file_id = NEW.file_id AND id != NEW.id;
END;

-- Trigger to automatically increment version numbers
CREATE TRIGGER auto_increment_version_number
BEFORE INSERT ON file_versions
WHEN NEW.version_number IS NULL
BEGIN
    UPDATE file_versions SET version_number = (
        SELECT COALESCE(MAX(version_number), 0) + 1 
        FROM file_versions 
        WHERE file_id = NEW.file_id
    ) WHERE id = NEW.id;
END;