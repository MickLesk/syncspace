# Store Refactoring Guide - Eliminate Client-Side State

## Problem

We had too many client-side stores violating Backend-First architecture:

```javascript
// ❌ WRONG - These should NOT exist (client-only state)
- customTheme.js       // Theme should come from backend
- preferences.js       // View preferences should sync from backend
- adminConsole.js      // Admin state should come from API
- ui.js               // UI state scattered everywhere
- userPreferences.js   // User prefs should be in DB
- And more...
```

## Solution

**ONE unified store that syncs with backend:**

```javascript
// ✅ RIGHT - Single source of truth
import {
  serverState,
  updateSettings,
  updatePreferences,
} from "./stores/serverState.js";
```

## How serverState.js Works

### Architecture

```
Backend Database
       ↓
   Backend API
       ↓
serverState Store (Svelte)
       ↓
   Components
```

### Data Flow

1. **On App Start**:

   ```javascript
   await initializeServerState();
   // Loads: settings, preferences, profile from backend
   ```

2. **When User Changes Theme**:

   ```javascript
   await updateSettings({ theme: "dark" });
   // API: PUT /api/users/settings { theme: 'dark' }
   // DB: INSERT INTO settings
   // Store: Updates automatically
   ```

3. **On Other Device**:
   ```javascript
   // WebSocket broadcast or next API call
   // Device sees: settings.theme = 'dark'
   // UI updates automatically
   ```

## Migration Steps

### Step 1: Identify Old Stores

```bash
# Find all the client-side stores to replace
ls -la frontend/src/stores/

# You should have ONLY these left:
# - auth.js (JWT token - OK)
# - serverState.js (NEW - replacement for all user state)
# - websocket.js (real-time events - OK)
# - Keep: files.js, search.js if they're for UI pagination state
```

### Step 2: Replace Theme Store

**Old Way** (`customTheme.js`):

```svelte
<script>
  import { customTheme } from './stores/customTheme.js';

  function toggleDark() {
    customTheme.toggleDarkMode(); // client-side only
  }
</script>
```

**New Way** (`serverState.js`):

```svelte
<script>
  import { serverState, updateSettings } from './stores/serverState.js';

  async function toggleDark() {
    await updateSettings({
      theme: $serverState.settings.theme === 'dark' ? 'light' : 'dark'
    });
  }
</script>

{#if $isDarkMode}
  <p>Dark mode is ON</p>
{/if}
```

### Step 3: Replace Preference Store

**Old Way** (`userPreferences.js`):

```svelte
<script>
  import { userPreferences } from './stores/userPreferences.js';

  function setViewMode(mode) {
    userPreferences.set({ viewMode: mode }); // client-side only
  }
</script>
```

**New Way** (`serverState.js`):

```svelte
<script>
  import { serverState, updatePreference } from './stores/serverState.js';

  async function setViewMode(mode) {
    await updatePreference('viewMode', mode); // syncs to backend
  }
</script>

{#if $serverState.preferences.viewMode === 'grid'}
  <GridView />
{/if}
```

### Step 4: Replace UI State Store

**Old Way** (`ui.js`):

```javascript
// ❌ Scattered state, no backend sync
localStorage.setItem("sidebarCollapsed", true);
export const sidebarCollapsed = writable(true);
```

**New Way** (`serverState.js`):

```javascript
// ✅ Backend-persisted preferences
updatePreference("sidebarCollapsed", true);
// Automatically syncs to DB, all devices get it
```

### Step 5: Replace Admin Console Store

**Old Way** (`adminConsole.js`):

```javascript
// ❌ Mock data, no backend
const adminData = {
  users: mockUsers,
  roles: mockRoles,
};
```

**New Way**:

```javascript
// ✅ Call backend API endpoints
const users = await api.users.listUsers();
const roles = await api.system.getRoles();
// Real data from DB, not mock
```

## Common Replacements

### Language Selection

```svelte
<!-- OLD -->
<select on:change={(e) => ui.setLanguage(e.target.value)}>
  <option value="en">English</option>
  <option value="de">Deutsch</option>
</select>

<!-- NEW -->
<select on:change={(e) => updateSettings({ language: e.target.value })}>
  <option value="en">English</option>
  <option value="de">Deutsch</option>
</select>

<!-- Access anywhere -->
{$currentLanguage} <!-- 'en' or 'de' -->
```

### Sidebar Collapse

```svelte
<!-- OLD -->
{#if !$sidebarCollapsed}
  <Sidebar />
{/if}

<!-- NEW -->
{#if !$serverState.preferences.sidebarCollapsed}
  <Sidebar />
{/if}

<!-- Toggle (syncs to backend) -->
<button on:click={() => updatePreference('sidebarCollapsed', !$serverState.preferences.sidebarCollapsed)}>
  Toggle Sidebar
</button>
```

### Recent Searches

```svelte
<!-- OLD - stuck in localStorage -->
{#each recentSearches as search}
  <button>{search}</button>
{/each}

<!-- NEW - syncs across devices -->
{#each $serverState.preferences.recentSearches as search}
  <button>{search}</button>
{/each}

<!-- Add search (auto-syncs) -->
<script>
  import { addRecentSearch } from './stores/serverState.js';

  function performSearch(query) {
    // Your search logic...
    addRecentSearch(query); // Backend will persist
  }
</script>
```

### View Settings

```svelte
<!-- OLD - no sync -->
<div class:grid={$viewMode === 'grid'}>
  {/* Files */}
</div>

<!-- NEW - syncs to all devices -->
<div class:grid={$serverState.preferences.viewMode === 'grid'}>
  {/* Files */}
</div>

<!-- Change view -->
<button on:click={() => updatePreference('viewMode', 'list')}>
  List View
</button>
<button on:click={() => updatePreference('viewMode', 'grid')}>
  Grid View
</button>
```

## Testing the Changes

### 1. Verify Backend State

After changing a setting, check backend:

```bash
# Check database
sqlite3 ./data/syncspace.db
> SELECT settings FROM users WHERE username = 'admin';
```

### 2. Test Multi-Device Sync

1. Open app in Firefox
2. Open app in Chrome (different browser = simulates device)
3. Change theme in Firefox
4. Refresh Chrome
5. Chrome should have new theme ✅

### 3. Test Offline Behavior

1. Open app
2. Close browser DevTools network
3. Change setting (should show spinner)
4. Enable network
5. Setting should sync ✅

## Stores to DELETE

After migration, these files should be removed:

```bash
# DELETE these (replaced by serverState.js)
rm frontend/src/stores/customTheme.js
rm frontend/src/stores/preferences.js
rm frontend/src/stores/adminConsole.js
rm frontend/src/stores/userPreferences.js
rm frontend/src/stores/ui.js

# Keep these
frontend/src/stores/auth.js           # JWT token
frontend/src/stores/serverState.js    # NEW - all user state
frontend/src/stores/websocket.js      # Real-time events
frontend/src/stores/uploadManager.js  # Upload queue (not user state)
frontend/src/stores/files.js          # File browser pagination
frontend/src/stores/search.js         # Search pagination
```

## Stores to KEEP

### auth.js - JWT Token

```javascript
// ✅ KEEP: Authentication token
import { auth } from "./stores/auth.js";

// Only localStorage item allowed per Backend-First rules
localStorage.setItem("authToken", token);
```

### websocket.js - Real-Time Updates

```javascript
// ✅ KEEP: WebSocket connection
import { websocketManager } from './stores/websocket.js';

// Receives broadcasts when other users make changes
ws.on('file_changed', (file) => {...});
```

### uploadManager.js - Upload Queue

```javascript
// ✅ KEEP: Temporary UI state for active uploads
import { uploadManager } from "./stores/uploadManager.js";

// This is ephemeral, not persisted to DB
addToUploadQueue(file);
```

## Verification Checklist

After refactoring:

- [ ] `serverState` loads on app start
- [ ] Theme changes persist in database
- [ ] Language preference persists in database
- [ ] Recent searches sync across devices
- [ ] Saved filters sync across devices
- [ ] Sidebar collapsed state persists
- [ ] No localStorage used except for JWT
- [ ] Old stores deleted
- [ ] All components updated
- [ ] Error checking passes
- [ ] App boots without errors

## Architecture Comparison

### Before (Wrong ❌)

```
Component A          Component B
   ↓                    ↓
localStorage       Separate Stores
   ↓                    ↓
Different values on different devices
No sync, chaos! 😱
```

### After (Correct ✅)

```
Component A    Component B    Component C
    ↓              ↓              ↓
  serverState (single store)
    ↓
Backend API
    ↓
Database
↓
All values identical across all devices
Perfect sync! 🎉
```

## Performance Impact

**Before**: Multiple stores, localStorage reads, cache invalidation needed
**After**: Single API call on init, all state synced, updates via optimistic update

**Result**: Faster, more consistent, less buggy.

## FAQ

**Q: Why move state to backend?**
A: Multi-device consistency, persistence, single source of truth, automatic sync.

**Q: Doesn't this add latency?**
A: Most operations are instant (optimistic update) + backend persists in background.

**Q: What about offline?**
A: IndexedDB queue + background sync via service worker (offline manager handles it).

**Q: Can I still use local stores?**
A: Only for temporary UI state (active uploads, editing, etc.) - never user preferences.

## Summary

1. **Replace all client-side stores** → `serverState.js`
2. **All user state comes from backend** → Multi-device sync automatic
3. **API calls persist everything** → No localStorage struggles
4. **Components just read/update** → Simple, predictable, testable

**Result**: True Backend-First architecture! 🚀
