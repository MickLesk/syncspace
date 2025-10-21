import { writable } from 'svelte/store';

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
          return { success: false, error: data.error || 'Invalid credentials' };
        }
      } catch (error) {
        console.error('❌ Login error:', error);
        return { success: false, error: 'Backend nicht erreichbar. Läuft der Server auf localhost:8080?' };
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
