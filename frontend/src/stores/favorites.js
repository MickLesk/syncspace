/**
 * Favorites Store (Backend-synchronized)
 * Manages user's favorite files and folders via backend API
 */

import { writable, derived } from 'svelte/store';
import api from '../lib/api.js';

function createFavoritesStore() {
  const { subscribe, set, update } = writable(new Map()); // Map of item_id -> favorite

  // Pending toggles map to prevent duplicate requests
  const pendingToggles = new Map();

  // Load favorites from API on startup
  async function loadFromAPI() {
    try {
      const favorites = await api.favorites.list();
      const map = new Map();
      
      if (Array.isArray(favorites)) {
        favorites.forEach(fav => {
          // Store by item_id (file/folder ID)
          map.set(fav.item_id, fav);
        });
      }
      
      set(map);
      return favorites;
    } catch (err) {
      console.error('Failed to load favorites from API:', err);
      return [];
    }
  }

  return {
    subscribe,
    
    // Load from API
    load: loadFromAPI,
    
    // Add favorite
    add: async (itemId, itemType = 'file') => {
      if (pendingToggles.has(itemId)) {
        console.log(`Operation for ${itemId} already pending, skipping`);
        return;
      }

      pendingToggles.set(itemId, true);

      try {
        const result = await api.favorites.add(itemId, itemType);
        
        if (result) {
          update(favorites => {
            const newFavorites = new Map(favorites);
            newFavorites.set(itemId, result);
            return newFavorites;
          });
        }
      } catch (err) {
        console.error('Failed to add favorite:', err);
        throw err;
      } finally {
        setTimeout(() => pendingToggles.delete(itemId), 300);
      }
    },
    
    // Check if item is favorite
    has: (itemId) => {
      let result = false;
      subscribe(favorites => {
        result = favorites.has(itemId);
      })();
      return result;
    },
    
    // Toggle favorite (add if not exists, remove if exists)
    toggle: async (itemId, itemType = 'file') => {
      let isFavorite = false;
      subscribe(favorites => {
        isFavorite = favorites.has(itemId);
      })();
      
      if (isFavorite) {
        return await createFavoritesStore().remove(itemId);
      } else {
        return await createFavoritesStore().add(itemId, itemType);
      }
    },
    
    // Get all favorites
    getAll: () => {
      let result = [];
      subscribe(favorites => {
        result = Array.from(favorites.values());
      })();
      return result;
    },
    
    // Remove favorite
    remove: async (itemId) => {
      try {
        // Get favorite ID from store
        let favId = null;
        update(favorites => {
          const fav = favorites.get(itemId);
          if (fav) favId = fav.id;
          return favorites;
        });
        
        if (!favId) {
          console.warn('Favorite not found for removal:', itemId);
          return;
        }
        
        await api.favorites.remove(favId);
        
        update(favorites => {
          const newFavorites = new Map(favorites);
          newFavorites.delete(itemId);
          return newFavorites;
        });
      } catch (err) {
        console.error('Failed to remove favorite:', err);
        throw err;
      }
    },
    
    clear: async () => {
      set(new Map());
    }
  };
}

export const favorites = createFavoritesStore();

// Derived store for getting favorite count
export const favoritesCount = derived(
  favorites,
  $favorites => $favorites.size
);
