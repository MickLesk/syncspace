import { writable } from 'svelte/store';

function createAuthStore() {
  const { subscribe, set, update } = writable({
    isLoggedIn: !!localStorage.getItem('authToken'),
    token: localStorage.getItem('authToken'),
    username: localStorage.getItem('username'),
    userId: localStorage.getItem('userId')
  });

  return {
    subscribe,
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
        console.error('Login error:', error);
        // Fallback Demo Mode
        if (username === 'admin' && password === 'admin') {
          const demoToken = 'demo-token-' + Date.now();
          localStorage.setItem('authToken', demoToken);
          localStorage.setItem('username', username);
          localStorage.setItem('userId', 'demo-user');
          
          set({
            isLoggedIn: true,
            token: demoToken,
            username: username,
            userId: 'demo-user'
          });
          
          return { success: true, demo: true };
        }
        return { success: false, error: 'Backend offline - use admin/admin' };
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
}

export const auth = createAuthStore();
