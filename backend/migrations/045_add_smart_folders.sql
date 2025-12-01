-- Add Smart Folders feature
-- Supports dynamic folder creation based on file rules

CREATE TABLE IF NOT EXISTS smart_folder_rules (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  
  -- JSON array of rule conditions
  -- Example: [{"field":"file_type","operator":"equals","value":"pdf"}]
  conditions TEXT NOT NULL DEFAULT '[]',
  
  -- Logic for combining conditions: "AND" or "OR"
  logic TEXT NOT NULL DEFAULT 'AND' CHECK (logic IN ('AND', 'OR')),
  
  -- UI customization
  icon TEXT,  -- Bootstrap icon name (e.g., "bi-file-pdf", "bi-image")
  color TEXT, -- Hex color (e.g., "#FF5733")
  
  -- Sorting preferences for files in this smart folder
  sort_by TEXT NOT NULL DEFAULT 'name' CHECK (sort_by IN ('name', 'modified', 'size', 'type')),
  sort_order TEXT NOT NULL DEFAULT 'asc' CHECK (sort_order IN ('asc', 'desc')),
  
  -- Activity
  is_active BOOLEAN NOT NULL DEFAULT 1,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  
  UNIQUE(user_id, name)
);

-- Index for quick lookups by user
CREATE INDEX IF NOT EXISTS idx_smart_folder_rules_user_id ON smart_folder_rules(user_id);
CREATE INDEX IF NOT EXISTS idx_smart_folder_rules_active ON smart_folder_rules(user_id, is_active);

-- Track when smart folder rules are accessed/used
CREATE TABLE IF NOT EXISTS smart_folder_activity (
  id TEXT PRIMARY KEY,
  rule_id TEXT NOT NULL REFERENCES smart_folder_rules(id) ON DELETE CASCADE,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  action TEXT NOT NULL, -- "view", "apply", "create", "update", "delete"
  matched_file_count INTEGER,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_smart_folder_activity_rule_id ON smart_folder_activity(rule_id);
CREATE INDEX IF NOT EXISTS idx_smart_folder_activity_user_id ON smart_folder_activity(user_id);
CREATE INDEX IF NOT EXISTS idx_smart_folder_activity_created_at ON smart_folder_activity(created_at);

-- Cache for matching results (optional - for performance)
-- This can be populated by a background job
CREATE TABLE IF NOT EXISTS smart_folder_matches (
  id TEXT PRIMARY KEY,
  rule_id TEXT NOT NULL REFERENCES smart_folder_rules(id) ON DELETE CASCADE,
  file_path TEXT NOT NULL,
  file_size INTEGER,
  file_type TEXT,
  matched_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  
  UNIQUE(rule_id, file_path)
);

CREATE INDEX IF NOT EXISTS idx_smart_folder_matches_rule_id ON smart_folder_matches(rule_id);
CREATE INDEX IF NOT EXISTS idx_smart_folder_matches_file_path ON smart_folder_matches(file_path);
