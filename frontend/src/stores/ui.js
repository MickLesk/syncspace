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

export const currentView = createViewStore();
export const currentTheme = createThemeStore();
export const currentLang = createLangStore();
export const sidebarCollapsed = createSidebarStore();
export const files = writable([]);
export const currentPath = createPathStore();
