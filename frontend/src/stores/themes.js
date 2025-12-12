import { writable } from 'svelte/store';
import { api } from '../lib/api.js';
import { auth } from './auth.js';

function createThemeStore() {
  const { subscribe, set, update } = writable({
    themes: [],
    activeTheme: null,
    presets: [],
    isLoading: false,
    error: null,
    hasUnsavedChanges: false,
    currentEditingTheme: null,
  });

  return {
    subscribe,
    
    // Load user's themes
    loadThemes: async () => {
      update(state => ({ ...state, isLoading: true, error: null }));
      
      try {
        const themes = await api.themes.getUserThemes();
        const presets = await api.themes.getPresets();
        
        update(state => ({ 
          ...state, 
          themes, 
          presets,
          isLoading: false 
        }));
        
        return themes;
      } catch (error) {
        console.error('Failed to load themes:', error);
        update(state => ({ 
          ...state, 
          error: error.message,
          isLoading: false 
        }));
        throw error;
      }
    },

    // Get active theme from user data
    setActiveTheme: (theme) => {
      update(state => ({ ...state, activeTheme: theme }));
      
      // Apply theme to document
      if (theme) {
        applyThemeToDocument(theme);
      }
    },

    // Create new theme
    createTheme: async (themeData) => {
      update(state => ({ ...state, isLoading: true, error: null }));
      
      try {
        const newTheme = await api.themes.createTheme(themeData);
        
        update(state => ({ 
          ...state,
          themes: [...state.themes, newTheme],
          isLoading: false 
        }));
        
        return newTheme;
      } catch (error) {
        console.error('Failed to create theme:', error);
        update(state => ({ 
          ...state, 
          error: error.message,
          isLoading: false 
        }));
        throw error;
      }
    },

    // Create theme from preset
    createFromPreset: async (preset, customName) => {
      const themeData = {
        theme_name: customName || preset.display_name,
        is_custom: true,
        primary_color: preset.primary_color,
        secondary_color: preset.secondary_color,
        accent_color: preset.accent_color,
        // Default values for other properties
        density: 'medium',
        font_size: 'medium',
        border_radius: 8,
        shadow_intensity: 5,
        glass_effect: true,
        animations: true,
        high_contrast: false,
      };

      return await this.createTheme(themeData);
    },

    // Update theme
    updateTheme: async (themeId, updates) => {
      update(state => ({ ...state, isLoading: true, error: null }));
      
      try {
        const updatedTheme = await api.themes.updateTheme(themeId, updates);
        
        update(state => ({
          ...state,
          themes: state.themes.map(t => t.id === themeId ? updatedTheme : t),
          activeTheme: state.activeTheme?.id === themeId ? updatedTheme : state.activeTheme,
          isLoading: false,
          hasUnsavedChanges: false
        }));
        
        // Re-apply theme if it's active
        if (state.activeTheme?.id === themeId) {
          applyThemeToDocument(updatedTheme);
        }
        
        return updatedTheme;
      } catch (error) {
        console.error('Failed to update theme:', error);
        update(state => ({ 
          ...state, 
          error: error.message,
          isLoading: false 
        }));
        throw error;
      }
    },

    // Delete theme
    deleteTheme: async (themeId) => {
      update(state => ({ ...state, isLoading: true, error: null }));
      
      try {
        await api.themes.deleteTheme(themeId);
        
        update(state => ({
          ...state,
          themes: state.themes.filter(t => t.id !== themeId),
          isLoading: false
        }));
      } catch (error) {
        console.error('Failed to delete theme:', error);
        update(state => ({ 
          ...state, 
          error: error.message,
          isLoading: false 
        }));
        throw error;
      }
    },

    // Activate theme
    activateTheme: async (themeId) => {
      update(state => ({ ...state, isLoading: true, error: null }));
      
      try {
        await api.themes.activateTheme(themeId);
        
        update(state => {
          const theme = state.themes.find(t => t.id === themeId);
          return {
            ...state,
            activeTheme: theme,
            isLoading: false
          };
        });

        // Apply theme to document
        const theme = get(themeStore).themes.find(t => t.id === themeId);
        if (theme) {
          applyThemeToDocument(theme);
        }
      } catch (error) {
        console.error('Failed to activate theme:', error);
        update(state => ({ 
          ...state, 
          error: error.message,
          isLoading: false 
        }));
        throw error;
      }
    },

    // Export theme
    exportTheme: async (themeId) => {
      try {
        const theme = await api.themes.exportTheme(themeId);
        
        // Create download
        const blob = new Blob([JSON.stringify(theme, null, 2)], { 
          type: 'application/json' 
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${theme.theme_name}.syncspace-theme.json`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      } catch (error) {
        console.error('Failed to export theme:', error);
        update(state => ({ ...state, error: error.message }));
        throw error;
      }
    },

    // Import theme
    importTheme: async (themeFile) => {
      try {
        const themeData = JSON.parse(await themeFile.text());
        
        // Remove ID and timestamps to create new theme
        delete themeData.id;
        delete themeData.created_at;
        delete themeData.updated_at;
        delete themeData.user_id;
        
        // Mark as custom and add import suffix if name exists
        themeData.is_custom = true;
        const existingNames = get(themeStore).themes.map(t => t.theme_name);
        if (existingNames.includes(themeData.theme_name)) {
          themeData.theme_name += ' (Imported)';
        }
        
        return await this.createTheme(themeData);
      } catch (error) {
        console.error('Failed to import theme:', error);
        update(state => ({ ...state, error: error.message }));
        throw error;
      }
    },

    // Set editing theme
    setEditingTheme: (theme) => {
      update(state => ({ 
        ...state, 
        currentEditingTheme: theme ? { ...theme } : null,
        hasUnsavedChanges: false
      }));
    },

    // Update editing theme (local only)
    updateEditingTheme: (updates) => {
      update(state => ({
        ...state,
        currentEditingTheme: state.currentEditingTheme ? 
          { ...state.currentEditingTheme, ...updates } : null,
        hasUnsavedChanges: true
      }));
    },

    // Save editing theme
    saveEditingTheme: async () => {
      const state = get(themeStore);
      if (!state.currentEditingTheme) return;
      
      const { id, ...updates } = state.currentEditingTheme;
      return await this.updateTheme(id, updates);
    },

    // Discard changes to editing theme
    discardEditingChanges: () => {
      const state = get(themeStore);
      if (state.currentEditingTheme) {
        const originalTheme = state.themes.find(t => t.id === state.currentEditingTheme.id);
        update(s => ({
          ...s,
          currentEditingTheme: originalTheme ? { ...originalTheme } : null,
          hasUnsavedChanges: false
        }));
      }
    },

    // Clear error
    clearError: () => {
      update(state => ({ ...state, error: null }));
    }
  };
}

// Helper function to apply theme to document
function applyThemeToDocument(theme) {
  if (!theme) return;

  const root = document.documentElement;
  
  // Apply CSS custom properties
  if (theme.primary_color) {
    root.style.setProperty('--primary-color', theme.primary_color);
  }
  if (theme.secondary_color) {
    root.style.setProperty('--secondary-color', theme.secondary_color);
  }
  if (theme.accent_color) {
    root.style.setProperty('--accent-color', theme.accent_color);
  }
  if (theme.background_color) {
    root.style.setProperty('--background-color', theme.background_color);
  }
  if (theme.surface_color) {
    root.style.setProperty('--surface-color', theme.surface_color);
  }
  if (theme.text_color) {
    root.style.setProperty('--text-color', theme.text_color);
  }
  
  // Layout properties
  if (theme.border_radius !== null) {
    root.style.setProperty('--border-radius', `${theme.border_radius}px`);
  }
  if (theme.shadow_intensity !== null) {
    root.style.setProperty('--shadow-intensity', theme.shadow_intensity);
  }
  
  // Font size
  if (theme.font_size) {
    const fontSizeMap = {
      'small': '14px',
      'medium': '16px',
      'large': '18px',
      'xlarge': '20px'
    };
    root.style.setProperty('--base-font-size', fontSizeMap[theme.font_size] || '16px');
  }
  
  // Density (spacing)
  if (theme.density) {
    const densityMap = {
      'compact': '0.75',
      'medium': '1',
      'comfortable': '1.25'
    };
    root.style.setProperty('--spacing-scale', densityMap[theme.density] || '1');
  }

  // Boolean properties as data attributes
  root.setAttribute('data-glass-effect', theme.glass_effect ? 'true' : 'false');
  root.setAttribute('data-animations', theme.animations ? 'true' : 'false');
  root.setAttribute('data-high-contrast', theme.high_contrast ? 'true' : 'false');
}

// Function to get theme store instance
function get(store) {
  let value;
  store.subscribe(v => value = v)();
  return value;
}

export const themeStore = createThemeStore();