-- Migration 029: Add Comment Reactions Support
-- Version: Enhanced Comments with Emoji Reactions

-- ==================== COMMENT REACTIONS ====================
CREATE TABLE IF NOT EXISTS comment_reactions (
    id TEXT PRIMARY KEY NOT NULL,
    
    -- Reference to commented item
    comment_id TEXT NOT NULL,
    
    -- Reaction data
    emoji TEXT NOT NULL,
    
    -- User who reacted
    user_id TEXT NOT NULL,
    
    -- Timestamps
    created_at TEXT NOT NULL,
    
    FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    
    -- Prevent duplicate reactions: one emoji per user per comment
    UNIQUE(comment_id, emoji, user_id)
);

CREATE INDEX IF NOT EXISTS idx_comment_reactions_comment ON comment_reactions(comment_id);
CREATE INDEX IF NOT EXISTS idx_comment_reactions_user ON comment_reactions(user_id);
CREATE INDEX IF NOT EXISTS idx_comment_reactions_emoji ON comment_reactions(emoji);
CREATE INDEX IF NOT EXISTS idx_comment_reactions_created ON comment_reactions(created_at);
