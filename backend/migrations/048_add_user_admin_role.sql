-- Migration 048: Add admin flag and role to users table
-- Enables proper admin detection in frontend

-- Add is_admin column with default 0 (not admin)
ALTER TABLE users ADD COLUMN is_admin INTEGER NOT NULL DEFAULT 0;

-- Add role column for role-based access control
ALTER TABLE users ADD COLUMN role TEXT DEFAULT 'user';

-- Set first user (usually admin) as admin
UPDATE users SET is_admin = 1, role = 'admin' 
WHERE id = (SELECT id FROM users ORDER BY created_at ASC LIMIT 1);

-- Create index for role lookups
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_is_admin ON users(is_admin);
