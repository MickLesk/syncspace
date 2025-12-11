# SyncSpace Build System
# Install: cargo install just
# Usage: just <recipe>

# ════════════════════════════════════════════════════════════════════════════════
# Configuration
# ════════════════════════════════════════════════════════════════════════════════

DOCKER_CLI := env("DOCKER_CLI", "docker")
IMAGE_NAME := env("IMAGE_NAME", "syncspace")
IMAGE_TAG := env("IMAGE_TAG", "latest")

# Platform-specific executable extension
exe := if os() == "windows" { ".exe" } else { "" }

# ════════════════════════════════════════════════════════════════════════════════
# Help
# ════════════════════════════════════════════════════════════════════════════════

[group("📒 Help")]
[private]
default:
    @just --list --list-heading $'🗂️ SyncSpace Commands:\n'

[doc("Show this help message")]
[group("📒 Help")]
help: default

[doc("Show project information")]
[group("📒 Help")]
info:
    @echo "SyncSpace - Self-Hosted File Sync"
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @echo "Backend:  Rust + axum 0.8 + SQLite"
    @echo "Frontend: Svelte 5 + Vite + Tailwind v4"
    @echo ""
    @echo "URLs:"
    @echo "  Frontend: http://localhost:5173"
    @echo "  Backend:  http://localhost:8080"
    @echo "  Login:    admin / admin"

# ════════════════════════════════════════════════════════════════════════════════
# Development
# ════════════════════════════════════════════════════════════════════════════════

[doc("Start backend server (release mode, recompiles if needed)")]
[group("🚀 Development")]
backend:
    @echo "🦀 Starting backend server..."
    cd backend && cargo run --release

[doc("Start backend server (fast, uses existing binary)")]
[group("🚀 Development")]
backend-fast:
    @echo "🦀 Starting backend server (fast)..."
    cd backend && ./target/release/syncbackend{{exe}}

[doc("Start frontend dev server")]
[group("🚀 Development")]
frontend:
    @echo "⚡ Starting frontend dev server..."
    cd frontend && npm run dev

[doc("Install frontend dependencies")]
[group("🚀 Development")]
frontend-install:
    @echo "📦 Installing frontend dependencies..."
    cd frontend && npm install

# ════════════════════════════════════════════════════════════════════════════════
# Build
# ════════════════════════════════════════════════════════════════════════════════

[doc("Build entire project for production")]
[group("🔨 Build")]
build: build-backend build-frontend
    @echo "✅ Build complete!"

[doc("Build backend binary (release mode)")]
[group("🔨 Build")]
build-backend:
    @echo "🔨 Building backend..."
    cd backend && cargo build --release

[doc("Build frontend for production")]
[group("🔨 Build")]
build-frontend:
    @echo "🔨 Building frontend..."
    cd frontend && npm run build

# ════════════════════════════════════════════════════════════════════════════════
# Code Quality
# ════════════════════════════════════════════════════════════════════════════════

[doc("Run all tests")]
[group("👆 Code Quality")]
test: test-backend test-frontend
    @echo "✅ All tests passed!"

[doc("Run backend tests")]
[group("👆 Code Quality")]
test-backend:
    @echo "🧪 Running backend tests..."
    cd backend && cargo test

[doc("Run frontend tests")]
[group("👆 Code Quality")]
test-frontend:
    @echo "🧪 Running frontend tests..."
    cd frontend && npm run test

[doc("Run all linters")]
[group("👆 Code Quality")]
lint: lint-backend lint-frontend
    @echo "✅ Linting complete!"

[doc("Lint backend with clippy")]
[group("👆 Code Quality")]
lint-backend:
    @echo "🔍 Running clippy..."
    cd backend && cargo clippy --all-targets -- -D warnings

[doc("Lint frontend with eslint")]
[group("👆 Code Quality")]
lint-frontend:
    @echo "🔍 Running eslint..."
    cd frontend && npm run lint

[doc("Format all code")]
[group("👆 Code Quality")]
format: format-backend format-frontend
    @echo "✅ Formatting complete!"

[doc("Format Rust code")]
[group("👆 Code Quality")]
format-backend:
    @echo "🎨 Formatting Rust code..."
    cd backend && cargo fmt

[doc("Format frontend code")]
[group("👆 Code Quality")]
format-frontend:
    @echo "🎨 Formatting frontend code..."
    cd frontend && npm run format

[doc("Check backend compilation without building")]
[group("👆 Code Quality")]
check:
    @echo "🔎 Checking backend compilation..."
    cd backend && cargo check --all-targets

[doc("Run format, lint, check, and test")]
[group("👆 Code Quality")]
pre-commit: format lint check test
    @echo "✅ All pre-commit checks passed!"

# ════════════════════════════════════════════════════════════════════════════════
# Database
# ════════════════════════════════════════════════════════════════════════════════

[doc("Show database path")]
[group("💾 Database")]
db-path:
    @echo "📁 Database: backend/data/syncspace.db"

[doc("Create database backup")]
[group("💾 Database")]
db-backup:
    @echo "💾 Creating database backup..."
    cp backend/data/syncspace.db backend/data/syncspace.db.backup
    @echo "✅ Backup created: backend/data/syncspace.db.backup"

# ════════════════════════════════════════════════════════════════════════════════
# Docker
# ════════════════════════════════════════════════════════════════════════════════

[doc("Start containers with Docker Compose")]
[group("🐳 Docker")]
docker-up cli=(DOCKER_CLI):
    @echo "🐳 Starting Docker containers..."
    {{cli}} compose up -d

[doc("Stop Docker containers")]
[group("🐳 Docker")]
docker-down cli=(DOCKER_CLI):
    @echo "🛑 Stopping Docker containers..."
    {{cli}} compose down

[doc("Rebuild Docker images")]
[group("🐳 Docker")]
docker-build cli=(DOCKER_CLI):
    @echo "🔨 Building Docker images..."
    {{cli}} compose build --no-cache

[doc("View Docker logs")]
[group("🐳 Docker")]
docker-logs cli=(DOCKER_CLI):
    {{cli}} compose logs -f

[doc("Build multi-arch images with buildx")]
[group("🐳 Docker")]
docker-buildx cli=(DOCKER_CLI) image=(IMAGE_NAME) tag=(IMAGE_TAG):
    @echo "🏗️ Building multi-architecture Docker images..."
    {{cli}} buildx build \
        --platform linux/amd64,linux/arm64 \
        --file docker/backend/Dockerfile \
        --tag {{image}}-backend:{{tag}} \
        .
    {{cli}} buildx build \
        --platform linux/amd64,linux/arm64 \
        --file docker/frontend/Dockerfile \
        --tag {{image}}-frontend:{{tag}} \
        .

[doc("Build and push multi-arch images")]
[group("🐳 Docker")]
docker-buildx-push registry cli=(DOCKER_CLI) image=(IMAGE_NAME) tag=(IMAGE_TAG):
    @echo "🚀 Building and pushing multi-architecture images..."
    {{cli}} buildx build \
        --platform linux/amd64,linux/arm64 \
        --file docker/backend/Dockerfile \
        --tag {{registry}}/{{image}}-backend:{{tag}} \
        --push \
        .
    {{cli}} buildx build \
        --platform linux/amd64,linux/arm64 \
        --file docker/frontend/Dockerfile \
        --tag {{registry}}/{{image}}-frontend:{{tag}} \
        --push \
        .

# ════════════════════════════════════════════════════════════════════════════════
# Cleanup
# ════════════════════════════════════════════════════════════════════════════════

[doc("Clean all build artifacts")]
[group("🧹 Cleanup")]
clean: clean-backend clean-frontend
    @echo "✅ Cleanup complete!"

[doc("Clean backend build artifacts")]
[group("🧹 Cleanup")]
clean-backend:
    @echo "🧹 Cleaning backend..."
    cd backend && cargo clean

[doc("Clean frontend build artifacts")]
[group("🧹 Cleanup")]
clean-frontend:
    @echo "🧹 Cleaning frontend..."
    rm -rf frontend/dist frontend/node_modules/.vite
