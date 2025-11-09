# Backend Implementation - Immediate Action Items

## 🚀 Start Here - Fastest Implementation Path

**Goal**: Make Frontend APIs work with Backend in 8 hours

---

## Phase 1: Setup (30 min)

### Step 1.1: Verify Backend Compiles

```bash
cd backend
cargo build --release 2>&1 | head -20
```

### Step 1.2: Check Database Tables

```bash
cd backend
sqlite3 ./data/syncspace.db ".tables"
sqlite3 ./data/syncspace.db "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;"
```

### Step 1.3: Review Existing API Module

```bash
wc -l src/api/*.rs | sort -n
# See which modules are largest
```

---

## Phase 2: Route Restructuring (3 hours)

### Problem: Frontend expects `/api/files/{path}/tags` but backend has `/api/tags`

### Solution: Create adapter routes

**File**: `backend/src/api/mod.rs`

Change from:

```rust
.merge(tags::router())
.merge(comments::router())
```

To:

```rust
.merge(files::router())  // This must handle /files/{*path}
.merge(tags::file_tags_router())  // Add new router for /files/{path}/tags
.merge(comments::file_comments_router())  // Add new router
```

### Step 2.1: Update `backend/src/api/tags.rs`

Add new router function:

```rust
pub fn file_tags_router() -> Router<AppState> {
    Router::new()
        // GET /api/files/path/to/file.txt/tags
        .route("/files/*path/tags", get(list_file_tags).post(create_file_tag))
        .route("/files/*path/tags/:tag_id", put(update_file_tag).delete(delete_file_tag))
}

async fn list_file_tags(
    State(state): State<AppState>,
    Path((path, _)): Path<(String, String)>,
    user: UserInfo,
) -> Result<Json<Vec<Tag>>, StatusCode> {
    // Get all tags for this file path
    let tags = sqlx::query_as::<_, Tag>(
        "SELECT id, name, color, created_by, created_at FROM file_tags WHERE file_path = ?"
    )
    .bind(&path)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tags))
}

async fn create_file_tag(
    State(state): State<AppState>,
    Path((path, _)): Path<(String, String)>,
    user: UserInfo,
    Json(req): Json<CreateTagRequest>,
) -> Result<(StatusCode, Json<Tag>), StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO file_tags (id, file_path, name, color, created_by, created_at)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&path)
    .bind(&req.name)
    .bind(req.color.unwrap_or_default())
    .bind(&user.id)
    .bind(&now)
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let tag = Tag {
        id,
        name: req.name,
        color: req.color,
        created_by: user.id,
        created_at: now,
    };

    Ok((StatusCode::CREATED, Json(tag)))
}
```

### Step 2.2: Update `backend/src/api/comments.rs`

Similar pattern for comments:

```rust
pub fn file_comments_router() -> Router<AppState> {
    Router::new()
        .route("/files/*path/comments", get(list_file_comments).post(create_file_comment))
        .route("/files/*path/comments/:comment_id", put(update_file_comment).delete(delete_file_comment))
        .route("/files/*path/comments/:comment_id/reactions", post(add_reaction))
}
```

### Step 2.3: Update `backend/src/api/file_versions.rs`

Version routes:

```rust
pub fn file_versions_router() -> Router<AppState> {
    Router::new()
        .route("/files/*path/versions", get(list_versions))
        .route("/files/*path/versions/:version_num", get(get_version))
        .route("/files/*path/versions/:version_num/download", get(download_version))
        .route("/files/*path/versions/:version_num/restore", post(restore_version))
        .route("/files/*path/versions/diff", post(diff_versions))
        .route("/files/*path/versions/:version_num", delete(delete_version))
        .route("/files/*path/versions/cleanup", post(cleanup_versions))
}
```

### Step 2.4: Update `backend/src/api/mod.rs`

```rust
pub fn build_api_router(state: AppState) -> Router<AppState> {
    Router::new()
        // ... existing routes ...

        // IMPORTANT: More specific routes FIRST
        .merge(file_versions::file_versions_router())
        .merge(tags::file_tags_router())
        .merge(comments::file_comments_router())
        .merge(sharing::public_router())  // Public shares (no auth)

        // General routes
        .merge(files::router())  // /files/{*path}

        // Protected routes layer
        .layer(middleware::from_fn_with_state(...))
}
```

---

## Phase 3: Public Share Endpoints (1 hour)

### Problem: Public shares need to bypass authentication

### Solution: Create separate public router

**File**: `backend/src/api/sharing.rs`

Add:

```rust
pub fn public_router() -> Router<AppState> {
    Router::new()
        // PUBLIC - No auth required
        .route("/sharing/public/:share_token", get(get_public_share))
        .route("/sharing/public/:share_token/download", get(download_public_share))
}

async fn get_public_share(
    State(state): State<AppState>,
    Path(share_token): Path<String>,
    Query(params): Query<PublicShareQuery>,
) -> Result<Json<PublicShareResponse>, StatusCode> {
    // Get share by token (public, no auth needed)
    let share: Share = sqlx::query_as(
        "SELECT * FROM shares WHERE share_token = ?"
    )
    .bind(&share_token)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check expiry
    if let Some(expires_at) = share.expires_at {
        if expires_at < Utc::now().to_rfc3339() {
            return Err(StatusCode::GONE); // 410 Gone
        }
    }

    // Check password if required
    if let Some(ref password) = share.password {
        if params.password.is_none() {
            return Err(StatusCode::FORBIDDEN);
        }
        // Verify password...
    }

    Ok(Json(PublicShareResponse {
        file_path: share.file_path,
        permission: share.permission,
        expires_at: share.expires_at,
        expires_soon: false,
    }))
}

async fn download_public_share(
    State(state): State<AppState>,
    Path(share_token): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // Similar to get_public_share but:
    // 1. Check downloads < limit
    // 2. Increment downloads
    // 3. Log access
    // 4. Stream file

    let share: Share = sqlx::query_as(
        "SELECT * FROM shares WHERE share_token = ?"
    )
    .bind(&share_token)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check download limit
    if let Some(limit) = share.download_limit {
        if share.downloads >= limit {
            return Err(StatusCode::FORBIDDEN); // Limit reached
        }
    }

    // Increment download counter
    sqlx::query("UPDATE shares SET downloads = downloads + 1 WHERE id = ?")
        .bind(&share.id)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log access
    sqlx::query(
        "INSERT INTO share_access_logs (id, share_id, ip_address, user_agent, timestamp)
         VALUES (?, ?, ?, ?, datetime('now'))"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&share.id)
    .bind("0.0.0.0") // Extract from request
    .bind("") // Extract from headers
    .execute(&state.db_pool)
    .await
    .ok(); // Ignore logging errors

    // Stream file...
    let file = tokio::fs::File::open(&share.file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Return stream...
}
```

---

## Phase 4: Diff Implementation (1 hour)

### Problem: Version comparison needs diff algorithm

### Solution: Use `similar` crate

**File**: `backend/Cargo.toml`

Add:

```toml
similar = "2.2"
```

**File**: `backend/src/api/file_versions.rs`

Add:

```rust
use similar::{ChangeTag, TextDiff};

async fn diff_versions(
    State(state): State<AppState>,
    Path((path, _)): Path<(String, String)>,
    user: UserInfo,
    Json(req): Json<DiffRequest>,
) -> Result<Json<DiffResponse>, StatusCode> {
    // Get both versions
    let v1 = get_version_content(&state, &path, req.version1).await?;
    let v2 = get_version_content(&state, &path, req.version2).await?;

    // Generate diff
    let diff = TextDiff::from_lines(&v1, &v2);

    let mut changes = Vec::new();
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                changes.push(DiffChange {
                    type_: "removed".to_string(),
                    line: change.value().to_string(),
                });
            }
            ChangeTag::Insert => {
                changes.push(DiffChange {
                    type_: "added".to_string(),
                    line: change.value().to_string(),
                });
            }
            ChangeTag::Equal => {
                // Skip unchanged lines for brevity
            }
        }
    }

    Ok(Json(DiffResponse { changes }))
}
```

---

## Phase 5: Database Verification (30 min)

### Check if tables exist:

```bash
sqlite3 ./data/syncspace.db << EOF
SELECT name FROM sqlite_master
WHERE type='table' AND name IN (
    'file_tags', 'file_comments', 'file_versions', 'shares', 'share_access_logs'
);
EOF
```

### If tables missing, run migrations:

```bash
cd backend
sqlx migrate add -r create_file_tags
# Create migration file with:
CREATE TABLE IF NOT EXISTS file_tags (
  id TEXT PRIMARY KEY,
  file_path TEXT NOT NULL,
  name TEXT NOT NULL,
  color TEXT DEFAULT '#808080',
  created_by TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

sqlx migrate run
```

---

## Phase 6: Testing (2 hours)

### Build & Run

```bash
cd backend
cargo build --release 2>&1 | grep -i error
cargo run --release
```

### Test Endpoints

```bash
# Get token first
TOKEN=$(curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}' | jq -r '.token')

# Test tags
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:8080/api/files/test.txt/tags

# Test public share
curl http://localhost:8080/api/sharing/public/{shareToken}
```

---

## 🎯 Checklist

- [ ] Phase 1: Setup & Verify (30 min)
- [ ] Phase 2: Route Restructuring (3 hours)
  - [ ] Tags routes refactored
  - [ ] Comments routes refactored
  - [ ] Versions routes refactored
  - [ ] Routes registered in mod.rs
- [ ] Phase 3: Public Endpoints (1 hour)
  - [ ] Public router created
  - [ ] No auth middleware for /sharing/public/\*
- [ ] Phase 4: Diff Algorithm (1 hour)
  - [ ] `similar` crate added
  - [ ] diff_versions endpoint working
- [ ] Phase 5: Database (30 min)
  - [ ] All tables verified
  - [ ] Migrations run
- [ ] Phase 6: Testing (2 hours)
  - [ ] Backend compiles
  - [ ] All endpoints tested
  - [ ] Frontend stores can call APIs

**Total Time**: 8 hours

---

## 📊 Expected Result

After completion:

- ✅ Frontend `tagsComments.js` → Backend APIs working
- ✅ Frontend `fileVersioning.js` → Backend APIs working
- ✅ Frontend `advancedSharing.js` → Backend APIs working + public shares
- ✅ Frontend `serverState.js` → Already working
- ✅ Multi-device sync enabled
- ✅ Feature #15, #16, #17 ready to test end-to-end

---

## 🚨 Common Issues & Fixes

### Issue: Route not matching

```
Error: POST /api/files/path/to/file.txt/tags → 404 Not Found
```

**Fix**: Check route registration order in mod.rs - specific routes must come before catch-all

### Issue: Auth failing on public routes

```
Error: GET /api/sharing/public/{token} → 401 Unauthorized
```

**Fix**: Make sure public router is NOT wrapped in auth middleware

### Issue: Database tables missing

```
Error: no such table: file_tags
```

**Fix**: Run `sqlx migrate run` or manually create tables

### Issue: Diff not working

```
Error: diff_versions not found
```

**Fix**: Make sure `similar` crate is in Cargo.toml and imported correctly

---

## 📞 Support

**Questions?**

- Check `/BACKEND_IMPLEMENTATION_GUIDE.md` for detailed API specs
- Check `/BACKEND_AUDIT_STATUS.md` for current status
- Check `/BACKEND_API_AUDIT.md` for frontend expectations
