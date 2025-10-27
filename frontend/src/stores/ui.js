import { writable } from 'svelte/store';
import api from '../lib/api.js';

// Theme store with backend sync
function createThemeStore() {
  const stored = localStorage.getItem('theme') || 'syncspace';
  const { subscribe, set, update } = writable(stored);
  
  return {
    subscribe,
    set: async (value) => {
      localStorage.setItem('theme', value);
      document.documentElement.setAttribute('data-theme', value);
      set(value);
      
      // Sync to backend if logged in
      try {
        const token = localStorage.getItem('authToken');
        if (token) {
          await api.put('/api/users', { theme: value });
        }
      } catch (err) {
        console.error('Failed to sync theme to backend:', err);
      }
    },
    // Method to load from backend
    loadFromBackend: async () => {
      try {
        const user = await api.get('/api/users/me');
        if (user && user.theme) {
          localStorage.setItem('theme', user.theme);
          document.documentElement.setAttribute('data-theme', user.theme);
          set(user.theme);
        }
      } catch (err) {
        console.error('Failed to load theme from backend:', err);
      }
    }
  };
}

// Language store with backend sync
function createLangStore() {
  const stored = localStorage.getItem('lang') || 'de';
  const { subscribe, set } = writable(stored);
  
  return {
    subscribe,
    set: async (value) => {
      localStorage.setItem('lang', value);
      set(value);
      
      // Sync to backend if logged in
      try {
        const token = localStorage.getItem('authToken');
        if (token) {
          await api.put('/api/users', { language: value });
        }
      } catch (err) {
        console.error('Failed to sync language to backend:', err);
      }
    },
    // Method to load from backend
    loadFromBackend: async () => {
      try {
        const user = await api.get('/api/users/me');
        if (user && user.language) {
          localStorage.setItem('lang', user.language);
          set(user.language);
        }
      } catch (err) {
        console.error('Failed to load language from backend:', err);
      }
    }
  };
}

// Persist sidebar state
function createSidebarStore() {
  const stored = localStorage.getItem('sidebarCollapsed') === 'true';
  const { subscribe, set, update } = writable(stored);
  
  return {
    subscribe,
    toggle: () => update(n => {
      const newValue = !n;
      localStorage.setItem('sidebarCollapsed', newValue.toString());
      return newValue;
    }),
    set: (value) => {
      localStorage.setItem('sidebarCollapsed', value.toString());
      set(value);
    }
  };
}

// Persist current path to sessionStorage (lost on tab close, which is desired)
function createPathStore() {
  const stored = sessionStorage.getItem('currentPath') || '/';
  console.log(`[ui.js] Initial currentPath from storage: "${stored}"`);
  const { subscribe, set, update } = writable(stored);
  
  return {
    subscribe,
    set: (value) => {
      console.log(`[ui.js] currentPath.set("${value}")`);
      sessionStorage.setItem('currentPath', value);
      set(value);
    },
    update: (fn) => {
      update(n => {
        const newValue = fn(n);
        console.log(`[ui.js] currentPath.update() → "${newValue}"`);
        sessionStorage.setItem('currentPath', newValue);
        return newValue;
      });
    }
  };
}

// Persist current view to sessionStorage
function createViewStore() {
  const stored = sessionStorage.getItem('currentView') || 'files';
  const { subscribe, set } = writable(stored);
  
  return {
    subscribe,
    set: (value) => {
      sessionStorage.setItem('currentView', value);
      set(value);
    }
  };
}

// Persist favorites feature toggle
function createFavoritesEnabledStore() {
  const stored = localStorage.getItem('favoritesEnabled') !== 'false'; // Default: true
  const { subscribe, set } = writable(stored);
  
  return {
    subscribe,
    set: (value) => {
      localStorage.setItem('favoritesEnabled', value.toString());
      set(value);
    }
  };
}

export const currentView = createViewStore();
export const currentTheme = createThemeStore();
export const currentLang = createLangStore();
export const sidebarCollapsed = createSidebarStore();
export const favoritesEnabled = createFavoritesEnabledStore();
export const files = writable([]);
export const currentPath = createPathStore();
