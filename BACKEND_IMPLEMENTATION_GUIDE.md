# Backend Implementation Guide - Complete API Setup

## 🎯 Mission

Implement ALL required backend APIs to support Frontend-First architecture with full multi-device sync.

**Status**: 48/51 Frontend features done ✅ → BLOCKED waiting for backend APIs ⏳

---

## 📊 Current Backend Status

### ✅ ALREADY IMPLEMENTED (Verified Working)

| API Module   | Endpoints                                         | Status  |
| ------------ | ------------------------------------------------- | ------- |
| **Users**    | GET/PUT /api/users/{profile,settings,preferences} | ✅ DONE |
| **Auth**     | POST /api/auth/{login,register,2fa}               | ✅ DONE |
| **Files**    | GET/POST/DELETE /api/files/\*                     | ✅ DONE |
| **Search**   | GET /api/search                                   | ✅ DONE |
| **Activity** | GET /api/activity/\*                              | ✅ DONE |

### ⏳ PENDING - NEED IMPLEMENTATION/FIXES

| Feature                | Endpoints Needed | Priority | Impact              |
| ---------------------- | ---------------- | -------- | ------------------- |
| Tags & Comments        | 10 endpoints     | P1       | Feature #15 blocked |
| File Versioning        | 7 endpoints      | P1       | Feature #16 blocked |
| Advanced Sharing       | 9 endpoints      | P1       | Feature #17 blocked |
| Collaboration Presence | 3 endpoints      | P2       | Feature #18 blocked |

---

## 🔴 CRITICAL BLOCKERS (Must Fix First)

### Issue #1: Tags API Missing

**Frontend**: `tagsComments.js` store expecting these APIs

```
GET /api/files/{path}/tags
POST /api/files/{path}/tags
PUT /api/files/{path}/tags/{tagId}
DELETE /api/files/{path}/tags/{tagId}
GET /api/tags/{tagName}/files
```

**Backend Status**: CHECK `/api/tags.rs` if exists

### Issue #2: Comments API Missing

**Frontend**: `tagsComments.js` store expecting these APIs

```
GET /api/files/{path}/comments
POST /api/files/{path}/comments
PUT /api/files/{path}/comments/{commentId}
DELETE /api/files/{path}/comments/{commentId}
POST /api/files/{path}/comments/{commentId}/reactions
```

**Backend Status**: CHECK `/api/comments.rs` if exists

### Issue #3: File Versions API Incomplete

**Frontend**: `fileVersioning.js` store expecting these APIs

```
GET /api/files/{path}/versions
GET /api/files/{path}/versions/{versionNumber}
GET /api/files/{path}/versions/{versionNumber}/download
POST /api/files/{path}/versions/{versionNumber}/restore
POST /api/files/{path}/versions/diff
DELETE /api/files/{path}/versions/{versionNumber}
POST /api/files/{path}/versions/cleanup
```

**Backend Status**: CHECK `/api/file_versions.rs` if exists (likely incomplete)

### Issue #4: Sharing API Incomplete

**Frontend**: `advancedSharing.js` store expecting these APIs

```
GET /api/sharing
GET /api/files/{path}/shares
POST /api/sharing
PUT /api/sharing/{shareId}
DELETE /api/sharing/{shareId}
POST /api/sharing/{shareId}/regenerate-token
GET /api/sharing/{shareId}/analytics
GET /api/sharing/{shareId}/access-log
GET /api/sharing/public/{shareToken}
GET /api/sharing/public/{shareToken}/download
```

**Backend Status**: CHECK `/api/sharing.rs` if exists

### Issue #5: Presence API Missing

**Frontend**: Will use for Collaboration Presence

```
GET /api/collaboration/presence
POST /api/collaboration/presence
DELETE /api/collaboration/presence
```

**Backend Status**: CHECK `/api/collaboration.rs` if exists

---

## 🛠️ Implementation Plan (By Priority)

### PHASE 1: User State Sync (P0 - CRITICAL)

**Status**: ✅ **ALREADY WORKING** - verified in `/api/users.rs`

- ✅ GET /api/users/settings
- ✅ PUT /api/users/settings
- ✅ GET /api/users/preferences
- ✅ PUT /api/users/preferences
- ✅ GET /api/users/profile
- ✅ PUT /api/users/profile

**Action**: ✅ NO WORK NEEDED - Frontend can use immediately!

**Database Tables**:

```sql
-- Already exists
ALTER TABLE users ADD COLUMN IF NOT EXISTS settings_json TEXT DEFAULT '{}';
ALTER TABLE users ADD COLUMN IF NOT EXISTS bio TEXT;

-- Already exists
CREATE TABLE IF NOT EXISTS user_preferences (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  view_mode TEXT DEFAULT 'grid',
  sort_by TEXT DEFAULT 'name',
  sort_order TEXT DEFAULT 'asc',
  sidebar_collapsed BOOLEAN DEFAULT FALSE,
  recent_searches TEXT,
  saved_filters TEXT,
  FOREIGN KEY (user_id) REFERENCES users(id)
);
```

---

### PHASE 2: Tags & Comments (P1 - BLOCKING FEATURE #15)

**Files to Create/Update**:

- `backend/src/api/tags.rs` (if missing)
- `backend/src/api/comments.rs` (if missing)
- `backend/src/services/tag_service_impl.rs` (if missing)

**Database Tables**:

```sql
CREATE TABLE IF NOT EXISTS file_tags (
  id TEXT PRIMARY KEY,
  file_path TEXT NOT NULL,
  name TEXT NOT NULL,
  color TEXT DEFAULT '#808080',
  created_by TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS file_comments (
  id TEXT PRIMARY KEY,
  file_path TEXT NOT NULL,
  text TEXT NOT NULL,
  author TEXT NOT NULL,
  parent_comment_id TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (author) REFERENCES users(id),
  FOREIGN KEY (parent_comment_id) REFERENCES file_comments(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS comment_reactions (
  id TEXT PRIMARY KEY,
  comment_id TEXT NOT NULL,
  emoji TEXT NOT NULL,
  user_id TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(comment_id, emoji, user_id),
  FOREIGN KEY (comment_id) REFERENCES file_comments(id) ON DELETE CASCADE,
  FOREIGN KEY (user_id) REFERENCES users(id)
);
```

**Endpoints to Implement**:

#### Tags Endpoints

```rust
GET /api/files/{path}/tags
  - List all tags for a file
  - Response: [ { id, name, color, createdBy, createdAt }, ... ]
  - Query: SELECT * FROM file_tags WHERE file_path = ? ORDER BY created_at DESC

PUT /api/files/{path}/tags/{tagId}
  - Update tag
  - Payload: { name?, color? }
  - Query: UPDATE file_tags SET name = ?, color = ? WHERE id = ? AND file_path = ?

POST /api/files/{path}/tags
  - Create tag
  - Payload: { name, color }
  - Validation: name not empty, color valid hex
  - Query: INSERT INTO file_tags VALUES (?, ?, ?, ?, ?, datetime('now'))
  - WebSocket: Broadcast tags_updated event

DELETE /api/files/{path}/tags/{tagId}
  - Delete tag
  - Query: DELETE FROM file_tags WHERE id = ? AND file_path = ?
  - WebSocket: Broadcast tags_updated event

GET /api/tags/{tagName}/files
  - Get all files with this tag
  - Response: [ files with this tag ]
  - Query: SELECT DISTINCT file_path FROM file_tags WHERE name = ?
```

#### Comments Endpoints

```rust
GET /api/files/{path}/comments
  - List all comments for a file (root level only)
  - Response: [ { id, text, author, createdAt, replies: [ ... ], reactions: [ ... ] }, ... ]
  - Query: SELECT * FROM file_comments WHERE file_path = ? AND parent_comment_id IS NULL

POST /api/files/{path}/comments
  - Create comment
  - Payload: { text, parentCommentId? }
  - Query: INSERT INTO file_comments VALUES (?, ?, ?, ?, ?, now(), now())
  - WebSocket: Broadcast comment_added event

PUT /api/files/{path}/comments/{commentId}
  - Edit comment (only own comments)
  - Payload: { text }
  - Query: UPDATE file_comments SET text = ?, updated_at = now() WHERE id = ? AND author = ?
  - WebSocket: Broadcast comment_updated event

DELETE /api/files/{path}/comments/{commentId}
  - Delete comment (cascade delete replies + reactions)
  - Query: DELETE FROM file_comments WHERE id = ? AND author = ?
  - WebSocket: Broadcast comment_deleted event

POST /api/files/{path}/comments/{commentId}/reactions
  - Add emoji reaction
  - Payload: { emoji }
  - Query: INSERT OR IGNORE INTO comment_reactions VALUES (?, ?, ?, ?, now())
  - WebSocket: Broadcast reaction_added event
```

---

### PHASE 3: File Versioning (P1 - BLOCKING FEATURE #16)

**Files to Create/Update**:

- `backend/src/api/file_versions.rs` (if incomplete)
- `backend/src/services/version_storage_service.rs` (likely exists)

**Database Tables**:

```sql
CREATE TABLE IF NOT EXISTS file_versions (
  id TEXT PRIMARY KEY,
  file_path TEXT NOT NULL,
  version_number INTEGER NOT NULL,
  file_hash TEXT NOT NULL,
  size_bytes INTEGER NOT NULL,
  created_by TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (created_by) REFERENCES users(id),
  UNIQUE(file_path, version_number)
);

-- File content storage (filesystem or DB)
-- Filesystem: ./data/versions/{file_path}/v{version_number}
-- DB Strategy: Store hash, retrieve from filesystem
```

**Endpoints to Implement**:

```rust
GET /api/files/{path}/versions
  - List all versions for a file
  - Response: [ { versionNumber, sizeBytes, fileHash, createdBy, createdAt }, ... ]
  - Query: SELECT * FROM file_versions WHERE file_path = ? ORDER BY version_number DESC

GET /api/files/{path}/versions/{versionNumber}
  - Get version metadata
  - Response: { versionNumber, sizeBytes, fileHash, createdBy, createdAt }
  - Query: SELECT * FROM file_versions WHERE file_path = ? AND version_number = ?

GET /api/files/{path}/versions/{versionNumber}/download
  - Download specific version (stream binary)
  - Response: Binary file content
  - Location: ./data/versions/{path}/v{versionNumber}
  - Header: Content-Disposition: attachment; filename="{filename}"

POST /api/files/{path}/versions/{versionNumber}/restore
  - Restore previous version (creates new version)
  - Response: { restored: true, newVersionNumber }
  - Action:
    1. Get old version content
    2. Create new version with current timestamp
    3. Write to active file location
    4. Insert new row in file_versions
  - WebSocket: Broadcast file_restored event

POST /api/files/{path}/versions/diff
  - Generate diff between two versions
  - Payload: { versionId1, versionId2 }
  - Response: { changes: [ { type, from, to, line }, ... ] }
  - Algorithm: diff-match-patch for text files
  - For binary: indicate "Binary files differ"

DELETE /api/files/{path}/versions/{versionNumber}
  - Delete old version
  - Query: DELETE FROM file_versions WHERE file_path = ? AND version_number = ?
  - Action: Remove ./data/versions/{path}/v{versionNumber}

POST /api/files/{path}/versions/cleanup
  - Delete versions older than N days
  - Payload: { daysOld }
  - Query: DELETE FROM file_versions WHERE file_path = ? AND created_at < datetime('now', '-' || ? || ' days')
  - Response: { deletedCount, freedSpace }
```

---

### PHASE 4: Advanced Sharing (P1 - BLOCKING FEATURE #17)

**Files to Create/Update**:

- `backend/src/api/sharing.rs` (likely exists, may need enhancement)

**Database Tables**:

```sql
CREATE TABLE IF NOT EXISTS shares (
  id TEXT PRIMARY KEY,
  file_path TEXT NOT NULL,
  share_token TEXT UNIQUE NOT NULL,
  permission TEXT NOT NULL DEFAULT 'read',  -- 'read' or 'write'
  expires_at DATETIME,
  password TEXT,
  download_limit INTEGER,
  downloads INTEGER DEFAULT 0,
  created_by TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS share_access_logs (
  id TEXT PRIMARY KEY,
  share_id TEXT NOT NULL,
  ip_address TEXT,
  user_agent TEXT,
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (share_id) REFERENCES shares(id) ON DELETE CASCADE
);
```

**Endpoints to Implement**:

```rust
GET /api/sharing
  - List all shares (user's own)
  - Response: [ shares ]
  - Query: SELECT * FROM shares WHERE created_by = ? ORDER BY created_at DESC

GET /api/files/{path}/shares
  - List shares for specific file
  - Response: [ shares ]
  - Query: SELECT * FROM shares WHERE file_path = ? ORDER BY created_at DESC

POST /api/sharing
  - Create share
  - Payload: { filePath, permission, expiresAt?, password?, downloadLimit? }
  - Validation: permission in ['read', 'write'], password length
  - Action:
    1. Generate random shareToken (32 chars)
    2. Hash password if provided
    3. INSERT into DB
  - Response: { id, shareToken, ... }
  - WebSocket: Broadcast share_created event

PUT /api/sharing/{shareId}
  - Update share settings
  - Payload: { permission?, expiresAt?, password?, downloadLimit? }
  - Query: UPDATE shares SET ... WHERE id = ? AND created_by = ?
  - WebSocket: Broadcast share_updated event

DELETE /api/sharing/{shareId}
  - Delete share
  - Query: DELETE FROM shares WHERE id = ? AND created_by = ?
  - WebSocket: Broadcast share_deleted event

POST /api/sharing/{shareId}/regenerate-token
  - Generate new token (invalidate old)
  - Action:
    1. Generate new shareToken
    2. UPDATE shares SET share_token = ?
    3. Old token becomes invalid
  - Response: { shareToken (new) }
  - WebSocket: Broadcast token_regenerated event

GET /api/sharing/{shareId}/analytics
  - Get share analytics
  - Response: { totalDownloads, lastAccessed, accessLog: [ ... ] }
  - Query: SELECT downloads, MAX(timestamp) FROM share_access_logs WHERE share_id = ?

GET /api/sharing/{shareId}/access-log
  - Get access history
  - Response: [ { ipAddress, userAgent, timestamp }, ... ]
  - Query: SELECT * FROM share_access_logs WHERE share_id = ? ORDER BY timestamp DESC

GET /api/sharing/public/{shareToken}
  - Public endpoint - NO AUTH REQUIRED
  - Get share info
  - Response: { filePath, file metadata, permission }
  - Validation: Check token exists, not expired, pass password check
  - Query: SELECT * FROM shares WHERE share_token = ?
  - Action: Log access in share_access_logs

GET /api/sharing/public/{shareToken}/download
  - Public download - NO AUTH REQUIRED
  - Stream file binary
  - Validation: Check token, expiry, downloads < limit
  - Action:
    1. Check share_token exists
    2. Check expires_at > now OR NULL
    3. Check downloads < download_limit OR NULL
    4. Increment downloads counter
    5. Log access
    6. Stream file
  - Response: Binary file
  - Header: Content-Disposition: attachment
```

---

### PHASE 5: Collaboration Presence (P2 - BLOCKING FEATURE #18)

**Database** (in-memory cache, optional DB backup):

```sql
CREATE TABLE IF NOT EXISTS user_presence (
  user_id TEXT PRIMARY KEY,
  file_path TEXT,
  status TEXT DEFAULT 'idle',  -- 'viewing', 'editing', 'idle'
  last_seen DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id)
);
```

**Endpoints to Implement**:

```rust
GET /api/collaboration/presence
  - List user presence for all files
  - Response: [ { userId, filePath, status, lastSeen, userProfile: { avatar, displayName } }, ... ]
  - Cache: Check Redis/in-memory first, then DB
  - Response Time: < 100ms (must be fast)

POST /api/collaboration/presence
  - Update user presence
  - Payload: { filePath, status }
  - Action:
    1. UPDATE in cache + DB
    2. Broadcast via WebSocket to all users
  - Response: { userId, filePath, status, timestamp }
  - WebSocket: Broadcast presence_updated event

DELETE /api/collaboration/presence
  - Remove presence (logout)
  - Action: DELETE from cache + DB, broadcast
  - Response: 204 No Content
  - WebSocket: Broadcast presence_cleared event
```

---

## 📋 Implementation Checklist

### PHASE 1: User State (READY NOW)

- [x] /api/users/settings (GET/PUT)
- [x] /api/users/preferences (GET/PUT)
- [x] /api/users/profile (GET/PUT)

### PHASE 2: Tags & Comments

- [ ] Database migration for tags table
- [ ] Database migration for comments table
- [ ] Database migration for reactions table
- [ ] `/api/tags.rs` - Tags endpoints
- [ ] `/api/comments.rs` - Comments endpoints
- [ ] Tag service implementation
- [ ] Comment service implementation
- [ ] WebSocket broadcasting for tags_updated
- [ ] WebSocket broadcasting for comment_added/updated/deleted/reactions
- [ ] Integration tests for all 10 endpoints
- [ ] Frontend testing with `tagsComments.js` store

### PHASE 3: File Versioning

- [ ] Database migration for file_versions table
- [ ] `/api/file_versions.rs` - Version endpoints (verify/complete)
- [ ] Version storage service (verify/complete)
- [ ] Diff algorithm implementation
- [ ] Version restoration logic
- [ ] Cleanup old versions logic
- [ ] WebSocket broadcasting for version_restored
- [ ] Integration tests for all 7 endpoints
- [ ] Frontend testing with `fileVersioning.js` store

### PHASE 4: Advanced Sharing

- [ ] Database migration for shares table
- [ ] Database migration for access_logs table
- [ ] `/api/sharing.rs` - Sharing endpoints (enhance)
- [ ] Share token generation (secure random)
- [ ] Public endpoint authentication bypass
- [ ] Download limit enforcement
- [ ] Password protection for shares
- [ ] Share analytics logic
- [ ] WebSocket broadcasting for share\_\* events
- [ ] Integration tests for all 9 endpoints
- [ ] Frontend testing with `advancedSharing.js` store

### PHASE 5: Presence

- [ ] Database table for user_presence
- [ ] `/api/collaboration.rs` - Presence endpoints (enhance)
- [ ] In-memory cache for presence (Redis optional)
- [ ] WebSocket broadcasting for presence_updated
- [ ] Integration tests for all 3 endpoints
- [ ] Frontend testing for Collaboration Presence

---

## 🚀 Quick Start Commands

### 1. Check Database Migrations

```bash
cd backend
sqlite3 ./data/syncspace.db ".tables"
sqlite3 ./data/syncspace.db "SELECT name FROM sqlite_master WHERE type='table';"
```

### 2. Run Migrations

```bash
cd backend
sqlx migrate run
# Or add new migrations to backend/migrations/
```

### 3. Test Backend

```bash
cd backend
cargo test --lib
cargo test --doc
cargo test
```

### 4. Run Backend Server

```bash
cd backend
cargo run --release
# Backend runs on http://localhost:8080
```

### 5. Test Endpoints

```bash
# Test tags endpoint
curl -H "Authorization: Bearer {token}" \
  http://localhost:8080/api/files/test.txt/tags

# Test public share endpoint
curl http://localhost:8080/api/sharing/public/{shareToken}
```

---

## 📞 WebSocket Broadcasting Events

Frontend listens for these events:

```javascript
// Tags & Comments
'tags_updated' - Tag was added/updated/deleted
'comment_added' - New comment posted
'comment_updated' - Comment edited
'comment_deleted' - Comment deleted
'reaction_added' - Emoji reaction added

// Versioning
'version_restored' - Version was restored
'version_deleted' - Version was deleted

// Sharing
'share_created' - Share link created
'share_updated' - Share settings changed
'share_deleted' - Share link deleted
'token_regenerated' - New share token generated

// Presence
'presence_updated' - User presence changed
'presence_cleared' - User logged out
```

---

## 🎯 Success Criteria

### Phase 1: User State

- ✅ All 3 endpoints working with JWT auth
- ✅ Settings/preferences persist across devices
- ✅ Frontend `serverState.js` successfully syncs

### Phase 2: Tags & Comments

- ✅ All 10 endpoints working
- ✅ Tags created/deleted/queried correctly
- ✅ Comments threading working
- ✅ Reactions display correctly
- ✅ WebSocket broadcasts work
- ✅ Frontend `tagsComments.js` fully functional

### Phase 3: Versioning

- ✅ All 7 endpoints working
- ✅ Version history stored correctly
- ✅ Restore creates new version
- ✅ Diff algorithm works
- ✅ Cleanup removes old versions
- ✅ Frontend `fileVersioning.js` fully functional

### Phase 4: Sharing

- ✅ All 9 endpoints working
- ✅ Share tokens generated uniquely
- ✅ Expiry enforced
- ✅ Passwords work
- ✅ Download limits enforced
- ✅ Public downloads work
- ✅ Frontend `advancedSharing.js` fully functional

### Phase 5: Presence

- ✅ All 3 endpoints working
- ✅ WebSocket broadcasts presence
- ✅ Frontend shows user avatars/presence
- ✅ Feature #18 fully implemented

---

## 📝 Notes

- All timestamps: UTC in RFC3339 format
- All IDs: UUID v4
- All passwords: Hashed with Argon2
- All tokens: 32-char alphanumeric
- All errors: HTTP status codes (400, 401, 403, 404, 500)
- All responses: JSON format
- All DB queries: Use sqlx prepared statements
- All endpoints: Require JWT auth (except /public/\*)

---

## 🔗 Related Files

**Frontend**:

- `/frontend/src/stores/serverState.js` - Syncs user settings
- `/frontend/src/stores/tagsComments.js` - Tags & comments
- `/frontend/src/stores/fileVersioning.js` - Version history
- `/frontend/src/stores/advancedSharing.js` - Share management
- `/frontend/BACKEND_API_AUDIT.md` - API requirements doc

**Backend**:

- `/backend/src/api/users.rs` - User endpoints ✅
- `/backend/src/api/tags.rs` - Tags endpoints (create/implement)
- `/backend/src/api/comments.rs` - Comments endpoints (create/implement)
- `/backend/src/api/file_versions.rs` - Versioning endpoints (verify/complete)
- `/backend/src/api/sharing.rs` - Sharing endpoints (verify/complete)
- `/backend/src/api/collaboration.rs` - Presence endpoints (verify/complete)
- `/backend/src/database.rs` - Database models (add if missing)
- `/backend/src/websocket/` - WebSocket event broadcasting
