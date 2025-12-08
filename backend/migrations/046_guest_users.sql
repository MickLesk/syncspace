-- Migration: Guest/External User Support
-- Description: Adds tables for temporary guest accounts, guest access links, and activity tracking

-- Guest Users Table
-- Stores temporary guest accounts with limited access
CREATE TABLE IF NOT EXISTS guest_users (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    email TEXT,
    created_by TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL,
    max_accesses INTEGER, -- NULL means unlimited
    access_count INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    last_accessed_at TEXT,
    notes TEXT,
    -- Permissions (as JSON array or flags)
    can_view INTEGER DEFAULT 1,
    can_download INTEGER DEFAULT 1,
    can_upload INTEGER DEFAULT 0,
    can_comment INTEGER DEFAULT 0
);

-- Guest Access Links Table
-- Links that allow guest access to specific files/folders
CREATE TABLE IF NOT EXISTS guest_access_links (
    id TEXT PRIMARY KEY,
    guest_id TEXT REFERENCES guest_users(id) ON DELETE CASCADE,
    token TEXT UNIQUE NOT NULL,
    file_path TEXT NOT NULL,
    access_type TEXT NOT NULL DEFAULT 'file' CHECK (access_type IN ('file', 'folder')),
    created_by TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT,
    password_hash TEXT, -- Optional password protection
    is_active INTEGER DEFAULT 1,
    access_count INTEGER DEFAULT 0,
    max_accesses INTEGER, -- NULL means unlimited
    last_accessed_at TEXT,
    -- Permissions for this specific link
    can_view INTEGER DEFAULT 1,
    can_download INTEGER DEFAULT 1,
    can_upload INTEGER DEFAULT 0
);

-- Guest Access Log Table
-- Tracks all guest access activity
CREATE TABLE IF NOT EXISTS guest_access_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guest_id TEXT REFERENCES guest_users(id) ON DELETE SET NULL,
    link_id TEXT REFERENCES guest_access_links(id) ON DELETE SET NULL,
    action TEXT NOT NULL CHECK (action IN ('view', 'download', 'upload', 'login', 'logout', 'link_access')),
    file_path TEXT,
    ip_address TEXT,
    user_agent TEXT,
    referrer TEXT,
    accessed_at TEXT NOT NULL DEFAULT (datetime('now')),
    metadata TEXT -- JSON for additional data
);

-- Guest Invitations Table
-- Email invitations sent to potential guests
CREATE TABLE IF NOT EXISTS guest_invitations (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL,
    invited_by TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL,
    token TEXT UNIQUE NOT NULL,
    message TEXT,
    is_accepted INTEGER DEFAULT 0,
    accepted_at TEXT,
    guest_id TEXT REFERENCES guest_users(id) ON DELETE SET NULL, -- Links to created guest account
    -- Pre-configured permissions for when guest accepts
    can_view INTEGER DEFAULT 1,
    can_download INTEGER DEFAULT 1,
    can_upload INTEGER DEFAULT 0,
    can_comment INTEGER DEFAULT 0
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_guest_users_created_by ON guest_users(created_by);
CREATE INDEX IF NOT EXISTS idx_guest_users_expires_at ON guest_users(expires_at);
CREATE INDEX IF NOT EXISTS idx_guest_users_is_active ON guest_users(is_active);

CREATE INDEX IF NOT EXISTS idx_guest_access_links_token ON guest_access_links(token);
CREATE INDEX IF NOT EXISTS idx_guest_access_links_guest_id ON guest_access_links(guest_id);
CREATE INDEX IF NOT EXISTS idx_guest_access_links_created_by ON guest_access_links(created_by);
CREATE INDEX IF NOT EXISTS idx_guest_access_links_expires_at ON guest_access_links(expires_at);

CREATE INDEX IF NOT EXISTS idx_guest_access_log_guest_id ON guest_access_log(guest_id);
CREATE INDEX IF NOT EXISTS idx_guest_access_log_link_id ON guest_access_log(link_id);
CREATE INDEX IF NOT EXISTS idx_guest_access_log_accessed_at ON guest_access_log(accessed_at);

CREATE INDEX IF NOT EXISTS idx_guest_invitations_email ON guest_invitations(email);
CREATE INDEX IF NOT EXISTS idx_guest_invitations_token ON guest_invitations(token);
CREATE INDEX IF NOT EXISTS idx_guest_invitations_invited_by ON guest_invitations(invited_by);
