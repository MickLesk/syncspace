# Contributing to SyncSpace

Thank you for your interest in contributing to SyncSpace!

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Set up the development environment (see README.md)

## Development Setup

```bash
# Install just (build tool)
cargo install just

# Backend
cd backend && cargo run --release

# Frontend (separate terminal)
cd frontend && npm install && npm run dev
```

## Making Changes

### Branch Naming

- `feat/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation
- `refactor/description` - Code refactoring

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add file versioning support
fix: resolve upload timeout issue
docs: update API documentation
refactor: simplify authentication flow
```

### Code Style

**Backend (Rust):**
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new functionality

**Frontend (Svelte):**
- Run `npm run format` before committing
- Run `npm run lint` and fix issues
- Use Tailwind CSS utilities (no custom CSS unless necessary)
- Follow Svelte 5 patterns (`$state`, `$derived`, `$effect`)

## Pull Request Process

1. Ensure all tests pass
2. Update documentation if needed
3. Fill out the PR template
4. Request review from maintainers

## Code of Conduct

Be respectful and constructive. We're all here to build something useful.

## Questions?

Open a GitHub Discussion or issue for questions.
