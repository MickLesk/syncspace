/**
 * Activity Timeline Store - BACKEND-BASIERT
 * Lädt Activities vom Backend für Multi-Client-Support (Web/Flutter/Windows)
 */

import { writable, derived } from 'svelte/store';
import { API_BASE, API_HOST } from '../lib/api.js';

// WebSocket base URL (ws:// or wss://)
const WS_BASE = API_HOST.replace(/^http/, 'ws') + '/api/ws';

// Get auth token
function getToken() {
  let token = localStorage.getItem("authToken");
  if (!token) {
    try {
      const authData = localStorage.getItem("auth");
      if (authData) {
        const parsed = JSON.parse(authData);
        token = parsed.token;
      }
    } catch (e) {
      // ignore
    }
  }
  return token;
}

function createActivityStore() {
  const { subscribe, set, update } = writable([]);
  const loading = writable(false);
  let websocket = null;
  let reconnectTimeout = null;

  // Icon mapping for display
  const icons = {
    created: '⬆️',
    upload: '⬆️',
    uploaded: '⬆️',
    downloaded: '⬇️',
    download: '⬇️',
    deleted: '🗑️',
    delete: '🗑️',
    renamed: '✏️',
    rename: '✏️',
    moved: '📦',
    move: '📦',
    copy: '📋',
    create: '📁',
    shared: '🔗'
  };

  // Connect to WebSocket for real-time updates
  function connectWebSocket() {
    if (websocket) return;
    
    try {
      websocket = new WebSocket(WS_BASE);
      
      websocket.onopen = () => {
        console.log('[Activity] WebSocket connected for real-time updates');
      };
      
      websocket.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          
          // Reload activities when file operations occur
          if (data.path && data.kind) {
            console.log('[Activity] File operation detected, reloading...', data);
            // Reload activities after a short delay to ensure DB write completed
            setTimeout(() => {
              loadActivities({ limit: 100 });
            }, 500);
          }
        } catch (e) {
          console.error('[Activity] WebSocket message parse error:', e);
        }
      };
      
      websocket.onerror = (error) => {
        console.error('[Activity] WebSocket error:', error);
      };
      
      websocket.onclose = () => {
        console.log('[Activity] WebSocket disconnected, reconnecting in 5s...');
        websocket = null;
        
        // Auto-reconnect after 5 seconds
        reconnectTimeout = setTimeout(() => {
          connectWebSocket();
        }, 5000);
      };
    } catch (e) {
      console.error('[Activity] WebSocket connection failed:', e);
    }
  }
  
  // Load activities function (can be called externally or internally)
  async function loadActivities(params = {}) {
    loading.set(true);
    
    try {
      const token = getToken();
      if (!token) {
        console.warn('Not authenticated - cannot load activities');
        set([]);
        loading.set(false);
        return;
      }
      
      const queryParams = new URLSearchParams();
      if (params.limit) queryParams.set('limit', params.limit.toString());
      if (params.offset) queryParams.set('offset', params.offset.toString());
      if (params.action) queryParams.set('action', params.action);
      
      const url = `${API_BASE}/activity?${queryParams.toString()}`;
      
      const response = await fetch(url, {
        headers: { Authorization: `Bearer ${token}` }
      });
      
      if (response.ok) {
        const activities = await response.json();
        
        // Transform to frontend format with rich details
        const transformed = activities.map(a => {
          let details = a.error_message || '';
          
          // Generate detailed descriptions for rename/move operations
          if ((a.action === 'rename' || a.action === 'move') && a.old_path) {
            const oldName = a.old_path.split('/').pop() || a.old_path;
            const newName = a.file_name;
            
            if (a.action === 'rename') {
              details = `Renamed from "${oldName}" to "${newName}"`;
            } else {
              const oldDir = a.old_path.substring(0, a.old_path.lastIndexOf('/')) || '/';
              const newDir = a.file_path.substring(0, a.file_path.lastIndexOf('/')) || '/';
              details = `Moved from "${oldDir}" to "${newDir}"`;
            }
          } else if (a.action === 'copy' && a.old_path) {
            details = `Copied from "${a.old_path}"`;
          } else if (a.action === 'upload') {
            const sizeKB = a.file_size ? Math.round(a.file_size / 1024) : 0;
            details = `Uploaded (${sizeKB} KB)`;
          } else if (a.action === 'download') {
            details = `Downloaded`;
          } else if (a.action === 'delete') {
            details = `Deleted permanently`;
          } else if (a.action === 'create') {
            details = `Created new folder`;
          }
          
          return {
            ...a,
            type: a.action,
            filename: a.file_name || a.file_path.split('/').pop() || a.file_path,
            path: a.file_path,
            timestamp: new Date(a.created_at).getTime(),
            details: details,
            icon: icons[a.action] || '📄'
          };
        });
        
        set(transformed);
      } else {
        console.error('Failed to load activities:', response.status);
        set([]);
      }
    } catch (err) {
      console.error('Failed to load activities from API:', err);
      set([]);
    } finally {
      loading.set(false);
    }
  }

  return {
    subscribe,
    
    /**
     * Initialize WebSocket connection for real-time updates
     */
    init: () => {
      connectWebSocket();
    },
    
    /**
     * Disconnect WebSocket
     */
    disconnect: () => {
      if (reconnectTimeout) {
        clearTimeout(reconnectTimeout);
        reconnectTimeout = null;
      }
      if (websocket) {
        websocket.close();
        websocket = null;
      }
    },
    
    /**
     * Load activities from backend API
     */
    load: loadActivities,
    
    /**
     * Get activity statistics from backend
     */
    getStats: async () => {
      try {
        const token = getToken();
        if (!token) return null;
        
        const response = await fetch(`${API_BASE}/activity/stats`, {
          headers: { Authorization: `Bearer ${token}` }
        });
        
        if (response.ok) {
          return await response.json();
        }
        
        return null;
      } catch (err) {
        console.error('Failed to load activity stats:', err);
        return null;
      }
    },
    
    /**
     * Get activities for today (client-side filter)
     */
    getToday: () => {
      const today = new Date();
      today.setHours(0, 0, 0, 0);
      const todayTimestamp = today.getTime();

      let result = [];
      const unsubscribe = subscribe(activities => {
        result = activities.filter(a => a.timestamp >= todayTimestamp);
      });
      unsubscribe();
      
      return result;
    },
    
    loading: { subscribe: loading.subscribe }
  };
}

export const activity = createActivityStore();

// Derived stores
export const activityCount = derived(
  activity,
  $activity => $activity.length
);

export const failedActivities = derived(
  activity,
  $activity => $activity.filter(a => a.status === 'failed')
);
