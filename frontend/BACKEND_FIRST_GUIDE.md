# Backend-First Architecture Guide

## The Core Principle

**SyncSpace follows strict Backend-First philosophy:**

```
❌ WRONG: Client-side state management
❌ WRONG: localStorage for app state
❌ WRONG: Separate stores for theme/language/preferences
❌ WRONG: Data syncing between devices requires manual logic

✅ RIGHT: Single backend source of truth
✅ RIGHT: All user state in database
✅ RIGHT: Automatic multi-device sync
✅ RIGHT: One API call = persisted everywhere
```

## What NEVER Goes in localStorage

### ❌ DO NOT store locally:

- Theme preference (`localStorage.setItem('theme', 'dark')`)
- Language choice (`localStorage.setItem('language', 'de')`)
- User settings (notifications, display options, etc.)
- Preferences (view mode, sort order, etc.)
- Search history
- Custom filters
- UI state that should sync across devices
- Any user data

### ✅ ONLY localStorage allowed:

- JWT authentication token (`authToken`) - needed for offline auth headers
- That's it. Nothing else.

## Backend API Structure

### Settings Endpoint

**User Settings** = system configuration (theme, language, notifications)

```javascript
// GET /api/users/settings
{
  theme: 'dark',           // 'light' | 'dark' | 'auto'
  language: 'de',          // 'en' | 'de' | ...
  defaultView: 'grid',     // 'grid' | 'list' | 'compact'
  emailNotifications: true,
  pushNotifications: true,
  twoFactorEnabled: true,
  autoSync: true,
  darkModeStartTime: '20:00',
  darkModeEndTime: '08:00'
}

// PUT /api/users/settings
{ theme: 'dark', language: 'de' }
```

### Preferences Endpoint

**User Preferences** = view/interaction state (sorting, filtering, layout)

```javascript
// GET /api/users/preferences
{
  viewMode: 'grid',                    // 'grid' | 'list' | 'compact'
  sortBy: 'name',                      // 'name' | 'date' | 'size' | 'type'
  sortOrder: 'asc',                    // 'asc' | 'desc'
  sidebarCollapsed: false,
  recentSearches: ['pdf', 'budget'],
  savedFilters: [
    {
      id: '123',
      name: 'My PDFs',
      fileType: 'pdf',
      dateRange: 'last-7-days'
    }
  ],
  filePreviewSize: 'medium',           // 'small' | 'medium' | 'large'
  showHiddenFiles: false,
  thumbnailsEnabled: true,
  groupByType: false,
  defaultUploadFolder: '/',
  itemsPerPage: 50,
  enableAutoPlay: false,
  volumeLevel: 100
}

// PUT /api/users/preferences
{ viewMode: 'list', sortBy: 'date' }
```

### Profile Endpoint

**User Profile** = personal data (name, avatar, bio)

```javascript
// GET /api/users/profile
{
  id: 'uuid-123',
  username: 'john',
  email: 'john@example.com',
  displayName: 'John Doe',
  bio: 'Software developer',
  avatar: 'data:image/png;base64,...',
  role: 'user',
  createdAt: '2024-01-15T10:30:00Z',
  updatedAt: '2024-11-09T14:22:00Z'
}

// PUT /api/users/profile
{ displayName: 'Jane Doe', bio: 'Designer' }
```

## Using serverState Store

### 1. Initialize on App Startup

In `App.svelte`:

```svelte
<script>
  import { onMount } from 'svelte';
  import { initializeServerState } from './stores/serverState.js';

  onMount(async () => {
    // Load all user state from backend
    await initializeServerState();
  });
</script>
```

### 2. Read Current Values

```svelte
<script>
  import { serverState, isDarkMode, currentLanguage } from './stores/serverState.js';
</script>

<!-- Access via reactive store -->
<div class:dark={$isDarkMode}>
  Language: {$currentLanguage}
  Theme: {$serverState.settings.theme}
  View Mode: {$serverState.preferences.viewMode}
  User: {$serverState.profile.displayName}
</div>

<!-- Iterate preferences -->
{#each $serverState.preferences.recentSearches as search}
  <button>{search}</button>
{/each}
```

### 3. Update User Settings

```svelte
<script>
  import { updateSettings } from './stores/serverState.js';

  async function toggleDarkMode() {
    // Single update syncs to backend
    await updateSettings({
      theme: 'dark'
    });
    // Store updates automatically ✅
    // All other devices get update ✅
    // Database persists it ✅
  }

  async function changeLanguage(lang) {
    await updateSettings({ language: lang });
    // Page re-renders with new i18n
  }
</script>

<button on:click={toggleDarkMode}>Dark Mode</button>
<select on:change={(e) => changeLanguage(e.target.value)}>
  <option value="en">English</option>
  <option value="de">Deutsch</option>
</select>
```

### 4. Update User Preferences

```svelte
<script>
  import { updatePreference, updatePreferences } from './stores/serverState.js';

  // Single preference
  function setViewMode(mode) {
    updatePreference('viewMode', mode);
  }

  // Multiple preferences
  function applyViewSettings() {
    updatePreferences({
      viewMode: 'list',
      sortBy: 'date',
      sortOrder: 'desc',
      itemsPerPage: 100
    });
  }

  // Manage recent searches
  import { addRecentSearch } from './stores/serverState.js';

  function performSearch(query) {
    // Search logic...
    addRecentSearch(query);  // Auto-syncs to backend
  }
</script>
```

### 5. Update User Profile

```svelte
<script>
  import { updateProfile } from './stores/serverState.js';

  async function saveProfile() {
    await updateProfile({
      displayName: newDisplayName,
      bio: newBio,
      avatar: base64Avatar
    });
  }
</script>
```

## Common Patterns

### Theme Switching (Backend-Persisted)

❌ **OLD WAY** (don't do this):

```javascript
// WRONG: localStorage theme
localStorage.setItem("theme", "dark");
document.documentElement.setAttribute("data-theme", "dark");
```

✅ **NEW WAY** (Backend-First):

```svelte
<script>
  import { isDarkMode, updateSettings } from './stores/serverState.js';
</script>

<div class:dark={$isDarkMode}>
  <button on:click={() => updateSettings({ theme: 'dark' })}>
    Dark Mode
  </button>
</div>

<style>
  div.dark { background: #1a1a1a; }
</style>
```

**Syncs to Backend → All Devices Get It → Database Persists**

### Language Selection (Backend-Persisted)

❌ **OLD WAY**:

```javascript
localStorage.setItem("language", "de");
```

✅ **NEW WAY**:

```svelte
<script>
  import { currentLanguage, updateSettings } from './stores/serverState.js';
  import { i18n } from './lib/i18n.js';

  function changeLanguage(lang) {
    updateSettings({ language: lang });
    i18n.setLanguage(lang);
  }
</script>

Current: {$currentLanguage}
<button on:click={() => changeLanguage('de')}>Deutsch</button>
<button on:click={() => changeLanguage('en')}>English</button>
```

### View Mode Switching (Backend-Persisted)

❌ **OLD WAY**:

```javascript
localStorage.setItem("viewMode", "list");
```

✅ **NEW WAY**:

```svelte
<script>
  import { serverState, updatePreference } from './stores/serverState.js';
</script>

{#if $serverState.preferences.viewMode === 'grid'}
  <GridView />
{:else if $serverState.preferences.viewMode === 'list'}
  <ListView />
{/if}

<button on:click={() => updatePreference('viewMode', 'grid')}>
  Grid
</button>
<button on:click={() => updatePreference('viewMode', 'list')}>
  List
</button>
```

### Recent Searches (Backend-Persisted)

```svelte
<script>
  import { serverState, addRecentSearch, clearRecentSearches } from './stores/serverState.js';

  function search(query) {
    // Your search logic...
    addRecentSearch(query);  // Auto-synced to backend
  }
</script>

Recent Searches:
{#each $serverState.preferences.recentSearches as search}
  <button on:click={() => executeSearch(search)}>
    {search}
  </button>
{/each}

<button on:click={clearRecentSearches}>Clear History</button>
```

### Saved Filters (Backend-Persisted)

```svelte
<script>
  import { serverState, saveFilter, deleteFilter } from './stores/serverState.js';

  function createFilter() {
    saveFilter({
      name: 'My Documents',
      fileType: ['pdf', 'docx'],
      dateRange: 'last-30-days'
    });
  }
</script>

Saved Filters:
{#each $serverState.preferences.savedFilters as filter (filter.id)}
  <div>
    {filter.name}
    <button on:click={() => deleteFilter(filter.id)}>Delete</button>
  </div>
{/each}

<button on:click={createFilter}>+ New Filter</button>
```

### Sidebar Toggle (Backend-Persisted)

```svelte
<script>
  import { serverState, updatePreference } from './stores/serverState.js';
</script>

<div class:collapsed={$serverState.preferences.sidebarCollapsed}>
  <aside>Sidebar</aside>
</div>

<button on:click={() => updatePreference('sidebarCollapsed', !$serverState.preferences.sidebarCollapsed)}>
  Toggle Sidebar
</button>

<style>
  div.collapsed aside { display: none; }
</style>
```

## Multi-Device Sync Example

### Device A - Changes Theme

```javascript
// Device A
updateSettings({ theme: "dark" });
// API call: PUT /api/users/settings { theme: 'dark' }
// Backend: UPDATE users SET settings = {..., theme: 'dark'} WHERE id = user_id
// Database: Persists immediately ✅
```

### Device B - Auto-Synced

```
// Device B detects WebSocket broadcast of settings change
// OR Device B next time calls GET /api/users/settings
// Receives: { ..., theme: 'dark' }
// Store updates automatically ✅
// UI re-renders with dark theme ✅
```

### Device C - Late Sync

```
// Device C offline, comes back online
// WebSocket reconnects, fetches latest state
// Gets: { ..., theme: 'dark' }
// All devices in sync ✅
```

## Error Handling

```svelte
<script>
  import { serverState, error, loading, updateSettings } from './stores/serverState.js';

  async function changeTheme() {
    try {
      await updateSettings({ theme: 'dark' });
    } catch (err) {
      console.error('Failed:', err);
      // Show toast/snackbar
    }
  }
</script>

{#if $loading}
  <p>Saving...</p>
{/if}

{#if $error}
  <p class="error">{$error}</p>
{/if}

<button on:click={changeTheme} disabled={$loading}>
  {$loading ? 'Saving...' : 'Dark Mode'}
</button>
```

## Migration Checklist

If you have old client-side stores:

- [ ] Move `theme` from `localStorage` → `serverState.settings.theme`
- [ ] Move `language` from `localStorage` → `serverState.settings.language`
- [ ] Move `viewMode` from store → `serverState.preferences.viewMode`
- [ ] Move `sortBy` from store → `serverState.preferences.sortBy`
- [ ] Move `sidebarCollapsed` from store → `serverState.preferences.sidebarCollapsed`
- [ ] Move all searches/filters → `serverState.preferences.recentSearches/savedFilters`
- [ ] Remove separate stores: `customTheme.js`, `theme.js`, `ui.js` (keep only auth.js)
- [ ] Add `initializeServerState()` call in `App.svelte` onMount
- [ ] Update all components to use `serverState` instead of local stores
- [ ] Test multi-device sync (open app in 2 browsers, change setting on one, verify other updates)

## Performance Notes

**Backend Requests**:

- `initializeServerState()` = 3 parallel requests (settings + preferences + profile)
- Each `updateSettings()` = 1 request
- Each `updatePreference()` = 1 request
- `updatePreferences(multiple)` = 1 request (batch better!)

**Optimization**:

```javascript
// ❌ BAD: 3 separate requests
await updatePreference("viewMode", "list");
await updatePreference("sortBy", "date");
await updatePreference("sortOrder", "desc");

// ✅ GOOD: 1 request
await updatePreferences({
  viewMode: "list",
  sortBy: "date",
  sortOrder: "desc",
});
```

## WebSocket Real-Time Sync

When backend implements WebSocket:

```javascript
// Listen for preference changes on other devices
ws.on("preferences_updated", (data) => {
  preferences.set(data); // Auto-update
});

// Listen for settings changes on other devices
ws.on("settings_updated", (data) => {
  settings.set(data); // Auto-update
});
```

## Summary

**Backend-First means:**

1. ✅ User state lives in database
2. ✅ API endpoints are source of truth
3. ✅ Frontend syncs from backend
4. ✅ Changes auto-persist everywhere
5. ✅ Multi-device consistency guaranteed
6. ✅ No client-side state management needed

**Implementation**:

- Use `serverState` store for all user state
- Call `initializeServerState()` on app startup
- Use `updateSettings()`, `updatePreferences()`, etc. to change data
- Let the backend handle persistence
- Let WebSocket handle real-time sync

**Result**: One source of truth, automatic sync, zero synchronization bugs.
