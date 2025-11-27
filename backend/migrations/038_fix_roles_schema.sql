-- Migration 038: Fix roles table schema
-- Adds missing columns: display_name, updated_at, is_default

-- Add display_name column (required for RBAC system)
ALTER TABLE roles ADD COLUMN display_name TEXT;

-- Add updated_at column (for tracking changes)
ALTER TABLE roles ADD COLUMN updated_at TEXT;

-- Add is_default column (for auto-assignment to new users)
ALTER TABLE roles ADD COLUMN is_default INTEGER NOT NULL DEFAULT 0;

-- Update existing roles with display_name (copy from name field, capitalize)
UPDATE roles SET display_name = 
    CASE name
        WHEN 'super_admin' THEN 'Super Administrator'
        WHEN 'admin' THEN 'Administrator'
        WHEN 'moderator' THEN 'Moderator'
        WHEN 'user' THEN 'Standard User'
        WHEN 'viewer' THEN 'Viewer'
        WHEN 'guest' THEN 'Guest'
        ELSE name
    END
WHERE display_name IS NULL;

-- Update existing roles with updated_at (same as created_at)
UPDATE roles SET updated_at = created_at WHERE updated_at IS NULL;

-- Make display_name NOT NULL after populating data
-- Note: SQLite doesn't support ALTER COLUMN, so we'll enforce it in application layer

-- Set default role (typically 'user' role)
UPDATE roles SET is_default = 1 WHERE name = 'user';
