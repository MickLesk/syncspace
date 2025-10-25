# SyncSpace Development Guide

## Project Architecture

**SyncSpace** is a modern self-hosted file synchronization service with a **Rust backend** (axum 0.8) and **Svelte 5 frontend** (Vite). The architecture follows a strict **Backend-First** pattern where ALL operations must go through the backend API to ensure multi-platform consistency.

**Core Stack**:

- **Backend**: Rust + axum 0.8 + SQLite + JWT auth (`backend/src/`)
- **Frontend**: Svelte 5 + Vite + TailwindCSS + DaisyUI (`frontend/src/`)
- **Database**: SQLite with comprehensive migrations (`backend/migrations/`)
- **Real-time**: WebSocket file system events (`ws://localhost:8080/api/ws`)

**Backend-First Philosophy**:

- ❌ **No localStorage/sessionStorage** for app state - use backend preferences
- ❌ **No direct file operations** in frontend - all via `/api/files/*`
- ❌ **No client-side only state** - everything synced to backend
- ✅ **All mutations** through backend APIs with WebSocket broadcasts
- ✅ **Multi-platform consistency** - settings sync across devices
- ✅ **Single source of truth** - backend owns all state

## Critical Development Workflows

### Running the Application

```bash
# Backend (Terminal 1)
cd backend && cargo run --release
# Serves on http://localhost:8080

# Frontend (Terminal 2)
cd frontend && npm run dev
# Serves on http://localhost:5173

# Default login: admin/admin (change in Settings!)
```

### Database Setup

- **Auto-migration**: Backend runs SQLite migrations on startup from `backend/migrations/`
- **Database path**: Always `./data/syncspace.db` (created if missing)
- **Pool**: Max 10 connections with WAL mode for concurrency
- **Test queries**: Use `backend/src/database.rs` models for type safety

### Adding New Features

**Backend API Endpoint**:

1. Add route in `backend/src/main.rs` router: `.route("/api/feature", get(handler))`
2. Create handler function returning `Result<impl IntoResponse, StatusCode>`
3. Use `State<AppState>` for database pool, auth, rate limiting
4. For mutations: broadcast `FileChangeEvent` via `state.fs_tx.send()`
5. Add database model in `backend/src/database.rs` with SQLx derives

**Frontend Component**:

1. Create `.svelte` file in `frontend/src/components/` or `frontend/src/pages/`
2. Import and register in `frontend/src/App.svelte` routing
3. Use `frontend/src/lib/api.js` for HTTP calls with automatic JWT headers
4. Follow Material 3 design tokens in `frontend/src/styles/design-tokens.css`
5. Use stores from `frontend/src/stores/` for global state

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

**Model Structure** (`backend/src/database.rs`):

```rust
#[derive(FromRow, Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub filename: String,
    pub file_path: String,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
    pub owner_id: Uuid,
}
```

**Query Patterns**:

- Use `sqlx::query_as!()` for type-safe queries
- Transactions: `pool.begin().await?` for multi-table operations
- Pagination: `LIMIT ? OFFSET ?` with `Query<PaginationParams>`
- Search: Full-text search via `tantivy` crate integration

## Frontend Architecture

**Svelte 5 Patterns**:

- **Runes**: Use `$state()`, `$derived()` for reactive state
- **Stores**: Global state in `frontend/src/stores/` (auth, ui, files)
- **API Integration**: `frontend/src/lib/api.js` exports categorized endpoints
- **Components**: Self-contained with props, events, and local state
- **Routing**: Hash-based routing in `App.svelte` with view switching

**Material 3 Design System**:

- **Tokens**: CSS custom properties in `design-tokens.css`
- **Colors**: Dynamic theme switching (light/dark) via `data-theme`
- **Typography**: Roboto font with proper scale and weights
- **Elevation**: Box shadows and surface colors for depth
- **Components**: Follow Material 3 specs with DaisyUI utility classes

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

- `main.rs` - Server setup, routing, middleware, core handlers (3000+ lines)
- `auth.rs` - JWT, password hashing, 2FA, rate limiting
- `database.rs` - SQLite models, connection pool, migrations runner
- `search.rs` - Full-text search with tantivy indexing
- `handlers/` - Feature-specific route handlers (sharing, backup, versioning)

**Frontend Structure** (`frontend/src/`):

- `App.svelte` - Main app shell with routing and authentication guard
- `lib/api.js` - HTTP client with JWT auth and error handling
- `stores/` - Reactive global state (auth, UI, file browser)
  - `preferences.js` - Backend-synchronized user preferences
  - `collaboration.js` - Real-time collaboration features
  - `conflictResolution.js` - Intelligent conflict resolution
- `components/` - Reusable UI components with Material 3 styling
- `pages/` - Full-page views (Files, Settings, Users, etc.)
- `utils/` - Helper utilities and migration tools

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
