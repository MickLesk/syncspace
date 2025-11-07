-- Migration 027: Enhance file versions with compression and differential storage

-- Add new columns to file_versions table
ALTER TABLE file_versions ADD COLUMN storage_path TEXT;
ALTER TABLE file_versions ADD COLUMN original_size INTEGER NOT NULL DEFAULT 0;
ALTER TABLE file_versions ADD COLUMN compressed_size INTEGER NOT NULL DEFAULT 0;
ALTER TABLE file_versions ADD COLUMN is_compressed INTEGER NOT NULL DEFAULT 0;
ALTER TABLE file_versions ADD COLUMN is_differential INTEGER NOT NULL DEFAULT 0;
ALTER TABLE file_versions ADD COLUMN base_version_id TEXT;
ALTER TABLE file_versions ADD COLUMN checksum TEXT NOT NULL DEFAULT '';

-- Add foreign key for base_version_id
CREATE INDEX IF NOT EXISTS idx_file_versions_base_version ON file_versions(base_version_id);

-- Add index for cleanup queries
CREATE INDEX IF NOT EXISTS idx_file_versions_created_at ON file_versions(created_at);
CREATE INDEX IF NOT EXISTS idx_file_versions_file_version ON file_versions(file_id, version_number);

-- Add index for checksum lookups
CREATE INDEX IF NOT EXISTS idx_file_versions_checksum ON file_versions(checksum);
