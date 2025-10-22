/**
 * Activity Timeline Store
 * Tracks file operations and user actions
 */

import { writable } from 'svelte/store';

const STORAGE_KEY = 'syncspace_activity';
const MAX_ACTIVITIES = 100;

/**
 * Activity entry structure:
 * {
 *   id: string,
 *   type: 'upload' | 'download' | 'delete' | 'rename' | 'create' | 'move' | 'share',
 *   filename: string,
 *   path: string,
 *   timestamp: number,
 *   details?: string,
 *   icon: string
 * }
 */

function createActivityStore() {
  // Load initial activities from localStorage
  const stored = typeof localStorage !== 'undefined' 
    ? localStorage.getItem(STORAGE_KEY) 
    : null;
  
  const initial = stored ? JSON.parse(stored) : [];
  
  const { subscribe, set, update } = writable(initial);

  return {
    subscribe,
    
    /**
     * Add a new activity
     */
    add: (type, filename, path = '', details = '') => {
      const icons = {
        upload: '⬆️',
        download: '⬇️',
        delete: '🗑️',
        rename: '✏️',
        create: '📁',
        move: '📦',
        share: '🔗',
        favorite: '⭐',
        unfavorite: '☆'
      };

      const activity = {
        id: `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
        type,
        filename,
        path,
        timestamp: Date.now(),
        details,
        icon: icons[type] || '📄'
      };

      update(activities => {
        const updated = [activity, ...activities].slice(0, MAX_ACTIVITIES);
        
        // Persist to localStorage
        if (typeof localStorage !== 'undefined') {
          localStorage.setItem(STORAGE_KEY, JSON.stringify(updated));
        }
        
        return updated;
      });
    },

    /**
     * Clear all activities
     */
    clear: () => {
      set([]);
      if (typeof localStorage !== 'undefined') {
        localStorage.removeItem(STORAGE_KEY);
      }
    },

    /**
     * Remove activities older than N days
     */
    cleanup: (daysOld = 30) => {
      const cutoff = Date.now() - (daysOld * 24 * 60 * 60 * 1000);
      
      update(activities => {
        const filtered = activities.filter(a => a.timestamp > cutoff);
        
        if (typeof localStorage !== 'undefined') {
          localStorage.setItem(STORAGE_KEY, JSON.stringify(filtered));
        }
        
        return filtered;
      });
    },

    /**
     * Get activities for today
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

    /**
     * Get activity stats
     */
    getStats: () => {
      let result = { total: 0, byType: {} };
      const unsubscribe = subscribe(activities => {
        result.total = activities.length;
        result.byType = activities.reduce((acc, a) => {
          acc[a.type] = (acc[a.type] || 0) + 1;
          return acc;
        }, {});
      });
      unsubscribe();
      
      return result;
    }
  };
}

export const activity = createActivityStore();
