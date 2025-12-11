# SyncSpace Development Guide

## Project Architecture

**SyncSpace** is a modern self-hosted file synchronization service with a **Rust backend** (axum 0.8) and **Svelte 5 frontend** (Vite + Tailwind v4). The architecture follows a strict **Backend-First** pattern where ALL operations must go through the backend API to ensure multi-platform consistency.

**Core Stack**:

- **Backend**: Rust + axum 0.8 + SQLite (SQLx) + JWT auth + Tantivy search (`backend/src/`)
- **Frontend**: Svelte 5 (runes) + Vite (Rolldown) + Tailwind CSS v4 (pure utility-first) + Bootstrap Icons (`frontend/src/`)
- **Database**: SQLite with 23+ migrations, WAL mode, connection pooling (`backend/migrations/`, `backend/src/database.rs`)
- **Real-time**: WebSocket file system events with `notify` crate (`ws://localhost:8080/api/ws`)
- **Search**: Tantivy 0.25 full-text search with fuzzy matching, BM25 ranking (`backend/src/search.rs`)

**Backend-First Philosophy**:

- ❌ **No localStorage/sessionStorage** for app state - use backend preferences
- ❌ **No direct file operations** in frontend - all via `/api/files/*`
- ❌ **No client-side only state** - everything synced to backend
- ✅ **All mutations** through backend APIs with WebSocket broadcasts
- ✅ **Multi-platform consistency** - settings sync across devices
- ✅ **Single source of truth** - backend owns all state

## Critical Development Workflows

### Terminal Management Rules

**CRITICAL: Only 2 Persistent Terminals Allowed**:

1. **Backend Terminal** (`powershell` named "Backend"):

   - Purpose: Run backend server only
   - If Rust code changed: `cd backend ; cargo run --release`
   - If NO Rust changes: `cd backend ; ./target/release/syncbackend.exe` (faster startup)
   - Never use for other commands (file operations, git, etc.)
   - Keep running during development

2. **Frontend Terminal** (`node` named "Frontend"):

   - Purpose: Run frontend dev server only
   - Command: `cd frontend ; npm run dev`
   - Never use for other commands (npm installs, file operations, etc.)
   - Keep running during development

3. **Temporary Terminals** (for all other operations):
   - File operations (move, copy, delete): Create NEW terminal, execute, then close
   - Git commands: Create NEW terminal, execute, then close
   - Database operations: Create NEW terminal, execute, then close
   - Package installs: Create NEW terminal, execute, then close
   - Always close after command completes (not background)

**Example Terminal Workflow**:

```bash
# WRONG: Using backend terminal for file operations
# (backend terminal) mv file.txt newfile.txt

# CORRECT: Create temporary terminal
# (new temporary terminal) mv file.txt newfile.txt ; exit

# WRONG: Using frontend terminal for npm install
# (frontend terminal) npm install new-package

# CORRECT: Create temporary terminal
# (new temporary terminal) cd frontend ; npm install new-package ; exit
```

**Backend Startup Decision Tree**:

- Rust files modified (\*.rs in backend/src/)? → `cargo run --release` (compiles + runs)
- No Rust changes, only frontend/config? → `./target/release/syncbackend.exe` (instant start)
- Uncertain if changed? → Check git status or use instant start (safer)

**Output Handling Rules (CRITICAL)**:

- ❌ **NEVER** use `2>&1` to redirect stderr to stdout
- ❌ **NEVER** use `Select-Object -Last X` or any output filtering
- ❌ **NEVER** suppress warnings or errors
- ✅ **ALWAYS** show complete, unfiltered output
- ✅ **ALWAYS** display all errors, warnings, and info messages
- ✅ **ALWAYS** let the user see full compiler/build output

**Example of WRONG output handling**:

```powershell
# WRONG: Suppresses errors and warnings
cargo build --release 2>&1 | Select-Object -Last 5

# WRONG: Hides stderr
npm install 2>$null
```

**Example of CORRECT output handling**:

```powershell
# CORRECT: Full output visible
cargo build --release

# CORRECT: All output streams shown
npm install
```

### Running the Application

```bash
# Backend (Terminal 1 - persistent)
cd backend && cargo run --release
# Or if no Rust changes: cd backend && ./target/release/syncbackend.exe
# Serves on http://localhost:8080

# Frontend (Terminal 2 - persistent)
cd frontend && npm run dev
# Serves on http://localhost:5173

# Default login: admin/admin (change in Settings!)

# All other operations use temporary terminals that are closed after completion
```

### Database Setup

- **Auto-migration**: Backend runs SQLite migrations on startup from `backend/migrations/`
- **Database path**: Always `./data/syncspace.db` (created if missing)
- **Pool**: Max 10 connections with WAL mode for concurrency
- **Test queries**: Use `backend/src/database.rs` models for type safety

### Adding New Features

**Backend API Endpoint**:

1. Add module in `backend/src/api/` directory (e.g., `my_feature.rs`)
2. Create `pub fn router() -> Router<AppState>` with routes
3. Create handler functions returning `Result<impl IntoResponse, StatusCode>`
4. Use `State<AppState>` for database pool, auth, rate limiting
5. Use `UserInfo` extractor for authenticated routes
6. For mutations: broadcast `FileChangeEvent` via `state.fs_tx.send()`
7. Register router in `backend/src/api/mod.rs` with `.merge(my_feature::router())`
8. Add database models in `backend/src/database.rs` with `#[derive(FromRow, Serialize, Deserialize)]`

**Frontend Component**:

1. Create `.svelte` file in `frontend/src/components/` or `frontend/src/pages/`
2. Import and register in `frontend/src/App.svelte` routing (hash-based)
3. Use `frontend/src/lib/api.js` for HTTP calls with automatic JWT headers
4. Use Tailwind v4 utilities + DaisyUI components for styling
5. Use Bootstrap Icons for icon elements: `<i class="bi bi-icon-name"></i>`
6. Use stores from `frontend/src/stores/` for global state
7. Follow Svelte 5 patterns: `$state()`, `$derived()`, `$effect()` for reactivity

## Backend-First API Patterns

**Complete File Operations** (All via Backend):

- `GET /api/files/{path}` - List directory contents
- `POST /api/upload/{path}` - Upload file
- `POST /api/dirs/{path}` - Create directory
- `PUT /api/rename/{path}` - Rename/move file
- `PUT /api/move/{path}` - Move file to new location
- `POST /api/copy/{path}` - Copy file to new location
- `DELETE /api/files/{path}` - Delete file/directory

**Enhanced Batch Operations** (Background processing with progress):

- `POST /api/batch/copy` - Batch copy with progress tracking
- `POST /api/batch/compress` - Batch compress into archives
- `GET /api/batch/operations/{job_id}` - Get operation status
- `POST /api/batch/operations/{job_id}/cancel` - Cancel operation

**Real-time Collaboration** (Multi-user coordination):

- `GET /api/collaboration/locks` - List file locks
- `POST /api/collaboration/locks` - Acquire file lock
- `DELETE /api/collaboration/locks/{file_path}` - Release lock
- `GET /api/collaboration/presence` - Get user presence
- `POST /api/collaboration/presence` - Update presence
- `GET /api/collaboration/activity` - Get collaboration activity

**User Preferences** (Replaces localStorage):

- `GET /api/users/preferences` - Get client preferences
- `PUT /api/users/preferences` - Update preferences
- Stores: view_mode, recent_searches, sidebar_collapsed, sort_by, sort_order
- Use `frontend/src/stores/preferences.js` store instead of localStorage

**Settings Synchronization**:

- `GET /api/users/settings` - Language, theme, default_view
- `PUT /api/users/settings` - Update user settings
- All settings persist across devices and platforms

## Authentication & Security

**JWT Implementation**:

- Login: `POST /api/auth/login` → JWT token in response
- Storage: Frontend stores token in `localStorage.getItem("authToken")`
- Headers: `Authorization: Bearer {token}` on all authenticated requests
- Validation: `backend/src/auth.rs` middleware extracts `UserInfo` from token
- 2FA: TOTP support with QR code setup (`/api/auth/setup-2fa`)

**Critical Security Patterns**:

- Rate limiting: `RateLimiter` in `AppState` prevents brute force
- Password hashing: Argon2 with salt (`backend/src/auth.rs`)
- CORS: Configured for `http://localhost:5173` in development
- Input validation: Always validate/sanitize file paths and user input

## Database Patterns

**SQLite Database** (`backend/src/database.rs`):

The `database.rs` file contains **ALL database models and connection setup**. It's the single source of truth for database schema.

**Key Models**:

- `User` - User accounts with UUID, username, password_hash, email, display_name, bio, avatar_base64, theme, language, default_view, 2FA settings
- `File` - File metadata with UUID, filename, file_path, size_bytes, mime_type, owner_id, timestamps
- `FileVersion` - File versioning with version_number, file_hash, size_bytes, created_by
- `Share` - File sharing with share_id, file_path, permission (read/write), expires_at, password
- `FileLock` - Collaboration file locking with locked_by, locked_at, expires_at
- `UserPreferences` - Client preferences (view_mode, sort_by, sort_order, sidebar_collapsed, recent_searches)
- `Activity` - Activity log with user_id, action, file_path, metadata
- `Notification` - User notifications with type, title, message, read status
- `Webhook` - Webhook integrations with URL, events, secret

**Query Patterns**:

- Use `sqlx::query_as!()` for compile-time verified queries
- Use `sqlx::query_as::<_, ModelName>()` for runtime queries
- Transactions: `pool.begin().await?` for multi-table operations
- Pagination: `LIMIT ? OFFSET ?` for large result sets
- Migrations: Auto-run on startup from `backend/migrations/*.sql` (23+ migrations)

**Database Connection**:

- Pool size: 10 connections with WAL mode for concurrency
- Path: `./data/syncspace.db` (relative to backend directory)
- Auto-creates database file if missing
- Runs migrations on every startup (idempotent)

## Frontend Architecture

**Svelte 5 Patterns**:

- **Runes**: Use `$state()`, `$derived()`, `$effect()` for reactive state
- **Stores**: Global state in `frontend/src/stores/` (auth, ui, files, preferences, collaboration)
- **API Integration**: `frontend/src/lib/api.js` exports categorized endpoints (files, auth, users, search, etc.)
- **Components**: Self-contained with props, events, and local state
- **Routing**: Hash-based routing in `App.svelte` with view switching (Files, Search, Settings, Profile, Users)

**Tailwind CSS v4**:

- **Framework**: Tailwind CSS v4 with `@tailwindcss/postcss` plugin (pure utility-first, no component library)
- **Icons**: Bootstrap Icons via npm package (`bootstrap-icons`)
- **Dark Mode**: CSS custom properties with `data-theme` attribute switching
- **Utility-First**: Pure Tailwind utilities for all styling (spacing, colors, typography, flexbox, grid)
- **Custom Classes**: Minimal custom CSS in component `<style>` blocks, prefer Tailwind utilities

**Advanced Frontend Features**:

- **File Preview**: Mammoth.js (DOCX), PrismJS (code highlighting), SheetJS (Excel), PDF.js, native video/image
- **Drag & Drop**: Native HTML5 Drag and Drop API with visual feedback
- **Multi-Select**: Checkbox-based selection with bulk operations (delete, move, copy)
- **Upload Progress**: XMLHttpRequest with progress events, real-time progress bars
- **Internationalization**: `frontend/src/lib/i18n.js` with EN/DE translations, localStorage persistence

## WebSocket Events & Real-time Updates

**Event Flow**:

1. Backend `notify` crate watches `./data/` directory
2. File changes → `broadcast::channel` → all WebSocket clients
3. Frontend `createWebSocket()` auto-reconnects and updates UI
4. Event format: `{path: string, kind: string, timestamp: string}`

**Integration Requirements**:

- **Mutation handlers**: Always call `state.fs_tx.send(FileChangeEvent)` after file operations
- **Frontend listeners**: Components subscribe to WebSocket for auto-refresh
- **Error handling**: WebSocket disconnection triggers UI notification

## Advanced Features & Patterns

**Real-time Collaboration**:

- File locking prevents concurrent edit conflicts
- User presence shows who's viewing/editing files
- Automatic lock renewal and cleanup
- Collaborative activity tracking
- Use `frontend/src/stores/collaboration.js` for integration

**Intelligent Conflict Resolution**:

- Automatic detection of edit conflicts, version mismatches
- Smart resolution strategies (auto-merge, rename, manual)
- Three-way merge for text files with conflict markers
- Background conflict resolution queue
- Use `frontend/src/stores/conflictResolution.js` for handling

**Enhanced Batch Operations**:

- Background processing with WebSocket progress updates
- Job queuing with cancellation support
- Automatic error recovery and retry logic
- Compression, bulk copy/move operations
- Use `api.batch.*` methods for bulk actions

**File Preview System**:

- **Universal Preview Modal**: Single component for all file types
- **Supported Formats**:
  - Images: JPG, PNG, GIF, WebP, SVG, BMP (native `<img>`)
  - Videos: MP4, WebM, OGG (native `<video>`)
  - PDFs: Inline viewer with PDF.js
  - Text: TXT, MD, JSON, JS, CSS, HTML, XML, CSV (syntax highlighting with PrismJS)
  - Documents: DOCX (Mammoth.js conversion to HTML)
  - Spreadsheets: XLSX, XLS (SheetJS with table rendering)
- **Keyboard Navigation**: Arrow keys for previous/next file, ESC to close
- **Automatic Format Detection**: Based on file extension

**Internationalization (i18n)**:

- **Languages**: English (default), German
- **Translation System**: `frontend/src/lib/i18n.js` with reactive stores
- **Storage**: Language preference saved in localStorage
- **Coverage**: All UI elements, error messages, tooltips
- **Usage**: `$t('key.path')` in components, `t()` in JavaScript

**Backend-First Migration Patterns**:

```javascript
// OLD: localStorage.setItem('setting', value)
// NEW: await userPreferences.updatePreference('setting', value)

// OLD: direct file operations
// NEW: all operations via /api/files/* endpoints

// OLD: client-only state
// NEW: backend-synchronized with WebSocket updates
```

## Module Organization

**Backend Modules** (`backend/src/`):

- `main.rs` - Server setup, routing, middleware, AppState configuration
- `auth.rs` - JWT, password hashing, 2FA (TOTP), rate limiting, UserInfo extractor
- `database.rs` - **CENTRAL FILE**: All SQLite models, connection pool, migrations (User, File, FileVersion, Share, FileLock, UserPreferences, Activity, Notification, Webhook)
- `search.rs` - Tantivy full-text search with fuzzy matching, BM25 ranking, PDF extraction
- `api/` - **Modular route handlers** (30+ modules):
  - `mod.rs` - Router aggregation and auth middleware application
  - `auth.rs` - Login, 2FA, password change (public + protected routes)
  - `users.rs` - User profile, settings, preferences (all with UserInfo)
  - `files.rs` - Upload, download, delete, list files
  - `directories.rs` - Create, navigate directories
  - `search.rs` - Full-text search endpoint
  - `sharing.rs` - File sharing with passwords and expiration
  - `activity.rs` - Activity logging and retrieval
  - `notifications.rs` - User notification management
  - `collaboration.rs` - File locking, presence, collaborative editing
  - `batch.rs` - Bulk operations (copy, compress, etc.)
  - `versions.rs` - File versioning and history
  - And 20+ more specialized modules...
- `services/` - Business logic layer:
  - `user_service_impl.rs` - User operations (get_profile, update_profile, get_settings, update_settings, get_preferences, update_preferences)
  - `file_service_impl.rs` - File operations
  - `search_service_impl.rs` - Search indexing and querying
  - `all_services_impl.rs` - Additional service implementations
- `middleware/` - Custom middleware (auth, rate limiting)
- `websocket/` - WebSocket event broadcasting with `notify` crate
- `performance.rs` - Caching, background jobs, performance monitoring

**Frontend Structure** (`frontend/src/`):

- `App.svelte` - Main app shell with hash-based routing, authentication guard, theme management
- `lib/` - Core utilities:
  - `api.js` - **CENTRAL HTTP CLIENT**: Categorized API methods (auth, files, users, search, batch, etc.) with automatic JWT headers
  - `i18n.js` - Internationalization with EN/DE translations
  - `utils.js` - Helper functions
- `stores/` - Svelte stores for global state:
  - `auth.js` - Authentication state (token, user info, login/logout)
  - `ui.js` - UI state (theme, language, sidebar)
  - `files.js` - File browser state (current directory, file list)
  - `preferences.js` - Backend-synchronized user preferences
  - `collaboration.js` - Real-time collaboration features
  - `conflictResolution.js` - Intelligent conflict resolution
- `pages/` - Full-page views:
  - `Login.svelte` - Login form with 2FA support
  - `FilesView.svelte` - File browser with drag & drop, multi-select, preview
  - `SearchView.svelte` - Search interface with fuzzy matching
  - `user/` - User management pages:
    - `UserSettingsView.svelte` - Settings (theme, language, default_view, notifications)
    - `UserProfileView.svelte` - Profile (display_name, bio, email, avatar)
    - `UsersView.svelte` - User list (admin only)
  - `ActivityView.svelte` - Activity log
  - `SecurityView.svelte` - 2FA setup, password change
- `components/` - Reusable UI components:
  - `ui/` - Generic UI components (AppBar, Sidebar, Modal, Toast)
  - `files/` - File-specific components (FileItem, FilePreview, UploadZone)
  - `search/` - Search components (SearchBar, SearchResult)
- `styles/` - Global styles:
  - `app.css` - Main stylesheet with Tailwind v4 imports
  - **NO** design-tokens.css (Tailwind handles everything)

## Performance & Optimization

**Backend Optimizations**:

- `spawn_blocking()` for CPU-intensive operations (search, file scanning)
- Connection pooling for SQLite with WAL mode
- Streaming responses for large file downloads
- Background tasks for indexing and cleanup operations

**Frontend Optimizations**:

- Vite bundling with tree shaking and code splitting
- Image lazy loading and thumbnail generation
- Virtual scrolling for large file lists
- WebSocket connection pooling and auto-reconnection

## Common Development Pitfalls

**Backend**:

- **Path safety**: Always validate paths to prevent directory traversal
- **Database transactions**: Use transactions for related operations
- **Error handling**: Convert all errors to HTTP status codes properly
- **File operations**: Check permissions and disk space before operations

**Frontend**:

- **API calls**: Always handle 401 responses for token expiry
- **Path encoding**: Use `encodeURIComponent()` for file paths in URLs
- **State updates**: Use Svelte runes correctly for reactivity
- **Memory leaks**: Unsubscribe from WebSocket and interval timers

## Accessibility (a11y) Requirements

**CRITICAL: All Svelte components MUST follow these accessibility patterns to avoid compiler warnings.**

### Buttons with Icons Only

```svelte
<!-- ❌ WRONG: No accessible label -->
<button onclick={close}>
  <i class="bi bi-x"></i>
</button>

<!-- ✅ CORRECT: Add aria-label -->
<button onclick={close} aria-label="Close">
  <i class="bi bi-x"></i>
</button>
```

### Modal Backdrops

```svelte
<!-- ❌ WRONG: div with onclick but no keyboard support -->
<div class="modal-backdrop" onclick={() => showModal = false}></div>

<!-- ✅ CORRECT: Add role, tabindex, and keyboard handler -->
<div
  class="modal-backdrop"
  role="button"
  tabindex="-1"
  onclick={() => showModal = false}
  onkeydown={(e) => e.key === 'Escape' && (showModal = false)}
></div>
```

### Form Labels

```svelte
<!-- ❌ WRONG: Label not associated with control -->
<label class="label">
  <span class="label-text">Username</span>
</label>
<input type="text" bind:value={username} />

<!-- ✅ CORRECT Option 1: Wrap input in label -->
<label class="label">
  <span class="label-text">Username</span>
  <input type="text" bind:value={username} />
</label>

<!-- ✅ CORRECT Option 2: Use for/id -->
<label for="username" class="label">
  <span class="label-text">Username</span>
</label>
<input id="username" type="text" bind:value={username} />

<!-- ✅ CORRECT Option 3: Use aria-label on input (for visual-only labels) -->
<div class="label">
  <span class="label-text">Username</span>
</div>
<input type="text" bind:value={username} aria-label="Username" />
```

### Interactive Elements Checklist

When creating UI components, always verify:

1. **Buttons**: Have visible text OR `aria-label`
2. **Links**: Have visible text OR `aria-label`
3. **Inputs**: Have associated `<label>` OR `aria-label`
4. **Clickable divs**: Must have `role="button"`, `tabindex`, and `onkeydown`
5. **Images**: Have `alt` attribute (empty `alt=""` for decorative images)
6. **Modals**: Close on Escape key, trap focus inside
7. **Icons**: Decorative icons use `aria-hidden="true"`

## Testing & Debugging

**Backend Testing**:

```bash
cd backend && cargo test
# Integration tests in tests/ directory
# Unit tests in individual modules
```

**Frontend Development**:

```bash
cd frontend && npm run dev
# Hot reload on http://localhost:5173
# Browser dev tools for debugging
```

**Database Inspection**:

```bash
sqlite3 ./data/syncspace.db ".tables"
sqlite3 ./data/syncspace.db "SELECT * FROM users;"
```

## Migration & Deployment Notes

- **Data persistence**: `./data/` directory contains files, database, and search index
- **Configuration**: SQLite settings and app config stored in database
- **Backup strategy**: Regular SQLite backups + file system snapshots recommended
- **Environment**: Currently development-focused (localhost, debug logging)
