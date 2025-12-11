/**
 * User Preferences Store (synchronized with backend)
 * Replaces localStorage for client-specific settings to enable multi-platform consistency
 */

import { writable } from 'svelte/store';
import api from '../lib/api.js';

const STORAGE_KEY = 'userPreferences';

// Default preferences
const defaultPreferences = {
  view_mode: 'grid',
  recent_searches: [],
  sidebar_collapsed: false,
  sort_by: 'name',
  sort_order: 'asc',
  auto_refresh: true,
  upload_progress_visible: true,
  grid_columns: 4,
};

// Create writable store - SINGLE SOURCE OF TRUTH
export const preferences = writable(defaultPreferences);

// Store implementation with backend sync
const createPreferencesStore = () => {
  // Use the SAME store instance as exported preferences
  const { subscribe, set, update } = preferences;
  
  return {
    subscribe,
    
    // Load preferences from backend
    async load() {
      try {
        const response = await api.users.getPreferences();
        console.log('🔍 [Preferences Store] Loaded from backend:', response);
        console.log('🔍 [Preferences Store] grid_columns value:', response?.grid_columns, 'type:', typeof response?.grid_columns);
        if (response) {
          set(response); // This now updates the EXPORTED preferences store!
          localStorage.setItem(STORAGE_KEY, JSON.stringify(response));
        }
      } catch (error) {
        // Silent fallback for 404 (endpoint not implemented) - this is expected
        if (error.message && error.message.includes('404')) {
          console.log('[Preferences] Backend endpoint not implemented, using localStorage');
        } else {
          console.error('Failed to load preferences from backend:', error);
        }
        // Fall back to localStorage
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) {
          try {
            const parsed = JSON.parse(stored);
            set(parsed);
          } catch (e) {
            console.error('Failed to parse stored preferences:', e);
          }
        }
      }
    },
    
    // Update a single preference and sync to backend
    async updatePreference(key, value) {
      try {
        const updateData = { [key]: value };
        const updatedPrefs = await api.users.updatePreferences(updateData);
        set(updatedPrefs);
        localStorage.setItem(STORAGE_KEY, JSON.stringify(updatedPrefs));
        return updatedPrefs;
      } catch (error) {
        console.error('Failed to update preference:', error);
        // Update locally as fallback
        update(prefs => {
          const newPrefs = { ...prefs, [key]: value };
          localStorage.setItem(STORAGE_KEY, JSON.stringify(newPrefs));
          return newPrefs;
        });
        throw error;
      }
    },
    
    // Update multiple preferences at once
    async updatePreferences(updates) {
      try {
        const updatedPrefs = await api.users.updatePreferences(updates);
        set(updatedPrefs);
        localStorage.setItem(STORAGE_KEY, JSON.stringify(updatedPrefs));
        return updatedPrefs;
      } catch (error) {
        console.error('Failed to update preferences:', error);
        // Update locally as fallback
        update(prefs => {
          const newPrefs = { ...prefs, ...updates };
          localStorage.setItem(STORAGE_KEY, JSON.stringify(newPrefs));
          return newPrefs;
        });
        throw error;
      }
    },
    
    // Add a search to recent searches
    async addRecentSearch(searchQuery) {
      try {
        let currentPrefs = defaultPreferences;
        const unsubscribe = subscribe(prefs => { currentPrefs = prefs || defaultPreferences; });
        unsubscribe();
        
        const recentSearches = [
          searchQuery,
          ...(currentPrefs.recent_searches || []).filter(s => s !== searchQuery)
        ].slice(0, 10); // Keep only last 10 searches
        
        // Stringify for backend storage
        return await this.updatePreference('recent_searches', JSON.stringify(recentSearches));
      } catch (error) {
        console.error('Failed to add recent search:', error);
        throw error;
      }
    },
    
    // Clear recent searches
    async clearRecentSearches() {
      return await this.updatePreference('recent_searches', JSON.stringify([]));
    },
    
    // Reset to defaults
    async reset() {
      try {
        const resetPrefs = await api.users.updatePreferences(defaultPreferences);
        set(resetPrefs);
        localStorage.setItem(STORAGE_KEY, JSON.stringify(resetPrefs));
        return resetPrefs;
      } catch (error) {
        console.error('Failed to reset preferences:', error);
        set(defaultPreferences);
        localStorage.setItem(STORAGE_KEY, JSON.stringify(defaultPreferences));
        throw error;
      }
    }
  };
};

export const userPreferences = createPreferencesStore();

// Helper functions for common preference operations
export const preferencesHelpers = {
  // Toggle sidebar collapsed state
  async toggleSidebar() {
    let currentPrefs = defaultPreferences;
    const unsubscribe = preferences.subscribe(prefs => { currentPrefs = prefs || defaultPreferences; });
    unsubscribe();
    
    return await userPreferences.updatePreference('sidebar_collapsed', !currentPrefs.sidebar_collapsed);
  },
  
  // Set view mode (grid/list)
  async setViewMode(mode) {
    if (!['grid', 'list'].includes(mode)) {
      throw new Error('Invalid view mode. Must be "grid" or "list"');
    }
    return await userPreferences.updatePreference('view_mode', mode);
  },
  
  // Set sort configuration
  async setSorting(sortBy, sortOrder = 'asc') {
    if (!['name', 'size', 'date', 'type'].includes(sortBy)) {
      throw new Error('Invalid sort field');
    }
    if (!['asc', 'desc'].includes(sortOrder)) {
      throw new Error('Invalid sort order');
    }
    
    return await userPreferences.updatePreferences({
      sort_by: sortBy,
      sort_order: sortOrder
    });
  }
};