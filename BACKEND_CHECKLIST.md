# ✅ BACKEND IMPLEMENTATION CHECKLIST

## 📊 Current Status

- **Frontend**: 48/51 features (94%) ✅
- **Backend**: APIs exist but route misalignment ⚠️
- **Blocker**: 4 route mismatches + 3 missing features
- **Timeline**: 8-12 hours to complete

---

## 🎯 PHASE 1: SETUP (30 min)

- [ ] Build backend

  ```bash
  cd /home/mick/Dokumente/GitHub/syncspace/backend
  cargo build --release
  ```

- [ ] Check database

  ```bash
  sqlite3 ./data/syncspace.db ".tables" | grep -E "file_tags|file_comments|file_versions|shares"
  ```

- [ ] Start backend server

  ```bash
  cargo run --release
  # Should start on http://localhost:8080
  ```

- [ ] Get auth token (for testing)
  ```bash
  curl -X POST http://localhost:8080/api/auth/login \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"admin"}' \
    | jq -r '.token' > /tmp/token.txt
  ```

---

## 🎯 PHASE 2: ROUTE RESTRUCTURING (3 hours)

### 2.1 Tags Routes Refactoring

**File**: `/backend/src/api/tags.rs`

- [ ] Understand existing structure

  ```bash
  grep -n "pub fn router" src/api/tags.rs
  ```

- [ ] Add new `file_tags_router()` function

  - GET /files/{path}/tags
  - POST /files/{path}/tags
  - PUT /files/{path}/tags/{tagId}
  - DELETE /files/{path}/tags/{tagId}

- [ ] Implement list_file_tags handler

  ```rust
  Query: SELECT * FROM file_tags WHERE file_path = ?
  ```

- [ ] Implement create_file_tag handler

  ```rust
  Query: INSERT INTO file_tags VALUES (?, ?, ?, ?, ?, ?)
  ```

- [ ] Implement update_file_tag handler

  ```rust
  Query: UPDATE file_tags SET name=?, color=? WHERE id=?
  ```

- [ ] Implement delete_file_tag handler
  ```rust
  Query: DELETE FROM file_tags WHERE id=?
  ```

### 2.2 Comments Routes Refactoring

**File**: `/backend/src/api/comments.rs`

- [ ] Add new `file_comments_router()` function

  - GET /files/{path}/comments
  - POST /files/{path}/comments
  - PUT /files/{path}/comments/{commentId}
  - DELETE /files/{path}/comments/{commentId}
  - POST /files/{path}/comments/{commentId}/reactions

- [ ] Implement list_file_comments handler

  ```rust
  Query: SELECT * FROM file_comments WHERE file_path=? AND parent_comment_id IS NULL
  ```

- [ ] Implement create_file_comment handler
- [ ] Implement edit_file_comment handler
- [ ] Implement delete_file_comment handler
- [ ] Implement add_reaction handler

### 2.3 Versions Routes Refactoring

**File**: `/backend/src/api/file_versions.rs`

- [ ] Add new `file_versions_router()` function

  - GET /files/{path}/versions
  - GET /files/{path}/versions/{versionNumber}
  - GET /files/{path}/versions/{versionNumber}/download
  - POST /files/{path}/versions/{versionNumber}/restore
  - POST /files/{path}/versions/diff
  - DELETE /files/{path}/versions/{versionNumber}
  - POST /files/{path}/versions/cleanup

- [ ] Implement each handler following existing patterns

### 2.4 Register Routes

**File**: `/backend/src/api/mod.rs`

- [ ] Add imports

  ```rust
  use crate::api::{tags, comments, file_versions, sharing};
  ```

- [ ] Register new routers (BEFORE catch-all routes)

  ```rust
  .merge(file_versions::file_versions_router())
  .merge(tags::file_tags_router())
  .merge(comments::file_comments_router())
  .merge(sharing::file_sharing_router())
  .merge(files::router())  // This is catch-all, must come last
  ```

- [ ] Compile and test
  ```bash
  cargo build --release 2>&1 | head -20
  ```

---

## 🎯 PHASE 3: PUBLIC SHARE ENDPOINTS (1 hour)

**File**: `/backend/src/api/sharing.rs`

- [ ] Add `public_router()` function

  - GET /api/sharing/public/{shareToken}
  - GET /api/sharing/public/{shareToken}/download

- [ ] Create `PublicShareResponse` struct

  ```rust
  pub struct PublicShareResponse {
      file_path: String,
      permission: String,
      expires_at: Option<String>,
  }
  ```

- [ ] Implement get_public_share handler

  - Query: SELECT \* FROM shares WHERE share_token = ?
  - Check: expires_at > now (return 410 if expired)
  - Check: password protection
  - Return: 200 with share info

- [ ] Implement download_public_share handler

  - Check download limit
  - Increment downloads counter
  - Log access in share_access_logs
  - Stream file
  - Header: Content-Disposition: attachment

- [ ] Register public router in mod.rs

  ```rust
  // BEFORE auth middleware
  .merge(sharing::public_router())

  // THEN auth middleware applied
  .layer(middleware::...)
  ```

- [ ] Test public endpoint
  ```bash
  curl http://localhost:8080/api/sharing/public/test-token
  # Should work WITHOUT Authorization header
  ```

---

## 🎯 PHASE 4: DIFF ALGORITHM (1 hour)

**File**: `/backend/Cargo.toml`

- [ ] Add dependency

  ```toml
  similar = "2.2"
  ```

- [ ] Rebuild
  ```bash
  cargo build --release
  ```

**File**: `/backend/src/api/file_versions.rs`

- [ ] Add import

  ```rust
  use similar::{ChangeTag, TextDiff};
  ```

- [ ] Create DiffRequest/DiffResponse structs

  ```rust
  pub struct DiffRequest {
      version1: i32,
      version2: i32,
  }

  pub struct DiffResponse {
      changes: Vec<DiffChange>,
  }

  pub struct DiffChange {
      type_: String,  // "added", "removed", "modified"
      line: String,
  }
  ```

- [ ] Implement diff_versions handler

  ```rust
  async fn diff_versions(...) {
      let v1_content = get_version_content(...);
      let v2_content = get_version_content(...);

      let diff = TextDiff::from_lines(&v1_content, &v2_content);

      let changes = diff.iter_all_changes()
          .filter_map(|change| match change.tag() {
              ChangeTag::Delete => Some(DiffChange { type_: "removed", line: ... }),
              ChangeTag::Insert => Some(DiffChange { type_: "added", line: ... }),
              _ => None,
          })
          .collect();

      Ok(Json(DiffResponse { changes }))
  }
  ```

- [ ] Register route

  ```rust
  .route("/files/:path/versions/diff", post(diff_versions))
  ```

- [ ] Test
  ```bash
  TOKEN=$(cat /tmp/token.txt)
  curl -X POST http://localhost:8080/api/files/test.txt/versions/diff \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"version1":1,"version2":2}' | jq
  ```

---

## 🎯 PHASE 5: COMPLETE FEATURES (2-3 hours)

### 5.1 Share Token Regeneration

**File**: `/backend/src/api/sharing.rs`

- [ ] Add route

  ```rust
  .route("/sharing/:share_id/regenerate-token", post(regenerate_share_token))
  ```

- [ ] Implement handler
  ```rust
  async fn regenerate_share_token(...) {
      // 1. Generate new token
      let new_token = Uuid::new_v4().to_string();

      // 2. Update in DB
      sqlx::query("UPDATE shares SET share_token = ? WHERE id = ? AND created_by = ?")
          .bind(&new_token)
          .bind(&share_id)
          .bind(&user.id)
          .execute(&state.db_pool)
          .await?;

      // 3. Return new token
      Ok(Json(json!({ "share_token": new_token })))
  }
  ```

### 5.2 Version Cleanup

**File**: `/backend/src/api/file_versions.rs`

- [ ] Add route

  ```rust
  .route("/files/:path/versions/cleanup", post(cleanup_versions))
  ```

- [ ] Implement handler
  ```rust
  async fn cleanup_versions(..., Json(req): Json<CleanupRequest>) {
      // req.days_old: i32

      // 1. Get versions to delete
      let old_versions: Vec<FileVersion> = sqlx::query_as(
          "SELECT * FROM file_versions
           WHERE file_path = ?
           AND created_at < datetime('now', '-' || ? || ' days')"
      )
      .bind(&path)
      .bind(req.days_old)
      .fetch_all(&state.db_pool)
      .await?;

      // 2. Delete from DB
      sqlx::query(
          "DELETE FROM file_versions
           WHERE file_path = ?
           AND created_at < datetime('now', '-' || ? || ' days')"
      )
      .bind(&path)
      .bind(req.days_old)
      .execute(&state.db_pool)
      .await?;

      // 3. Calculate freed space
      let freed_space: i64 = old_versions.iter().map(|v| v.size_bytes).sum();

      Ok(Json(json!({
          "deleted_count": old_versions.len(),
          "freed_space": freed_space
      })))
  }
  ```

### 5.3 Comment Reactions

**File**: `/backend/src/api/comments.rs`

- [ ] Create comment_reactions table (if not exists)

  ```sql
  CREATE TABLE comment_reactions (
    id TEXT PRIMARY KEY,
    comment_id TEXT,
    emoji TEXT,
    user_id TEXT,
    UNIQUE(comment_id, emoji, user_id)
  );
  ```

- [ ] Add route

  ```rust
  .route("/files/:path/comments/:comment_id/reactions", post(add_reaction))
  ```

- [ ] Implement handler
  ```rust
  async fn add_reaction(..., Json(req): Json<ReactionRequest>) {
      // req.emoji: String

      sqlx::query(
          "INSERT OR IGNORE INTO comment_reactions
           VALUES (?, ?, ?, ?)"
      )
      .bind(Uuid::new_v4().to_string())
      .bind(&comment_id)
      .bind(&req.emoji)
      .bind(&user.id)
      .execute(&state.db_pool)
      .await?;

      Ok(StatusCode::CREATED)
  }
  ```

### 5.4 Share Analytics

**File**: `/backend/src/api/sharing.rs`

- [ ] Add route

  ```rust
  .route("/sharing/:share_id/analytics", get(get_share_analytics))
  ```

- [ ] Implement handler
  ```rust
  async fn get_share_analytics(...) {
      let stats = sqlx::query!(
          "SELECT COUNT(*) as downloads, MAX(timestamp) as last_access
           FROM share_access_logs WHERE share_id = ?",
          share_id
      )
      .fetch_one(&state.db_pool)
      .await?;

      Ok(Json(json!({
          "total_downloads": stats.downloads,
          "last_accessed": stats.last_access
      })))
  }
  ```

---

## 🎯 PHASE 6: TESTING (2 hours)

### 6.1 Compilation Check

- [ ] Build clean

  ```bash
  cd backend
  cargo clean
  cargo build --release 2>&1 | grep -i "error\|warning"
  ```

- [ ] Fix any compilation errors
  - Check types match
  - Check imports
  - Check database queries

### 6.2 Endpoint Testing

- [ ] Tags endpoints

  ```bash
  TOKEN=$(cat /tmp/token.txt)

  # Create tag
  curl -X POST http://localhost:8080/api/files/test.txt/tags \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"name":"important","color":"#FF0000"}'

  # List tags
  curl -H "Authorization: Bearer $TOKEN" \
    http://localhost:8080/api/files/test.txt/tags
  ```

- [ ] Comments endpoints

  ```bash
  # Create comment
  curl -X POST http://localhost:8080/api/files/test.txt/comments \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"text":"Great file!"}'
  ```

- [ ] Versions endpoints

  ```bash
  # List versions
  curl -H "Authorization: Bearer $TOKEN" \
    http://localhost:8080/api/files/test.txt/versions

  # Diff versions
  curl -X POST http://localhost:8080/api/files/test.txt/versions/diff \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"version1":1,"version2":2}'
  ```

- [ ] Public sharing (NO auth needed!)
  ```bash
  # This should work WITHOUT Authorization header
  curl http://localhost:8080/api/sharing/public/test-share-token
  ```

### 6.3 Frontend Integration

- [ ] Start frontend

  ```bash
  cd frontend
  npm run dev
  # Should be on http://localhost:5173
  ```

- [ ] Open browser console (F12)
- [ ] Check for errors
- [ ] Navigate to Tags section
- [ ] Create a tag (should appear immediately)
- [ ] Create a comment (should appear immediately)
- [ ] Create a share (should appear immediately)
- [ ] Check WebSocket in Network tab (should connect)

### 6.4 Multi-device Sync Test

- [ ] Open frontend in 2 browser tabs
- [ ] In Tab 1: Create a tag
- [ ] Check Tab 2: Tag appears automatically
- [ ] Verify WebSocket broadcast is working

### 6.5 Public Share Test

- [ ] Create a share in frontend
- [ ] Copy share link
- [ ] Open link in INCOGNITO mode (no auth)
- [ ] Should see file without login
- [ ] Download should work

---

## ✅ FINAL VERIFICATION

- [ ] All 4 route patterns working

  - `/api/files/{path}/tags` ✅
  - `/api/files/{path}/comments` ✅
  - `/api/files/{path}/versions` ✅
  - `/api/sharing/{id}/` ✅

- [ ] Public endpoints work WITHOUT auth

  - `GET /api/sharing/public/{token}` ✅

- [ ] Diff algorithm working

  - Version comparison showing changes ✅

- [ ] Database has all data

  - Tags stored and retrieved ✅
  - Comments stored and retrieved ✅
  - Versions stored and retrieved ✅
  - Shares stored and retrieved ✅

- [ ] WebSocket broadcasting

  - tags_updated events ✅
  - comment_added events ✅
  - version_restored events ✅
  - share_created events ✅

- [ ] Frontend stores syncing
  - tagsComments.js working ✅
  - fileVersioning.js working ✅
  - advancedSharing.js working ✅
  - serverState.js working ✅

---

## 🎁 SUCCESS!

Once all checks pass:

- ✅ Feature #15 (Tags & Comments) unlocked
- ✅ Feature #16 (File Versioning) unlocked
- ✅ Feature #17 (Advanced Sharing) unlocked
- ✅ Feature #18 (Collaboration Presence) ready to start
- ✅ Multi-device sync working
- ✅ Backend-First architecture complete

**Frontend Progress**: 48/51 → 51/51 (100%) 🎉

---

## 📞 Support

**If stuck on**:

- Route registration → Check mod.rs examples
- Database queries → Check existing endpoints in same file
- Type mismatches → Check request/response structs
- Compilation errors → Run `cargo check` for detailed messages
- Tests failing → Run `cargo test` for unit test results

**Documentation**:

- `/BACKEND_IMPLEMENTATION_ACTION.md` - Detailed step-by-step
- `/BACKEND_AUDIT_STATUS.md` - Current state analysis
- `/BACKEND_IMPLEMENTATION_GUIDE.md` - Complete specs
- `/BACKEND_API_AUDIT.md` - Frontend expectations

---

## 🚀 Ready to Begin?

```
Next Step: Pick a phase and start implementing!
Timeline: 8-12 hours total work
Result: All frontend features enabled + Backend-First sync working
```
