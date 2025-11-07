import { writable } from 'svelte/store';
import { t } from '../i18n.js';

// TODO: Get from settings store once implemented
const getCurrentLanguage = () => localStorage.getItem('language') || 'de';

function createAuthStore() {
  // Clean up old demo tokens
  const storedToken = localStorage.getItem('authToken');
  const storedRefreshToken = localStorage.getItem('refreshToken');
  
  if (storedToken && storedToken.startsWith('demo-token-')) {
    console.warn('🧹 Cleaning up old demo token, please login again');
    localStorage.removeItem('authToken');
    localStorage.removeItem('refreshToken');
    localStorage.removeItem('username');
    localStorage.removeItem('userId');
  }

  // Auto-refresh timer
  let refreshTimer = null;
  let refreshTokenExpiryCheck = null;

  const { subscribe, set, update } = writable({
    isLoggedIn: false, // Start als ausgeloggt
    token: storedToken,
    refreshToken: storedRefreshToken,
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
          
          // Update both access token and refresh token
          localStorage.setItem('authToken', data.token);
          if (data.refresh_token) {
            localStorage.setItem('refreshToken', data.refresh_token);
          }
          
          update(state => ({
            ...state,
            token: data.token,
            refreshToken: data.refresh_token || state.refreshToken
          }));
          
          console.log('✅ Token refreshed successfully');
          
          // Schedule next refresh
          authStore.scheduleTokenRefresh();
          
          return true;
        } else {
          console.warn('❌ Token refresh failed (status:', response.status, '), logging out');
          authStore.logout();
          return false;
        }
      } catch (error) {
        console.error('❌ Token refresh error:', error);
        // Don't logout immediately on network errors, retry later
        console.log('⏰ Will retry token refresh in 1 minute...');
        setTimeout(() => authStore.refreshToken(), 60000);
        return false;
      }
    },
    // Schedule automatic token refresh
    scheduleTokenRefresh: () => {
      // Clear existing timers
      if (refreshTimer) {
        clearInterval(refreshTimer);
        refreshTimer = null;
      }
      if (refreshTokenExpiryCheck) {
        clearInterval(refreshTokenExpiryCheck);
        refreshTokenExpiryCheck = null;
      }

      // Refresh access token 5 minutes before its 24h expiry
      // JWT expires after 24 hours, so refresh after 23h 55m = 86100000ms
      const ACCESS_TOKEN_REFRESH_INTERVAL = 23 * 60 * 60 * 1000 + 55 * 60 * 1000; // 23h 55m
      
      refreshTimer = setInterval(() => {
        console.log('⏰ Auto-refresh triggered for access token');
        authStore.refreshToken();
      }, ACCESS_TOKEN_REFRESH_INTERVAL);
      
      // Also check refresh token expiry daily (refresh token expires after 7 days)
      // If refresh token expires in less than 1 day, force re-login
      const REFRESH_TOKEN_EXPIRY_CHECK_INTERVAL = 24 * 60 * 60 * 1000; // 24 hours
      refreshTokenExpiryCheck = setInterval(() => {
        console.log('⏰ Checking refresh token validity...');
        authStore.validateToken();
      }, REFRESH_TOKEN_EXPIRY_CHECK_INTERVAL);
      
      console.log(`⏰ Token auto-refresh scheduled (access: every ${(ACCESS_TOKEN_REFRESH_INTERVAL / 1000 / 60 / 60).toFixed(2)}h, refresh check: every 24h)`);
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
          
          // Store refresh token if provided
          if (data.refresh_token) {
            localStorage.setItem('refreshToken', data.refresh_token);
          }
          
          set({
            isLoggedIn: true,
            token: data.token,
            refreshToken: data.refresh_token,
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
      // Clear auto-refresh timers
      if (refreshTimer) {
        clearInterval(refreshTimer);
        refreshTimer = null;
      }
      if (refreshTokenExpiryCheck) {
        clearInterval(refreshTokenExpiryCheck);
        refreshTokenExpiryCheck = null;
      }
      console.log('⏰ Token auto-refresh stopped');
      
      localStorage.removeItem('authToken');
      localStorage.removeItem('refreshToken');
      localStorage.removeItem('username');
      localStorage.removeItem('userId');
      set({
        isLoggedIn: false,
        token: null,
        refreshToken: null,
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
