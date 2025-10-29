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

  // Auto-refresh timer
  let refreshTimer = null;

  const { subscribe, set, update } = writable({
    isLoggedIn: false, // Start als ausgeloggt
    token: localStorage.getItem('authToken'),
    username: localStorage.getItem('username'),
    userId: localStorage.getItem('userId'),
    isValidating: true // Flag um zu wissen dass wir noch validieren
  });

  const authStore = {
    subscribe,
    // Refresh token function
    refreshToken: async () => {
      const token = localStorage.getItem('authToken');
      if (!token) {
        console.warn('⚠️ No token to refresh');
        return false;
      }

      try {
        console.log('🔄 Refreshing access token...');
        const response = await fetch('http://localhost:8080/api/auth/refresh', {
          method: 'POST',
          headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
          }
        });

        if (response.ok) {
          const data = await response.json();
          
          // Update tokens
          localStorage.setItem('authToken', data.token);
          
          update(state => ({
            ...state,
            token: data.token
          }));
          
          console.log('✅ Token refreshed successfully');
          
          // Schedule next refresh (10 minutes before 24h expiry = refresh after 23h50m)
          authStore.scheduleTokenRefresh();
          
          return true;
        } else {
          console.warn('❌ Token refresh failed, logging out');
          authStore.logout();
          return false;
        }
      } catch (error) {
        console.error('❌ Token refresh error:', error);
        authStore.logout();
        return false;
      }
    },
    // Schedule automatic token refresh
    scheduleTokenRefresh: () => {
      // Clear existing timer
      if (refreshTimer) {
        clearInterval(refreshTimer);
        refreshTimer = null;
      }

      // Refresh token 10 minutes before expiry
      // JWT expires after 24 hours, so refresh after 23h 50m = 85800000ms
      const refreshInterval = 23 * 60 * 60 * 1000 + 50 * 60 * 1000; // 23h 50m
      
      refreshTimer = setInterval(() => {
        console.log('⏰ Auto-refresh triggered');
        authStore.refreshToken();
      }, refreshInterval);
      
      console.log(`⏰ Token auto-refresh scheduled (every ${(refreshInterval / 1000 / 60 / 60).toFixed(2)}h)`);
    },
    // Validate token on app start
    validateToken: async () => {
      const token = localStorage.getItem('authToken');
      if (!token) {
        update(state => ({ ...state, isValidating: false, isLoggedIn: false }));
        return;
      }

      try {
        // Verify token with backend
        const response = await fetch('http://localhost:8080/api/users/me', {
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });

        if (response.ok) {
          const userData = await response.json();
          update(state => ({
            ...state,
            isLoggedIn: true,
            isValidating: false,
            username: userData.username,
            userId: userData.id
          }));
          console.log('✅ Token valid, user authenticated');
          
          // Start auto-refresh schedule
          authStore.scheduleTokenRefresh();
        } else {
          console.warn('❌ Token invalid, logging out');
          authStore.logout();
        }
      } catch (error) {
        console.error('❌ Token validation failed:', error);
        authStore.logout();
      }
    },
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
            userId: data.user.id,
            isValidating: false
          });
          
          // Start auto-refresh after successful login
          authStore.scheduleTokenRefresh();
          
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
      // Clear auto-refresh timer
      if (refreshTimer) {
        clearInterval(refreshTimer);
        refreshTimer = null;
        console.log('⏰ Token auto-refresh stopped');
      }
      
      localStorage.removeItem('authToken');
      localStorage.removeItem('username');
      localStorage.removeItem('userId');
      set({
        isLoggedIn: false,
        token: null,
        username: null,
        userId: null,
        isValidating: false
      });
    }
  };

  // Validate token on store creation (app start)
  if (typeof window !== 'undefined') {
    authStore.validateToken();
  }
  
  return authStore;
}

export const auth = createAuthStore();
