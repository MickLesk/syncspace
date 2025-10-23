# SyncSpace Roadmap & TODO

## 🎯 Phase 1: Core Stability & Bug Fixes ✅ (Completed)

### ✅ Activity Logging

- [x] Upload/Delete/Rename logging to file_history
- [x] User tracking with user_id

### ✅ Comments & Tags System

- [x] Backend API with SQLite storage
- [x] Frontend integration with async stores
- [x] CommentsPanel with loading/error states

### ✅ Internationalization (i18n)

- [x] Error message translation keys
- [x] German/English support
- [x] t() wrapper in key components

### ✅ Security & Performance

- [x] Request debouncing (300ms for favorites)
- [x] Upload queue (max 2 concurrent)
- [x] Retry logic with exponential backoff
- [x] Thumbnail caching with file.id
- [x] 50MB cache limit with LRU cleanup

---

## 🚀 Phase 2: Critical Fixes & Polish (Priority: HIGH)

### 🔧 Bug Fixes

- [ ] **Fix missing utility functions in FilesView.svelte**

  - [ ] Implement `formatFileSize()` helper
  - [ ] Implement `formatDate()` helper
  - [ ] Fix `openPreview()` reference or remove
  - [ ] Fix `touchGesture.startY` property access

- [ ] **Fix TypeScript errors**

  - [ ] Add proper types for IndexedDB events in thumbnailGenerator.js
  - [ ] Fix date arithmetic in CommentsPanel (Date - Date)
  - [ ] Add type definitions for window.Chart, window.JSZip

- [ ] **Fix accessibility issues (a11y)**
  - [ ] Add ARIA roles to clickable divs
  - [ ] Add keyboard handlers (onKeyDown) to click events
  - [ ] Add proper labels to form inputs
  - [ ] Fix self-closing tags (textarea, video, option)
  - [ ] Add captions to video elements

### 🎨 UI/UX Improvements

- [ ] **Lazy Loading with IntersectionObserver**

  - [ ] Implement virtual scrolling for large file lists
  - [ ] Load thumbnails only when visible in viewport
  - [ ] Progressive loading for search results

- [ ] **Improved Upload Experience**

  - [ ] Upload progress visualization per file
  - [ ] Drag-and-drop zone highlighting
  - [ ] Cancel upload functionality
  - [ ] Pause/resume uploads

- [ ] **Mobile Responsiveness**
  - [ ] Touch gesture improvements
  - [ ] Mobile-optimized file grid
  - [ ] Bottom sheet for actions (instead of context menu)
  - [ ] Swipe-to-delete for files

---

## 🌟 Phase 3: Feature Enhancements (Priority: MEDIUM)

### 📁 File Management

- [ ] **Advanced File Operations**

  - [ ] Bulk move/copy operations
  - [ ] File versioning system
  - [ ] Conflict resolution UI
  - [ ] Duplicate file detection (by hash)

- [ ] **Search & Filter**

  - [ ] Full-text search in file contents
  - [ ] Advanced filter persistence
  - [ ] Saved searches
  - [ ] Search history

- [ ] **File Sharing**
  - [ ] Generate shareable links with expiry
  - [ ] Permission management (view/edit/download)
  - [ ] Share via email
  - [ ] Public folder links

### 🔄 Sync & Backup

- [ ] **Real-time Sync**

  - [ ] Peer-to-peer file synchronization
  - [ ] Conflict detection and resolution
  - [ ] Sync status indicators
  - [ ] Selective sync (ignore patterns)

- [ ] **Backup & Restore**
  - [ ] Scheduled automatic backups
  - [ ] Incremental backup support
  - [ ] Point-in-time restore
  - [ ] Backup verification

### 👥 Collaboration

- [ ] **Multi-user Features**

  - [ ] Real-time collaboration indicators
  - [ ] File locking mechanism
  - [ ] User presence (who's viewing what)
  - [ ] Activity feed per file/folder

- [ ] **Comments & Tags Enhancements**
  - [ ] Edit comment functionality (backend + frontend)
  - [ ] Tag autocomplete with suggestions
  - [ ] Tag filtering in file list
  - [ ] Comment threading (replies)
  - [ ] Mention users in comments (@username)

---

## 🔐 Phase 4: Security & Authentication (Priority: MEDIUM)

### 🔒 Enhanced Security

- [ ] **Authentication**

  - [ ] 2FA (Two-Factor Authentication) implementation
  - [ ] OAuth2 integration (Google, GitHub)
  - [ ] SSO (Single Sign-On) support
  - [ ] Session management improvements

- [ ] **Encryption**

  - [ ] End-to-end encryption for files
  - [ ] Encrypted file storage at rest
  - [ ] Secure key management
  - [ ] Client-side encryption option

- [ ] **Audit & Compliance**
  - [ ] Detailed audit logs
  - [ ] User action tracking
  - [ ] GDPR compliance tools
  - [ ] Data export functionality

### 🛡️ Rate Limiting & Protection

- [ ] **API Protection**
  - [ ] Rate limiting per user/IP
  - [ ] DDoS protection
  - [ ] Request signing
  - [ ] API key management

---

## 📊 Phase 5: Analytics & Monitoring (Priority: LOW)

### 📈 Insights

- [ ] **Usage Analytics**

  - [ ] Storage usage by user
  - [ ] Most accessed files
  - [ ] Upload/download statistics
  - [ ] User activity heatmap

- [ ] **Performance Monitoring**
  - [ ] Frontend performance metrics
  - [ ] Backend response times
  - [ ] Error tracking (Sentry integration)
  - [ ] Custom dashboards

### 📧 Notifications

- [ ] **Alert System**
  - [ ] Email notifications for shares
  - [ ] Push notifications for mobile
  - [ ] In-app notification center
  - [ ] Customizable notification preferences

---

## 🧪 Phase 6: Developer Experience (Priority: LOW)

### 🛠️ Tooling

- [ ] **Testing**

  - [ ] Unit tests for critical functions
  - [ ] Integration tests for API
  - [ ] E2E tests with Playwright
  - [ ] Visual regression testing

- [ ] **Documentation**

  - [ ] API documentation (OpenAPI/Swagger)
  - [ ] Component storybook
  - [ ] Developer setup guide
  - [ ] Architecture decision records (ADRs)

- [ ] **CI/CD**
  - [ ] GitHub Actions workflow
  - [ ] Automated testing
  - [ ] Docker deployment
  - [ ] Automated releases

### 🔌 Extensibility

- [ ] **Plugin System**
  - [ ] Plugin API design
  - [ ] Example plugins
  - [ ] Plugin marketplace
  - [ ] Custom file processors

---

## 🌈 Phase 7: Advanced Features (Priority: FUTURE)

### 🤖 AI Integration

- [ ] **Smart Features**
  - [ ] AI-powered file organization
  - [ ] Automatic tagging based on content
  - [ ] Smart search with NLP
  - [ ] Duplicate detection with ML

### 🎯 Platform Expansion

- [ ] **Mobile Apps**

  - [ ] iOS native app (Swift/SwiftUI)
  - [ ] Android native app (Kotlin/Jetpack Compose)
  - [ ] React Native cross-platform app
  - [ ] Mobile-specific features

- [ ] **Desktop Apps**
  - [ ] Tauri desktop app improvements
  - [ ] System tray integration
  - [ ] Native OS notifications
  - [ ] Auto-start on login

### 🔗 Integrations

- [ ] **Third-party Services**
  - [ ] Cloud storage integration (S3, Google Drive, Dropbox)
  - [ ] Webhook support
  - [ ] Zapier integration
  - [ ] Slack/Discord notifications

---

## 📋 Immediate Next Steps (This Week)

1. **Fix critical bugs** (formatFileSize, formatDate, a11y issues)
2. **Implement lazy loading** for thumbnails with IntersectionObserver
3. **Add edit comment** functionality (backend + frontend)
4. **Improve mobile responsiveness** (touch gestures, responsive grid)
5. **Add file versioning** basics (track old versions in file_history)
6. **Create unit tests** for debounce utilities
7. **Add E2E test** for upload flow
8. **Documentation**: API endpoints in README

---

## 🎨 UI/UX Wishlist

- [ ] Dark mode improvements (better contrast)
- [ ] Customizable themes
- [ ] File preview improvements (PDF viewer, code syntax highlighting)
- [ ] Keyboard shortcuts panel (show available shortcuts)
- [ ] Drag-and-drop file reorganization
- [ ] Folder tree navigation (sidebar)
- [ ] Breadcrumb navigation improvements
- [ ] Quick actions toolbar (sticky on scroll)
- [ ] File comparison view
- [ ] Gallery view for images

---

## 📚 Technical Debt

- [ ] Refactor FilesView.svelte (split into smaller components)
- [ ] Extract reusable logic into composables/utilities
- [ ] Standardize error handling across components
- [ ] Migrate to TypeScript for better type safety
- [ ] Add JSDoc comments to all functions
- [ ] Clean up unused CSS (remove warnings)
- [ ] Optimize bundle size (code splitting)
- [ ] Add service worker for offline support

---

## 🏆 Performance Goals

- [ ] First Contentful Paint (FCP) < 1.5s
- [ ] Time to Interactive (TTI) < 3s
- [ ] Lighthouse score > 90
- [ ] API response time < 200ms (p95)
- [ ] Upload speed optimization (chunked uploads)
- [ ] Download speed optimization (range requests)

---

## 📝 Notes

**Current Tech Stack:**

- Backend: Rust, Axum 0.7, SQLite, Tokio
- Frontend: Svelte 5, Vite, Material 3 Design
- Desktop: Tauri
- Mobile: Flutter (placeholder)

**Architecture Decisions:**

- Local-first approach (offline support planned)
- REST API with WebSocket for real-time updates
- File-based storage (no external dependencies)
- Single-binary deployment for ease of use

**Community Contributions Welcome:**

- Translations for other languages
- Custom themes
- Plugin development
- Bug reports and feature requests

---

Last Updated: 2025-10-23
Version: 0.3.0
