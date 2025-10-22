import { writable, derived } from 'svelte/store';

// Load favorites from localStorage
function loadFavorites() {
  if (typeof window === 'undefined') return new Set();
  try {
    const stored = localStorage.getItem('syncspace_favorites');
    return stored ? new Set(JSON.parse(stored)) : new Set();
  } catch (e) {
    console.error('Failed to load favorites:', e);
    return new Set();
  }
}

// Save favorites to localStorage
function saveFavorites(favorites) {
  if (typeof window === 'undefined') return;
  try {
    localStorage.setItem('syncspace_favorites', JSON.stringify([...favorites]));
  } catch (e) {
    console.error('Failed to save favorites:', e);
  }
}

function createFavoritesStore() {
  const { subscribe, set, update } = writable(loadFavorites());

  return {
    subscribe,
    
    toggle: (path) => {
      update(favorites => {
        const newFavorites = new Set(favorites);
        if (newFavorites.has(path)) {
          newFavorites.delete(path);
        } else {
          newFavorites.add(path);
        }
        saveFavorites(newFavorites);
        return newFavorites;
      });
    },
    
    add: (path) => {
      update(favorites => {
        const newFavorites = new Set(favorites);
        newFavorites.add(path);
        saveFavorites(newFavorites);
        return newFavorites;
      });
    },
    
    remove: (path) => {
      update(favorites => {
        const newFavorites = new Set(favorites);
        newFavorites.delete(path);
        saveFavorites(newFavorites);
        return newFavorites;
      });
    },
    
    has: (path, $favorites) => {
      return $favorites.has(path);
    },
    
    clear: () => {
      set(new Set());
      saveFavorites(new Set());
    }
  };
}

export const favorites = createFavoritesStore();

// Derived store for getting favorite count
export const favoritesCount = derived(
  favorites,
  $favorites => $favorites.size
);
