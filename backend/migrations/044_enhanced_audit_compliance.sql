-- Enhanced Audit & Compliance System Migration
-- Adds comprehensive audit logging, compliance reports, and retention policies

-- Enhanced audit_logs table with more fields
CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    username TEXT,
    action TEXT NOT NULL,
    action_category TEXT DEFAULT 'general', -- 'auth', 'file', 'admin', 'sharing', 'security'
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    resource_name TEXT,
    ip_address TEXT,
    user_agent TEXT,
    session_id TEXT,
    request_method TEXT,
    request_path TEXT,
    response_status INTEGER,
    metadata TEXT, -- JSON
    old_value TEXT, -- JSON - for tracking changes
    new_value TEXT, -- JSON - for tracking changes
    severity TEXT DEFAULT 'info', -- 'debug', 'info', 'warning', 'error', 'critical'
    is_sensitive BOOLEAN DEFAULT 0,
    is_compliance_relevant BOOLEAN DEFAULT 0,
    retention_until TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Indexes for faster queries
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action_category ON audit_logs(action_category);
CREATE INDEX IF NOT EXISTS idx_audit_logs_resource_type ON audit_logs(resource_type);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_audit_logs_severity ON audit_logs(severity);
CREATE INDEX IF NOT EXISTS idx_audit_logs_is_compliance ON audit_logs(is_compliance_relevant);

-- Data retention policies table
CREATE TABLE IF NOT EXISTS data_retention_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    resource_type TEXT NOT NULL,
    retention_days INTEGER NOT NULL DEFAULT 365,
    auto_delete BOOLEAN DEFAULT 0,
    archive_before_delete BOOLEAN DEFAULT 1,
    notify_before_delete BOOLEAN DEFAULT 1,
    notify_days_before INTEGER DEFAULT 7,
    is_active BOOLEAN DEFAULT 1,
    created_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT,
    last_applied_at TEXT,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Compliance reports table
CREATE TABLE IF NOT EXISTS compliance_reports (
    id TEXT PRIMARY KEY,
    report_type TEXT NOT NULL, -- 'gdpr', 'hipaa', 'sox', 'iso27001', 'pci_dss', 'custom'
    report_name TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'pending', -- 'pending', 'generating', 'completed', 'failed'
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    filters TEXT, -- JSON - user_ids, action_categories, resource_types, etc.
    total_records INTEGER DEFAULT 0,
    file_path TEXT, -- Path to generated report file
    file_format TEXT DEFAULT 'json', -- 'json', 'csv', 'pdf', 'xlsx'
    file_size_bytes INTEGER,
    checksum TEXT,
    generated_by TEXT NOT NULL,
    generated_at TEXT,
    expires_at TEXT,
    download_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (generated_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_compliance_reports_type ON compliance_reports(report_type);
CREATE INDEX IF NOT EXISTS idx_compliance_reports_status ON compliance_reports(status);
CREATE INDEX IF NOT EXISTS idx_compliance_reports_generated_by ON compliance_reports(generated_by);

-- Audit log archives (for long-term storage)
CREATE TABLE IF NOT EXISTS audit_log_archives (
    id TEXT PRIMARY KEY,
    archive_name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    record_count INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    file_size_bytes INTEGER,
    checksum TEXT,
    compression_type TEXT DEFAULT 'gzip',
    is_encrypted BOOLEAN DEFAULT 0,
    encryption_key_id TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Compliance templates for quick report generation
CREATE TABLE IF NOT EXISTS compliance_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    report_type TEXT NOT NULL,
    description TEXT,
    filters TEXT, -- JSON - predefined filters
    columns TEXT, -- JSON - columns to include
    is_default BOOLEAN DEFAULT 0,
    is_public BOOLEAN DEFAULT 1,
    created_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Insert default retention policies
INSERT OR IGNORE INTO data_retention_policies (id, name, description, resource_type, retention_days, auto_delete, archive_before_delete)
VALUES 
    ('pol-audit-logs', 'Audit Logs', 'Retention for audit log entries', 'audit_logs', 365, 1, 1),
    ('pol-trash', 'Trash Items', 'Retention for deleted files in trash', 'trash', 30, 1, 0),
    ('pol-sessions', 'User Sessions', 'Retention for expired sessions', 'sessions', 90, 1, 0),
    ('pol-reports', 'Compliance Reports', 'Retention for generated reports', 'compliance_reports', 180, 1, 1),
    ('pol-versions', 'File Versions', 'Retention for old file versions', 'file_versions', 365, 0, 1);

-- Insert default compliance templates
INSERT OR IGNORE INTO compliance_templates (id, name, report_type, description, filters, columns, is_default)
VALUES 
    ('tpl-gdpr-access', 'GDPR Data Access Report', 'gdpr', 'User data access and export for GDPR compliance', 
     '{"action_categories": ["file", "sharing"], "include_sensitive": true}',
     '["timestamp", "user", "action", "resource", "ip_address"]', 1),
    ('tpl-gdpr-deletion', 'GDPR Data Deletion Report', 'gdpr', 'Data deletion tracking for GDPR right to erasure',
     '{"actions": ["delete", "permanent_delete"], "include_sensitive": true}',
     '["timestamp", "user", "resource", "resource_name", "metadata"]', 1),
    ('tpl-security-audit', 'Security Audit Trail', 'custom', 'Security-related events including login attempts and permission changes',
     '{"action_categories": ["auth", "security", "admin"], "severity": ["warning", "error", "critical"]}',
     '["timestamp", "user", "action", "ip_address", "user_agent", "metadata"]', 1),
    ('tpl-admin-actions', 'Administrative Actions Report', 'custom', 'All administrative operations',
     '{"action_categories": ["admin"]}',
     '["timestamp", "user", "action", "resource", "old_value", "new_value"]', 1),
    ('tpl-file-activity', 'File Activity Report', 'custom', 'All file operations including uploads, downloads, and modifications',
     '{"action_categories": ["file"]}',
     '["timestamp", "user", "action", "resource_name", "metadata"]', 1);

-- View for recent critical events
CREATE VIEW IF NOT EXISTS v_critical_audit_events AS
SELECT 
    al.id,
    al.user_id,
    al.username,
    al.action,
    al.action_category,
    al.resource_type,
    al.resource_name,
    al.ip_address,
    al.severity,
    al.metadata,
    al.created_at
FROM audit_logs al
WHERE al.severity IN ('error', 'critical')
ORDER BY al.created_at DESC
LIMIT 100;

-- View for compliance summary
CREATE VIEW IF NOT EXISTS v_compliance_summary AS
SELECT 
    action_category,
    COUNT(*) as total_events,
    SUM(CASE WHEN is_compliance_relevant = 1 THEN 1 ELSE 0 END) as compliance_events,
    SUM(CASE WHEN severity = 'critical' THEN 1 ELSE 0 END) as critical_events,
    SUM(CASE WHEN severity = 'error' THEN 1 ELSE 0 END) as error_events,
    SUM(CASE WHEN severity = 'warning' THEN 1 ELSE 0 END) as warning_events,
    date(created_at) as log_date
FROM audit_logs
GROUP BY action_category, date(created_at)
ORDER BY log_date DESC;
