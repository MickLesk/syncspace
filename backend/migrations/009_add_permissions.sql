-- Migration: Add advanced permissions system
-- Created: 2025-10-24

-- Roles table
CREATE TABLE IF NOT EXISTS roles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    permissions TEXT NOT NULL, -- JSON array of permission strings
    is_system INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

-- User roles table
CREATE TABLE IF NOT EXISTS user_roles (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT 'global', -- 'global', 'folder:{id}', 'file:{id}'
    granted_by TEXT NOT NULL,
    granted_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (granted_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_roles_user ON user_roles(user_id);
CREATE INDEX idx_user_roles_role ON user_roles(role_id);

-- Access Control List (ACL) table
CREATE TABLE IF NOT EXISTS access_control (
    id TEXT PRIMARY KEY,
    resource_type TEXT NOT NULL CHECK(resource_type IN ('file', 'folder')),
    resource_id TEXT NOT NULL,
    principal_type TEXT NOT NULL CHECK(principal_type IN ('user', 'role', 'group')),
    principal_id TEXT NOT NULL,
    permissions TEXT NOT NULL, -- JSON array: ["read", "write", "delete", "share"]
    inherited INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_acl_resource ON access_control(resource_type, resource_id);
CREATE INDEX idx_acl_principal ON access_control(principal_type, principal_id);
