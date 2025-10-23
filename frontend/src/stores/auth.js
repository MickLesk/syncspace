import { writable } from 'svelte/store';
import { t } from '../i18n.js';

// TODO: Get from settings store once implemented
const getCurrentLanguage = () => localStorage.getItem('language') || 'de';

function createAuthStore() {
  // Clean up old demo tokens
  const storedToken = localStorage.getItem('authToken');
  if (storedToken && storedToken.startsWith('demo-token-')) {
    console.warn('🧹 Cleaning up old demo token, please login again');
    localStorage.removeItem('authToken');
    localStorage.removeItem('username');
    localStorage.removeItem('userId');
  }

  const { subscribe, set, update } = writable({
    isLoggedIn: !!localStorage.getItem('authToken'),
    token: localStorage.getItem('authToken'),
    username: localStorage.getItem('username'),
    userId: localStorage.getItem('userId')
  });

  const authStore = {
    subscribe,
    // Helper to check if response is auth error
    checkAuthError: (response) => {
      if (response.status === 401 || response.status === 403) {
        console.warn('🔒 Authentication failed, logging out...');
        authStore.logout();
        return true;
      }
      return false;
    },
    login: async (username, password) => {
      try {
        const response = await fetch('http://localhost:8080/api/auth/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username, password })
        });

        const data = await response.json();

        if (response.ok && data.token) {
          localStorage.setItem('authToken', data.token);
          localStorage.setItem('username', data.user.username);
          localStorage.setItem('userId', data.user.id);
          
          set({
            isLoggedIn: true,
            token: data.token,
            username: data.user.username,
            userId: data.user.id
          });
          
          return { success: true };
        } else {
          const lang = getCurrentLanguage();
          return { success: false, error: data.error || t(lang, 'invalidCredentials') };
        }
      } catch (error) {
        console.error('❌ Login error:', error);
        const lang = getCurrentLanguage();
        return { success: false, error: t(lang, 'backendNotReachable') };
      }
    },
    logout: () => {
      localStorage.removeItem('authToken');
      localStorage.removeItem('username');
      localStorage.removeItem('userId');
      set({
        isLoggedIn: false,
        token: null,
        username: null,
        userId: null
      });
    }
  };
  
  return authStore;
}

export const auth = createAuthStore();
