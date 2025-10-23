/**
 * Comments & Tags Store
 * Manages file comments and tags with API persistence
 */

import { writable } from 'svelte/store';
import { comments as commentsAPI, tags as tagsAPI } from '../lib/api.js';

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
 * Comments store
 * Structure: { "path/to/file.txt": [{ id, author_id, text, created_at }, ...] }
 */
function createCommentsStore() {
  const { subscribe, set, update } = writable({});

  return {
    subscribe,
    
    /**
     * Load comments for a file from API
     */
    loadForFile: async (filePath) => {
      try {
        const apiComments = await commentsAPI.list(filePath);
        update(comments => ({
          ...comments,
          [filePath]: apiComments || []
        }));
      } catch (e) {
        console.error('Failed to load comments:', e);
      }
    },

    /**
     * Add a comment to a file
     */
    addComment: async (filePath, user, text) => {
      try {
        const comment = await commentsAPI.create({
          item_type: 'file',
          item_id: filePath,
          file_path: filePath,
          text
        });

        update(comments => {
          const fileComments = comments[filePath] || [];
          return {
            ...comments,
            [filePath]: [...fileComments, comment]
          };
        });

        return comment.id;
      } catch (e) {
        console.error('Failed to add comment:', e);
        throw e;
      }
    },

    /**
     * Edit a comment (not implemented in backend yet)
     */
    editComment: (filePath, commentId, newText) => {
      console.warn('Edit comment not yet implemented in backend');
      // Would need PUT /api/comments/:id endpoint
    },

    /**
     * Delete a comment
     */
    deleteComment: async (filePath, commentId) => {
      try {
        await commentsAPI.delete(commentId);

        update(comments => {
          if (!comments[filePath]) return comments;

          return {
            ...comments,
            [filePath]: comments[filePath].filter(c => c.id !== commentId)
          };
        });
      } catch (e) {
        console.error('Failed to delete comment:', e);
        throw e;
      }
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
     * Clear all comments (local only)
     */
    clear: () => {
      set({});
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
 * Structure: { userTags: [{ id, name, color }], fileTags: { "path": [tag_ids] } }
 */
function createTagsStore() {
  const { subscribe, set, update } = writable({ userTags: [], fileTags: {} });

  return {
    subscribe,

    /**
     * Load all user tags from API
     */
    loadAll: async () => {
      try {
        const userTags = await tagsAPI.list();
        update(state => ({
          ...state,
          userTags: userTags || []
        }));
      } catch (e) {
        console.error('Failed to load tags:', e);
      }
    },

    /**
     * Add a tag to a file
     */
    addTag: async (filePath, tagName) => {
      try {
        // First, check if tag exists or create it
        let tag;
        let currentState = { userTags: [], fileTags: {} };
        
        update(s => {
          currentState = s;
          return s;
        });

        tag = currentState.userTags.find(t => t.name === tagName);
        
        if (!tag) {
          // Create new tag
          const colorIndex = tagName.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0) % TAG_COLORS.length;
          tag = await tagsAPI.create({
            name: tagName,
            color: TAG_COLORS[colorIndex]
          });

          update(s => ({
            ...s,
            userTags: [...s.userTags, tag]
          }));
        }

        // Tag the file
        await tagsAPI.tagFile({
          item_type: 'file',
          file_id: filePath,
          file_path: filePath,
          tag_ids: [tag.id]
        });

        update(s => ({
          ...s,
          fileTags: {
            ...s.fileTags,
            [filePath]: [...(s.fileTags[filePath] || []), tag.id]
          }
        }));

      } catch (e) {
        console.error('Failed to add tag:', e);
        throw e;
      }
    },

    /**
     * Remove a tag from a file
     */
    removeTag: async (filePath, tagName) => {
      try {
        let currentState = { userTags: [], fileTags: {} };
        
        update(s => {
          currentState = s;
          return s;
        });

        const tag = currentState.userTags.find(t => t.name === tagName);
        if (!tag) return;

        // Find file_tag_id (we don't have it, so this is a limitation)
        // For now, we can't implement this without getting file_tags from backend
        console.warn('Remove tag requires file_tag_id from backend - not fully implemented');

        update(s => ({
          ...s,
          fileTags: {
            ...s.fileTags,
            [filePath]: (s.fileTags[filePath] || []).filter(id => id !== tag.id)
          }
        }));

      } catch (e) {
        console.error('Failed to remove tag:', e);
        throw e;
      }
    },

    /**
     * Get all tags for a file
     */
    getTags: (filePath, state) => {
      const tagIds = state.fileTags[filePath] || [];
      return state.userTags.filter(t => tagIds.includes(t.id));
    },

    /**
     * Get all unique tag names across all files
     */
    getAllTagNames: (state) => {
      return state.userTags.map(t => t.name).sort();
    },

    /**
     * Find files with a specific tag
     */
    findByTag: (tagName, state) => {
      const tag = state.userTags.find(t => t.name === tagName);
      if (!tag) return [];

      const files = [];
      Object.entries(state.fileTags).forEach(([filePath, tagIds]) => {
        if (tagIds.includes(tag.id)) {
          files.push(filePath);
        }
      });
      return files;
    },

    /**
     * Clear all tags (local only)
     */
    clear: () => {
      set({ userTags: [], fileTags: {} });
    }
  };
}

export const comments = createCommentsStore();
export const tags = createTagsStore();
export { TAG_COLORS };
