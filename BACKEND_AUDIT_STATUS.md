# Backend Audit Status - November 9, 2025

## 🎯 Mission Critical: Check if existing APIs match Frontend expectations

---

## ✅ EXISTING BACKEND MODULES (35 API files)

All located in `backend/src/api/`:

- ✅ auth.rs (login, register, 2FA)
- ✅ auth_security.rs (sessions, password policy)
- ✅ users.rs (settings, preferences, profile) **← CRITICAL**
- ✅ files.rs (file operations)
- ✅ directories.rs (directory operations)
- ✅ search.rs (full-text search)
- ✅ sharing.rs (file sharing)
- ✅ collaboration.rs (presence, locking)
- ✅ tags.rs (tag management)
- ✅ comments.rs (comment system)
- ✅ file_versions.rs (version history)
- ✅ versions.rs (another versioning module)
- ✅ activity.rs (activity log)
- ✅ quota.rs (storage quota)
- ✅ backup.rs (backup/restore)
- ✅ notifications.rs (notifications)
- ✅ batch.rs (batch operations)
- ✅ favorites.rs (favorites)
- ✅ trash.rs (trash/recycle bin)
- ✅ folder_colors.rs (folder colors)
- ✅ upload_chunk.rs (chunked uploads)
- ✅ performance.rs (performance metrics)
- ✅ system.rs (system info)
- ✅ config.rs (configuration)
- ✅ groups.rs (user groups)
- ✅ peers.rs (peer networking)
- ✅ recent.rs (recent files)
- ✅ duplicates.rs (duplicate detection)
- ✅ jobs.rs (background jobs)
- ✅ cron.rs (cron scheduling)
- ✅ database_health.rs (DB monitoring)
- ✅ errors.rs (error reporting)
- ✅ setup.rs (setup wizard)
- ✅ db_health.rs (DB health)

---

## 📋 ROUTE COMPARISON: Frontend Expects vs Backend Has

### Frontend: `/api/files/{path}/tags`

**Expected by**: `tagsComments.js` store

```javascript
GET / api / files / { path } / tags;
POST / api / files / { path } / tags;
PUT / api / files / { path } / tags / { tagId };
DELETE / api / files / { path } / tags / { tagId };
```

**Backend Currently Has** (from `tags.rs`):

```rust
GET /tags
POST /tags
DELETE /tags/{tag_id}
POST /file-tags
DELETE /file-tags/{file_tag_id}
```

**STATUS**: ❌ **ROUTE MISMATCH** - Tags work but with different endpoint structure

---

### Frontend: `/api/files/{path}/comments`

**Expected by**: `tagsComments.js` store

```javascript
GET / api / files / { path } / comments;
POST / api / files / { path } / comments;
PUT / api / files / { path } / comments / { commentId };
DELETE / api / files / { path } / comments / { commentId };
POST / api / files / { path } / comments / { commentId } / reactions;
```

**Backend Currently Has** (from `comments.rs`):

```rust
POST /comments
GET /comments
DELETE /comments/{comment_id}
PATCH /comments/{comment_id}/resolve
```

**STATUS**: ❌ **ROUTE MISMATCH** - Comments exist but different structure

---

### Frontend: `/api/files/{path}/versions`

**Expected by**: `fileVersioning.js` store

```javascript
GET / api / files / { path } / versions;
GET / api / files / { path } / versions / { versionNumber };
GET / api / files / { path } / versions / { versionNumber } / download;
POST / api / files / { path } / versions / { versionNumber } / restore;
POST / api / files / { path } / versions / diff;
DELETE / api / files / { path } / versions / { versionNumber };
POST / api / files / { path } / versions / cleanup;
```

**Backend Currently Has** (from `file_versions.rs` + `versions.rs`):

```rust
GET /file-versions/{file_id}
POST /file-versions/restore
DELETE /file-versions/{version_id}
```

**STATUS**: ⚠️ **INCOMPLETE** - Some endpoints exist but incomplete

---

### Frontend: `/api/sharing`

**Expected by**: `advancedSharing.js` store

```javascript
GET / api / sharing;
GET / api / files / { path } / shares;
POST / api / sharing;
PUT / api / sharing / { shareId };
DELETE / api / sharing / { shareId };
POST / api / sharing / { shareId } / regenerate - token;
GET / api / sharing / { shareId } / analytics;
GET / api / sharing / { shareId } / access - log;
GET / api / sharing / public / { shareToken };
GET / api / sharing / public / { shareToken } / download;
```

**Backend Currently Has** (from `sharing.rs`):

```rust
GET /shares
POST /shares
GET /shares/{share_id}
PUT /shares/{share_id}
DELETE /shares/{share_id}
PUT /shares/{share_id}/permissions
GET /shared-with-me
```

**STATUS**: ⚠️ **PARTIAL** - Sharing works but missing public endpoints

---

### Frontend: `/api/collaboration/presence`

**Expected by**: Future `presence.js` store (Feature #18)

```javascript
GET / api / collaboration / presence;
POST / api / collaboration / presence;
DELETE / api / collaboration / presence;
```

**Backend Currently Has** (from `collaboration.rs`):

```rust
GET /collaboration/locks
POST /collaboration/locks
DELETE /collaboration/locks/{file_path}
GET /collaboration/presence
POST /collaboration/presence
```

**STATUS**: ⚠️ **PARTIAL** - Presence endpoints exist!

---

### Frontend: `/api/users/settings`

**Expected by**: `serverState.js` store

```javascript
GET / api / users / settings;
PUT / api / users / settings;
```

**Backend Currently Has** (from `users.rs`):

```rust
GET /users/settings
PUT /users/settings
GET /users/preferences
PUT /users/preferences
GET /users/profile
PUT /users/profile
GET /users/me
```

**STATUS**: ✅ **WORKING** - All user endpoints exist!

---

## 🔴 CRITICAL ISSUES

### Issue #1: Route Path Mismatch

**Problem**: Frontend expects `/api/files/{path}/tags` but backend has `/api/tags`

**Impact**: Frontend API calls will fail because routes don't match

**Solution**:

1. Option A: Update backend routes to match frontend (recommended)
2. Option B: Update frontend stores to use existing routes (not recommended - breaks Backend-First)

**Recommended**: Update backend to use `/api/files/{path}/tags` pattern for consistency

---

### Issue #2: Missing Public Share Endpoints

**Problem**: Frontend `advancedSharing.js` calls:

- `GET /api/sharing/public/{shareToken}` (no auth)
- `GET /api/sharing/public/{shareToken}/download` (no auth)

**Impact**: Public share downloads won't work

**Solution**: Add public endpoints to `sharing.rs` that skip auth middleware

---

### Issue #3: Incomplete Version Diff

**Problem**: Frontend expects diff viewer but backend may not have diff endpoint

**Impact**: Version comparison won't work

**Solution**: Implement diff algorithm (diff-match-patch crate)

---

### Issue #4: Share Token Generation

**Problem**: Need secure token generation + regenerate endpoint

**Impact**: Public shares won't have unique tokens

**Solution**: Implement UUID-based token generation + regenerate endpoint

---

## 🎯 IMPLEMENTATION TODO (Prioritized)

### PRIORITY 1 - ROUTING FIXES (Must do first)

- [ ] **Tags API**: Refactor `/tags/*` to `/files/{path}/tags/*`

  - GET /api/files/{path}/tags
  - POST /api/files/{path}/tags
  - PUT /api/files/{path}/tags/{tagId}
  - DELETE /api/files/{path}/tags/{tagId}
  - Keep: GET /api/tags/{tagName}/files (search by tag)

- [ ] **Comments API**: Refactor `/comments/*` to `/files/{path}/comments/*`

  - GET /api/files/{path}/comments
  - POST /api/files/{path}/comments
  - PUT /api/files/{path}/comments/{commentId}
  - DELETE /api/files/{path}/comments/{commentId}
  - POST /api/files/{path}/comments/{commentId}/reactions

- [ ] **Versions API**: Refactor `/file-versions/*` to `/api/files/{path}/versions/*`

  - GET /api/files/{path}/versions
  - GET /api/files/{path}/versions/{versionNumber}
  - GET /api/files/{path}/versions/{versionNumber}/download
  - POST /api/files/{path}/versions/{versionNumber}/restore
  - POST /api/files/{path}/versions/diff
  - DELETE /api/files/{path}/versions/{versionNumber}
  - POST /api/files/{path}/versions/cleanup

- [ ] **Shares API**: Add public endpoints to `/sharing.rs`
  - GET /api/sharing/public/{shareToken} (NO AUTH)
  - GET /api/sharing/public/{shareToken}/download (NO AUTH)

### PRIORITY 2 - FEATURE COMPLETION

- [ ] **Share Tokens**: Implement secure UUID-based token generation

  - Generate unique 32-char tokens
  - Implement regenerate endpoint
  - Invalidate old tokens on regenerate

- [ ] **Version Diff**: Add diff algorithm

  - Use `similar` crate for text diff
  - Handle binary files
  - Return: lines added/removed/modified

- [ ] **Presence WebSocket**: Verify WebSocket broadcasts work

  - Broadcast presence_updated
  - Broadcast presence_cleared

- [ ] **Comments Reactions**: Implement emoji reactions
  - POST /api/files/{path}/comments/{commentId}/reactions
  - Store in DB
  - Broadcast via WebSocket

### PRIORITY 3 - COMPLETENESS

- [ ] **Versioning Cleanup**: Implement cleanup_old_versions

  - Delete versions older than N days
  - Return count + freed space

- [ ] **Share Analytics**: Implement analytics/access logs

  - Track downloads
  - Track access attempts
  - Return stats

- [ ] **Tag Deletion**: Cascade delete tags when file deleted
  - Clean up orphaned tags

---

## 🚀 Quick Fix Strategy

### Step 1: Update API Routing (2 hours)

```rust
// Current
router.route("/tags", get(list_tags))

// Target
router.route("/files/:path/tags", get(list_tags))
```

### Step 2: Add Public Endpoints (1 hour)

```rust
// Add to sharing.rs
.route("/sharing/public/:shareToken", get(get_public_share))
.route("/sharing/public/:shareToken/download", get(download_public_share))
```

### Step 3: Implement Diff (2 hours)

```rust
// Add to versions.rs
use similar::{ChangeTag, TextDiff};
// Implement diff_versions function
```

### Step 4: Test (2 hours)

```bash
cd backend
cargo test
curl -H "Authorization: Bearer {token}" http://localhost:8080/api/files/test.txt/tags
```

---

## 📊 Status Summary

| Component    | Current                | Target                    | Gap       | Priority |
| ------------ | ---------------------- | ------------------------- | --------- | -------- |
| Users API    | ✅ Working             | ✅ Working                | ✅ 0      | N/A      |
| Tags API     | ⚠️ Different routes    | ✅ /files/{path}/tags     | 🔴 High   | P1       |
| Comments API | ⚠️ Different routes    | ✅ /files/{path}/comments | 🔴 High   | P1       |
| Versions API | ⚠️ Incomplete          | ✅ Full versioning        | 🟡 Medium | P1       |
| Sharing API  | ⚠️ No public endpoints | ✅ Public + private       | 🟡 Medium | P1       |
| Presence API | ✅ Endpoints exist     | ✅ Keep current           | ✅ 0      | N/A      |

---

## 🎯 Success Criteria

✅ **When complete**:

1. Frontend `tagsComments.js` can call API successfully
2. Frontend `fileVersioning.js` can call API successfully
3. Frontend `advancedSharing.js` can call API successfully
4. Public share links work without authentication
5. All WebSocket events broadcast correctly
6. `serverState.js` syncs user settings across devices

---

## 📝 Related Files

**Backend**:

- `/backend/src/api/tags.rs`
- `/backend/src/api/comments.rs`
- `/backend/src/api/file_versions.rs`
- `/backend/src/api/versions.rs`
- `/backend/src/api/sharing.rs`
- `/backend/src/api/collaboration.rs`
- `/backend/src/api/users.rs` ✅
- `/backend/src/services/tag_service.rs`
- `/backend/src/services/comment_service.rs`
- `/backend/src/services/sharing_service.rs`

**Frontend**:

- `/frontend/src/stores/tagsComments.js`
- `/frontend/src/stores/fileVersioning.js`
- `/frontend/src/stores/advancedSharing.js`
- `/frontend/src/stores/serverState.js` ✅
- `/frontend/BACKEND_API_AUDIT.md`

---

## 🔗 Next Steps

1. ✅ Create routing structure for `/files/{path}/*` pattern
2. ✅ Migrate tags/comments/versions to new routing
3. ✅ Add public share endpoints
4. ✅ Implement diff algorithm
5. ✅ Test all endpoints with frontend
6. ✅ Enable frontend stores to sync with backend

**Estimated Time**: 8-12 hours development + 2-4 hours testing
