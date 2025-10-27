/**
 * User Preferences Store (synchronized with backend)
 * Replaces localStorage for client-specific settings to enable multi-platform consistency
 */

import { writable } from 'svelte/store';
import api from '../lib/api.js';

// Default preferences
const defaultPreferences = {
  view_mode: 'grid',
  recent_searches: [],
  sidebar_collapsed: false,
  sort_by: 'name',
  sort_order: 'asc',
  auto_refresh: true,
  upload_progress_visible: true,
};

// Create writable store
export const preferences = writable(defaultPreferences);

// Store implementation with backend sync
const createPreferencesStore = () => {
  const { subscribe, set, update } = writable(defaultPreferences);
  
  return {
    subscribe,
    
    // Load preferences from backend
    async load() {
      try {
        const prefs = await api.auth.getPreferences();
        // Parse JSON fields
        if (prefs.recent_searches && typeof prefs.recent_searches === 'string') {
          try {
            prefs.recent_searches = JSON.parse(prefs.recent_searches);
          } catch {
            prefs.recent_searches = [];
          }
        }
        if (prefs.search_filters && typeof prefs.search_filters === 'string') {
          try {
            prefs.search_filters = JSON.parse(prefs.search_filters);
          } catch {
            prefs.search_filters = null;
          }
        }
        set(prefs);
        return prefs;
      } catch (error) {
        console.warn('Failed to load preferences from backend:', error);
        // Keep default values
        return defaultPreferences;
      }
    },
    
    // Update a single preference and sync to backend
    async updatePreference(key, value) {
      try {
        const updateData = { [key]: value };
        const updatedPrefs = await api.auth.updatePreferences(updateData);
        set(updatedPrefs);
        return updatedPrefs;
      } catch (error) {
        console.error('Failed to update preference:', error);
        // Update locally as fallback
        update(prefs => ({ ...prefs, [key]: value }));
        throw error;
      }
    },
    
    // Update multiple preferences at once
    async updatePreferences(updates) {
      try {
        const updatedPrefs = await api.auth.updatePreferences(updates);
        set(updatedPrefs);
        return updatedPrefs;
      } catch (error) {
        console.error('Failed to update preferences:', error);
        // Update locally as fallback
        update(prefs => ({ ...prefs, ...updates }));
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
        const resetPrefs = await api.auth.updatePreferences(defaultPreferences);
        set(resetPrefs);
        return resetPrefs;
      } catch (error) {
        console.error('Failed to reset preferences:', error);
        set(defaultPreferences);
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