<div align="center"><div align="center"><div align="center"><div align="center">

# 🚀 SyncSpace# 🚀 SyncSpace <img src=".github/images/logo-banner.png" alt="SyncSpace Logo" width="600" />

**Modern Self-Hosted File Synchronization\*\***Modern Self-Hosted File Synchronization\*\*# 🚀 SyncSpace

Fast, secure, and beautiful file sync built with Rust + Svelte 5.Fast, secure, and beautiful file sync built with Rust + Svelte 5. # SyncSpace

[![Version](https://img.shields.io/badge/version-0.3.0-blue)](https://github.com/MickLesk/syncspace)[![Version](https://img.shields.io/badge/version-0.3.0-blue)](https://github.com/MickLesk/syncspace)**Modern Self-Hosted File Synchronization Service**

[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)

[![Rust](https://img.shields.io/badge/Rust-axum%200.8-orange)](https://github.com/tokio-rs/axum)[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)

[![Svelte](https://img.shields.io/badge/Svelte-5-red)](https://svelte.dev)

[![Tailwind](https://img.shields.io/badge/Tailwind-v4-38bdf8)](https://tailwindcss.com)[![Rust](https://img.shields.io/badge/Rust-axum%200.8-orange)](https://github.com/tokio-rs/axum) **Modern Material 3 File Synchronization Service**

</div>[![Svelte](https://img.shields.io/badge/Svelte-5-red)](https://svelte.dev)

---[![Tailwind](https://img.shields.io/badge/Tailwind-v4-38bdf8)](https://tailwindcss.com)A beautiful, fast, and secure file sync solution built with Rust + Svelte 5.

## ⚡ Quick Start</div> A self-hosted, cross-platform file sync solution with a beautiful Material Design 3 Expressive interface.

`````bash---[![Version](https://img.shields.io/badge/version-0.3.0-blue)](https://github.com/MickLesk/syncspace) Built with Rust (backend) and Svelte 5 (frontend).

# Backend

cd backend && cargo run --release## ⚡ Quick Start[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)</div>



# Frontend (new terminal)````bash[![Rust](https://img.shields.io/badge/Rust-axum%200.8-orange)](https://github.com/tokio-rs/axum)

cd frontend && npm install && npm run dev

```# Backend



**Access**: `http://localhost:5173`  cd backend && cargo run --release[![Svelte](https://img.shields.io/badge/Svelte-5-red)](https://svelte.dev)<div align="center">

**Login**: `admin` / `admin` _(change immediately!)_



---

# Frontend (new terminal)[![Tailwind](https://img.shields.io/badge/Tailwind-v4-38bdf8)](https://tailwindcss.com)

## ✨ Features

cd frontend && npm install && npm run dev

- 🎨 **Modern UI** - Tailwind CSS v4, responsive design

- 🔐 **Secure** - JWT + 2FA (TOTP), Argon2 hashing```![Version](https://img.shields.io/badge/version-0.3.0-blue)

- 📁 **File Management** - Drag & drop, preview, multi-select

- 🔍 **Search** - Tantivy full-text search with fuzzy matching

- 🌐 **Real-Time** - WebSocket updates

- 🌍 **i18n** - English & German**Access**: `http://localhost:5173`  </div>![License](https://img.shields.io/badge/license-Apache--2.0-green)

- 📱 **Responsive** - Desktop, tablet, mobile

**Login**: `admin` / `admin` _(change immediately!)_

---

![Material 3](https://img.shields.io/badge/Material-3%20Expressive-purple)

## 🏗️ Tech Stack

---

**Backend**: Rust + axum 0.8 + SQLite (SQLx) + Tantivy 0.25

**Frontend**: Svelte 5 + Vite (Rolldown) + Tailwind v4 + Bootstrap Icons  ---![Rust](https://img.shields.io/badge/Rust-axum%200.8-orange)

**Search**: Tantivy with BM25 ranking, fuzzy matching

**Real-time**: WebSocket with `notify` file system watching## ✨ Features



---![Svelte](https://img.shields.io/badge/Svelte-5-red)



## 📁 Structure- 🎨 **Modern UI** - Tailwind CSS v4, responsive design



```- 🔐 **Secure** - JWT + 2FA (TOTP), Argon2 hashing## ⚡ Quick Start

syncspace/

├── backend/          # Rust API + WebSocket- 📁 **File Management** - Drag & drop, preview, multi-select

│   ├── src/          # Source code

│   ├── migrations/   # SQLite migrations (23+)- 🔍 **Search** - Tantivy full-text search with fuzzy matching</div>

│   └── data/         # Files + database

├── frontend/         # Svelte 5 UI- 🌐 **Real-Time** - WebSocket updates

│   ├── src/          # Components + pages

│   └── public/       # Static assets- 🌍 **i18n** - English & German```bash

└── docs/             # Documentation

```- 📱 **Responsive** - Desktop, tablet, mobile



---# Clone the repository---



## 🔐 Security---



- JWT authentication (24h expiration)git clone https://github.com/MickLesk/syncspace.git

- Two-factor authentication (TOTP)

- Argon2 password hashing## 🏗️ Tech Stack

- Rate limiting (5 attempts/min)

- CORS protectioncd syncspace## 📂 Directory Structure



---**Backend**: Rust + axum 0.8 + SQLite (SQLx) + Tantivy 0.25



## 🌐 API Highlights**Frontend**: Svelte 5 + Vite (Rolldown) + Tailwind v4 + Bootstrap Icons



**Auth**: `POST /api/auth/login`, `POST /api/auth/setup-2fa`  **Search**: Tantivy with BM25 ranking, fuzzy matching

**Files**: `GET /api/files/{path}`, `POST /api/upload/{path}`, `DELETE /api/files/{path}`

**Search**: `GET /api/search?q={query}`  **Real-time**: WebSocket with `notify` file system watching# Start backend (Terminal 1)- `backend/` – Rust backend with REST API + WebSocket (migrating to **axum 0.7**)

**WebSocket**: `GET /api/ws` (real-time updates)



---

---cd backend && cargo run --release- `frontend/` – Svelte 5 + Vite frontend with Material 3 styling

## 🎨 UI Features



- Drag & drop upload with progress

- File preview (images, PDFs, videos, text, DOCX, Excel)## 📁 Structure- `data/` – File storage (created automatically)

- Multi-select bulk operations

- Breadcrumb navigation

- Dark/Light theme

- Keyboard shortcuts```# Start frontend (Terminal 2)- `docs/` – Comprehensive documentation



---syncspace/



## 📚 Documentation├── backend/          # Rust API + WebSocketcd frontend && npm install && npm run dev- `scripts/` – Testing and utility scripts



- [QUICKSTART.md](docs/QUICKSTART.md) - 5-minute setup│   ├── src/          # Source code

- [FEATURES.md](docs/FEATURES.md) - Complete features

- [DATABASE.md](docs/DATABASE.md) - Schema & migrations│   ├── migrations/   # SQLite migrations (23+)```- `start.ps1` / `start.bat` – One-command startup scripts

- [SEARCH_FEATURE.md](docs/SEARCH_FEATURE.md) - Search details

- [AUTH_README.md](docs/AUTH_README.md) - Authentication│   └── data/         # Files + database



---├── frontend/         # Svelte 5 UI



## 🚧 Roadmap│   ├── src/          # Components + pages



- [x] Core file management│   └── public/       # Static assets**Access**: Open `http://localhost:5173` in your browser  ---

- [x] JWT + 2FA authentication

- [x] Full-text search (Tantivy)└── docs/             # Documentation

- [x] Tailwind v4 migration

- [ ] Peer-to-peer sync```**Login**: `admin` / `admin` _(change immediately!)_

- [ ] File versioning

- [ ] Mobile/Desktop apps



------## ⚡ Quick Start



## 🤝 Contributing



Contributions welcome! Follow [Conventional Commits](https://www.conventionalcommits.org/):## 🔐 Security---



```bash

feat: add feature

fix: bug fix- JWT authentication (24h expiration)**One-Command Startup** (Windows):

docs: documentation

```- Two-factor authentication (TOTP)



---- Argon2 password hashing## ✨ Key Features



## 📄 License- Rate limiting (5 attempts/min)



Apache License 2.0 - see [LICENSE](LICENSE)- CORS protection```powershell



---



## 🙏 Credits---- 🎨 **Modern UI** - Tailwind CSS v4 with DaisyUI components.\start.ps1   # PowerShell with process monitoring



[Svelte](https://svelte.dev) • [axum](https://github.com/tokio-rs/axum) • [Tantivy](https://github.com/quickwit-oss/tantivy) • [Tailwind](https://tailwindcss.com)



**Made with ❤️ by [MickLesk](https://github.com/MickLesk)**## 🌐 API Highlights- 🔐 **Secure Auth** - JWT + 2FA (TOTP) with Argon2 hashing```




**Auth**: `POST /api/auth/login`, `POST /api/auth/setup-2fa`  - 📁 **File Management** - Drag & drop, preview, multi-select, batch operations

**Files**: `GET /api/files/{path}`, `POST /api/upload/{path}`, `DELETE /api/files/{path}`

**Search**: `GET /api/search?q={query}`  - 🔍 **Full-Text Search** - Tantivy-powered search with fuzzy matching```batch

**WebSocket**: `GET /api/ws` (real-time updates)

- 🌐 **Real-Time Sync** - WebSocket updates across all clientsstart.bat     # Batch with auto-browser

---

- 🌍 **i18n** - English and German translations```

## 🎨 UI Features

- 📱 **Responsive** - Works on desktop, tablet, and mobile

- Drag & drop upload with progress

- File preview (images, PDFs, videos, text, DOCX, Excel)- 🗄️ **SQLite Backend** - Lightweight, embedded database with migrations**Manual Startup**:

- Multi-select bulk operations

- Breadcrumb navigation

- Dark/Light theme

- Keyboard shortcuts---1. **Backend:**



---



## 📚 Documentation## 🏗️ Tech Stack   ```bash



- [QUICKSTART.md](docs/QUICKSTART.md) - 5-minute setup   cd backend

- [FEATURES.md](docs/FEATURES.md) - Complete features

- [DATABASE.md](docs/DATABASE.md) - Schema & migrations### Backend (Rust)   cargo run --release

- [SEARCH_FEATURE.md](docs/SEARCH_FEATURE.md) - Search details

- [AUTH_README.md](docs/AUTH_README.md) - Authentication- **Framework**: axum 0.8 + Tower middleware   ```



---- **Database**: SQLite with SQLx (async queries)



## 🚧 Roadmap- **Search**: Tantivy 0.25 (full-text search engine)   Backend runs on `http://localhost:8080`



- [x] Core file management- **Auth**: JWT + Argon2 + TOTP

- [x] JWT + 2FA authentication

- [x] Full-text search (Tantivy)- **Real-time**: WebSocket with `notify` file system watching2. **Frontend:**

- [x] Tailwind v4 migration

- [ ] Peer-to-peer sync

- [ ] File versioning

- [ ] Mobile/Desktop apps### Frontend (Svelte 5)   ```bash



---- **Framework**: Svelte 5 with runes (`$state`, `$derived`)   cd frontend



## 🤝 Contributing- **Build Tool**: Vite (Rolldown variant for faster builds)   npm install



Contributions welcome! Follow [Conventional Commits](https://www.conventionalcommits.org/):- **Styling**: Tailwind CSS v4 + DaisyUI + Bootstrap Icons   npm run dev



```bash- **Preview**: Mammoth (DOCX), PrismJS (code), SheetJS (Excel)   ```

feat: add feature

fix: bug fix

docs: documentation

```---   Frontend runs on `http://localhost:5173`



---



## 📄 License## 📁 Project Structure3. **Login:**



Apache License 2.0 - see [LICENSE](LICENSE)   - Username: `admin`



---```   - Password: `admin`



## 🙏 Creditssyncspace/   - _(Change immediately in Settings!)_



[Svelte](https://svelte.dev) • [axum](https://github.com/tokio-rs/axum) • [Tantivy](https://github.com/quickwit-oss/tantivy) • [Tailwind](https://tailwindcss.com)├── backend/



**Made with ❤️ by [MickLesk](https://github.com/MickLesk)**│   ├── src/---


│   │   ├── main.rs              # Server setup, routing

│   │   ├── auth.rs              # JWT, 2FA, rate limiting## ✨ Features

│   │   ├── database.rs          # SQLite models, pool

│   │   ├── search.rs            # Tantivy search### 🎨 **Material 3 Expressive Design**

│   │   └── api/                 # Route handlers

│   ├── migrations/              # Database migrations (23+)- Beautiful gradient app bar with smooth animations

│   └── data/                    # Files + SQLite DB- Adaptive dark/light theme with system integration

├── frontend/- Elevated cards with proper shadows and depth

│   ├── src/- Material Design 3 color tokens and typography

│   │   ├── App.svelte           # Main app shell- Responsive layout for desktop and mobile

│   │   ├── pages/               # View components

│   │   ├── components/          # Reusable UI components### 🌍 **Internationalization**

│   │   ├── stores/              # Global state management

│   │   └── lib/- **English** and **German** translations

│   │       ├── api.js           # HTTP client- Easy language switcher in app bar

│   │       └── i18n.js          # Translations- LocalStorage persistence

│   └── tailwind.config.js       # Tailwind v4 config- Extensible translation system

└── docs/                        # Comprehensive documentation

```### 📁 **File Management**



---- **Drag & drop upload** with visual feedback

- **Breadcrumb navigation** for folder hierarchy

## 🔐 Security Features- **File operations**: Rename, Delete, Download

- **File Preview Modal** - Images, videos, PDFs, text files

- **JWT Authentication** with 24h expiration- **Multi-Select Mode** - Bulk operations with checkboxes

- **Two-Factor Authentication** (TOTP with QR codes)- **Upload Progress Tracking** - Per-file progress bars

- **Argon2** memory-hard password hashing- Directory navigation with keyboard shortcuts

- **Rate Limiting** (5 attempts/minute)- File size display with proper formatting

- **CORS** protection- Icon-based file type indicators

- **Input validation** & path sanitization- **Universal system file filtering** - Hides .git, .DS_Store, database files, etc.



---A self-hosted, cross-platform file sync solution with a beautiful Material Design 3 Expressive interface. Built with Rust (backend) and Web Components (frontend).## Directory structure



## 🌐 API Highlights![Version](https://img.shields.io/badge/version-0.2.0-blue)- `backend` – Rust backend exposing a REST API and WebSocket for file operations, peer management, search, rename and stats.



### Authentication![License](https://img.shields.io/badge/license-Apache--2.0-green)- `frontend` – Material‑inspired web UI built with Lit. You can run it directly in a browser or embed it in Tauri or Electron.

- `POST /api/auth/login` - Login (with optional 2FA)

- `POST /api/auth/setup-2fa` - Generate 2FA QR code![Material 3](https://img.shields.io/badge/Material-3%20Expressive-purple)- `desktop-app` – Placeholder for a Tauri configuration. A desktop app can embed the frontend here.

- `POST /api/auth/change-password` - Change password

- `mobile-app` – Placeholder for a Flutter app. Use `flutter create` to generate the mobile client.

### Files (All protected by JWT)

- `GET /api/files/{path}` - List directory---

- `POST /api/upload/{path}` - Upload file

- `DELETE /api/files/{path}` - Delete file/folder## Running locally

- `PUT /api/rename/{path}` - Rename/move

- `POST /api/dirs/{path}` - Create directory## ✨ Features



### Advanced1. **Backend:** Navigate to `backend` and run the server with Cargo (requires Rust and Cargo installed):

- `GET /api/search?q={query}` - Full-text search

- `GET /api/ws` - WebSocket for real-time updates### 🎨 **Material 3 Expressive Design**

- `GET /api/users/profile` - User profile management

- `GET /api/users/settings` - User settings (theme, language)- Beautiful gradient app bar with smooth animations ```bash



---- Adaptive dark/light theme with system integration cd backend



## 🎨 UI Features- Elevated cards with proper shadows and depth cargo run



- **Drag & Drop Upload** with progress tracking- Material Design 3 color tokens and typography ```

- **File Preview** (images, PDFs, videos, text, DOCX, Excel)

- **Multi-Select Mode** for bulk operations- Responsive layout for desktop and mobile

- **Breadcrumb Navigation** with folder hierarchy

- **Dark/Light Theme** with auto-detection  The backend listens on `http://localhost:8080`. It automatically creates a `data` folder for synchronised files and a `config.json` for peers and settings.

- **Keyboard Shortcuts** (arrow keys in preview, ESC to close)

- **Search Highlighting** with fuzzy matching### 🌍 **Internationalization**



---- **English** and **German** translations2. **Frontend:** Open `frontend/index.html` in a browser or serve the `frontend` folder with a static file server (e.g. using `python -m http.server`). The UI connects to the backend at `http://localhost:8080` and `ws://localhost:8080`.



## 📚 Documentation- Easy language switcher in app bar



- **[QUICKSTART.md](docs/QUICKSTART.md)** - 5-minute setup guide- LocalStorage persistence## Features

- **[FEATURES.md](docs/FEATURES.md)** - Complete feature list

- **[DATABASE.md](docs/DATABASE.md)** - SQLite schema & migrations- Extensible translation system

- **[SEARCH_FEATURE.md](docs/SEARCH_FEATURE.md)** - Tantivy implementation

- **[AUTH_README.md](docs/AUTH_README.md)** - Authentication details- **File browsing:** Navigate through directories, download files, rename or delete entries and create new folders.



---### 📁 **File Management**- **Upload:** Upload files to any subfolder using the upload widget.



## 🚧 Roadmap- **Drag & drop upload** with visual feedback- **Search:** Perform case‑insensitive searches across all files and directories.



- [x] Core file management & authentication- **Breadcrumb navigation** for folder hierarchy- **Stats:** View the total number of files and their combined size.

- [x] Full-text search with Tantivy

- [x] Tailwind v4 migration- **File operations**: Rename, Delete, Download, Preview- **Peers:** Add peers via the API; peer information is persisted in `config.json`.

- [x] User profile & settings

- [ ] Peer-to-peer synchronization- Directory navigation- **Live updates:** The backend emits file system events via WebSocket. The UI automatically refreshes on changes.

- [ ] File versioning & conflict resolution

- [ ] Mobile apps (Flutter)- File size display with proper formatting

- [ ] Desktop app (Tauri)

- [ ] End-to-end encryption- Icon-based file type indicators



---### 🔐 **Security & Authentication**



## 🤝 Contributing- JWT-based authentication with Argon2 password hashing

- **Two-Factor Authentication (2FA)** with TOTP

Contributions are welcome! Please follow [Conventional Commits](https://www.conventionalcommits.org/):- Rate limiting (5 attempts/minute)

- Secure password change

```bash- Default admin account (admin/admin)

feat: add new feature

fix: bug fix### 🔍 **Search & Organization**

docs: documentation

refactor: code restructuring- **Full-text search** powered by Tantivy (Rust search engine)

```- **Fuzzy matching** - Find files even with typos (2-edit distance)

- **Content indexing** - Search inside text files, code, and PDFs

---- **BM25 ranking** - Results sorted by relevance

- **Background indexing** - Non-blocking automatic indexing on upload/delete

## 📄 License- **Debounced search** - 300ms delay for smooth UX

- **40+ file types** supported (text, code, documents)

Apache License 2.0 - see [LICENSE](LICENSE) file.- Search results with file paths and metadata



---### ⚡ **Real-Time Updates**



## 🙏 Acknowledgments- WebSocket connections for live file events

- Automatic UI refresh on file changes

[Svelte](https://svelte.dev) • [axum](https://github.com/tokio-rs/axum) • [Tantivy](https://github.com/quickwit-oss/tantivy) • [Tailwind CSS](https://tailwindcss.com)- File system monitoring with `notify` crate



**Made with ❤️ by [MickLesk](https://github.com/MickLesk)**### 🎯 **Modern Tech Stack**


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
`````

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
