import { writable, derived } from 'svelte/store';
import { debounce } from '../utils/debounce.js';

// Utility: Get auth token
function getToken() {
  let token = localStorage.getItem("authToken");
  if (!token) {
    const authData = localStorage.getItem("auth");
    if (authData) {
      try {
        const parsed = JSON.parse(authData);
        token = parsed.token;
      } catch (e) {
        // ignore
      }
    }
  }
  return token;
}

function createFavoritesStore() {
  const { subscribe, set, update } = writable(new Map()); // Map of item_id -> favorite

  // Pending toggles map to prevent duplicate requests
  const pendingToggles = new Map();

  // Load favorites from API on startup
  async function loadFromAPI() {
    try {
      const token = getToken();
      if (!token) return;
      
      const response = await fetch('http://localhost:8080/api/favorites', {
        headers: { Authorization: `Bearer ${token}` }
      });
      
      if (response.ok) {
        const favorites = await response.json();
        const map = new Map();
        favorites.forEach(fav => {
          // Store by item_id (file/folder ID or path)
          map.set(fav.item_id, fav);
        });
        set(map);
      }
    } catch (err) {
      console.error('Failed to load favorites from API:', err);
    }
  }

  return {
    subscribe,
    
    // Load from API
    load: loadFromAPI,
    
    // Toggle favorite via API with debouncing
    toggle: async (itemId, itemType = 'file') => {
      // Prevent duplicate toggles for the same item
      if (pendingToggles.has(itemId)) {
        console.log(`Toggle for ${itemId} already pending, skipping`);
        return;
      }

      pendingToggles.set(itemId, true);

      try {
        const token = getToken();
        if (!token) {
          console.error('Not authenticated');
          return;
        }
        
        const response = await fetch('http://localhost:8080/api/favorites', {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${token}`,
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            item_type: itemType,
            item_id: itemId
          })
        });
        
        if (response.ok) {
          const data = await response.json();
          update(favorites => {
            const newFavorites = new Map(favorites);
            if (data.status === 'added') {
              newFavorites.set(itemId, {
                id: itemId,
                item_type: itemType,
                item_id: itemId
              });
            } else {
              newFavorites.delete(itemId);
            }
            return newFavorites;
          });
        }
      } catch (err) {
        console.error('Failed to toggle favorite:', err);
      } finally {
        // Remove from pending after a short delay to prevent rapid re-toggles
        setTimeout(() => {
          pendingToggles.delete(itemId);
        }, 300);
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
        const token = getToken();
        if (!token) return;
        
        // Get favorite ID from store
        let favId = null;
        subscribe(favorites => {
          const fav = favorites.get(itemId);
          if (fav) favId = fav.id;
        })();
        
        if (!favId) return;
        
        const response = await fetch(`http://localhost:8080/api/favorites/${favId}`, {
          method: 'DELETE',
          headers: { Authorization: `Bearer ${token}` }
        });
        
        if (response.ok) {
          update(favorites => {
            const newFavorites = new Map(favorites);
            newFavorites.delete(itemId);
            return newFavorites;
          });
        }
      } catch (err) {
        console.error('Failed to remove favorite:', err);
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
