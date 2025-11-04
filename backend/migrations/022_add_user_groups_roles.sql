-- Migration 022: User Groups and Advanced Role Management
-- Adds user groups, role assignments, and enhanced permissions

-- User Groups
CREATE TABLE IF NOT EXISTS user_groups (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_user_groups_name ON user_groups(name);

-- Group Membership
CREATE TABLE IF NOT EXISTS user_group_members (
    id TEXT PRIMARY KEY NOT NULL,
    group_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    added_by TEXT NOT NULL,
    added_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (group_id) REFERENCES user_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (added_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_group_members_unique ON user_group_members(group_id, user_id);
CREATE INDEX IF NOT EXISTS idx_group_members_user ON user_group_members(user_id);
CREATE INDEX IF NOT EXISTS idx_group_members_group ON user_group_members(group_id);

-- Group Permissions (folder/file access for groups)
CREATE TABLE IF NOT EXISTS group_permissions (
    id TEXT PRIMARY KEY NOT NULL,
    group_id TEXT NOT NULL,
    resource_type TEXT NOT NULL, -- 'file' or 'folder'
    resource_id TEXT NOT NULL,
    permission_level TEXT NOT NULL, -- 'read', 'write', 'admin'
    granted_by TEXT NOT NULL,
    granted_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT,
    FOREIGN KEY (group_id) REFERENCES user_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (granted_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_group_perms_group ON group_permissions(group_id);
CREATE INDEX IF NOT EXISTS idx_group_perms_resource ON group_permissions(resource_type, resource_id);

-- Enhanced Roles (extending existing roles table if not exists)
CREATE TABLE IF NOT EXISTS user_system_roles (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT NOT NULL, -- 'admin', 'moderator', 'user', 'guest', 'readonly'
    assigned_by TEXT,
    assigned_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_user_system_roles_unique ON user_system_roles(user_id, role);
CREATE INDEX IF NOT EXISTS idx_user_system_roles_user ON user_system_roles(user_id);

-- User Suspension/Ban
CREATE TABLE IF NOT EXISTS user_suspensions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    reason TEXT NOT NULL,
    suspended_by TEXT NOT NULL,
    suspended_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT, -- NULL = permanent
    is_active INTEGER NOT NULL DEFAULT 1,
    lifted_at TEXT,
    lifted_by TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (suspended_by) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (lifted_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_user_suspensions_user ON user_suspensions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_suspensions_active ON user_suspensions(is_active);

-- Session Management (active sessions per user)
CREATE TABLE IF NOT EXISTS user_sessions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    session_token TEXT UNIQUE NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL,
    last_activity TEXT NOT NULL DEFAULT (datetime('now')),
    is_active INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_sessions_user ON user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_user_sessions_active ON user_sessions(is_active);
CREATE INDEX IF NOT EXISTS idx_user_sessions_expires ON user_sessions(expires_at);

-- Add user status and profile fields to users table
ALTER TABLE users ADD COLUMN status TEXT DEFAULT 'active'; -- 'active', 'suspended', 'pending'
ALTER TABLE users ADD COLUMN bio TEXT;
ALTER TABLE users ADD COLUMN location TEXT;
ALTER TABLE users ADD COLUMN website TEXT;

CREATE INDEX IF NOT EXISTS idx_users_status ON users(status);
