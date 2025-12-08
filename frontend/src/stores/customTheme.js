import { writable, derived } from 'svelte/store';
import { showToast } from './toast.js';
import { t } from '../i18n.js';

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
  // Popular Developer Themes
  solarizedLight: {
    name: 'Solarized Light',
    description: 'Augenschonend, warm',
    primaryColor: '#268BD2',
    secondaryColor: '#2AA198',
    accentColor: '#CB4B16',
    surfaceColor: '#FDF6E3',
    textColor: '#657B83',
    borderColor: '#EEE8D5',
    darkSurfaceColor: '#002B36',
    darkTextColor: '#839496',
    darkBorderColor: '#073642',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    fontSize: 'base',
    fontWeight: 'normal',
    density: 'comfortable',
    borderRadius: 'base',
    shadowIntensity: 'light',
  },
  solarizedDark: {
    name: 'Solarized Dark',
    description: 'Augenschonend, dunkel',
    primaryColor: '#268BD2',
    secondaryColor: '#2AA198',
    accentColor: '#CB4B16',
    surfaceColor: '#FDF6E3',
    textColor: '#657B83',
    borderColor: '#EEE8D5',
    darkSurfaceColor: '#002B36',
    darkTextColor: '#839496',
    darkBorderColor: '#073642',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    fontSize: 'base',
    fontWeight: 'normal',
    density: 'comfortable',
    borderRadius: 'base',
    shadowIntensity: 'normal',
  },
  nord: {
    name: 'Nord',
    description: 'Arktische, kühle Töne',
    primaryColor: '#5E81AC',
    secondaryColor: '#81A1C1',
    accentColor: '#88C0D0',
    surfaceColor: '#ECEFF4',
    textColor: '#2E3440',
    borderColor: '#D8DEE9',
    darkSurfaceColor: '#2E3440',
    darkTextColor: '#ECEFF4',
    darkBorderColor: '#3B4252',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    fontSize: 'base',
    fontWeight: 'normal',
    density: 'comfortable',
    borderRadius: 'sm',
    shadowIntensity: 'light',
  },
  dracula: {
    name: 'Dracula',
    description: 'Dunkle Vampir-Ästhetik',
    primaryColor: '#BD93F9',
    secondaryColor: '#50FA7B',
    accentColor: '#FF79C6',
    surfaceColor: '#F8F8F2',
    textColor: '#282A36',
    borderColor: '#E0E0E0',
    darkSurfaceColor: '#282A36',
    darkTextColor: '#F8F8F2',
    darkBorderColor: '#44475A',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    fontSize: 'base',
    fontWeight: 'normal',
    density: 'comfortable',
    borderRadius: 'base',
    shadowIntensity: 'normal',
  },
  gruvbox: {
    name: 'Gruvbox',
    description: 'Retro, warme Farben',
    primaryColor: '#458588',
    secondaryColor: '#98971A',
    accentColor: '#D79921',
    surfaceColor: '#FBF1C7',
    textColor: '#3C3836',
    borderColor: '#EBDBB2',
    darkSurfaceColor: '#282828',
    darkTextColor: '#EBDBB2',
    darkBorderColor: '#3C3836',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    fontSize: 'base',
    fontWeight: 'normal',
    density: 'comfortable',
    borderRadius: 'sm',
    shadowIntensity: 'normal',
  },
  monokai: {
    name: 'Monokai',
    description: 'Klassischer Editor-Look',
    primaryColor: '#66D9EF',
    secondaryColor: '#A6E22E',
    accentColor: '#F92672',
    surfaceColor: '#F8F8F2',
    textColor: '#272822',
    borderColor: '#E0E0E0',
    darkSurfaceColor: '#272822',
    darkTextColor: '#F8F8F2',
    darkBorderColor: '#49483E',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    fontSize: 'base',
    fontWeight: 'normal',
    density: 'comfortable',
    borderRadius: 'base',
    shadowIntensity: 'normal',
  },
  catppuccin: {
    name: 'Catppuccin',
    description: 'Sanfte Pastelltöne',
    primaryColor: '#8CAAEE',
    secondaryColor: '#A6D189',
    accentColor: '#F4B8E4',
    surfaceColor: '#EFF1F5',
    textColor: '#4C4F69',
    borderColor: '#DCE0E8',
    darkSurfaceColor: '#303446',
    darkTextColor: '#C6D0F5',
    darkBorderColor: '#414559',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    fontSize: 'base',
    fontWeight: 'normal',
    density: 'comfortable',
    borderRadius: 'lg',
    shadowIntensity: 'light',
  },
  tokyoNight: {
    name: 'Tokyo Night',
    description: 'Urbane Nachtszene',
    primaryColor: '#7AA2F7',
    secondaryColor: '#9ECE6A',
    accentColor: '#BB9AF7',
    surfaceColor: '#D5D6DB',
    textColor: '#343B58',
    borderColor: '#9AA5CE',
    darkSurfaceColor: '#1A1B26',
    darkTextColor: '#A9B1D6',
    darkBorderColor: '#24283B',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    fontSize: 'base',
    fontWeight: 'normal',
    density: 'comfortable',
    borderRadius: 'base',
    shadowIntensity: 'normal',
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
