<div align="center">
  <img src=".github/images/logo-banner.png" alt="SyncSpace Logo" width="600" />
  
  # SyncSpace
  
  **Modern Material 3 File Synchronization Service**
  
  A self-hosted, cross-platform file sync solution with a beautiful Material Design 3 Expressive interface.  
  Built with Rust (backend) and Svelte 5 (frontend).
</div>

<div align="center">

![Version](https://img.shields.io/badge/version-0.3.0-blue)
![License](https://img.shields.io/badge/license-Apache--2.0-green)
![Material 3](https://img.shields.io/badge/Material-3%20Expressive-purple)
![Rust](https://img.shields.io/badge/Rust-axum%200.8-orange)
![Svelte](https://img.shields.io/badge/Svelte-5-red)

</div>

---

## 📂 Directory Structure

- `backend/` – Rust backend with REST API + WebSocket (migrating to **axum 0.7**)
- `frontend/` – Svelte 5 + Vite frontend with Material 3 styling
- `data/` – File storage (created automatically)
- `docs/` – Comprehensive documentation
- `scripts/` – Testing and utility scripts
- `start.ps1` / `start.bat` – One-command startup scripts

---

## ⚡ Quick Start

**One-Command Startup** (Windows):
```powershell
.\start.ps1   # PowerShell with process monitoring
```
```batch
start.bat     # Batch with auto-browser
```

**Manual Startup**:

1. **Backend:**
   ```bash
   cd backend
   cargo run --release
   ```
   Backend runs on `http://localhost:8080`

2. **Frontend:**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```
   Frontend runs on `http://localhost:5173`

3. **Login:**
   - Username: `admin`
   - Password: `admin`
   - _(Change immediately in Settings!)_

---

## ✨ Features

### 🎨 **Material 3 Expressive Design**

- Beautiful gradient app bar with smooth animations
- Adaptive dark/light theme with system integration
- Elevated cards with proper shadows and depth
- Material Design 3 color tokens and typography
- Responsive layout for desktop and mobile

### 🌍 **Internationalization**

- **English** and **German** translations
- Easy language switcher in app bar
- LocalStorage persistence
- Extensible translation system

### 📁 **File Management**

- **Drag & drop upload** with visual feedback
- **Breadcrumb navigation** for folder hierarchy
- **File operations**: Rename, Delete, Download
- **File Preview Modal** - Images, videos, PDFs, text files
- **Multi-Select Mode** - Bulk operations with checkboxes
- **Upload Progress Tracking** - Per-file progress bars
- Directory navigation with keyboard shortcuts
- File size display with proper formatting
- Icon-based file type indicators
- **Universal system file filtering** - Hides .git, .DS_Store, database files, etc.

A self-hosted, cross-platform file sync solution with a beautiful Material Design 3 Expressive interface. Built with Rust (backend) and Web Components (frontend).## Directory structure

![Version](https://img.shields.io/badge/version-0.2.0-blue)- `backend` – Rust backend exposing a REST API and WebSocket for file operations, peer management, search, rename and stats.

![License](https://img.shields.io/badge/license-Apache--2.0-green)- `frontend` – Material‑inspired web UI built with Lit. You can run it directly in a browser or embed it in Tauri or Electron.

![Material 3](https://img.shields.io/badge/Material-3%20Expressive-purple)- `desktop-app` – Placeholder for a Tauri configuration. A desktop app can embed the frontend here.

- `mobile-app` – Placeholder for a Flutter app. Use `flutter create` to generate the mobile client.

---

## Running locally

## ✨ Features

1. **Backend:** Navigate to `backend` and run the server with Cargo (requires Rust and Cargo installed):

### 🎨 **Material 3 Expressive Design**

- Beautiful gradient app bar with smooth animations ```bash

- Adaptive dark/light theme with system integration cd backend

- Elevated cards with proper shadows and depth cargo run

- Material Design 3 color tokens and typography ```

- Responsive layout for desktop and mobile

  The backend listens on `http://localhost:8080`. It automatically creates a `data` folder for synchronised files and a `config.json` for peers and settings.

### 🌍 **Internationalization**

- **English** and **German** translations2. **Frontend:** Open `frontend/index.html` in a browser or serve the `frontend` folder with a static file server (e.g. using `python -m http.server`). The UI connects to the backend at `http://localhost:8080` and `ws://localhost:8080`.

- Easy language switcher in app bar

- LocalStorage persistence## Features

- Extensible translation system

- **File browsing:** Navigate through directories, download files, rename or delete entries and create new folders.

### 📁 **File Management**- **Upload:** Upload files to any subfolder using the upload widget.

- **Drag & drop upload** with visual feedback- **Search:** Perform case‑insensitive searches across all files and directories.

- **Breadcrumb navigation** for folder hierarchy- **Stats:** View the total number of files and their combined size.

- **File operations**: Rename, Delete, Download, Preview- **Peers:** Add peers via the API; peer information is persisted in `config.json`.

- Directory navigation- **Live updates:** The backend emits file system events via WebSocket. The UI automatically refreshes on changes.

- File size display with proper formatting

- Icon-based file type indicators

### 🔐 **Security & Authentication**

- JWT-based authentication with Argon2 password hashing
- **Two-Factor Authentication (2FA)** with TOTP
- Rate limiting (5 attempts/minute)
- Secure password change
- Default admin account (admin/admin)

### 🔍 **Search & Organization**

- **Full-text search** powered by Tantivy (Rust search engine)
- **Fuzzy matching** - Find files even with typos (2-edit distance)
- **Content indexing** - Search inside text files, code, and PDFs
- **BM25 ranking** - Results sorted by relevance
- **Background indexing** - Non-blocking automatic indexing on upload/delete
- **Debounced search** - 300ms delay for smooth UX
- **40+ file types** supported (text, code, documents)
- Search results with file paths and metadata

### ⚡ **Real-Time Updates**

- WebSocket connections for live file events
- Automatic UI refresh on file changes
- File system monitoring with `notify` crate

### 🎯 **Modern Tech Stack**

- **Backend**: Rust with **axum 0.7** (migrating from warp), Tokio, async/await, SQLx (SQLite), Tantivy (search)
- **Frontend**: **Svelte 5** + Vite with TypeScript
- **Search**: Tantivy 0.22 (BM25 ranking, fuzzy matching, PDF extraction with lopdf)
- **Architecture**: REST API + WebSocket with tower middleware
- **Developer Experience**: Hot reload, startup scripts, organized project structure

---

## � Prerequisites

- **Rust 1.70+** ([Install Rust](https://rustup.rs/))
- **Node.js 18+** ([Install Node](https://nodejs.org/))
- Modern web browser (Chrome 119+, Firefox 121+, Safari 17+)

---

## 🛠️ Installation

### Option 1: One-Command Startup (Recommended)

**Windows:**
```powershell
# PowerShell (with process monitoring)
.\start.ps1

# Batch (with auto-browser)
start.bat
```

This will:
- Build and start the Rust backend on `localhost:8080`
- Install dependencies and start Vite dev server on `localhost:5173`
- Open browser automatically (batch version)
- Monitor processes (PowerShell version)

### Option 2: Manual Startup

**Terminal 1 - Backend:**
```bash
cd backend
cargo run --release
```

**Terminal 2 - Frontend:**
```bash
cd frontend
npm install
npm run dev
```

**Access:** Open browser to `http://localhost:5173`

**Default Login:**
- Username: `admin`
- Password: `admin`
- ⚠️ **Change immediately in Settings!**

---

## 📖 Usage

### File Upload

- **Drag & drop**: Drag files onto the drop zone (auto-opens for first-time users)
- **Upload progress**: Real-time progress bars for each file
- **Multiple files**: Upload multiple files simultaneously
- **First-time user experience**: Upload panel automatically opens on first visit

### File Preview

- **Click any file** to preview (instead of download)
- **Supported formats**:
  - Images: JPG, PNG, GIF, WebP, SVG, BMP
  - Videos: MP4, WebM, OGG
  - PDFs: Inline viewer
  - Text: TXT, MD, JSON, JS, CSS, HTML, XML, CSV
- **Keyboard navigation**:
  - Arrow keys: Previous/Next file
  - ESC: Close preview

### Multi-Select Operations

- **Toggle multi-select mode** via button in header
- **Checkboxes** appear on all files
- **Bulk operations**:
  - Select All
  - Deselect All
  - Delete Selected
- **Visual highlighting** for selected files

### Navigation

- Click folders to navigate into them
- Use breadcrumbs at the top to go back
- Home icon returns to root directory
- Universal system file filtering (hides .git, .DS_Store, database files)

### File Operations

- **Download**: Click download icon (in non-preview mode)
- **Preview**: Click file name
- **Rename**: Click edit icon, enter new name
- **Delete**: Click delete icon, confirm deletion
- **Multi-delete**: Use multi-select mode for bulk operations

### Settings

- **Theme**: Toggle dark/light mode
- **2FA**: Set up two-factor authentication
- **Password**: Change your password
- **Language**: Switch between English/German

---

## 🏗️ Architecture

### Backend (Rust)

```
backend/
├── src/
│   ├── main.rs      # API routes, WebSocket, file operations
│   └── auth.rs      # Authentication, JWT, 2FA, rate limiting
├── Cargo.toml       # Dependencies (axum 0.7, tower, tantivy)
└── data/            # File storage (auto-created)
    ├── syncspace.db       # SQLite database
    ├── search_index/      # Tantivy search index
    └── [user files]       # Uploaded files
```

**Migration Status**: 🔄 **Transitioning from warp 0.3 to axum 0.7**
- Dependencies updated
- Code migration in progress
- Enables native multipart upload support

**Key Dependencies:**

- `axum` 0.7 - Modern web framework with tower ecosystem
- `tower` / `tower-http` - Middleware (CORS, static files, tracing)
- `tokio` - Async runtime
- `jsonwebtoken` - JWT authentication
- `argon2` - Password hashing
- `totp-lite` - 2FA implementation
- `notify` - File system monitoring
- `tantivy` 0.22 - Full-text search engine
- `sqlx` - Async SQLite driver

### Frontend (Svelte 5)

```
frontend/
├── src/
│   ├── App.svelte                 # Main app component
│   ├── pages/
│   │   ├── Login.svelte          # Auth page
│   │   ├── FilesView.svelte      # File browser (preview, multi-select)
│   │   ├── Settings.svelte       # User settings
│   │   └── Search.svelte         # Search interface
│   ├── components/
│   │   ├── ui/
│   │   │   ├── PreviewModal.svelte  # File preview component
│   │   │   ├── AppBar.svelte        # Top navigation
│   │   │   └── Card.svelte          # Material card
│   │   └── FileItem.svelte       # File list item
│   ├── lib/
│   │   ├── api.js               # API client with upload progress
│   │   ├── auth.js              # Auth state management
│   │   └── i18n.js              # Internationalization
│   └── styles/
│       └── theme.css            # Material 3 tokens
├── index.html
├── vite.config.js
└── package.json
```

**Features:**

- Svelte 5 with runes and snippets
- Vite for fast HMR
- Material 3 design system
- TypeScript support
- Complete i18n (EN/DE)
- Upload progress tracking with `XMLHttpRequest`
- File preview modal with keyboard navigation
- Multi-select mode with bulk operations

---

## 🔐 Security

### Authentication Flow

1. User enters credentials
2. Backend validates with **Argon2** (memory-hard hashing)
3. Optional **2FA verification** (TOTP)
4. **JWT token** issued (24h expiration)
5. Token stored in `localStorage`
6. All API calls include `Authorization: Bearer <token>` header

### 2FA Setup

1. Navigate to **Settings** page
2. Click **"Setup 2FA"**
3. Scan **QR code** with authenticator app (Google Authenticator, Authy, etc.)
4. Enter **verification code** from app
5. 2FA is now **enabled** for your account

### Rate Limiting

- **5 login attempts per minute** per IP address
- Automatic cooldown after limit reached
- Prevents brute-force attacks

### Best Practices

⚠️ **IMPORTANT**: Change the default `admin/admin` credentials immediately after first login!

**Recommendations:**
- Use strong passwords (12+ characters, mixed case, numbers, symbols)
- Enable 2FA for all accounts
- Regularly update passwords
- Monitor login attempts
- Keep Rust dependencies up to date (`cargo update`)

---

## 🎨 Material 3 Design System

### Color Tokens

The app uses the complete Material 3 color system:

- **Primary**: Purple (`#6750A4`)
- **Secondary**: Lavender (`#625B71`)
- **Tertiary**: Rose (`#7D5260`)
- **Surface**: Adaptive (light/dark)
- **Error**: Red for warnings

### Typography

- Roboto font family
- Material typescale styles
- Proper hierarchy and readability

### Components Used

- `<md-filled-button>` - Primary actions
- `<md-filled-text-field>` - Input fields
- `<md-icon-button>` - Icon actions
- `<md-fab>` - Floating action button
- `<md-switch>` - Toggle switches
- Material Symbols Outlined icons

---

## 🌐 API Reference

### Authentication

- `POST /api/auth/register` - Create new user
- `POST /api/auth/login` - Login with credentials (+ optional 2FA)
- `GET /api/auth/me` - Get current user info
- `POST /api/auth/2fa/setup` - Generate 2FA secret + QR code
- `POST /api/auth/2fa/enable` - Enable 2FA with verification
- `POST /api/auth/2fa/disable` - Disable 2FA
- `POST /api/auth/change-password` - Change password

### Files (Protected)

- `GET /api/files/:path` - List directory entries (filters system files)
- `GET /api/file/:path` - Download file
- `POST /api/upload/:path` - Upload file (raw bytes)
- `POST /api/upload-multipart` - Upload with FormData + progress _(coming with axum)_
- `DELETE /api/files/:path` - Delete file/folder (recursive)
- `POST /api/dirs/:path` - Create directory (creates parents)
- `PUT /api/rename/:old-path` - Rename/move file (JSON: `{new_path: string}`)

### Search & Stats

- `GET /api/search?q=:query` - Full-text search with fuzzy matching
- `GET /api/stats` - File count and total size

### Configuration

- `GET /api/config` - Get current config
- `PUT /api/config` - Update config (JSON body)
- `GET /api/peers` - List known peers
- `POST /api/peers` - Add new peer

### Real-Time

- `GET /api/ws` - WebSocket upgrade for file system events
  - **Server → Client**: `{path: string, kind: string, timestamp: DateTime}`
  - Auto-refresh UI on file changes

---

## 🛠️ Configuration

### Backend Config (`config.json`)

```json
{
  "sync_dirs": ["./data"],
  "peers": [],
  "api_key": "optional-api-key"
}
```

Auto-created on first run in `backend/` directory.

### User Database (`syncspace.db`)

SQLite database with:
- User accounts (UUID, username, password hash)
- TOTP secrets (if 2FA enabled)
- Timestamps (created, last login)

Managed automatically by SQLx migrations.

### Search Index (`data/search_index/`)

Tantivy index directory:
- Auto-created on first search
- Background indexing on file changes
- Supports 40+ file types
- BM25 ranking with fuzzy matching

### System File Filtering

Automatically hides from all directory listings:
- Database files: `syncspace.db`, `*.db-shm`, `*.db-wal`
- Search index: `search_index/`, `.tantivy-*`
- Lock files: `*.lock`
- Version control: `.git/`
- OS files: `.DS_Store`, `Thumbs.db`

---

## 🚧 Roadmap

### Phase 1: Core Features ✅

- [x] Authentication with JWT
- [x] 2FA with TOTP
- [x] File upload/download
- [x] Directory navigation
- [x] Material 3 UI
- [x] Dark mode
- [x] i18n (EN/DE)
- [x] Full-text search with Tantivy
- [x] SQLite database integration

### Phase 2: Enhanced UX ✅

- [x] Drag & drop upload
- [x] Breadcrumb navigation
- [x] **File preview** (images, text, PDF, video)
- [x] **Multi-select mode** with bulk operations
- [x] **Upload progress indicators**
- [x] **System file filtering** (universal, all directories)
- [x] First-time user experience (auto-open upload panel)
- [x] Startup scripts (PowerShell + Batch)

### Phase 3: Framework Migration 🔄 (In Progress)

- [x] Dependencies updated (axum 0.7, tower ecosystem)
- [ ] Code migration from warp to axum
- [ ] **Multipart upload** with native axum support
- [ ] Upload progress API endpoint
- [ ] File context menus (right-click)
- [ ] Material dialogs for confirm actions

### Phase 4: Sync & Collaboration

- [ ] Peer-to-peer file synchronization
- [ ] Conflict resolution
- [ ] File versioning
- [ ] Shared folders
- [ ] User permissions
- [ ] Real-time collaboration indicators

### Phase 5: Advanced Features

- [ ] Mobile apps (Flutter)
- [ ] Desktop app (Tauri)
- [ ] Selective sync
- [ ] Encryption at rest
- [ ] Audit logging
- [ ] Bulk operations API
- [ ] Advanced search filters (type, date, size)
- [ ] Upload queue management (pause/resume/retry)
- [ ] Folder upload support

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. **Fork the repository**
2. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/syncspace.git
   cd syncspace
   ```
3. **Install dependencies:**
   ```bash
   # Backend dependencies (auto-installed by Cargo)
   cd backend
   cargo build
   
   # Frontend dependencies
   cd ../frontend
   npm install
   ```
4. **Create a feature branch:**
   ```bash
   git checkout -b feature/amazing-feature
   ```
5. **Start development servers:**
   ```powershell
   # Windows: Use startup scripts
   .\start.ps1
   
   # Or manually in separate terminals
   cd backend && cargo run --release
   cd frontend && npm run dev
   ```
6. **Make your changes and test thoroughly**
7. **Commit your changes:**
   ```bash
   git commit -m 'feat: add amazing feature'
   ```
8. **Push to your fork:**
   ```bash
   git push origin feature/amazing-feature
   ```
9. **Open a Pull Request**

### Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code formatting (no logic changes)
- `refactor:` Code restructuring (no behavior changes)
- `perf:` Performance improvements
- `test:` Adding or updating tests
- `chore:` Maintenance tasks (dependencies, configs)
- `ci:` CI/CD changes

**Examples:**
```bash
feat: add file preview modal with keyboard navigation
fix: resolve upload progress tracking race condition
docs: update README with axum migration status
refactor: migrate from warp to axum framework
```

### Code Style

**Rust:**
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Use `?` operator for error propagation
- Document public functions with `///` rustdoc comments

**JavaScript/Svelte:**
- Use ESLint configuration (provided)
- Prefer arrow functions for callbacks
- Use TypeScript for type safety
- Follow Svelte best practices (runes, snippets)
- Keep components focused and reusable

**General:**
- Write descriptive commit messages
- Add tests for new features
- Update documentation
- Keep PRs focused on single features/fixes

---

## � Documentation

**Essential Docs** (in `/docs` folder):

- **[QUICKSTART.md](docs/QUICKSTART.md)** - Get up and running in 5 minutes
- **[FEATURES.md](docs/FEATURES.md)** - Complete feature reference
- **[DATABASE.md](docs/DATABASE.md)** - SQLite schema and migrations
- **[SEARCH_FEATURE.md](docs/SEARCH_FEATURE.md)** - Tantivy search implementation
- **[AUTH_README.md](docs/AUTH_README.md)** - Authentication system details
- **[KEYBOARD_SHORTCUTS.md](docs/KEYBOARD_SHORTCUTS.md)** - Keyboard shortcuts reference
- **[ROADMAP.md](docs/ROADMAP.md)** - Future plans and development timeline

**Test Scripts** (in `/scripts` folder):

- `test-api.ps1` - Comprehensive API testing script
- `test-api-simple.ps1` - Basic API smoke tests

---

## 🔧 Troubleshooting

### Backend won't compile

**Issue:** Migration to axum in progress
```
error[E0599]: no method named `and` found for...
```

**Solution:** The codebase is currently being migrated from warp to axum. If you encounter compilation errors:
1. Ensure you're on the correct branch
2. Check that all dependencies in `Cargo.toml` are up to date
3. Run `cargo clean && cargo build`

### Frontend shows "Failed to fetch"

**Issue:** Backend not running or CORS misconfigured

**Solution:**
1. Ensure backend is running on `http://localhost:8080`
2. Check backend terminal for errors
3. Verify CORS is enabled in backend configuration

### Upload fails silently

**Issue:** Multipart upload endpoint not yet available

**Solution:**
- Currently only single-file upload supported
- Multipart upload coming with axum migration
- Check browser console for specific error messages

### Search returns no results

**Issue:** Search index not created

**Solution:**
1. Upload some files to trigger indexing
2. Check `data/search_index/` directory exists
3. Backend logs will show indexing progress
4. Wait a few seconds for background indexing to complete

---

## 🙏 Acknowledgments

- [Lit](https://lit.dev/) for web component inspiration
- [Material Design 3](https://m3.material.io/) by Google - Design system
- [Svelte](https://svelte.dev/) - Reactive UI framework
- [axum](https://github.com/tokio-rs/axum) - Modern web framework
- [Tantivy](https://github.com/quickwit-oss/tantivy) - Full-text search
- [Tower](https://github.com/tower-rs/tower) - Middleware ecosystem
- [Home Assistant](https://www.home-assistant.io/) - SPA design inspiration

---

## 📄 License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/MickLesk/syncspace/issues)
- **Discussions**: [GitHub Discussions](https://github.com/MickLesk/syncspace/discussions)
- **Documentation**: See `/docs` folder

---

**Made with ❤️ by [MickLesk](https://github.com/MickLesk)**

**Material 3 Expressive Design** • **Rust (axum) Backend** • **Svelte 5 Frontend** • **Self-Hosted Sync**

