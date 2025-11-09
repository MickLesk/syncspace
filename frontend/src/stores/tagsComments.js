/**
 * Tags & Comments Management Store
 * 
 * Backend-First Architecture:
 * - All tags and comments stored in database
 * - API endpoints: GET/POST/PUT/DELETE /api/files/{path}/tags, /api/files/{path}/comments
 * - Real-time sync via WebSocket
 * - Optimistic updates for instant UI feedback
 * 
 * Database Models:
 * - Tag: { id, fileId, name, color, createdBy, createdAt }
 * - Comment: { id, fileId, text, author, createdAt, updatedAt, replies[] }
 * 
 * Usage:
 * ```svelte
 * import { fileMetadata, addTag, removeTag, addComment } from './stores/tagsComments.js';
 * 
 * // Get all metadata for a file
 * {#each $fileMetadata.tags as tag}
 *   <Tag {tag} on:delete={() => removeTag(tag.id)} />
 * {/each}
 * ```
 */

import { writable, derived } from 'svelte/store';

// ============================================================================
// STATE STRUCTURE
// ============================================================================

const defaultFileMetadata = {
  tags: [],
  comments: [],
  stats: {
    tagCount: 0,
    commentCount: 0,
    lastCommentedAt: null,
  },
};

// ============================================================================
// INTERNAL STORES
// ============================================================================

// Current file being viewed: { filePath, tags[], comments[] }
const _currentFileMetadata = writable(defaultFileMetadata);
const _allFileMetadata = writable(new Map()); // filePath → metadata
const _loading = writable(false);
const _error = writable(null);

// Public exports
export const currentFileMetadata = _currentFileMetadata;
export const allFileMetadata = _allFileMetadata;
export const loading = _loading;
export const error = _error;

// Derived stores
export const tagCloud = derived(allFileMetadata, ($allMetadata) => {
  const tags = new Map();
  
  for (const metadata of $allMetadata.values()) {
    for (const tag of metadata.tags) {
      if (!tags.has(tag.name)) {
        tags.set(tag.name, { ...tag, count: 0 });
      }
      tags.get(tag.name).count++;
    }
  }
  
  return Array.from(tags.values()).sort((a, b) => b.count - a.count);
});

export const recentComments = derived(allFileMetadata, ($allMetadata) => {
  const comments = [];
  
  for (const metadata of $allMetadata.values()) {
    comments.push(...metadata.comments);
  }
  
  return comments
    .sort((a, b) => new Date(b.createdAt) - new Date(a.createdAt))
    .slice(0, 10);
});

// ============================================================================
// API COMMUNICATION
// ============================================================================

const API_BASE = 'http://localhost:8080/api';

function getAuthToken() {
  if (typeof window !== 'undefined') {
    return localStorage.getItem('authToken');
  }
  return null;
}

async function request(endpoint, options = {}) {
  const token = getAuthToken();
  const headers = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  _error.set(null);

  try {
    const response = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers,
    });

    if (response.status === 401) {
      localStorage.removeItem('authToken');
      if (typeof window !== 'undefined') {
        window.location.hash = '#/login';
      }
      throw new Error('Unauthorized');
    }

    if (!response.ok) {
      const error = await response.json().catch(() => ({
        message: response.statusText,
      }));
      throw new Error(error.message || `HTTP ${response.status}`);
    }

    return await response.json();
  } catch (err) {
    _error.set(err.message);
    throw err;
  }
}

// ============================================================================
// PUBLIC API - TAGS
// ============================================================================

/**
 * Load all tags for a file
 * @param {string} filePath - File path
 */
export async function loadTags(filePath) {
  _loading.set(true);

  try {
    const tags = await request(
      `/files/${encodeURIComponent(filePath)}/tags`
    );

    _currentFileMetadata.update((metadata) => ({
      ...metadata,
      tags: tags || [],
      stats: {
        ...metadata.stats,
        tagCount: tags?.length || 0,
      },
    }));

    _allFileMetadata.update((map) => {
      const current = map.get(filePath) || { ...defaultFileMetadata };
      map.set(filePath, { ...current, tags: tags || [] });
      return map;
    });
  } catch (err) {
    _error.set(`Failed to load tags: ${err.message}`);
  } finally {
    _loading.set(false);
  }
}

/**
 * Add a tag to a file
 * @param {string} filePath - File path
 * @param {object} tagData - { name, color }
 */
export async function addTag(filePath, tagData) {
  // Optimistic update
  const newTag = {
    id: Date.now().toString(),
    ...tagData,
    createdAt: new Date().toISOString(),
  };

  _currentFileMetadata.update((metadata) => ({
    ...metadata,
    tags: [...metadata.tags, newTag],
    stats: {
      ...metadata.stats,
      tagCount: metadata.tags.length + 1,
    },
  }));

  try {
    const created = await request(
      `/files/${encodeURIComponent(filePath)}/tags`,
      {
        method: 'POST',
        body: JSON.stringify(tagData),
      }
    );

    // Update with server response
    _currentFileMetadata.update((metadata) => ({
      ...metadata,
      tags: metadata.tags.map((t) =>
        t.id === newTag.id ? created : t
      ),
    }));
  } catch (err) {
    // Rollback on error
    _currentFileMetadata.update((metadata) => ({
      ...metadata,
      tags: metadata.tags.filter((t) => t.id !== newTag.id),
    }));
    _error.set(`Failed to add tag: ${err.message}`);
  }
}

/**
 * Remove a tag from a file
 * @param {string} filePath - File path
 * @param {string} tagId - Tag ID
 */
export async function removeTag(filePath, tagId) {
  // Optimistic update
  _currentFileMetadata.update((metadata) => ({
    ...metadata,
    tags: metadata.tags.filter((t) => t.id !== tagId),
    stats: {
      ...metadata.stats,
      tagCount: metadata.tags.length - 1,
    },
  }));

  try {
    await request(
      `/files/${encodeURIComponent(filePath)}/tags/${tagId}`,
      { method: 'DELETE' }
    );
  } catch (err) {
    // Rollback - reload tags
    await loadTags(filePath);
    _error.set(`Failed to remove tag: ${err.message}`);
  }
}

/**
 * Update tag (name, color)
 * @param {string} filePath - File path
 * @param {string} tagId - Tag ID
 * @param {object} updates - { name?, color? }
 */
export async function updateTag(filePath, tagId, updates) {
  // Optimistic update
  _currentFileMetadata.update((metadata) => ({
    ...metadata,
    tags: metadata.tags.map((t) =>
      t.id === tagId ? { ...t, ...updates } : t
    ),
  }));

  try {
    const updated = await request(
      `/files/${encodeURIComponent(filePath)}/tags/${tagId}`,
      {
        method: 'PUT',
        body: JSON.stringify(updates),
      }
    );

    _currentFileMetadata.update((metadata) => ({
      ...metadata,
      tags: metadata.tags.map((t) =>
        t.id === tagId ? updated : t
      ),
    }));
  } catch (err) {
    // Rollback
    await loadTags(filePath);
    _error.set(`Failed to update tag: ${err.message}`);
  }
}

/**
 * Get files by tag
 * @param {string} tagName - Tag name
 */
export async function getFilesByTag(tagName) {
  try {
    return await request(`/tags/${encodeURIComponent(tagName)}/files`);
  } catch (err) {
    _error.set(`Failed to get files by tag: ${err.message}`);
    return [];
  }
}

// ============================================================================
// PUBLIC API - COMMENTS
// ============================================================================

/**
 * Load all comments for a file
 * @param {string} filePath - File path
 */
export async function loadComments(filePath) {
  _loading.set(true);

  try {
    const comments = await request(
      `/files/${encodeURIComponent(filePath)}/comments`
    );

    _currentFileMetadata.update((metadata) => ({
      ...metadata,
      comments: comments || [],
      stats: {
        ...metadata.stats,
        commentCount: comments?.length || 0,
        lastCommentedAt: comments?.[0]?.createdAt || null,
      },
    }));

    _allFileMetadata.update((map) => {
      const current = map.get(filePath) || { ...defaultFileMetadata };
      map.set(filePath, { ...current, comments: comments || [] });
      return map;
    });
  } catch (err) {
    _error.set(`Failed to load comments: ${err.message}`);
  } finally {
    _loading.set(false);
  }
}

/**
 * Add a comment to a file
 * @param {string} filePath - File path
 * @param {string} text - Comment text
 * @param {string?} parentCommentId - Reply to comment
 */
export async function addComment(filePath, text, parentCommentId = null) {
  // Optimistic update
  const newComment = {
    id: Date.now().toString(),
    filePath,
    text,
    author: 'You',
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    replies: [],
    parentCommentId: parentCommentId || null,
    isPending: true,
  };

  _currentFileMetadata.update((metadata) => ({
    ...metadata,
    comments: [...metadata.comments, newComment],
    stats: {
      ...metadata.stats,
      commentCount: metadata.comments.length + 1,
      lastCommentedAt: newComment.createdAt,
    },
  }));

  try {
    const created = await request(
      `/files/${encodeURIComponent(filePath)}/comments`,
      {
        method: 'POST',
        body: JSON.stringify({ text, parentCommentId }),
      }
    );

    // Update with server response
    _currentFileMetadata.update((metadata) => ({
      ...metadata,
      comments: metadata.comments.map((c) =>
        c.id === newComment.id
          ? { ...created, isPending: false }
          : c
      ),
    }));
  } catch (err) {
    // Rollback
    _currentFileMetadata.update((metadata) => ({
      ...metadata,
      comments: metadata.comments.filter((c) => c.id !== newComment.id),
    }));
    _error.set(`Failed to add comment: ${err.message}`);
  }
}

/**
 * Edit a comment
 * @param {string} filePath - File path
 * @param {string} commentId - Comment ID
 * @param {string} text - New text
 */
export async function editComment(filePath, commentId, text) {
  // Optimistic update
  _currentFileMetadata.update((metadata) => ({
    ...metadata,
    comments: metadata.comments.map((c) =>
      c.id === commentId
        ? { ...c, text, updatedAt: new Date().toISOString() }
        : c
    ),
  }));

  try {
    const updated = await request(
      `/files/${encodeURIComponent(filePath)}/comments/${commentId}`,
      {
        method: 'PUT',
        body: JSON.stringify({ text }),
      }
    );

    _currentFileMetadata.update((metadata) => ({
      ...metadata,
      comments: metadata.comments.map((c) =>
        c.id === commentId ? updated : c
      ),
    }));
  } catch (err) {
    // Rollback
    await loadComments(filePath);
    _error.set(`Failed to edit comment: ${err.message}`);
  }
}

/**
 * Delete a comment
 * @param {string} filePath - File path
 * @param {string} commentId - Comment ID
 */
export async function deleteComment(filePath, commentId) {
  // Optimistic update
  _currentFileMetadata.update((metadata) => ({
    ...metadata,
    comments: metadata.comments.filter((c) => c.id !== commentId),
    stats: {
      ...metadata.stats,
      commentCount: metadata.comments.length - 1,
    },
  }));

  try {
    await request(
      `/files/${encodeURIComponent(filePath)}/comments/${commentId}`,
      { method: 'DELETE' }
    );
  } catch (err) {
    // Rollback
    await loadComments(filePath);
    _error.set(`Failed to delete comment: ${err.message}`);
  }
}

/**
 * Add reaction to comment (emoji)
 * @param {string} filePath - File path
 * @param {string} commentId - Comment ID
 * @param {string} emoji - Emoji reaction
 */
export async function addReaction(filePath, commentId, emoji) {
  try {
    await request(
      `/files/${encodeURIComponent(filePath)}/comments/${commentId}/reactions`,
      {
        method: 'POST',
        body: JSON.stringify({ emoji }),
      }
    );

    // Reload comments to get updated reactions
    await loadComments(filePath);
  } catch (err) {
    _error.set(`Failed to add reaction: ${err.message}`);
  }
}

// ============================================================================
// UTILITIES
// ============================================================================

/**
 * Get all unique tags across all files
 */
export function getAllTags() {
  let allTags = [];
  let map;

  allFileMetadata.subscribe((m) => (map = m))();

  if (map) {
    for (const metadata of map.values()) {
      allTags.push(...metadata.tags);
    }
  }

  // Remove duplicates
  const unique = new Map();
  for (const tag of allTags) {
    if (!unique.has(tag.name)) {
      unique.set(tag.name, tag);
    }
  }

  return Array.from(unique.values()).sort((a, b) =>
    a.name.localeCompare(b.name)
  );
}

/**
 * Search comments
 * @param {string} query - Search query
 */
export function searchComments(query) {
  let comments = [];
  let map;

  allFileMetadata.subscribe((m) => (map = m))();

  if (map) {
    for (const metadata of map.values()) {
      comments.push(...metadata.comments);
    }
  }

  const q = query.toLowerCase();
  return comments.filter(
    (c) =>
      c.text.toLowerCase().includes(q) ||
      c.author.toLowerCase().includes(q)
  );
}

/**
 * Get comment thread (parent + replies)
 * @param {string} filePath - File path
 * @param {string} commentId - Parent comment ID
 */
export function getCommentThread(filePath, commentId) {
  let metadata;

  currentFileMetadata.subscribe((m) => (metadata = m))();

  const parent = metadata.comments.find((c) => c.id === commentId);
  if (!parent) return null;

  const replies = metadata.comments.filter(
    (c) => c.parentCommentId === commentId
  );

  return { parent, replies };
}

/**
 * Clear all metadata for a file (when file deleted)
 * @param {string} filePath - File path
 */
export function clearFileMetadata(filePath) {
  _allFileMetadata.update((map) => {
    map.delete(filePath);
    return map;
  });
}

/**
 * Export all tags and comments for a file
 * @param {string} filePath - File path
 */
export function exportFileMetadata(filePath) {
  let metadata;

  allFileMetadata.subscribe((m) => (metadata = m.get(filePath)))();

  if (!metadata) return null;

  return {
    filePath,
    tags: metadata.tags,
    comments: metadata.comments,
    exportedAt: new Date().toISOString(),
  };
}

export default {
  currentFileMetadata,
  allFileMetadata,
  loading,
  error,
  tagCloud,
  recentComments,
  loadTags,
  addTag,
  removeTag,
  updateTag,
  getFilesByTag,
  loadComments,
  addComment,
  editComment,
  deleteComment,
  addReaction,
  getAllTags,
  searchComments,
  getCommentThread,
  clearFileMetadata,
  exportFileMetadata,
};
