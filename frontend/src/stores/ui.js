import { writable } from 'svelte/store';

// Persist theme to localStorage
function createThemeStore() {
  const stored = localStorage.getItem('theme') || 'light';
  const { subscribe, set } = writable(stored);
  
  return {
    subscribe,
    set: (value) => {
      localStorage.setItem('theme', value);
      document.documentElement.setAttribute('data-theme', value);
      set(value);
    }
  };
}

// Persist language to localStorage
function createLangStore() {
  const stored = localStorage.getItem('lang') || 'de';
  const { subscribe, set } = writable(stored);
  
  return {
    subscribe,
    set: (value) => {
      localStorage.setItem('lang', value);
      set(value);
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

export const currentView = writable('files');
export const currentTheme = createThemeStore();
export const currentLang = createLangStore();
export const sidebarCollapsed = createSidebarStore();
export const files = writable([]);
export const currentPath = writable('');
