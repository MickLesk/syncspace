-- 037_workflow_automation.sql
-- Add workflow automation with IFTTT-style rules

-- Workflow rules table (defines automation rules)
CREATE TABLE IF NOT EXISTS workflow_rules (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    description TEXT,
    
    -- Trigger configuration
    trigger_type TEXT NOT NULL CHECK (trigger_type IN (
        'file_upload', 'file_delete', 'file_move', 'file_rename',
        'file_share', 'file_tag', 'file_version', 'scheduled',
        'webhook', 'manual'
    )),
    trigger_config TEXT NOT NULL DEFAULT '{}', -- JSON: {path_pattern, file_types, schedule_cron, etc.}
    
    -- Condition configuration (optional filters)
    condition_config TEXT NOT NULL DEFAULT '{}', -- JSON: {file_size_min, file_size_max, mime_types, tags, etc.}
    
    -- Action configuration
    action_type TEXT NOT NULL CHECK (action_type IN (
        'convert_file', 'compress_file', 'send_notification', 
        'add_tag', 'move_file', 'copy_file', 'delete_file',
        'send_webhook', 'send_email', 'run_script'
    )),
    action_config TEXT NOT NULL DEFAULT '{}', -- JSON: {target_format, target_path, webhook_url, email_to, etc.}
    
    -- Rule state
    is_active BOOLEAN NOT NULL DEFAULT 1,
    priority INTEGER NOT NULL DEFAULT 100, -- Higher = earlier execution
    
    -- Metadata
    created_by TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

-- Create index for active rules lookup
CREATE INDEX idx_workflow_rules_active ON workflow_rules(is_active, priority DESC);
CREATE INDEX idx_workflow_rules_trigger ON workflow_rules(trigger_type, is_active);

-- Workflow executions table (logs all workflow runs)
CREATE TABLE IF NOT EXISTS workflow_executions (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    rule_id TEXT NOT NULL,
    
    -- Execution context
    triggered_by TEXT, -- user_id if manual/user action
    trigger_event TEXT NOT NULL, -- JSON: {event_type, file_path, timestamp, etc.}
    
    -- Condition evaluation
    condition_met BOOLEAN NOT NULL,
    condition_result TEXT, -- JSON: details of condition evaluation
    
    -- Action execution
    action_executed BOOLEAN NOT NULL,
    action_result TEXT, -- JSON: details of action execution
    
    -- Execution metadata
    status TEXT NOT NULL CHECK (status IN ('success', 'failed', 'skipped', 'cancelled')),
    error_message TEXT,
    execution_time_ms INTEGER, -- Duration in milliseconds
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (rule_id) REFERENCES workflow_rules(id) ON DELETE CASCADE,
    FOREIGN KEY (triggered_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Create index for execution history queries
CREATE INDEX idx_workflow_executions_rule ON workflow_executions(rule_id, created_at DESC);
CREATE INDEX idx_workflow_executions_status ON workflow_executions(status, created_at DESC);
CREATE INDEX idx_workflow_executions_triggered_by ON workflow_executions(triggered_by, created_at DESC);

-- Insert 3 default workflow templates for common use cases
-- Only insert if users table has entries; otherwise skip with INSERT OR IGNORE

-- 1. Auto-compress images on upload
INSERT OR IGNORE INTO workflow_rules (id, name, display_name, description, trigger_type, trigger_config, condition_config, action_type, action_config, is_active, priority, created_by)
SELECT 
    'default_compress_images',
    'auto_compress_images',
    'Auto Compress Images',
    'Automatically compress images larger than 5MB when uploaded',
    'file_upload',
    '{"path_pattern": "**/*", "file_types": ["jpg", "jpeg", "png", "gif", "webp"]}',
    '{"file_size_min": 5242880}',
    'compress_file',
    '{"quality": 85, "keep_original": false}',
    0,
    100,
    id
FROM users ORDER BY created_at ASC LIMIT 1;

-- 2. Move videos to media folder on upload
INSERT OR IGNORE INTO workflow_rules (id, name, display_name, description, trigger_type, trigger_config, condition_config, action_type, action_config, is_active, priority, created_by)
SELECT 
    'default_organize_videos',
    'organize_videos',
    'Organize Videos',
    'Automatically move uploaded videos to /Media/Videos folder',
    'file_upload',
    '{"path_pattern": "**/*", "file_types": ["mp4", "avi", "mkv", "mov", "webm"]}',
    '{}',
    'move_file',
    '{"target_path": "/Media/Videos", "create_folders": true}',
    0,
    90,
    id
FROM users ORDER BY created_at ASC LIMIT 1;

-- 3. Send notification when large file shared
INSERT OR IGNORE INTO workflow_rules (id, name, display_name, description, trigger_type, trigger_config, condition_config, action_type, action_config, is_active, priority, created_by)
SELECT 
    'default_notify_large_share',
    'notify_large_share',
    'Notify on Large Share',
    'Send notification when files larger than 100MB are shared',
    'file_share',
    '{"path_pattern": "**/*"}',
    '{"file_size_min": 104857600}',
    'send_notification',
    '{"notification_type": "info", "title": "Large File Shared", "message": "A large file has been shared with you"}',
    0,
    80,
    id
FROM users ORDER BY created_at ASC LIMIT 1;
