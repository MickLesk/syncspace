/**
 * Activity Timeline Store - BACKEND-BASIERT
 * Lädt Activities vom Backend für Multi-Client-Support (Web/Flutter/Windows)
 */

import { writable, derived } from 'svelte/store';

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

  // Icon mapping for display
  const icons = {
    created: '⬆️',
    uploaded: '⬆️',
    downloaded: '⬇️',
    deleted: '🗑️',
    renamed: '✏️',
    moved: '📦',
    shared: '🔗'
  };

  return {
    subscribe,
    
    /**
     * Load activities from backend API
     */
    load: async (params = {}) => {
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
        
        const url = `http://localhost:8080/api/activity?${queryParams.toString()}`;
        
        const response = await fetch(url, {
          headers: { Authorization: `Bearer ${token}` }
        });
        
        if (response.ok) {
          const activities = await response.json();
          
          // Transform to frontend format
          const transformed = activities.map(a => ({
            ...a,
            type: a.action,
            filename: a.file_path.split('/').pop() || a.file_path,
            path: a.file_path,
            timestamp: new Date(a.created_at).getTime(),
            details: a.error_message || '',
            icon: icons[a.action] || '📄'
          }));
          
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
    },
    
    /**
     * Get activity statistics from backend
     */
    getStats: async () => {
      try {
        const token = getToken();
        if (!token) return null;
        
        const response = await fetch('http://localhost:8080/api/activity/stats', {
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
