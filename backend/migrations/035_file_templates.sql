-- Migration 035: File Templates System
-- Enables users to create and manage file templates with variable substitution

-- File templates table
CREATE TABLE IF NOT EXISTS file_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL DEFAULT 'general',
    content TEXT NOT NULL,
    file_extension TEXT,
    variables TEXT, -- JSON array of variable names
    is_public INTEGER NOT NULL DEFAULT 0,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

-- Template categories table (for organization)
CREATE TABLE IF NOT EXISTS template_categories (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    icon TEXT,
    created_at TEXT NOT NULL
);

-- User favorite templates (for quick access)
CREATE TABLE IF NOT EXISTS user_template_favorites (
    user_id TEXT NOT NULL,
    template_id TEXT NOT NULL,
    added_at TEXT NOT NULL,
    PRIMARY KEY (user_id, template_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (template_id) REFERENCES file_templates(id) ON DELETE CASCADE
);

-- Template usage statistics
CREATE TABLE IF NOT EXISTS template_usage (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    used_at TEXT NOT NULL,
    FOREIGN KEY (template_id) REFERENCES file_templates(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_templates_category ON file_templates(category);
CREATE INDEX IF NOT EXISTS idx_templates_created_by ON file_templates(created_by);
CREATE INDEX IF NOT EXISTS idx_templates_public ON file_templates(is_public);
CREATE INDEX IF NOT EXISTS idx_template_usage_template ON template_usage(template_id);
CREATE INDEX IF NOT EXISTS idx_template_usage_user ON template_usage(user_id);

-- Insert default categories
INSERT OR IGNORE INTO template_categories (id, name, description, icon, created_at) VALUES
    ('cat-documents', 'Documents', 'Document templates (README, reports, etc.)', 'file-text', datetime('now')),
    ('cat-code', 'Code', 'Programming templates (Python, JavaScript, etc.)', 'code-slash', datetime('now')),
    ('cat-config', 'Configuration', 'Config file templates (JSON, YAML, etc.)', 'gear', datetime('now')),
    ('cat-data', 'Data', 'Data file templates (CSV, SQL, etc.)', 'table', datetime('now')),
    ('cat-web', 'Web', 'Web development templates (HTML, CSS, etc.)', 'globe', datetime('now')),
    ('cat-general', 'General', 'General purpose templates', 'file-earmark', datetime('now'));

-- Insert some default templates
INSERT OR IGNORE INTO file_templates (id, name, description, category, content, file_extension, variables, is_public, created_by, created_at, updated_at) VALUES
    (
        'tpl-readme',
        'README.md',
        'Standard README template for projects',
        'cat-documents',
        '# {{project_name}}

{{description}}

## Installation

```bash
# Add installation instructions
```

## Usage

```bash
# Add usage examples
```

## Features

- Feature 1
- Feature 2
- Feature 3

## License

{{license}}

## Author

{{author}} - {{email}}',
        'md',
        '["project_name", "description", "license", "author", "email"]',
        1,
        '00000000-0000-0000-0000-000000000000',
        datetime('now'),
        datetime('now')
    ),
    (
        'tpl-python',
        'Python Script',
        'Basic Python script template',
        'cat-code',
        '#!/usr/bin/env python3
"""
{{description}}

Author: {{author}}
Date: {{date}}
"""

def main():
    """Main function."""
    print("Hello from {{project_name}}!")

if __name__ == "__main__":
    main()',
        'py',
        '["description", "author", "date", "project_name"]',
        1,
        '00000000-0000-0000-0000-000000000000',
        datetime('now'),
        datetime('now')
    ),
    (
        'tpl-config-json',
        'JSON Config',
        'Configuration file template',
        'cat-config',
        '{
  "name": "{{project_name}}",
  "version": "1.0.0",
  "author": "{{author}}",
  "description": "{{description}}",
  "settings": {
    "option1": true,
    "option2": false
  }
}',
        'json',
        '["project_name", "author", "description"]',
        1,
        '00000000-0000-0000-0000-000000000000',
        datetime('now'),
        datetime('now')
    );
