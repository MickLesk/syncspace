-- Migration 036: Role-Based Access Control (RBAC) System
-- Implements roles, permissions, and user role assignments

-- Roles table
CREATE TABLE IF NOT EXISTS roles (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    display_name TEXT NOT NULL,
    description TEXT,
    permissions TEXT NOT NULL, -- JSON array of permission strings
    is_system INTEGER NOT NULL DEFAULT 0, -- System roles cannot be deleted
    is_default INTEGER NOT NULL DEFAULT 0, -- Assigned to new users
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- User roles junction table
CREATE TABLE IF NOT EXISTS user_roles (
    user_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    granted_by TEXT, -- User who granted this role
    granted_at TEXT NOT NULL,
    expires_at TEXT, -- Optional expiration
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (granted_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Permission audit log
CREATE TABLE IF NOT EXISTS permission_audit (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    action TEXT NOT NULL, -- grant_role, revoke_role, create_role, update_role, delete_role
    target_user_id TEXT, -- User affected by role change
    role_id TEXT,
    role_name TEXT,
    permissions_before TEXT, -- JSON snapshot
    permissions_after TEXT, -- JSON snapshot
    performed_by TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (performed_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_user_roles_user ON user_roles(user_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_role ON user_roles(role_id);
CREATE INDEX IF NOT EXISTS idx_permission_audit_user ON permission_audit(user_id);
CREATE INDEX IF NOT EXISTS idx_permission_audit_action ON permission_audit(action);
CREATE INDEX IF NOT EXISTS idx_permission_audit_created ON permission_audit(created_at);

-- Insert system roles with comprehensive permissions
INSERT OR IGNORE INTO roles (id, name, display_name, description, permissions, is_system, is_default, created_at, updated_at) VALUES
    (
        'role-super-admin',
        'super_admin',
        'Super Administrator',
        'Full system access with all permissions',
        '["system.admin", "system.config", "user.manage", "user.create", "user.delete", "role.manage", "role.create", "role.delete", "file.read", "file.write", "file.delete", "file.share", "file.download", "share.manage", "share.create", "share.delete", "backup.manage", "backup.create", "backup.restore", "audit.view", "settings.manage"]',
        1,
        0,
        datetime('now'),
        datetime('now')
    ),
    (
        'role-admin',
        'admin',
        'Administrator',
        'Administrative access with user and file management',
        '["user.manage", "user.create", "role.assign", "file.read", "file.write", "file.delete", "file.share", "file.download", "share.manage", "share.create", "backup.view", "audit.view", "settings.view"]',
        1,
        0,
        datetime('now'),
        datetime('now')
    ),
    (
        'role-moderator',
        'moderator',
        'Moderator',
        'Moderate content and manage shares',
        '["file.read", "file.write", "file.delete", "file.share", "file.download", "share.manage", "share.create", "user.view", "audit.view"]',
        1,
        0,
        datetime('now'),
        datetime('now')
    ),
    (
        'role-user',
        'user',
        'Standard User',
        'Standard file access and sharing',
        '["file.read", "file.write", "file.delete", "file.share", "file.download", "share.create"]',
        1,
        1,
        datetime('now'),
        datetime('now')
    ),
    (
        'role-viewer',
        'viewer',
        'Viewer',
        'Read-only access to files',
        '["file.read", "file.download"]',
        1,
        0,
        datetime('now'),
        datetime('now')
    ),
    (
        'role-guest',
        'guest',
        'Guest',
        'Limited read-only access',
        '["file.read"]',
        1,
        0,
        datetime('now'),
        datetime('now')
    );

-- Assign default role to existing users without roles
INSERT OR IGNORE INTO user_roles (user_id, role_id, granted_by, granted_at)
SELECT 
    u.id,
    'role-user',
    u.id,
    datetime('now')
FROM users u
WHERE NOT EXISTS (
    SELECT 1 FROM user_roles ur WHERE ur.user_id = u.id
);

-- Create admin role for first user (typically the setup admin)
INSERT OR IGNORE INTO user_roles (user_id, role_id, granted_by, granted_at)
SELECT 
    u.id,
    'role-super-admin',
    u.id,
    datetime('now')
FROM users u
ORDER BY u.created_at ASC
LIMIT 1;
