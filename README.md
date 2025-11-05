<div align="center">

# 🚀 SyncSpace

**Modern Self-Hosted File Sync**

Fast, secure, and beautiful file sync built with Rust + Svelte 5.

[![Version](https://img.shields.io/badge/version-0.3.0-blue)](https://github.com/MickLesk/syncspace) [![Rust](https://img.shields.io/badge/Rust-axum%200.8-orange)](https://github.com/tokio-rs/axum) [![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)[![Svelte](https://img.shields.io/badge/Svelte-5-red)](https://svelte.dev) [![Tailwind](https://img.shields.io/badge/Tailwind-v4-38bdf8)](https://tailwindcss.com)

</div>

---
## ⚡ Quick Start

### Backend:

```bash
cd backend && cargo run --release # Terminal 1
```

### Frontend:

```bash
cd frontend && npm install && npm run dev  # Terminal 2
```

**Open**: http://localhost:5173 | **Login**: admin/admin

---

### Core

- 🎨 **Modern UI** - Tailwind CSS v4, responsive design
- 🔐 **Secure Auth** - JWT authentication + 2FA (TOTP)## Docs
- 📁 **File Management** - Upload, download, rename, delete, preview
- 🔍 **Full-Text Search** - Tantivy search engine with fuzzy matching
- 🌐 **Real-Time Sync** - WebSocket updates across all clients

---

### User Experience

- 📤 **Drag & Drop** - Upload files with visual feedback## License

- 👁️ **File Preview** - Images, PDFs, videos, text, DOCX, Excel

- ✅ **Multi-Select** - Bulk operations with checkboxesApache-2.0 - see [LICENSE](LICENSE)

- 🗂️ **Breadcrumbs** - Easy folder navigation

- 🌍 **Internationalization** - English & German**Made with by [MickLesk](https://github.com/MickLesk)**
- 🌓 **Dark/Light Theme** - Auto-switching

---

## 🏗️ Tech Stack

**Backend**

- Rust + axum 0.8
- SQLite (SQLx) with 23+ migrations
- Tantivy 0.25 full-text search
- JWT + Argon2 + TOTP
- WebSocket with `notify` crate

**Frontend**

- Svelte 5 (runes: `$state`, `$derived`)
- Vite (Rolldown variant)
- Tailwind CSS v4 (pure utility-first)
- Bootstrap Icons
- Mammoth (DOCX), PrismJS (code), SheetJS (Excel)

---

## 📁 Project Structure

```

syncspace/
├── backend/ # Rust API + WebSocket
│ ├── src/ # Source code (main.rs, auth.rs, database.rs, search.rs, api/)
│ ├── migrations/ # SQLite migrations (23+)
│ └── data/ # File storage + database
├── frontend/ # Svelte 5 UI
│ ├── src/
│ │ ├── pages/ # Views (Files, Search, Settings, Profile)
│ │ ├── components/ # Reusable components
│ │ ├── stores/ # Global state
│ │ └── lib/ # API client, i18n
│ └── public/ # Static assets
└── docs/ # Documentation

```

---

## 🔐 Security

- **JWT Authentication** - 24h token expiration
- **Two-Factor Auth** - TOTP with QR code setup
- **Argon2 Hashing** - Memory-hard password protection
- **Rate Limiting** - 5 attempts per minute
- **CORS Protection** - Configured for localhost

⚠️ **Important**: Change default `admin/admin` credentials immediately!

---

## 🌐 API Highlights

### Authentication

- `POST /api/auth/login` - Login with optional 2FA
- `POST /api/auth/setup-2fa` - Generate 2FA QR code
- `POST /api/auth/change-password` - Change password

### Files

- `GET /api/files/{path}` - List directory
- `POST /api/upload/{path}` - Upload file
- `DELETE /api/files/{path}` - Delete file/folder
- `PUT /api/rename/{path}` - Rename/move file
- `POST /api/dirs/{path}` - Create directory

### Search & Real-Time

- `GET /api/search?q={query}` - Full-text search
- `GET /api/ws` - WebSocket connection for live updates

---

## 📚 Documentation

- **[QUICKSTART.md](docs/QUICKSTART.md)** - 5-minute setup guide
- **[FEATURES.md](docs/FEATURES.md)** - Complete feature reference
- **[DATABASE.md](docs/DATABASE.md)** - SQLite schema & migrations
- **[SEARCH_FEATURE.md](docs/SEARCH_FEATURE.md)** - Tantivy search implementation
- **[AUTH_README.md](docs/AUTH_README.md)** - Authentication details

---

## 🚧 Roadmap

- [x] Core file management
- [x] JWT + 2FA authentication
- [x] Full-text search (Tantivy)
- [x] Tailwind v4 migration
- [x] User profiles & settings
- [ ] Peer-to-peer synchronization
- [ ] File versioning & conflict resolution
- [ ] Mobile apps (Flutter)
- [ ] Desktop app (Tauri)

---

## 🤝 Contributing

Contributions welcome! Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
feat: add new feature
fix: bug fix
docs: update documentation
```

---

## 📄 License

Apache License 2.0 - see [LICENSE](LICENSE)

---

## 🙏 Credits

Built with [Svelte](https://svelte.dev) • [axum](https://github.com/tokio-rs/axum) • [Tantivy](https://github.com/quickwit-oss/tantivy) • [Tailwind CSS](https://tailwindcss.com)

**Made with ❤️ by [MickLesk](https://github.com/MickLesk)**
