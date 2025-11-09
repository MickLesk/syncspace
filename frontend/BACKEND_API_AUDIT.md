# Backend API Requirements Audit

## Status Check: Frontend vs Backend Alignment

**Mission**: Ensure ALL frontend features have corresponding backend APIs

---

## ✅ COMPLETED - These APIs likely exist (old features)

- ✅ `POST /api/auth/login` - Login
- ✅ `POST /api/auth/register` - Register
- ✅ `GET /api/auth/me` - Get current user
- ✅ `POST /api/auth/setup-2fa` - Setup 2FA
- ✅ `POST /api/auth/verify-2fa` - Verify 2FA code
- ✅ `POST /api/auth/change-password` - Change password
- ✅ `GET /api/files/list?path=X` - List directory
- ✅ `POST /api/upload/{path}` - Upload file
- ✅ `POST /api/dirs/{path}` - Create directory
- ✅ `DELETE /api/files/{path}` - Delete file
- ✅ `PUT /api/files/rename/{path}` - Rename file
- ✅ `PUT /api/files/move/{path}` - Move file
- ✅ `POST /api/files/copy/{path}` - Copy file
- ✅ `GET /api/search` - Full-text search
- ✅ `POST /api/batch/delete` - Batch delete
- ✅ `POST /api/batch/copy` - Batch copy
- ✅ `POST /api/batch/move` - Batch move

---

## ⏳ PENDING - NEW Backend APIs Required for Wave 4/5 Features

### 1. Backend-First User State (NEW)

**Status**: REQUIRED for serverState.js to work

```
⏳ GET /api/users/settings
   Response: { theme, language, defaultView, emailNotifications, ... }
   Endpoint: Get user settings from DB
   Storage: DB table `users` column `settings_json`

⏳ PUT /api/users/settings
   Payload: { theme?, language?, ... }
   Response: Updated settings object
   Endpoint: Save to DB
   Storage: DB table `users` column `settings_json`

⏳ GET /api/users/preferences
   Response: { viewMode, sortBy, sortOrder, recentSearches, savedFilters, ... }
   Endpoint: Get user preferences from DB
   Storage: DB table `user_preferences`

⏳ PUT /api/users/preferences
   Payload: { viewMode?, sortBy?, ... }
   Response: Updated preferences object
   Endpoint: Save to DB
   Storage: DB table `user_preferences`

⏳ GET /api/users/profile
   Response: { id, username, email, displayName, bio, avatar, role, ... }
   Endpoint: Get user profile
   Storage: DB table `users`

⏳ PUT /api/users/profile
   Payload: { displayName?, bio?, avatar?, ... }
   Response: Updated profile object
   Endpoint: Save to DB
   Storage: DB table `users`
```

### 2. Tags Management (NEW)

**Status**: REQUIRED for tagComments feature

```
⏳ GET /api/files/{path}/tags
   Response: [ { id, name, color, createdBy, createdAt }, ... ]
   Endpoint: Get all tags for a file
   Storage: DB table `file_tags`

⏳ POST /api/files/{path}/tags
   Payload: { name, color }
   Response: { id, name, color, createdBy, createdAt }
   Endpoint: Create new tag
   Storage: Insert into DB

⏳ PUT /api/files/{path}/tags/{tagId}
   Payload: { name?, color? }
   Response: Updated tag object
   Endpoint: Update tag
   Storage: Update in DB

⏳ DELETE /api/files/{path}/tags/{tagId}
   Response: 204 No Content
   Endpoint: Delete tag
   Storage: Delete from DB

⏳ GET /api/tags/{tagName}/files
   Response: [ files with this tag ]
   Endpoint: Get files by tag
   Storage: Query DB
```

### 3. Comments Management (NEW)

**Status**: REQUIRED for comments feature

```
⏳ GET /api/files/{path}/comments
   Response: [ { id, text, author, createdAt, parentCommentId, replies[], ... }, ... ]
   Endpoint: Get all comments for a file
   Storage: DB table `file_comments`

⏳ POST /api/files/{path}/comments
   Payload: { text, parentCommentId? }
   Response: { id, text, author, createdAt, ... }
   Endpoint: Create comment
   Storage: Insert into DB
   Broadcasting: WebSocket broadcast to other users

⏳ PUT /api/files/{path}/comments/{commentId}
   Payload: { text }
   Response: Updated comment object
   Endpoint: Edit comment
   Storage: Update in DB
   Broadcasting: WebSocket broadcast

⏳ DELETE /api/files/{path}/comments/{commentId}
   Response: 204 No Content
   Endpoint: Delete comment
   Storage: Delete from DB + child replies
   Broadcasting: WebSocket broadcast

⏳ POST /api/files/{path}/comments/{commentId}/reactions
   Payload: { emoji }
   Response: Updated comment with reactions
   Endpoint: Add emoji reaction
   Storage: Update in DB
   Broadcasting: WebSocket broadcast
```

### 4. File Versioning (NEW)

**Status**: REQUIRED for versioning feature

```
⏳ GET /api/files/{path}/versions
   Response: [ { id, versionNumber, sizeBytes, createdBy, createdAt, fileHash }, ... ]
   Endpoint: Get version history
   Storage: DB table `file_versions`

⏳ GET /api/files/{path}/versions/{versionNumber}
   Response: { version metadata + binary content headers }
   Endpoint: Get specific version
   Storage: DB table `file_versions` + filesystem

⏳ GET /api/files/{path}/versions/{versionNumber}/download
   Response: File binary stream
   Endpoint: Download specific version
   Storage: Filesystem

⏳ POST /api/files/{path}/versions/{versionNumber}/restore
   Response: { restored: true, newVersionNumber }
   Endpoint: Restore previous version
   Storage: Create new version, backup current to version history
   Action: Actual restore operation
   Broadcasting: WebSocket broadcast file changed

⏳ POST /api/files/{path}/versions/diff
   Payload: { versionId1, versionId2 }
   Response: { type, changes[] }
   Endpoint: Generate diff between versions
   Service: Use diff algorithm (text files)

⏳ DELETE /api/files/{path}/versions/{versionNumber}
   Response: 204 No Content
   Endpoint: Delete old version
   Storage: Delete from DB and filesystem

⏳ POST /api/files/{path}/versions/cleanup
   Payload: { daysOld }
   Response: { deletedCount, freedSpace }
   Endpoint: Delete versions older than N days
   Storage: Batch delete old versions
```

### 5. Advanced Sharing (NEW)

**Status**: REQUIRED for sharing feature

```
⏳ GET /api/sharing
   Response: [ { id, filePath, shareToken, permission, expiresAt, password, downloadLimit, ... }, ... ]
   Endpoint: Get all shares (user's)
   Storage: DB table `shares`

⏳ GET /api/files/{path}/shares
   Response: [ shares for this file ]
   Endpoint: Get shares for specific file
   Storage: Query DB

⏳ POST /api/sharing
   Payload: { filePath, permission, expiresAt?, password?, downloadLimit? }
   Response: { id, shareToken, ... }
   Endpoint: Create share
   Storage: Insert into DB
   Generation: Generate unique shareToken

⏳ PUT /api/sharing/{shareId}
   Payload: { permission?, expiresAt?, password?, downloadLimit? }
   Response: Updated share object
   Endpoint: Update share settings
   Storage: Update in DB

⏳ DELETE /api/sharing/{shareId}
   Response: 204 No Content
   Endpoint: Delete share
   Storage: Delete from DB

⏳ POST /api/sharing/{shareId}/regenerate-token
   Response: { shareToken (new), ... }
   Endpoint: Generate new token (invalidate old)
   Storage: Update in DB
   Impact: Old link stops working

⏳ GET /api/sharing/{shareId}/analytics
   Response: { totalDownloads, lastAccessed, accessLog[] }
   Endpoint: Get share analytics
   Storage: Query DB

⏳ GET /api/sharing/{shareId}/access-log
   Response: [ { ipAddress, userAgent, timestamp }, ... ]
   Endpoint: Get access logs
   Storage: DB table `share_access_logs`

⏳ GET /api/sharing/public/{shareToken}
   Response: { filePath, file metadata, permission }
   Endpoint: Get share info (public, no auth)
   Auth: None required, check token expiry + downloads
   Access: Password check if required

⏳ GET /api/sharing/public/{shareToken}/download
   Response: File binary stream
   Endpoint: Download via share link
   Auth: None, check token + password
   Logging: Record download in access log
   Limits: Check downloadLimit counter
```

### 6. Collaboration - Presence (NEW)

**Status**: REQUIRED for presence feature

```
⏳ GET /api/collaboration/presence
   Response: [ { userId, filePath, status, lastSeen }, ... ]
   Endpoint: Get user presence for all files
   Storage: Redis or in-memory cache

⏳ POST /api/collaboration/presence
   Payload: { filePath, status }
   Response: { userId, filePath, status, timestamp }
   Endpoint: Update user presence
   Action: Record in cache + broadcast via WebSocket
   Broadcasting: Send to all connected users

⏳ DELETE /api/collaboration/presence
   Response: 204 No Content
   Endpoint: Remove presence (logout)
   Action: Clean up from cache
   Broadcasting: Notify other users
```

### 7. Collaboration - File Locking (probably exists, verify)

**Status**: Check if working properly

```
⏳ GET /api/collaboration/locks
   Response: [ { filePath, lockedBy, lockedAt }, ... ]
   Endpoint: Get all file locks
   Storage: DB table `file_locks`

⏳ POST /api/collaboration/locks
   Payload: { filePath }
   Response: { filePath, lockedBy, lockedAt, expiresAt }
   Endpoint: Acquire lock
   Storage: Insert into DB
   Validation: Check not already locked
   Renewal: Auto-renew on client activity

⏳ DELETE /api/collaboration/locks/{filePath}
   Response: 204 No Content
   Endpoint: Release lock
   Storage: Delete from DB
   Broadcasting: WebSocket notify lock released
```

---

## 📋 MAPPING: Frontend Store → Required Backend API

| Feature          | Store              | Requires                                       | Status |
| ---------------- | ------------------ | ---------------------------------------------- | ------ |
| Theme/Language   | serverState.js     | GET/PUT /api/users/settings                    | ⏳     |
| View Preferences | serverState.js     | GET/PUT /api/users/preferences                 | ⏳     |
| User Profile     | serverState.js     | GET/PUT /api/users/profile                     | ⏳     |
| Tags             | tagsComments.js    | GET/POST/PUT/DELETE /api/files/{path}/tags     | ⏳     |
| Comments         | tagsComments.js    | GET/POST/PUT/DELETE /api/files/{path}/comments | ⏳     |
| Versions         | fileVersioning.js  | GET/POST/DELETE /api/files/{path}/versions     | ⏳     |
| Sharing          | advancedSharing.js | GET/POST/PUT/DELETE /api/sharing               | ⏳     |
| Presence         | (future)           | GET/POST /api/collaboration/presence           | ⏳     |

---

## 🚨 CRITICAL ISSUES

### Issue 1: No Backend APIs for User Settings

**Impact**: serverState.js won't work, theme/language/preferences can't persist
**Fix**: Implement /api/users/{settings,preferences,profile} endpoints

### Issue 2: No Tags API

**Impact**: Tags feature incomplete, can't create/delete/query tags
**Fix**: Implement /api/files/{path}/tags endpoints

### Issue 3: No Comments API

**Impact**: Comments feature incomplete, no backend persistence
**Fix**: Implement /api/files/{path}/comments endpoints

### Issue 4: No Versioning API

**Impact**: Version history not accessible, restore broken
**Fix**: Implement /api/files/{path}/versions endpoints

### Issue 5: No Sharing API

**Impact**: Advanced sharing broken, no share management
**Fix**: Implement /api/sharing endpoints + share tokens

### Issue 6: No Presence API

**Impact**: Real-time collaboration presence missing
**Fix**: Implement /api/collaboration/presence endpoints

---

## ✅ VERIFICATION CHECKLIST

Before moving to Backend-First Collaboration Presence (#18):

- [ ] All 3 settings/preferences/profile APIs exist
- [ ] All 5 tags endpoints working
- [ ] All 5 comments endpoints working
- [ ] All 5 versioning endpoints working
- [ ] All 9 sharing endpoints working
- [ ] WebSocket broadcasts working for tags/comments/versioning/sharing
- [ ] Share tokens generating correctly
- [ ] Share expiry checking
- [ ] Share password protection
- [ ] Download limits enforced
- [ ] Version diff algorithm implemented
- [ ] Version restore working
- [ ] All 6 features tested end-to-end

---

## 📝 TODO for Backend Team

Priority order:

1. **Priority 1** (Blocking everything):

   - [ ] Implement /api/users/settings GET/PUT
   - [ ] Implement /api/users/preferences GET/PUT
   - [ ] Implement /api/users/profile GET/PUT

2. **Priority 2** (Core features):

   - [ ] Implement /api/files/{path}/tags (all endpoints)
   - [ ] Implement /api/files/{path}/comments (all endpoints)
   - [ ] Implement /api/files/{path}/versions (all endpoints)

3. **Priority 3** (Sharing):

   - [ ] Implement /api/sharing (all endpoints)
   - [ ] Share token generation
   - [ ] Public share endpoint

4. **Priority 4** (Real-time):
   - [ ] WebSocket: tags_updated broadcast
   - [ ] WebSocket: comment_added broadcast
   - [ ] WebSocket: version_restored broadcast
   - [ ] WebSocket: share_created broadcast
   - [ ] WebSocket: presence_updated broadcast

---

## 💡 Implementation Notes

### Database Tables Needed

```sql
-- User settings (already have users table, add column)
ALTER TABLE users ADD COLUMN settings_json TEXT DEFAULT '{}';
ALTER TABLE users ADD COLUMN bio TEXT;

-- User preferences (new table)
CREATE TABLE user_preferences (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  view_mode TEXT DEFAULT 'grid',
  sort_by TEXT DEFAULT 'name',
  sort_order TEXT DEFAULT 'asc',
  sidebar_collapsed BOOLEAN DEFAULT FALSE,
  recent_searches TEXT,
  saved_filters TEXT,
  preferences_json TEXT DEFAULT '{}',
  FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Tags (new table)
CREATE TABLE file_tags (
  id TEXT PRIMARY KEY,
  file_path TEXT NOT NULL,
  name TEXT NOT NULL,
  color TEXT,
  created_by TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Comments (new table)
CREATE TABLE file_comments (
  id TEXT PRIMARY KEY,
  file_path TEXT NOT NULL,
  text TEXT NOT NULL,
  author TEXT NOT NULL,
  parent_comment_id TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Versions (probably exists, verify)
-- Create if missing:
-- CREATE TABLE file_versions (...)

-- Shares (new table)
CREATE TABLE shares (
  id TEXT PRIMARY KEY,
  file_path TEXT NOT NULL,
  share_token TEXT UNIQUE NOT NULL,
  permission TEXT DEFAULT 'read',
  expires_at DATETIME,
  password TEXT,
  download_limit INTEGER,
  downloads INTEGER DEFAULT 0,
  created_by TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Share access logs
CREATE TABLE share_access_logs (
  id TEXT PRIMARY KEY,
  share_id TEXT NOT NULL,
  ip_address TEXT,
  user_agent TEXT,
  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (share_id) REFERENCES shares(id)
);
```

---

## Next Steps

1. **Backend Team**: Implement Priority 1 APIs (user settings/preferences/profile)
2. **QA**: Test each API endpoint thoroughly
3. **Frontend**: Enable serverState.js and verify multi-device sync
4. **Then Continue**: Move to Priority 2 (tags, comments, versioning)
