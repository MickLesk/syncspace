# SyncSpace - Quick Fixes & Improvements

## ✅ Completed (This Session)

### Core Features

- ✅ Activity logging integration (upload/delete/rename)
- ✅ Comments & Tags backend + frontend migration
- ✅ i18n error message translation (15 new keys)
- ✅ Security: Debounce, upload queue, retry logic
- ✅ Performance: Thumbnail caching with file.id, 50MB limit

### Bug Fixes

- ✅ Fixed upload queue syntax error
- ✅ Fixed formatFileSize → formatSize reference
- ✅ Fixed formatDate → new Date().toLocaleString()
- ✅ Fixed openPreview → showPreviewModal

---

## 🔥 Critical Fixes (Do Next)

### 1. Fix touchGesture.startY Error

**File**: `frontend/src/pages/FilesView.svelte:1138`
**Issue**: Property 'startY' doesn't exist on TouchGestureHandler
**Fix**:

```javascript
// Check TouchGestureHandler class implementation
// Add getter or use different property name
const startY = touchGesture.getStartY() || 0;
```

### 2. Add Keyboard Handlers (A11y)

**Files**: Multiple Svelte components
**Issue**: Clickable divs without keyboard handlers
**Fix**:

```svelte
<!-- Before -->
<div on:click={handleClick}>

<!-- After -->
<div
  role="button"
  tabindex="0"
  on:click={handleClick}
  on:keydown={(e) => e.key === 'Enter' && handleClick()}
>
```

### 3. Fix Self-Closing Tags

**Files**: CommentsPanel.svelte, PreviewModal.svelte
**Issue**: `<textarea />`, `<video />`, `<option />` should not be self-closing
**Fix**:

```svelte
<!-- Before -->
<textarea bind:value={text} />

<!-- After -->
<textarea bind:value={text}></textarea>
```

---

## 🎯 High-Priority Features

### 1. Lazy Loading with IntersectionObserver

**Estimated Time**: 2-3 hours
**Impact**: High (performance boost for large file lists)

**Implementation**:

```javascript
// Create new file: frontend/src/utils/lazyLoad.js
export function createLazyLoader(callback) {
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          callback(entry.target);
          observer.unobserve(entry.target);
        }
      });
    },
    { rootMargin: "50px" }
  );

  return observer;
}

// Use in FilesView.svelte
const thumbnailObserver = createLazyLoader(async (element) => {
  const fileId = element.dataset.fileId;
  const thumbnail = await getThumbnail(fileObj);
  element.src = thumbnail;
});
```

### 2. Edit Comment Backend

**Estimated Time**: 1 hour
**Impact**: Medium (completes comments feature)

**Backend**:

```rust
// Add to main.rs
async fn update_comment_handler(
    State(state): State<AppState>,
    user: auth::User,
    Path(id): Path<String>,
    Json(req): Json<UpdateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>), StatusCode> {
    sqlx::query(
        "UPDATE comments SET text = ?, updated_at = datetime('now')
         WHERE id = ? AND author_id = ?"
    )
    .bind(&req.text)
    .bind(&id)
    .bind(&user.id.to_string())
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return updated comment
}

// Add route
.route("/api/comments/:id", patch(update_comment_handler))
```

### 3. File Versioning Basics

**Estimated Time**: 3-4 hours
**Impact**: High (enables undo for destructive operations)

**Migration**:

```sql
-- Already tracked in file_history!
-- Just need to store file content snapshots

CREATE TABLE IF NOT EXISTS file_versions (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    size INTEGER NOT NULL,
    content_hash TEXT NOT NULL,
    storage_path TEXT NOT NULL, -- e.g., "./data/.versions/abc123.v1"
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (created_by) REFERENCES users(id)
);
```

**Backend Handler**:

```rust
async fn list_file_versions_handler(
    State(state): State<AppState>,
    Path(file_path): Path<String>,
) -> Result<Json<Vec<FileVersion>>, StatusCode> {
    let versions = sqlx::query_as!(
        FileVersion,
        "SELECT * FROM file_versions WHERE file_path = ? ORDER BY version_number DESC",
        file_path
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(versions))
}
```

---

## 🧪 Testing Checklist

### Unit Tests Needed

- [ ] `debounce()` function with timer checks
- [ ] `throttle()` function with rate limiting
- [ ] `RequestQueue` with concurrency control
- [ ] `retryWithBackoff()` with mock failures
- [ ] `enforceCacheSizeLimit()` with size calculations

### E2E Tests Needed

- [ ] Upload flow (select file → progress → success)
- [ ] Delete flow with confirmation dialog
- [ ] Search functionality
- [ ] Comments creation and display
- [ ] Tag addition and filtering

**Setup**:

```bash
npm install -D @playwright/test
npx playwright install
```

**Example Test**:

```javascript
// tests/upload.spec.js
import { test, expect } from "@playwright/test";

test("upload file successfully", async ({ page }) => {
  await page.goto("http://localhost:5173");

  // Login
  await page.fill('[name="username"]', "admin");
  await page.fill('[name="password"]', "admin123");
  await page.click('button:has-text("Login")');

  // Upload file
  const fileInput = await page.locator('input[type="file"]');
  await fileInput.setInputFiles("./tests/fixtures/test.pdf");

  // Wait for upload success toast
  await expect(page.locator(".toast-success")).toBeVisible();

  // Verify file appears in list
  await expect(page.locator('.file-card:has-text("test.pdf")')).toBeVisible();
});
```

---

## 📊 Performance Optimizations

### Current Metrics (Before)

- Bundle size: ~450KB (uncompressed)
- Initial load: ~2.5s
- Thumbnail generation: ~500ms per image

### Optimization Targets

- [ ] Code splitting: Reduce initial bundle to <300KB
- [ ] Lazy load routes: Load components on-demand
- [ ] Image optimization: WebP format, progressive loading
- [ ] Service worker: Cache static assets
- [ ] Preload critical resources: Fonts, CSS

**Vite Config Updates**:

```javascript
// vite.config.js
export default {
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ["svelte"],
          ui: ["./src/components/ui"],
        },
      },
    },
  },
};
```

---

## 🎨 UI Polish Ideas

### Micro-interactions

- [ ] File card hover animation (lift + shadow)
- [ ] Upload progress with animated circle
- [ ] Toast notifications slide in from top-right
- [ ] Loading skeleton screens (already implemented!)
- [ ] Smooth transitions between views

### Visual Improvements

- [ ] File type icons with color coding
- [ ] Folder icons with depth shading
- [ ] Grid/List view toggle with animation
- [ ] Drag preview ghost image
- [ ] Context menu with icons

### Dark Mode Enhancements

- [ ] Improved contrast ratios (WCAG AAA)
- [ ] Smooth theme transition animation
- [ ] Remember user preference in localStorage
- [ ] System theme auto-detection

---

## 🔐 Security Hardening

### Input Validation

- [ ] Sanitize file paths (prevent directory traversal)
- [ ] Validate file extensions (configurable whitelist)
- [ ] Limit file sizes (configurable max size)
- [ ] Scan uploads for malware (ClamAV integration)

### Authentication

- [ ] Implement refresh tokens
- [ ] Add CSRF protection
- [ ] Rate limit login attempts
- [ ] Add password strength requirements

### API Security

- [ ] Add request signing
- [ ] Implement API versioning
- [ ] Add CORS whitelist configuration
- [ ] Log all failed auth attempts

---

## 📚 Documentation Tasks

### README Updates

- [ ] Add installation instructions
- [ ] Document environment variables
- [ ] Add screenshot/GIF demo
- [ ] List all API endpoints
- [ ] Add contribution guidelines

### Code Documentation

- [ ] Add JSDoc to all exported functions
- [ ] Document component props with TypeScript
- [ ] Add inline comments for complex logic
- [ ] Create architecture diagrams

### User Guide

- [ ] How to upload files
- [ ] How to share files
- [ ] How to use comments and tags
- [ ] Keyboard shortcuts reference
- [ ] FAQ section

---

## 🚀 Deployment Checklist

### Production Readiness

- [ ] Environment-based configuration
- [ ] Health check endpoint
- [ ] Graceful shutdown handling
- [ ] Error logging to file
- [ ] Request logging (access logs)

### Docker Setup

```dockerfile
# Dockerfile
FROM rust:1.75 as backend-builder
WORKDIR /app
COPY backend/ .
RUN cargo build --release

FROM node:20 as frontend-builder
WORKDIR /app
COPY frontend/ .
RUN npm ci && npm run build

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=backend-builder /app/target/release/syncbackend /usr/local/bin/
COPY --from=frontend-builder /app/dist /var/www/syncspace
ENV FRONTEND_PATH=/var/www/syncspace
EXPOSE 8080
CMD ["syncbackend"]
```

### Nginx Reverse Proxy

```nginx
server {
    listen 80;
    server_name syncspace.example.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

---

## 💡 Ideas for Future

### Community Features

- User profiles with avatars
- Follow other users
- Public file gallery
- Activity feed (global + per-user)

### Advanced Search

- Saved search queries
- Smart folders (auto-update based on rules)
- Search by metadata (EXIF, ID3 tags)
- Search inside files (full-text)

### Automation

- Scheduled tasks (auto-cleanup, backups)
- File watchers (trigger on upload/delete)
- Webhooks for external integrations
- Auto-tagging rules

---

**Priority Order for Next Session:**

1. Fix critical bugs (touchGesture, a11y, self-closing tags)
2. Implement lazy loading with IntersectionObserver
3. Add edit comment functionality
4. Create basic unit tests
5. Start file versioning system

Estimated Time: 6-8 hours for top 5 priorities
