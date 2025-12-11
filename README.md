# SyncSpace

Self-hosted file synchronization built with Rust and Svelte.

![Version](https://img.shields.io/badge/version-1.0--beta-blue)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Overview

SyncSpace is a modern file management and synchronization platform designed for self-hosting. It provides a web-based interface for managing files with real-time updates, full-text search, and secure authentication.

**Tech Stack:**
- Backend: Rust (axum), SQLite, Tantivy search
- Frontend: Svelte 5, Tailwind CSS v4, Vite

## Quick Start

**Prerequisites:** Rust 1.75+, Node.js 18+

```bash
# Backend (Terminal 1)
cd backend && cargo run --release

# Frontend (Terminal 2)
cd frontend && npm install && npm run dev
```

Open http://localhost:5173 and login with `admin` / `admin`.

> Change the default credentials immediately after first login.

## Features

### File Management
- Upload, download, rename, move, delete files and folders
- Drag & drop upload with progress tracking
- File preview (images, videos, PDFs, text, DOCX, Excel)
- Multi-select with bulk operations
- Folder navigation with breadcrumbs

### Search
- Full-text search powered by Tantivy
- Fuzzy matching and relevance ranking
- Search within file contents (text, PDF)

### Security
- JWT authentication with 24h expiration
- Two-factor authentication (TOTP)
- Argon2 password hashing
- Rate limiting (5 attempts/minute)

### Real-Time
- WebSocket-based live updates
- File changes sync across all connected clients

### User Experience
- Dark and light theme
- English and German localization
- Responsive design for desktop and mobile

## Project Structure

```
syncspace/
├── backend/           # Rust API server
│   ├── src/           # Source code
│   │   ├── api/       # Route handlers
│   │   ├── auth.rs    # Authentication
│   │   ├── database.rs# Database models
│   │   └── search.rs  # Search engine
│   ├── migrations/    # SQLite migrations
│   └── data/          # Runtime data (db, files)
├── frontend/          # Svelte web app
│   ├── src/
│   │   ├── pages/     # Page components
│   │   ├── components/# Reusable UI
│   │   ├── stores/    # State management
│   │   └── lib/       # Utilities, API client
│   └── public/        # Static assets
└── docker/            # Docker configuration
```

## Configuration

### Backend

The backend uses SQLite and stores all data in `backend/data/`:
- `syncspace.db` - Database
- `search_index/` - Tantivy search index
- Uploaded files

Environment variables (optional):
- `RUST_LOG` - Log level (default: `info`)
- `DATABASE_URL` - Database path (default: `./data/syncspace.db`)

### Frontend

Configure the API endpoint in `frontend/src/lib/api.js` if running on a different host.

## API

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/login` | Login (returns JWT) |
| POST | `/api/auth/setup-2fa` | Enable 2FA |
| POST | `/api/auth/change-password` | Change password |

### Files
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/files/{path}` | List directory |
| POST | `/api/upload/{path}` | Upload file |
| DELETE | `/api/files/{path}` | Delete file/folder |
| PUT | `/api/rename/{path}` | Rename or move |
| POST | `/api/dirs/{path}` | Create directory |

### Other
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/search?q=` | Full-text search |
| GET | `/api/ws` | WebSocket connection |
| GET | `/api/users/profile` | User profile |
| PUT | `/api/users/settings` | User settings |

## Development

### Using just (recommended)

Install: `cargo install just`

```bash
just backend      # Start backend
just frontend     # Start frontend
just build        # Production build
just test         # Run tests
just lint         # Run linters
just format       # Format code
just clean        # Clean artifacts
```

### Manual Commands

```bash
# Backend
cd backend
cargo run --release    # Development
cargo build --release  # Production build
cargo test             # Run tests
cargo clippy           # Lint

# Frontend
cd frontend
npm install            # Install dependencies
npm run dev            # Development server
npm run build          # Production build
npm run test           # Run tests
npm run lint           # Lint
```

## Docker

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

## Documentation

Additional documentation is available in the `docs/` directory:

- [QUICKSTART.md](docs/QUICKSTART.md) - Setup guide
- [DATABASE.md](docs/DATABASE.md) - Database schema
- [AUTH_README.md](docs/AUTH_README.md) - Authentication details

## Roadmap

- [x] Core file management
- [x] JWT + 2FA authentication
- [x] Full-text search
- [x] Real-time sync (WebSocket)
- [x] File preview
- [ ] Peer-to-peer sync
- [ ] File versioning
- [ ] Mobile app (Flutter)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

Please follow [Conventional Commits](https://www.conventionalcommits.org/) for commit messages.

## License

MIT License - see [LICENSE](LICENSE)

---

Created by [MickLesk](https://github.com/MickLesk)
