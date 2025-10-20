# SyncSpace Development Guide

## Project Vision

**SyncSpace** is a modern, cross-platform, self-hosted file synchronization service inspired by Syncthing and DaemonSync. Unlike Syncthing's decentralized approach, SyncSpace focuses on **local-first device synchronization** with a strong, expressive web interface featuring real-time file monitoring, multi-peer config, and WebSocket-based live updates.

**Core Philosophy**: Simple single-binary deployment, high-performance Rust backend, Material 3 expressive UI, modular architecture prepared for future mobile/desktop clients.

## Architecture Overview

SyncSpace has **four deployment targets**:

- **Backend**: Rust/Tokio/Warp REST+WebSocket server (`backend/src/main.rs`) - the single source of truth
- **Frontend**: Lit web components with Material 3 expressive styling (`frontend/`) - zero build tools
- **Desktop**: Tauri wrapper (placeholder in `desktop-app/`)
- **Mobile**: Flutter app (placeholder in `mobile-app/`)

The backend owns all state. The frontend is a **stateless UI** that connects to `localhost:8080` via HTTP and `ws://localhost:8080/api/ws` for live updates.

## Critical Data Flows

### File System Events → WebSocket Broadcasting

1. `notify` crate watches `./data` directory recursively
2. FS events → `broadcast::channel` → all connected WebSocket clients
3. Frontend's `initWebSocket()` auto-refreshes UI on any event
4. **Pattern**: Every mutating operation (upload, delete, rename, mkdir) manually broadcasts a `FileChangeEvent` to the channel

### Complete API Reference

All endpoints follow `/api/{resource}/{path-tail}` convention. The backend serves as a **stateless HTTP API** with persistent config in `config.json`:

**File Operations**:

- `GET /api/files/{subpath}` - List directory entries (returns `EntryInfo[]`)
- `GET /api/file/{subpath}` - Download file content (binary response)
- `POST /api/upload/{subpath}` - Upload file (body = raw bytes)
- `DELETE /api/files/{subpath}` - Remove file/directory (recursive for dirs)
- `POST /api/dirs/{subpath}` - Create directory (creates parents automatically)
- `PUT /api/rename/{old-path}` - Rename/move file (JSON body: `{new_path: string}`)

**Discovery & Metadata**:

- `GET /api/search?q=term` - Case-insensitive filename search (uses `walkdir`)
- `GET /api/stats` - File count and total size (computed via `spawn_blocking`)

**Configuration & Peers**:

- `GET /api/config` - Get current config (peers, api_key, sync_dirs)
- `PUT /api/config` - Update config (JSON body: full `Config` object)
- `GET /api/peers` - List known peers
- `POST /api/peers` - Add new peer (JSON body: `{id: Uuid, address: string}`)

**Real-time Events**:

- `GET /api/ws` - WebSocket upgrade for file system events

## Running the Application

**Backend**:

```bash
cd backend
cargo run  # Starts on http://localhost:8080, creates ./data and ./config.json
```

**Frontend** (development):

```bash
cd frontend
python -m http.server 8000  # Or open index.html directly in browser
```

**Desktop app** (requires npm):

```bash
cd desktop-app
# tauri.conf.json expects npm commands (beforeDevCommand/beforeBuildCommand)
# Currently references ../frontend directly
```

## Key Conventions

### Backend (Rust)

- **Single-file architecture**: All 556 lines in `main.rs` (no modules)
- **DATA_DIR constant**: Hardcoded to `"./data"` - all file operations relative to this
- **Config persistence**: `config.json` stores peers and optional `api_key` (not enforced yet)
- **CORS**: Enabled for all origins via `.with(warp::cors().allow_any_origin())`
- **Error handling**: Routes return `Result<impl warp::Reply, Infallible>` - errors converted to HTTP status codes

### Frontend (Lit)

- **Single web component**: `<my-app>` in `main.js` (482 lines)
- **No build step**: Imports Lit directly from CDN (`https://cdn.jsdelivr.net/gh/lit/dist@3/core/lit-all.min.js`)
- **Material 3 styling**: CSS custom properties with gradient header (`--primary-color: #6750a4`)
- **Path navigation**: `this.currentPath` tracks current directory, nav breadcrumbs use `navigateTo(path)`
- **State management**: All state in LitElement properties (`files`, `peers`, `stats`, `searchResults`)

### WebSocket Protocol

- **Server → Client only**: Client never sends messages (recv_task ignores input)
- **Event format**: JSON `{path: string, kind: string, timestamp: DateTime<Utc>}`
- **Client reconnection**: Not implemented - refresh page if connection drops

## Debugging & Development Patterns

### Adding a new API endpoint

1. Define route in `routes()` function using warp filters
2. Add handler function (async, returns `Result<impl warp::Reply, Infallible>`)
3. If mutation, broadcast `FileChangeEvent` to `fs_tx.clone()`
4. Include in final `.or()` chain before `.with(warp::cors())`

### Frontend UI patterns

- **File list item**: Uses `@click` handlers on styled divs (no semantic HTML buttons)
- **Prompts for input**: Native `prompt()` for rename/mkdir (not custom modals)
- **Upload**: `<input type="file">` with `@change` handler, reads as `ArrayBuffer`
- **Cards**: `.card` class with `border-radius: 24px` and box-shadow
- **Material 3 Expressive**: Inspired by lovelace-material-components with vibrant gradients, soft shadows, rounded corners

### Example Event Flow

1. User uploads file `example.pdf` via frontend
2. Backend writes to `./data/example.pdf`
3. `notify` watcher detects file creation → broadcasts via WebSocket
4. Frontend receives event → auto-refreshes file list and stats
5. User clicks rename → sends `PUT /api/rename` → broadcasts event → refreshes UI

### Common pitfalls

- **Path encoding**: Always use `encodeURIComponent()` in frontend URLs
- **Blocking operations**: `walkdir` search/stats run in `spawn_blocking()` to avoid blocking executor
- **WebSocket lifetime**: Client doesn't auto-reconnect - must refresh page

## Dependencies

- **Backend**: `warp` (web), `notify` (FS watching), `tokio` (async runtime), `walkdir` (recursive search), `uuid`, `chrono`, `serde`/`serde_json`
- **Frontend**: Lit 3.x (web components via CDN), Roboto font (Google Fonts)
- **No package.json**: Frontend has no npm dependencies or build process - pure ES modules

## Code Style & Guidelines

### Rust

- Use `?` operator for error propagation (all routes return `Result<impl warp::Reply, Infallible>`)
- Avoid blocking I/O in async tasks (use `tokio::task::spawn_blocking` for `walkdir`)
- Document public functions with `///` rustdoc comments
- Follow Rust 2024 edition guidelines

### JavaScript (Lit)

- Use arrow functions for event callbacks: `@click=${() => this.handleClick()}`
- Keep functions pure when possible
- Use Lit `@state()` decorators for reactive properties
- No inline `onclick`; always bind via `@click=${...}`

### CSS / Material 3

- Use CSS custom properties: `--primary-color`, `--secondary-color`, `--surface-color`
- Consistent spacing (8px baseline grid implied by 16px/24px padding)
- Expressive shadow levels: `box-shadow: 0 4px 12px rgba(0,0,0,0.08)` for cards
- Prefer `border-radius: 24px` for cards, `12px` for smaller elements
- Gradient headers: `linear-gradient(135deg, #667eea 0%, #764ba2 100%)`

## Security & Performance Notes

- **API key**: `api_key` field exists in `config.json` but not enforced yet (no auth middleware)
- **CORS**: Currently allows all origins - restrict for production use
- **Blocking operations**: Search and stats use `spawn_blocking` to avoid blocking Tokio executor
- **Rate limiting**: Not implemented - add before WAN exposure
- **Encryption**: Data at rest and in transit not encrypted - future enhancement

## Future Integration Points

- Peer-to-peer sync not implemented (peers stored but unused - planned TCP/QUIC mesh)
- Mobile app placeholder expects REST API at `localhost:8080` (needs networking permissions)
- Desktop app (Tauri) requires `npm` commands defined in `tauri.conf.json`
- Version control / file history not implemented
- Database/index for file metadata (currently computed on-demand)
