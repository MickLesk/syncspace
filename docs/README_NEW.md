# 🔄 SyncSpace v0.2.0

**Modern, secure file synchronization with built-in authentication and 2FA support.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Beta-yellow.svg)]()

---

## 🎯 What is SyncSpace?

SyncSpace is a **cross-platform file synchronization tool** with a modern web interface. Unlike Syncthing's decentralized approach, SyncSpace focuses on **local-first device synchronization** with:

- 🔐 **Built-in Authentication** (JWT + Argon2)
- 🔒 **Two-Factor Authentication** (TOTP)
- 🌓 **Dark Mode** Support
- 📁 **Real-time File Sync** via WebSockets
- 🎨 **Material 3 Design** UI
- 🚀 **Single Binary** Deployment

---

## ✨ Features

### 🔐 Security

- **JWT Token Authentication** (24h lifetime)
- **Argon2 Password Hashing** (industry standard)
- **TOTP 2FA** (Google Authenticator compatible)
- **Rate Limiting** (5 login attempts/minute)
- **Protected Routes** (all file operations require auth)

### 📁 File Management

- **Upload/Download** with drag & drop
- **Create/Delete** directories
- **Rename/Move** files
- **Search** across all files
- **Real-time Updates** via WebSocket
- **File Statistics** (count, total size)

### 🎨 Modern UI

- **Material 3 Design** with expressive styling
- **Dark/Light Mode** toggle
- **Responsive** for mobile & desktop
- **Toast Notifications**
- **Loading States**
- **Settings Page** for profile management

### 👥 Multi-User

- **User Registration**
- **Multiple Accounts**
- **Per-User File Access**
- **Peer Management** (for future sync)

---

## 🚀 Quick Start

### Prerequisites

- **Rust** (1.70+): Install from [rust-lang.org](https://www.rust-lang.org/)
- **Python** (3.x) or any static file server

### 1. Start Backend

```bash
cd backend
cargo run
```

**Default Admin User:**

- Username: `admin`
- Password: `admin`

Backend runs on: `http://localhost:8080`

### 2. Start Frontend

```bash
cd frontend
python -m http.server 8000
```

Frontend runs on: `http://localhost:8000`

### 3. Open Browser

Navigate to: **http://localhost:8000/index-new.html**

Login with `admin` / `admin` and start syncing! 🎉

---

## 📖 Documentation

- **[Quick Start Guide](QUICKSTART.md)** - Getting started in 5 minutes
- **[Authentication Guide](AUTH_README.md)** - Security features & API reference
- **[Copilot Instructions](.github/copilot-instructions.md)** - Architecture overview

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Frontend (Port 8000)              │
│  • Material 3 UI (Vanilla JS)                       │
│  • Dark Mode Toggle                                 │
│  • File Upload/Download                             │
│  • 2FA Setup Interface                              │
└────────────────────┬────────────────────────────────┘
                     │ HTTP + WebSocket
                     ▼
┌─────────────────────────────────────────────────────┐
│                 Backend (Port 8080)                  │
│  • Rust/Tokio/Warp                                  │
│  • JWT Authentication                               │
│  • Argon2 Password Hashing                          │
│  • TOTP 2FA                                         │
│  • File System Watcher (notify)                     │
│  • Protected REST API                               │
└────────────────────┬────────────────────────────────┘
                     │
         ┌───────────┴───────────┐
         ▼                       ▼
    ┌─────────┐            ┌──────────┐
    │ ./data/ │            │users.json│
    │ (Files) │            │ (Users)  │
    └─────────┘            └──────────┘
```

---

## 📋 API Endpoints

### Authentication

```
POST   /api/auth/register        - Create new user
POST   /api/auth/login           - Login (returns JWT)
GET    /api/auth/me              - Get current user
POST   /api/auth/2fa/setup       - Generate 2FA secret
POST   /api/auth/2fa/enable      - Enable 2FA
POST   /api/auth/2fa/disable     - Disable 2FA
PUT    /api/auth/change-password - Change password
```

### File Operations (Protected)

```
GET    /api/files/{path}         - List directory
GET    /api/file/{path}          - Download file
POST   /api/upload/{path}        - Upload file
DELETE /api/files/{path}         - Delete file/dir
POST   /api/dirs/{path}          - Create directory
PUT    /api/rename/{old}         - Rename/move
GET    /api/search?q=term        - Search files
GET    /api/stats                - Statistics
```

### Real-time

```
GET    /api/ws                   - WebSocket events
```

---

## 🧪 Testing

### Automated Tests

```powershell
# Run full API test suite
.\test-api-simple.ps1
```

Tests include:

- ✅ User Registration
- ✅ Login Flow
- ✅ JWT Token Validation
- ✅ File Upload/Download
- ✅ Protected Routes
- ✅ 2FA Setup
- ✅ Search & Statistics

### Manual Testing

```bash
# Login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}'

# Upload file (with token)
curl -X POST http://localhost:8080/api/upload/test.txt \
  -H "Authorization: Bearer YOUR_TOKEN" \
  --data-binary @test.txt
```

---

## 🔧 Configuration

### JWT Secret (Production)

⚠️ **Change this!** Edit `backend/src/auth.rs`:

```rust
const JWT_SECRET: &str = "your-secret-key-here";
```

Or use environment variable:

```bash
export SYNCSPACE_JWT_SECRET="your-secret-key"
```

### Token Expiration

```rust
const TOKEN_EXPIRATION_HOURS: i64 = 24; // Default: 24 hours
```

### Rate Limiting

```rust
// In auth.rs
rate_limiter.check_rate_limit(&username, 5, 60)
// 5 attempts per 60 seconds
```

---

## 📁 Project Structure

```
syncspace/
├── backend/
│   ├── src/
│   │   ├── main.rs        # Main server with all routes
│   │   └── auth.rs        # Authentication logic
│   ├── Cargo.toml         # Dependencies
│   ├── data/              # File storage
│   ├── users.json         # User database
│   └── config.json        # Configuration
│
├── frontend/
│   ├── index-new.html     # Main UI (with auth)
│   ├── app.js             # JavaScript logic
│   └── app.html           # Old version (no auth)
│
├── desktop-app/           # Tauri (placeholder)
├── mobile-app/            # Flutter (placeholder)
│
├── QUICKSTART.md          # Quick start guide
├── AUTH_README.md         # Authentication docs
└── test-api-simple.ps1    # API test suite
```

---

## 🔒 Security Best Practices

1. **Change default admin password** immediately
2. **Use HTTPS** in production
3. **Set strong JWT secret** via environment variable
4. **Enable 2FA** for all users
5. **Regular backups** of `users.json` and `data/`
6. **Update dependencies** regularly

---

## 🎯 Roadmap

- [x] JWT Authentication
- [x] 2FA/TOTP Support
- [x] Dark Mode
- [x] Protected Routes
- [x] Rate Limiting
- [ ] Audit Logging
- [ ] User Roles (admin, user, readonly)
- [ ] Email Verification
- [ ] Password Reset Flow
- [ ] Multi-Device Sync
- [ ] Mobile Apps (Flutter)
- [ ] Desktop App (Tauri)

---

## 🤝 Contributing

PRs welcome! Please:

1. Test authentication flow
2. Follow Rust coding standards
3. Update documentation
4. Run `cargo fmt` and `cargo clippy`

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file

---

## 🙏 Credits

Built with:

- [Rust](https://www.rust-lang.org/) - Backend
- [Warp](https://github.com/seanmonstar/warp) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Argon2](https://en.wikipedia.org/wiki/Argon2) - Password hashing
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - JWT
- [Material Design 3](https://m3.material.io/) - UI design

---

**Made with ❤️ by MickLesk**

🌟 Star this repo if you find it useful!
