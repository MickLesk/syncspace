import { writable } from 'svelte/store';
import api from '../lib/api.js';

// Theme store with backend sync
function createThemeStore() {
  const stored = localStorage.getItem('theme') || 'light';
  const { subscribe, set, update } = writable(stored);
  
  // Apply initial theme on load
  if (typeof document !== 'undefined') {
    const isDark = stored === 'dark' || stored === 'syncspace-dark';
    document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');
    
    if (isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }
  
  return {
    subscribe,
    set: async (value) => {
      const isDark = value === 'dark' || value === 'syncspace-dark';
      
      localStorage.setItem('theme', value);
      document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');
      
      // Set .dark class for Tailwind
      if (isDark) {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
      
      set(value);
      
      // Sync to backend if logged in
      try {
        const token = localStorage.getItem('authToken');
        if (token) {
          await api.users.updateSettings({ theme: value });
        }
      } catch (err) {
        console.error('Failed to sync theme to backend:', err);
      }
    },
    // Method to load from backend
    async loadFromBackend() {
      try {
        const response = await api.users.getSettings();
        if (response?.theme) {
          const theme = response.theme;
          const isDark = theme === 'dark' || theme === 'syncspace-dark';
          
          localStorage.setItem('theme', theme);
          document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');
          
          if (isDark) {
            document.documentElement.classList.add('dark');
          } else {
            document.documentElement.classList.remove('dark');
          }
          
          set(theme);
        }
      } catch (error) {
        // Silent fallback for 404
        if (error.message && error.message.includes('404')) {
          console.log('[Theme] Backend endpoint not implemented, using localStorage');
        } else {
          console.error('Failed to load theme from backend:', error);
        }
      }
    },
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
          await api.users.updateSettings({ language: value });
        }
      } catch (err) {
        console.error('Failed to sync language to backend:', err);
      }
    },
    // Method to load from backend
    loadFromBackend: async () => {
      try {
        const settings = await api.users.getSettings();
        if (settings && settings.language) {
          localStorage.setItem('lang', settings.language);
          set(settings.language);
        }
      } catch (err) {
        // Silent fallback for 404 (endpoint not implemented) - this is expected
        if (err.message && err.message.includes('404')) {
          console.log('[Language] Backend endpoint not implemented, using localStorage');
        } else {
          console.error('Failed to load language from backend:', err);
        }
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

// Aliases for backward compatibility
export const theme = currentTheme;
export const language = currentLang;

// Global loading state
function createLoadingStore() {
  const { subscribe, set, update } = writable({
    isLoading: false,
    message: '',
    progress: null // For upload progress: { current: number, total: number }
  });
  
  return {
    subscribe,
    show: (message = 'Loading...', progress = null) => {
      set({ isLoading: true, message, progress });
    },
    hide: () => {
      set({ isLoading: false, message: '', progress: null });
    },
    updateProgress: (current, total) => {
      update(state => ({ ...state, progress: { current, total } }));
    }
  };
}

export const loading = createLoadingStore();
