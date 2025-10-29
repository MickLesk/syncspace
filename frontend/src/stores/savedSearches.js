/**
 * Saved Searches Store
 * Manages user-saved search queries with filters
 */

import { writable, derived } from 'svelte/store';
import { userPreferences } from './preferences.js';

// Create saved searches store from user preferences
function createSavedSearchesStore() {
  const { subscribe, set, update } = writable([]);
  
  // Load saved searches from preferences on init
  let unsubscribe;
  if (typeof window !== 'undefined') {
    unsubscribe = userPreferences.subscribe(prefs => {
      if (prefs && prefs.saved_searches) {
        set(prefs.saved_searches);
      }
    });
  }
  
  /**
   * Save a new search query with filters
   * @param {Object} search - Search object
   * @param {string} search.name - User-defined name for this search
   * @param {string} search.query - Search query string
   * @param {Object} search.filters - Active filters
   * @param {string} search.sortBy - Sort field
   * @param {string} search.sortOrder - Sort direction
   */
  async function saveSearch(search) {
    const newSearch = {
      id: Date.now().toString(),
      name: search.name,
      query: search.query,
      filters: search.filters,
      sortBy: search.sortBy || 'name',
      sortOrder: search.sortOrder || 'asc',
      createdAt: new Date().toISOString(),
      lastUsed: null,
      useCount: 0,
    };
    
    let searches = [];
    const unsubscribeCurrent = subscribe(current => { searches = current; });
    unsubscribeCurrent();
    
    // Add new search
    searches = [newSearch, ...searches];
    
    // Save to backend preferences
    await userPreferences.updatePreferences({ saved_searches: searches });
    
    set(searches);
  }
  
  /**
   * Delete a saved search
   * @param {string} searchId - ID of search to delete
   */
  async function deleteSearch(searchId) {
    let searches = [];
    const unsubscribeCurrent = subscribe(current => { searches = current; });
    unsubscribeCurrent();
    
    searches = searches.filter(s => s.id !== searchId);
    
    await userPreferences.updatePreferences({ saved_searches: searches });
    set(searches);
  }
  
  /**
   * Update search metadata when used
   * @param {string} searchId - ID of search that was used
   */
  async function markSearchAsUsed(searchId) {
    let searches = [];
    const unsubscribeCurrent = subscribe(current => { searches = current; });
    unsubscribeCurrent();
    
    searches = searches.map(s => {
      if (s.id === searchId) {
        return {
          ...s,
          lastUsed: new Date().toISOString(),
          useCount: (s.useCount || 0) + 1,
        };
      }
      return s;
    });
    
    await userPreferences.updatePreferences({ saved_searches: searches });
    set(searches);
  }
  
  /**
   * Rename a saved search
   * @param {string} searchId - ID of search to rename
   * @param {string} newName - New name
   */
  async function renameSearch(searchId, newName) {
    let searches = [];
    const unsubscribeCurrent = subscribe(current => { searches = current; });
    unsubscribeCurrent();
    
    searches = searches.map(s => {
      if (s.id === searchId) {
        return { ...s, name: newName };
      }
      return s;
    });
    
    await userPreferences.updatePreferences({ saved_searches: searches });
    set(searches);
  }
  
  return {
    subscribe,
    saveSearch,
    deleteSearch,
    markSearchAsUsed,
    renameSearch,
  };
}

export const savedSearches = createSavedSearchesStore();

// Derived stores
export const savedSearchesCount = derived(savedSearches, $searches => $searches.length);
export const recentlyUsedSearches = derived(savedSearches, $searches => 
  [...$searches]
    .filter(s => s.lastUsed)
    .sort((a, b) => new Date(b.lastUsed).getTime() - new Date(a.lastUsed).getTime())
    .slice(0, 5)
);
export const mostUsedSearches = derived(savedSearches, $searches => 
  [...$searches]
    .sort((a, b) => (b.useCount || 0) - (a.useCount || 0))
    .slice(0, 5)
);
