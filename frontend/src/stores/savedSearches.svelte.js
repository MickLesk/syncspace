/**
 * Saved Searches Store
 * Manages user's saved search queries with filters, names, and quick access
 * Uses Svelte 5 runes for reactive state management
 */

import { writable, derived } from 'svelte/store';
import { api } from '../lib/api';
import { toast } from './ui';
import { t } from '../lib/i18n';

/**
 * Create saved searches store
 */
function createSavedSearchesStore() {
  // Saved searches list
  const searches = writable([]);
  
  // Recently used searches
  const recentSearches = writable([]);
  
  // Favorite searches
  const favoriteSearches = derived(searches, $searches => 
    $searches.filter(s => s.isFavorite).sort((a, b) => 
      new Date(b.lastUsed) - new Date(a.lastUsed)
    )
  );

  /**
   * Load saved searches from backend
   */
  async function loadSearches() {
    try {
      const response = await api.search.getSavedSearches?.();
      if (response?.data) {
        searches.set(response.data);
      }
    } catch (error) {
      console.error('Error loading saved searches:', error);
    }
  }

  /**
   * Save a new search
   */
  async function saveSearch(name, query, filters = {}, tags = []) {
    try {
      if (!name?.trim()) {
        toast.show(t('savedSearches.nameRequired'), 'warning');
        return null;
      }

      const search = {
        id: `search_${Date.now()}`,
        name: name.trim(),
        query,
        filters,
        tags,
        isFavorite: false,
        createdAt: new Date().toISOString(),
        lastUsed: new Date().toISOString(),
        usageCount: 0
      };

      // Save to backend if API available
      if (api.search.saveSearch) {
        const response = await api.search.saveSearch(search);
        if (response?.data) {
          search.id = response.data.id;
        }
      }

      searches.update(s => [...s, search]);
      toast.show(t('savedSearches.saved', { name }), 'success');
      return search;
    } catch (error) {
      console.error('Error saving search:', error);
      toast.show(t('errors.failedToSaveSearch'), 'error');
      return null;
    }
  }

  /**
   * Update existing search
   */
  async function updateSearch(id, updates) {
    try {
      searches.update(s => 
        s.map(search => 
          search.id === id 
            ? { ...search, ...updates, updatedAt: new Date().toISOString() }
            : search
        )
      );

      // Update on backend if API available
      if (api.search.updateSearch) {
        await api.search.updateSearch(id, updates);
      }

      return true;
    } catch (error) {
      console.error('Error updating search:', error);
      toast.show(t('errors.failedToUpdateSearch'), 'error');
      return false;
    }
  }

  /**
   * Delete search
   */
  async function deleteSearch(id) {
    try {
      const searchToDelete = (() => {
        let result;
        searches.subscribe(s => {
          result = s.find(search => search.id === id);
        })();

        searches.update(s => s.filter(search => search.id !== id));
        return result;
      })();

      // Delete from backend if API available
      if (api.search.deleteSearch) {
        await api.search.deleteSearch(id);
      }

      toast.show(t('savedSearches.deleted', { name: searchToDelete?.name }), 'success');
      return true;
    } catch (error) {
      console.error('Error deleting search:', error);
      toast.show(t('errors.failedToDeleteSearch'), 'error');
      return false;
    }
  }

  /**
   * Toggle favorite status
   */
  function toggleFavorite(id) {
    searches.update(s =>
      s.map(search =>
        search.id === id
          ? { ...search, isFavorite: !search.isFavorite }
          : search
      )
    );

    // Update backend if API available
    if (api.search.updateSearch) {
      searches.subscribe(s => {
        const search = s.find(sr => sr.id === id);
        if (search) {
          api.search.updateSearch(id, { isFavorite: search.isFavorite });
        }
      })();
    }
  }

  /**
   * Use search (increment counter and update lastUsed)
   */
  function useSearch(id) {
    searches.update(s =>
      s.map(search =>
        search.id === id
          ? {
              ...search,
              usageCount: (search.usageCount || 0) + 1,
              lastUsed: new Date().toISOString()
            }
          : search
      )
    );

    // Update recent searches
    searches.subscribe(s => {
      const used = s.find(search => search.id === id);
      if (used) {
        recentSearches.update(r => {
          const filtered = r.filter(rs => rs.id !== id);
          return [used, ...filtered].slice(0, 5); // Keep top 5
        });
      }
    })();

    // Update backend if API available
    if (api.search.updateSearch) {
      searches.subscribe(s => {
        const search = s.find(sr => sr.id === id);
        if (search) {
          api.search.updateSearch(id, {
            usageCount: search.usageCount,
            lastUsed: search.lastUsed
          });
        }
      })();
    }
  }

  /**
   * Search saved searches by name or tag
   */
  function searchSavedSearches(query) {
    return derived(searches, $searches => {
      if (!query?.trim()) return $searches;
      
      const q = query.toLowerCase();
      return $searches.filter(search =>
        search.name.toLowerCase().includes(q) ||
        search.tags?.some(tag => tag.toLowerCase().includes(q)) ||
        search.query.toLowerCase().includes(q)
      );
    });
  }

  /**
   * Get searches by tag
   */
  function getSearchesByTag(tag) {
    return derived(searches, $searches =>
      $searches.filter(search =>
        search.tags?.includes(tag)
      )
    );
  }

  /**
   * Get trending searches (most used)
   */
  function getTrendingSearches(limit = 5) {
    return derived(searches, $searches =>
      $searches
        .sort((a, b) => (b.usageCount || 0) - (a.usageCount || 0))
        .slice(0, limit)
    );
  }

  /**
   * Get all unique tags from saved searches
   */
  function getAllTags() {
    return derived(searches, $searches => {
      const tags = new Set();
      $searches.forEach(search => {
        search.tags?.forEach(tag => tags.add(tag));
      });
      return Array.from(tags).sort();
    });
  }

  /**
   * Duplicate search (copy with new name)
   */
  async function duplicateSearch(id, newName) {
    const original = (() => {
      let result;
      searches.subscribe(s => {
        result = s.find(search => search.id === id);
      })();
      return result;
    })();

    if (!original) return null;

    return saveSearch(
      newName || `${original.name} (Copy)`,
      original.query,
      original.filters,
      original.tags
    );
  }

  /**
   * Export searches as JSON
   */
  function exportSearches() {
    let result;
    searches.subscribe(s => {
      result = s;
    })();
    
    const json = JSON.stringify(result, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `saved-searches-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  /**
   * Import searches from JSON
   */
  async function importSearches(file) {
    try {
      const text = await file.text();
      const imported = JSON.parse(text);
      
      if (!Array.isArray(imported)) {
        toast.show(t('savedSearches.invalidFormat'), 'error');
        return false;
      }

      // Add imported searches
      searches.update(s => [
        ...s,
        ...imported.map((search, idx) => ({
          ...search,
          id: `search_${Date.now()}_${idx}`,
          createdAt: new Date().toISOString()
        }))
      ]);

      toast.show(
        t('savedSearches.imported', { count: imported.length }),
        'success'
      );
      return true;
    } catch (error) {
      console.error('Error importing searches:', error);
      toast.show(t('errors.failedToImportSearches'), 'error');
      return false;
    }
  }

  /**
   * Clear all searches
   */
  async function clearAll() {
    if (!confirm(t('savedSearches.confirmClearAll'))) {
      return false;
    }

    try {
      searches.set([]);
      
      if (api.search.clearSavedSearches) {
        await api.search.clearSavedSearches();
      }

      toast.show(t('savedSearches.cleared'), 'success');
      return true;
    } catch (error) {
      console.error('Error clearing searches:', error);
      toast.show(t('errors.failedToClearSearches'), 'error');
      return false;
    }
  }

  return {
    searches,
    recentSearches,
    favoriteSearches,
    loadSearches,
    saveSearch,
    updateSearch,
    deleteSearch,
    toggleFavorite,
    useSearch,
    searchSavedSearches,
    getSearchesByTag,
    getTrendingSearches,
    getAllTags,
    duplicateSearch,
    exportSearches,
    importSearches,
    clearAll
  };
}

export const savedSearches = createSavedSearchesStore();
