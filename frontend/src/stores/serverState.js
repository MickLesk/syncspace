/**
 * Server State Store - Backend-First Architecture
 * 
 * CRITICAL: This is the ONLY source of truth for all user state!
 * 
 * Backend-First Rules:
 * - ❌ NO localStorage for app state (only JWT token allowed)
 * - ❌ NO client-side only state
 * - ❌ NO separate stores for theme/language/preferences
 * - ✅ ALL state synced from backend APIs
 * - ✅ Multi-device consistency guaranteed
 * - ✅ Single source of truth in database
 * 
 * Backend API Endpoints (auto-persisted to DB):
 * - GET /api/users/settings → { theme, language, defaultView, ... }
 * - PUT /api/users/settings → { theme: 'dark', language: 'de', ... }
 * - GET /api/users/preferences → { viewMode, sortBy, sidebarCollapsed, ... }
 * - PUT /api/users/preferences → { [key]: value }
 * - GET /api/users/profile → { displayName, bio, avatar, ... }
 * 
 * Usage in Components:
 * ```svelte
 * import { serverState } from './stores/serverState.js';
 * 
 * // Access current values
 * {$serverState.settings.theme}
 * {$serverState.preferences.viewMode}
 * {$serverState.profile.displayName}
 * 
 * // Update (automatically syncs to backend)
 * serverState.updateSettings({ theme: 'dark' })
 * serverState.updatePreference('viewMode', 'list')
 * serverState.updateProfile({ displayName: 'New Name' })
 * ```
 */

import { writable, derived, readonly } from 'svelte/store';

// ============================================================================
// STATE STRUCTURE
// ============================================================================

/**
 * User Settings (Backend-Persisted)
 * 
 * Stored in: DB table `users` (settings_json column)
 * Endpoints:
 * - GET /api/users/settings
 * - PUT /api/users/settings
 * 
 * Fields:
 * - theme: 'light' | 'dark' | 'auto'
 * - language: 'en' | 'de'
 * - defaultView: 'grid' | 'list' | 'compact'
 * - emailNotifications: boolean
 * - pushNotifications: boolean
 * - twoFactorEnabled: boolean
 */
const defaultSettings = {
  theme: 'auto',
  language: 'en',
  defaultView: 'grid',
  emailNotifications: true,
  pushNotifications: true,
  twoFactorEnabled: false,
  autoSync: true,
  darkModeStartTime: '20:00',
  darkModeEndTime: '08:00',
};

/**
 * User Preferences (Backend-Persisted)
 * 
 * Stored in: DB table `user_preferences`
 * Endpoints:
 * - GET /api/users/preferences
 * - PUT /api/users/preferences
 * 
 * Fields:
 * - viewMode: 'grid' | 'list' | 'compact'
 * - sortBy: 'name' | 'date' | 'size' | 'type'
 * - sortOrder: 'asc' | 'desc'
 * - sidebarCollapsed: boolean
 * - recentSearches: string[]
 * - savedFilters: object[]
 * - filePreviewSize: 'small' | 'medium' | 'large'
 * - showHiddenFiles: boolean
 * - thumbnailsEnabled: boolean
 * - groupByType: boolean
 * - defaultUploadFolder: string
 * - itemsPerPage: number
 * - enableAutoPlay: boolean
 * - volumeLevel: number (0-100)
 */
const defaultPreferences = {
  viewMode: 'grid',
  sortBy: 'name',
  sortOrder: 'asc',
  sidebarCollapsed: false,
  recentSearches: [],
  savedFilters: [],
  filePreviewSize: 'medium',
  showHiddenFiles: false,
  thumbnailsEnabled: true,
  groupByType: false,
  defaultUploadFolder: '/',
  itemsPerPage: 50,
  enableAutoPlay: false,
  volumeLevel: 100,
};

/**
 * User Profile (Backend-Persisted)
 * 
 * Stored in: DB table `users`
 * Endpoints:
 * - GET /api/users/profile
 * - PUT /api/users/profile
 * 
 * Fields:
 * - id: string (UUID)
 * - username: string
 * - email: string
 * - displayName: string
 * - bio: string
 * - avatar: string (base64)
 * - role: 'user' | 'admin'
 * - createdAt: ISO string
 * - updatedAt: ISO string
 */
const defaultProfile = {
  id: null,
  username: '',
  email: '',
  displayName: '',
  bio: '',
  avatar: null,
  role: 'user',
  createdAt: null,
  updatedAt: null,
};

// ============================================================================
// STORE CREATION
// ============================================================================

/**
 * Internal writable stores
 * (not exposed to components - use readonly versions)
 */
const _settings = writable(defaultSettings);
const _preferences = writable(defaultPreferences);
const _profile = writable(defaultProfile);
const _loading = writable(false);
const _error = writable(null);

/**
 * Readonly exports for components
 */
export const settings = readonly(_settings);
export const preferences = readonly(_preferences);
export const profile = readonly(_profile);
export const loading = readonly(_loading);
export const error = readonly(_error);

/**
 * Derived stores for common use cases
 */
export const isDarkMode = derived(settings, ($settings) => {
  if ($settings.theme === 'light') return false;
  if ($settings.theme === 'dark') return true;
  // 'auto' mode: check system preference
  if (typeof window !== 'undefined') {
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  }
  return false;
});

export const currentLanguage = derived(settings, ($settings) => $settings.language);

/**
 * Combined server state for easy access
 */
export const serverState = derived(
  [_settings, _preferences, _profile],
  ([$settings, $preferences, $profile]) => ({
    settings: $settings,
    preferences: $preferences,
    profile: $profile,
  })
);

// ============================================================================
// API COMMUNICATION LAYER
// ============================================================================

const API_BASE = 'http://localhost:8080/api';

/**
 * Helper: Get JWT token
 */
function getAuthToken() {
  if (typeof window !== 'undefined') {
    return localStorage.getItem('authToken');
  }
  return null;
}

/**
 * Helper: Make authenticated request
 */
async function request(endpoint, options = {}) {
  const token = getAuthToken();

  const headers = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  _error.set(null);

  try {
    const response = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers,
    });

    if (response.status === 401) {
      localStorage.removeItem('authToken');
      if (typeof window !== 'undefined') {
        window.location.hash = '#/login';
      }
      throw new Error('Unauthorized: Token expired');
    }

    if (!response.ok) {
      const error = await response.json().catch(() => ({
        message: response.statusText,
      }));
      throw new Error(error.message || `HTTP ${response.status}`);
    }

    return await response.json();
  } catch (err) {
    _error.set(err.message);
    throw err;
  }
}

// ============================================================================
// PUBLIC API - STATE MANAGEMENT FUNCTIONS
// ============================================================================

/**
 * Load user state from backend on app initialization
 * Call this ONCE after login
 */
export async function initializeServerState() {
  _loading.set(true);

  try {
    // Load all 3 state sources in parallel
    const [settingsData, preferencesData, profileData] = await Promise.all([
      request('/users/settings'),
      request('/users/preferences'),
      request('/users/profile'),
    ]);

    _settings.set({
      ...defaultSettings,
      ...settingsData,
    });

    _preferences.set({
      ...defaultPreferences,
      ...preferencesData,
    });

    _profile.set({
      ...defaultProfile,
      ...profileData,
    });
  } catch (err) {
    _error.set(`Failed to initialize server state: ${err.message}`);
  } finally {
    _loading.set(false);
  }
}

/**
 * Update user settings (theme, language, notifications, etc.)
 * Syncs immediately to backend
 * 
 * @param {object} updates - Partial settings object
 * @example
 * updateSettings({ theme: 'dark', language: 'de' })
 */
export async function updateSettings(updates) {
  _loading.set(true);

  try {
    const updated = await request('/users/settings', {
      method: 'PUT',
      body: JSON.stringify(updates),
    });

    _settings.update((current) => ({
      ...current,
      ...updated,
    }));

    return updated;
  } catch (err) {
    _error.set(`Failed to update settings: ${err.message}`);
    throw err;
  } finally {
    _loading.set(false);
  }
}

/**
 * Update single user preference
 * Syncs immediately to backend
 * 
 * @param {string} key - Preference key
 * @param {any} value - New value
 * @example
 * updatePreference('viewMode', 'list')
 * updatePreference('sortBy', 'date')
 * updatePreference('sidebarCollapsed', true)
 */
export async function updatePreference(key, value) {
  _loading.set(true);

  try {
    const updated = await request('/users/preferences', {
      method: 'PUT',
      body: JSON.stringify({ [key]: value }),
    });

    _preferences.update((current) => ({
      ...current,
      ...updated,
    }));

    return updated;
  } catch (err) {
    _error.set(`Failed to update preference: ${err.message}`);
    throw err;
  } finally {
    _loading.set(false);
  }
}

/**
 * Update multiple user preferences at once
 * Syncs immediately to backend
 * 
 * @param {object} updates - Partial preferences object
 * @example
 * updatePreferences({ viewMode: 'list', sortBy: 'date', sortOrder: 'desc' })
 */
export async function updatePreferences(updates) {
  _loading.set(true);

  try {
    const updated = await request('/users/preferences', {
      method: 'PUT',
      body: JSON.stringify(updates),
    });

    _preferences.update((current) => ({
      ...current,
      ...updated,
    }));

    return updated;
  } catch (err) {
    _error.set(`Failed to update preferences: ${err.message}`);
    throw err;
  } finally {
    _loading.set(false);
  }
}

/**
 * Update user profile (displayName, bio, avatar)
 * Syncs immediately to backend
 * 
 * @param {object} updates - Partial profile object
 * @example
 * updateProfile({ displayName: 'John Doe', bio: 'Developer' })
 */
export async function updateProfile(updates) {
  _loading.set(true);

  try {
    const updated = await request('/users/profile', {
      method: 'PUT',
      body: JSON.stringify(updates),
    });

    _profile.update((current) => ({
      ...current,
      ...updated,
    }));

    return updated;
  } catch (err) {
    _error.set(`Failed to update profile: ${err.message}`);
    throw err;
  } finally {
    _loading.set(false);
  }
}

/**
 * Add to recent searches list
 * @param {string} query - Search query
 * @param {number} maxRecent - Max items to keep (default: 10)
 */
export async function addRecentSearch(query, maxRecent = 10) {
  let recentSearches;

  _preferences.update((current) => {
    // Add new search, remove duplicate if exists, limit to maxRecent
    let searches = current.recentSearches || [];
    searches = searches.filter((s) => s !== query);
    searches.unshift(query);
    searches = searches.slice(0, maxRecent);

    recentSearches = searches;

    return {
      ...current,
      recentSearches: searches,
    };
  });

  // Sync to backend
  try {
    await request('/users/preferences', {
      method: 'PUT',
      body: JSON.stringify({ recentSearches }),
    });
  } catch (err) {
    _error.set(`Failed to save recent search: ${err.message}`);
  }
}

/**
 * Clear recent searches
 */
export async function clearRecentSearches() {
  try {
    await updatePreference('recentSearches', []);
  } catch (err) {
    _error.set(`Failed to clear recent searches: ${err.message}`);
  }
}

/**
 * Save a custom filter for later
 * @param {object} filter - Filter configuration
 * @example
 * saveFilter({ name: 'My PDFs', fileType: 'pdf', dateRange: 'last-7-days' })
 */
export async function saveFilter(filter) {
  let savedFilters;

  _preferences.update((current) => {
    const filters = current.savedFilters || [];
    filters.push({
      ...filter,
      id: Date.now().toString(),
      createdAt: new Date().toISOString(),
    });

    savedFilters = filters;

    return {
      ...current,
      savedFilters: filters,
    };
  });

  try {
    await request('/users/preferences', {
      method: 'PUT',
      body: JSON.stringify({ savedFilters }),
    });
  } catch (err) {
    _error.set(`Failed to save filter: ${err.message}`);
  }
}

/**
 * Delete a saved filter
 * @param {string} filterId - Filter ID
 */
export async function deleteFilter(filterId) {
  let savedFilters;

  _preferences.update((current) => {
    const filters = (current.savedFilters || []).filter((f) => f.id !== filterId);
    savedFilters = filters;
    return {
      ...current,
      savedFilters: filters,
    };
  });

  try {
    await request('/users/preferences', {
      method: 'PUT',
      body: JSON.stringify({ savedFilters }),
    });
  } catch (err) {
    _error.set(`Failed to delete filter: ${err.message}`);
  }
}

/**
 * Reset all preferences to defaults
 */
export async function resetPreferences() {
  try {
    await updatePreferences(defaultPreferences);
  } catch (err) {
    _error.set(`Failed to reset preferences: ${err.message}`);
  }
}

/**
 * Reset all settings to defaults
 */
export async function resetSettings() {
  try {
    await updateSettings(defaultSettings);
  } catch (err) {
    _error.set(`Failed to reset settings: ${err.message}`);
  }
}

/**
 * Export state as JSON (for backup/transfer between devices)
 */
export function exportState() {
  let current = {
    settings: {},
    preferences: {},
    profile: {},
  };

  _settings.subscribe((v) => (current.settings = v));
  _preferences.subscribe((v) => (current.preferences = v));
  _profile.subscribe((v) => (current.profile = v));

  return JSON.stringify(current, null, 2);
}

/**
 * Toggle Dark Mode (Backend-First)
 * Switches theme between 'light' and 'dark'
 * Automatically syncs to backend
 */
export async function toggleDarkMode() {
  try {
    const currentTheme = _currentSettings.theme;
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    return await updateSettings({ theme: newTheme });
  } catch (err) {
    console.error('Failed to toggle dark mode:', err);
    throw err;
  }
}

/**
 * Get current state values (synchronous)
 * Use this only when you need immediate values outside of reactive context
 */
let _currentSettings = defaultSettings;
let _currentPreferences = defaultPreferences;
let _currentProfile = defaultProfile;

_settings.subscribe((v) => (_currentSettings = v));
_preferences.subscribe((v) => (_currentPreferences = v));
_profile.subscribe((v) => (_currentProfile = v));

export function getCurrentState() {
  return {
    settings: { ..._currentSettings },
    preferences: { ..._currentPreferences },
    profile: { ..._currentProfile },
  };
}

export default {
  settings,
  preferences,
  profile,
  loading,
  error,
  serverState,
  isDarkMode,
  currentLanguage,
  initializeServerState,
  updateSettings,
  updatePreference,
  updatePreferences,
  updateProfile,
  toggleDarkMode,
  addRecentSearch,
  clearRecentSearches,
  saveFilter,
  deleteFilter,
  resetPreferences,
  resetSettings,
  exportState,
  getCurrentState,
};
