# SyncSpace - Makefile
# Development automation for backend and frontend

.PHONY: help dev backend frontend test lint format clean migration-create migration-run

# Default target
help:
	@echo "SyncSpace Development Commands:"
	@echo ""
	@echo "  make dev          - Start backend + frontend in parallel"
	@echo "  make backend      - Start Rust backend (release mode)"
	@echo "  make frontend     - Start Svelte frontend (dev mode)"
	@echo "  make test         - Run all tests (backend + frontend)"
	@echo "  make lint         - Run linters (cargo clippy + eslint)"
	@echo "  make format       - Format code (rustfmt + prettier)"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make build        - Build production binaries"
	@echo ""

# Development - Start both backend and frontend
dev:
	@echo "🚀 Starting SyncSpace in development mode..."
	@powershell -Command "Start-Process powershell -ArgumentList '-NoExit', '-Command', 'cd backend; cargo run --release'"
	@powershell -Command "Start-Process powershell -ArgumentList '-NoExit', '-Command', 'cd frontend; npm run dev'"

# Backend only
backend:
	@echo "🦀 Starting Rust backend..."
	cd backend && cargo run --release

# Frontend only
frontend:
	@echo "⚡ Starting Svelte frontend..."
	cd frontend && npm run dev

# Run tests
test:
	@echo "🧪 Running tests..."
	@echo "Backend tests..."
	cd backend && cargo test
	@echo "Frontend tests..."
	cd frontend && npm run test

# Linting
lint:
	@echo "🔍 Running linters..."
	@echo "Cargo clippy..."
	cd backend && cargo clippy -- -D warnings
	@echo "ESLint..."
	cd frontend && npm run lint

# Code formatting
format:
	@echo "🎨 Formatting code..."
	@echo "Rustfmt..."
	cd backend && cargo fmt
	@echo "Prettier..."
	cd frontend && npm run format

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cd backend && cargo clean
	cd frontend && rm -rf dist node_modules/.vite

# Production build
build:
	@echo "📦 Building production binaries..."
	@echo "Building backend..."
	cd backend && cargo build --release
	@echo "Building frontend..."
	cd frontend && npm run build
	@echo "✅ Build complete!"
	@echo "Backend binary: backend/target/release/syncbackend.exe"
	@echo "Frontend dist: frontend/dist/"

# Database migrations
migration-create:
	@echo "📝 Creating new migration..."
	@powershell -Command "$$timestamp = Get-Date -Format 'yyyyMMddHHmmss'; New-Item -Path 'backend/migrations/$${timestamp}_migration_name.sql' -ItemType File"

migration-run:
	@echo "🗄️ Running database migrations..."
	cd backend && cargo run --release -- migrate

# Install dependencies
install:
	@echo "📥 Installing dependencies..."
	@echo "Backend dependencies (cargo)..."
	cd backend && cargo build
	@echo "Frontend dependencies (npm)..."
	cd frontend && npm install
	@echo "✅ Dependencies installed!"

# Docker build (optional)
docker-build:
	@echo "🐳 Building Docker image..."
	docker build -t syncspace:latest .

# Docker run (optional)
docker-run:
	@echo "🐳 Running Docker container..."
	docker run -p 8080:8080 -p 5173:5173 -v $$(pwd)/data:/app/data syncspace:latest
