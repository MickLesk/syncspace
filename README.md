# 🚀 SyncSpace# SyncSpace Prototype



**Modern Material 3 File Synchronization Service**This repository contains a cross‑platform synchronisation tool called **SyncSpace**. It consists of a Rust backend and a browser‑based frontend built with [Lit](https://lit.dev/) and styled in a Material 3 expressive aesthetic. The system allows you to manage a local folder, synchronise files, manage peers and receive live updates via WebSockets.



A self-hosted, cross-platform file sync solution with a beautiful Material Design 3 Expressive interface. Built with Rust (backend) and Web Components (frontend).## Directory structure



![Version](https://img.shields.io/badge/version-0.2.0-blue)- `backend` – Rust backend exposing a REST API and WebSocket for file operations, peer management, search, rename and stats.

![License](https://img.shields.io/badge/license-Apache--2.0-green)- `frontend` – Material‑inspired web UI built with Lit.  You can run it directly in a browser or embed it in Tauri or Electron.

![Material 3](https://img.shields.io/badge/Material-3%20Expressive-purple)- `desktop-app` – Placeholder for a Tauri configuration. A desktop app can embed the frontend here.

- `mobile-app` – Placeholder for a Flutter app. Use `flutter create` to generate the mobile client.

---

## Running locally

## ✨ Features

1. **Backend:** Navigate to `backend` and run the server with Cargo (requires Rust and Cargo installed):

### 🎨 **Material 3 Expressive Design**

- Beautiful gradient app bar with smooth animations   ```bash

- Adaptive dark/light theme with system integration   cd backend

- Elevated cards with proper shadows and depth   cargo run

- Material Design 3 color tokens and typography   ```

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
- Real-time file search
- Search results with file paths
- Fast backend search with `walkdir`

### ⚡ **Real-Time Updates**
- WebSocket connections for live file events
- Automatic UI refresh on file changes
- File system monitoring with `notify` crate

### 🎯 **Modern Tech Stack**
- **Backend**: Rust with Warp, Tokio, and async/await
- **Frontend**: Material Web Components from CDN
- **No Build Tools**: Pure ES modules, instant reload
- **Minified Assets**: Production-ready compressed CSS/JS

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Modern web browser (Chrome 119+, Firefox 121+, Safari 17+)

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/MickLesk/syncspace.git
cd syncspace
```

2. **Start the backend**
```bash
cd backend
cargo run --release
```

3. **Access the app**
Open your browser to: **http://localhost:8080**

4. **Login**
- Username: `admin`
- Password: `admin`
- *(Change this immediately in Settings!)*

---

## 📖 Usage

### File Upload
- **Drag & drop**: Drag files onto the drop zone
- **FAB button**: Click the floating action button (bottom right)
- **Multiple files**: Upload multiple files at once

### Navigation
- Click folders to navigate into them
- Use breadcrumbs at the top to go back
- Home icon returns to root directory

### File Operations
- **Download**: Click download icon
- **Rename**: Click edit icon, enter new name
- **Delete**: Click delete icon, confirm deletion
- **Preview**: Click on file name *(coming soon)*

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
│   ├── main.rs      # API routes, WebSocket, static serving
│   └── auth.rs      # Authentication, JWT, 2FA, rate limiting
├── Cargo.toml       # Dependencies
└── data/            # File storage (created automatically)
```

**Key Dependencies:**
- `warp` - Web framework
- `tokio` - Async runtime
- `jsonwebtoken` - JWT authentication
- `argon2` - Password hashing
- `totp-lite` - 2FA implementation
- `notify` - File system monitoring

### Frontend
```
frontend/
├── index.html       # Entry point with Material Web imports
├── styles.css       # Material 3 tokens (minified)
└── app.js           # Complete app logic with i18n (minified)
```

**Features:**
- Material Web Components via CDN (`@material/web`)
- No build process required
- Minified for production (< 10KB combined)
- Complete i18n support built-in

---

## 🔐 Security

### Authentication Flow
1. User enters credentials
2. Backend validates with Argon2
3. Optional 2FA verification
4. JWT token issued (24h expiration)
5. Token stored in localStorage
6. All API calls include Authorization header

### 2FA Setup
1. Go to Settings
2. Click "Setup 2FA"
3. Scan QR code with authenticator app
4. Enter verification code
5. 2FA enabled

### Rate Limiting
- 5 login attempts per minute per IP
- Automatic cooldown after limit reached

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
- `POST /api/auth/login` - Login with credentials
- `GET /api/auth/me` - Get current user info
- `POST /api/auth/2fa/setup` - Generate 2FA secret
- `POST /api/auth/2fa/enable` - Enable 2FA
- `POST /api/auth/2fa/disable` - Disable 2FA
- `POST /api/auth/change-password` - Change password

### Files (Protected)
- `GET /api/files/:path` - List directory
- `GET /api/file/:path` - Download file
- `POST /api/upload/:path` - Upload file
- `DELETE /api/files/:path` - Delete file/folder
- `POST /api/dirs/:path` - Create directory
- `PUT /api/rename/:path` - Rename/move file

### Utility
- `GET /api/search?q=:query` - Search files
- `GET /api/stats` - File count and size
- `GET /api/config` - Get config
- `GET /api/peers` - List peers

### Real-Time
- `GET /api/ws` - WebSocket for file events

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

### User Database (`users.json`)
Automatically created and managed by the backend. Includes:
- User ID (UUID)
- Username
- Password hash (Argon2)
- TOTP secret (if 2FA enabled)
- Created/last login timestamps

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

### Phase 2: Enhanced UX (In Progress)
- [x] Drag & drop upload
- [x] Breadcrumb navigation
- [ ] File preview (images, text, PDF)
- [ ] Material dialogs for confirm actions
- [ ] Upload progress indicators
- [ ] File context menus

### Phase 3: Sync & Collaboration
- [ ] Peer-to-peer file synchronization
- [ ] Conflict resolution
- [ ] File versioning
- [ ] Shared folders
- [ ] User permissions

### Phase 4: Advanced Features
- [ ] Mobile apps (Flutter)
- [ ] Desktop app (Tauri)
- [ ] Selective sync
- [ ] Encryption at rest
- [ ] Audit logging
- [ ] Bulk operations

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'feat: add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Commit Convention
We follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `style:` Formatting
- `refactor:` Code restructuring
- `test:` Adding tests
- `chore:` Maintenance

---

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Material Design 3](https://m3.material.io/) by Google
- [Material Web Components](https://github.com/material-components/material-web)
- [Warp Web Framework](https://github.com/seanmonstar/warp)
- [Lit](https://lit.dev/) for web component inspiration
- [Home Assistant](https://www.home-assistant.io/) for SPA design patterns

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/MickLesk/syncspace/issues)
- **Discussions**: [GitHub Discussions](https://github.com/MickLesk/syncspace/discussions)

---

**Made with ❤️ by [MickLesk](https://github.com/MickLesk)**

**Material 3 Expressive Design** • **Rust Backend** • **Zero Build Frontend**
