import { writable, derived } from 'svelte/store';
import { toast } from './toast.js';
import { t } from '$lib/i18n.js';

/**
 * Theme Customization Store
 * Manages user-defined theme colors, fonts, and density settings
 */

const DEFAULT_THEME = {
  primaryColor: '#3B82F6',
  secondaryColor: '#10B981',
  accentColor: '#F59E0B',
  surfaceColor: '#FFFFFF',
  textColor: '#1F2937',
  borderColor: '#E5E7EB',
  
  // Dark mode variants
  darkSurfaceColor: '#111827',
  darkTextColor: '#F3F4F6',
  darkBorderColor: '#374151',
  
  // Fonts
  fontFamily: 'system-ui, -apple-system, sans-serif',
  fontSize: 'base', // sm, base, lg, xl
  fontWeight: 'normal', // light, normal, semibold
  
  // Density
  density: 'comfortable', // compact, comfortable, spacious
  borderRadius: 'base', // none, sm, base, lg, xl
  
  // Shadows
  shadowIntensity: 'normal', // light, normal, strong
};

const PRESET_THEMES = {
  default: {
    name: 'Default',
    description: 'Moderne, saubere Farben',
    ...DEFAULT_THEME,
  },
  ocean: {
    name: 'Ocean',
    description: 'Ruhige Blautöne',
    primaryColor: '#0369A1',
    secondaryColor: '#06B6D4',
    accentColor: '#14B8A6',
    ...DEFAULT_THEME,
  },
  forest: {
    name: 'Forest',
    description: 'Grüntöne der Natur',
    primaryColor: '#15803D',
    secondaryColor: '#16A34A',
    accentColor: '#84CC16',
    ...DEFAULT_THEME,
  },
  sunset: {
    name: 'Sunset',
    description: 'Warme, energische Farben',
    primaryColor: '#DC2626',
    secondaryColor: '#F97316',
    accentColor: '#FBBF24',
    ...DEFAULT_THEME,
  },
  cyberpunk: {
    name: 'Cyberpunk',
    description: 'Neon-Vibrationen',
    primaryColor: '#A855F7',
    secondaryColor: '#EC4899',
    accentColor: '#06B6D4',
    ...DEFAULT_THEME,
  },
};

function createThemeStore() {
  const stored = typeof window !== 'undefined' ? localStorage.getItem('userTheme') : null;
  const initialTheme = stored ? JSON.parse(stored) : DEFAULT_THEME;

  const { subscribe, set, update } = writable(initialTheme);

  return {
    subscribe,

    // Load theme from storage
    load() {
      if (typeof window !== 'undefined') {
        const stored = localStorage.getItem('userTheme');
        if (stored) {
          set(JSON.parse(stored));
          applyTheme(JSON.parse(stored));
        }
      }
    },

    // Set custom theme
    setTheme(theme) {
      try {
        set(theme);
        applyTheme(theme);
        localStorage.setItem('userTheme', JSON.stringify(theme));
        toast.success(t('theme.applied'));
      } catch (error) {
        toast.error(t('theme.error_applying'));
      }
    },

    // Apply preset theme
    applyPreset(presetKey) {
      const preset = PRESET_THEMES[presetKey];
      if (preset) {
        this.setTheme(preset);
      }
    },

    // Update single property
    updateProperty(key, value) {
      update(theme => {
        const updated = { ...theme, [key]: value };
        applyTheme(updated);
        localStorage.setItem('userTheme', JSON.stringify(updated));
        return updated;
      });
    },

    // Reset to default
    reset() {
      set(DEFAULT_THEME);
      applyTheme(DEFAULT_THEME);
      localStorage.removeItem('userTheme');
      toast.success(t('theme.reset'));
    },

    // Get all presets
    getPresets() {
      return PRESET_THEMES;
    },

    // Export theme
    export() {
      let theme = {};
      let themeExport = {};
      const unsubscribe = subscribe(t => {
        theme = t;
      });
      unsubscribe();

      const json = JSON.stringify(theme, null, 2);
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `theme-${new Date().getTime()}.json`;
      a.click();
      URL.revokeObjectURL(url);
    },

    // Import theme
    async import(file) {
      try {
        const text = await file.text();
        const imported = JSON.parse(text);
        
        // Validate imported theme
        if (!imported.primaryColor || !imported.fontSize) {
          throw new Error('Invalid theme file');
        }

        this.setTheme(imported);
        toast.success(t('theme.imported'));
      } catch (error) {
        toast.error(t('theme.import_failed'));
      }
    },
  };
}

/**
 * Apply theme to DOM
 */
function applyTheme(theme) {
  if (typeof document === 'undefined') return;

  const root = document.documentElement;
  
  // Set CSS custom properties
  root.style.setProperty('--color-primary', theme.primaryColor);
  root.style.setProperty('--color-secondary', theme.secondaryColor);
  root.style.setProperty('--color-accent', theme.accentColor);
  root.style.setProperty('--color-surface', theme.surfaceColor);
  root.style.setProperty('--color-text', theme.textColor);
  root.style.setProperty('--color-border', theme.borderColor);
  root.style.setProperty('--color-surface-dark', theme.darkSurfaceColor);
  root.style.setProperty('--color-text-dark', theme.darkTextColor);
  root.style.setProperty('--color-border-dark', theme.darkBorderColor);
  
  // Set font properties
  root.style.setProperty('--font-family', theme.fontFamily);
  
  // Set density data attribute for CSS targeting
  root.setAttribute('data-density', theme.density);
  root.setAttribute('data-font-size', theme.fontSize);
  root.setAttribute('data-border-radius', theme.borderRadius);
  root.setAttribute('data-shadow-intensity', theme.shadowIntensity);
}

// Create store
export const customTheme = createThemeStore();

// Derived store for current preset name
export const currentPresetName = derived(
  customTheme,
  ($theme) => {
    for (const [key, preset] of Object.entries(PRESET_THEMES)) {
      if (preset.primaryColor === $theme.primaryColor &&
          preset.secondaryColor === $theme.secondaryColor &&
          preset.accentColor === $theme.accentColor) {
        return key;
      }
    }
    return 'custom';
  }
);

// Export presets and defaults
export { PRESET_THEMES, DEFAULT_THEME };
