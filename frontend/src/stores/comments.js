/**
 * Comments & Tags Store
 * Manages file comments and tags with localStorage persistence
 */

import { writable } from 'svelte/store';

const COMMENTS_KEY = 'syncspace_file_comments';
const TAGS_KEY = 'syncspace_file_tags';
const TAG_COLORS = [
  '#e57373', // red
  '#f06292', // pink
  '#ba68c8', // purple
  '#9575cd', // deep purple
  '#7986cb', // indigo
  '#64b5f6', // blue
  '#4fc3f7', // light blue
  '#4dd0e1', // cyan
  '#4db6ac', // teal
  '#81c784', // green
  '#aed581', // light green
  '#fff176', // yellow
  '#ffd54f', // amber
  '#ffb74d', // orange
  '#ff8a65', // deep orange
];

/**
 * Load comments from localStorage
 */
function loadComments() {
  try {
    const stored = localStorage.getItem(COMMENTS_KEY);
    return stored ? JSON.parse(stored) : {};
  } catch (e) {
    console.error('Failed to load comments:', e);
    return {};
  }
}

/**
 * Load tags from localStorage
 */
function loadTags() {
  try {
    const stored = localStorage.getItem(TAGS_KEY);
    return stored ? JSON.parse(stored) : {};
  } catch (e) {
    console.error('Failed to load tags:', e);
    return {};
  }
}

/**
 * Comments store
 * Structure: { "path/to/file.txt": [{ id, user, text, timestamp }, ...] }
 */
function createCommentsStore() {
  const { subscribe, set, update } = writable(loadComments());

  return {
    subscribe,
    
    /**
     * Add a comment to a file
     */
    addComment: (filePath, user, text) => {
      const comment = {
        id: `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
        user,
        text,
        timestamp: Date.now(),
      };

      update(comments => {
        const fileComments = comments[filePath] || [];
        const updated = {
          ...comments,
          [filePath]: [...fileComments, comment]
        };
        localStorage.setItem(COMMENTS_KEY, JSON.stringify(updated));
        return updated;
      });

      return comment.id;
    },

    /**
     * Edit a comment
     */
    editComment: (filePath, commentId, newText) => {
      update(comments => {
        if (!comments[filePath]) return comments;

        const updated = {
          ...comments,
          [filePath]: comments[filePath].map(c =>
            c.id === commentId ? { ...c, text: newText, edited: Date.now() } : c
          )
        };
        localStorage.setItem(COMMENTS_KEY, JSON.stringify(updated));
        return updated;
      });
    },

    /**
     * Delete a comment
     */
    deleteComment: (filePath, commentId) => {
      update(comments => {
        if (!comments[filePath]) return comments;

        const updated = {
          ...comments,
          [filePath]: comments[filePath].filter(c => c.id !== commentId)
        };

        // Remove empty arrays
        if (updated[filePath].length === 0) {
          delete updated[filePath];
        }

        localStorage.setItem(COMMENTS_KEY, JSON.stringify(updated));
        return updated;
      });
    },

    /**
     * Get all comments for a file
     */
    getComments: (filePath) => {
      let result = [];
      update(comments => {
        result = comments[filePath] || [];
        return comments;
      });
      return result;
    },

    /**
     * Clear all comments
     */
    clear: () => {
      set({});
      localStorage.removeItem(COMMENTS_KEY);
    },

    /**
     * Get comment count for a file
     */
    getCount: (filePath, comments) => {
      return comments[filePath]?.length || 0;
    }
  };
}

/**
 * Tags store
 * Structure: { "path/to/file.txt": [{ name, color }, ...] }
 */
function createTagsStore() {
  const { subscribe, set, update } = writable(loadTags());

  return {
    subscribe,

    /**
     * Add a tag to a file
     */
    addTag: (filePath, tagName) => {
      update(tags => {
        const fileTags = tags[filePath] || [];
        
        // Don't add duplicate tags
        if (fileTags.some(t => t.name === tagName)) {
          return tags;
        }

        // Assign color based on tag name hash
        const colorIndex = tagName.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0) % TAG_COLORS.length;
        
        const newTag = {
          name: tagName,
          color: TAG_COLORS[colorIndex]
        };

        const updated = {
          ...tags,
          [filePath]: [...fileTags, newTag]
        };
        localStorage.setItem(TAGS_KEY, JSON.stringify(updated));
        return updated;
      });
    },

    /**
     * Remove a tag from a file
     */
    removeTag: (filePath, tagName) => {
      update(tags => {
        if (!tags[filePath]) return tags;

        const updated = {
          ...tags,
          [filePath]: tags[filePath].filter(t => t.name !== tagName)
        };

        // Remove empty arrays
        if (updated[filePath].length === 0) {
          delete updated[filePath];
        }

        localStorage.setItem(TAGS_KEY, JSON.stringify(updated));
        return updated;
      });
    },

    /**
     * Get all tags for a file
     */
    getTags: (filePath, tags) => {
      return tags[filePath] || [];
    },

    /**
     * Get all unique tag names across all files
     */
    getAllTagNames: (tags) => {
      const allTags = new Set();
      Object.values(tags).forEach(fileTags => {
        fileTags.forEach(tag => allTags.add(tag.name));
      });
      return Array.from(allTags).sort();
    },

    /**
     * Find files with a specific tag
     */
    findByTag: (tagName, tags) => {
      const files = [];
      Object.entries(tags).forEach(([filePath, fileTags]) => {
        if (fileTags.some(t => t.name === tagName)) {
          files.push(filePath);
        }
      });
      return files;
    },

    /**
     * Clear all tags
     */
    clear: () => {
      set({});
      localStorage.removeItem(TAGS_KEY);
    }
  };
}

export const comments = createCommentsStore();
export const tags = createTagsStore();
export { TAG_COLORS };
