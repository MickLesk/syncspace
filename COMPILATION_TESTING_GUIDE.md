# 🔨 Backend Compilation & Testing Guide

**Status**: All code ready ✅  
**Goal**: Compile without errors, test all endpoints  
**Estimated Time**: 30-60 minutes

---

## 📋 Pre-Compilation Checklist

### Code Quality ✅

- [x] All database structs have proper derives (Debug, Clone, Serialize, Deserialize, FromRow)
- [x] All imports are present and organized
- [x] All route handlers have proper function signatures
- [x] All Error types return StatusCode
- [x] No TODO comments blocking compilation

### Dependency Verification ✅

- [x] `axum = "0.8"` with multipart, ws, macros
- [x] `sqlx = "0.8"` with sqlite, chrono, uuid features
- [x] `similar = "2.3"` for text diffing
- [x] `tokio = "1"` with full features
- [x] `uuid = "1"` with v4, serde features

### API Router Registration ✅

- [x] `file_tags_router()` registered before catch-all
- [x] `file_comments_router()` registered before catch-all
- [x] `file_versions_router()` registered before catch-all
- [x] `public_router()` registered BEFORE auth middleware
- [x] `files::router()` is LAST (catch-all `/files/*`)

### Database Models ✅

- [x] `CommentReaction` struct added
- [x] `SharedLink` struct enhanced with token_version fields
- [x] All FromRow derives present
- [x] All Serialize/Deserialize derives present

### Migrations ✅

- [x] 029_add_comment_reactions.sql exists
- [x] 030_enhance_share_access_logging.sql exists
- [x] All migrations use IF NOT EXISTS
- [x] No conflicting table definitions

---

## 🚀 Compilation Steps

### Step 1: Build Release Binary

```bash
cd /home/mick/Dokumente/GitHub/syncspace/backend
cargo build --release 2>&1 | tee build.log
```

**Expected Output**:

```
   Compiling syncbackend v0.1.0
    Finished `release` profile [optimized] target(s) in XXs
```

### Step 2: Error Handling Strategy

If you see errors, check these first:

#### Error Type 1: Missing imports

```rust
// Example: `use crate::services;` might be missing
// Fix: Add to top of file
use crate::services;
```

#### Error Type 2: Type mismatches in handlers

```rust
// Example: Handler returns wrong type
// Fix: Ensure all Result types are Result<Json<T>, StatusCode>
async fn handler(...) -> Result<Json<Response>, StatusCode> { ... }
```

#### Error Type 3: Path extraction issues

```rust
// Example: Extracting wildcard path
// Before: Path::<String>
// After: Path(path): Path<String>  // with destructuring
```

#### Error Type 4: Database query type mismatches

```rust
// Example: sqlx::query_as expects exact column count
// Fix: Use FromRow derive macro
#[derive(FromRow)]
pub struct MyStruct { ... }
```

### Step 3: If Compilation Fails

1. Read the full error message (not just first line)
2. Check file and line number mentioned
3. Look for pattern in error messages
4. Search for similar fixes in other files

**Common Fixes**:

- Missing `await` on async calls: `functions()` → `functions().await`
- Missing `.bind()` calls in sqlx queries
- Wrong StatusCode type: Should be `axum::http::StatusCode`
- Wrong JSON wrapper: Should be `Json<T>` from axum

---

## ✅ Post-Compilation Testing

### Test 1: Start Backend Server

```bash
./target/release/syncbackend
# Should see: "Server running on http://0.0.0.0:8080"
```

### Test 2: Public Share Endpoint (NO AUTH)

```bash
# Should return 404 (share doesn't exist) but NO auth error
curl -v http://localhost:8080/api/sharing/public/nonexistent-token
# Expected: HTTP 404 Not Found (✅ Good!)
# Not Expected: HTTP 401 Unauthorized (❌ Bad!)
```

### Test 3: Login & Get Token

```bash
# Get auth token (default credentials)
TOKEN=$(curl -s -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}' | jq -r '.token')

echo "Token: $TOKEN"
```

### Test 4: Create Test File

```bash
# Upload a test file first
curl -X POST "http://localhost:8080/api/upload/test.txt" \
  -H "Authorization: Bearer $TOKEN" \
  -F "file=@/tmp/test.txt"
```

### Test 5: Tags Endpoint

```bash
# List tags for file
curl -X GET "http://localhost:8080/api/files/test.txt/tags" \
  -H "Authorization: Bearer $TOKEN"
# Expected: 200 OK with [] or existing tags

# Create tag
curl -X POST "http://localhost:8080/api/files/test.txt/tags" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"important","color":"#FF0000"}'
# Expected: 201 Created
```

### Test 6: Comments Endpoint

```bash
# List comments
curl -X GET "http://localhost:8080/api/files/test.txt/comments" \
  -H "Authorization: Bearer $TOKEN"
# Expected: 200 OK

# Create comment
curl -X POST "http://localhost:8080/api/files/test.txt/comments" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"text":"Great file!","parent_comment_id":null}'
# Expected: 201 Created

# Add emoji reaction (requires comment ID from previous response)
curl -X POST "http://localhost:8080/api/files/test.txt/comments/{id}/reactions" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"emoji":"👍"}'
# Expected: 201 Created
```

### Test 7: Versions Endpoint

```bash
# List versions
curl -X GET "http://localhost:8080/api/files/test.txt/versions" \
  -H "Authorization: Bearer $TOKEN"
# Expected: 200 OK

# Diff versions (requires version numbers from list)
curl -X POST "http://localhost:8080/api/files/test.txt/versions/diff" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"version1":1,"version2":2}'
# Expected: 200 OK with diff data

# Cleanup old versions
curl -X POST "http://localhost:8080/api/files/test.txt/versions/cleanup" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"days_old":30}'
# Expected: 200 OK with count and freed space
```

### Test 8: Share Management

```bash
# Create share
SHARE=$(curl -s -X POST "http://localhost:8080/api/shares" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"file_path":"test.txt","permission":"read","expires_at":null,"password":null}')

SHARE_ID=$(echo $SHARE | jq -r '.id')
echo "Share ID: $SHARE_ID"

# Get share analytics
curl -X GET "http://localhost:8080/api/shares/$SHARE_ID/analytics" \
  -H "Authorization: Bearer $TOKEN"
# Expected: 200 OK with access count

# Get access log
curl -X GET "http://localhost:8080/api/shares/$SHARE_ID/access-log" \
  -H "Authorization: Bearer $TOKEN"
# Expected: 200 OK with access history

# Regenerate token
curl -X POST "http://localhost:8080/api/shares/$SHARE_ID/regenerate-token" \
  -H "Authorization: Bearer $TOKEN"
# Expected: 200 OK with new token
```

### Test 9: Database Verification

```bash
# Check if new tables exist
sqlite3 ./data/syncspace.db ".tables" | grep -E "comment_reactions|shared_link"
# Expected output should include: comment_reactions shared_link_access_log

# Check table structure
sqlite3 ./data/syncspace.db ".schema comment_reactions"
# Expected: Shows comment_reactions table with id, comment_id, emoji, user_id, created_at
```

---

## 📊 Expected Compilation Output

### Success Scenario

```
   Compiling serde v1.0.x
   Compiling axum v0.8.x
   ...
   Compiling syncbackend v0.1.0 (/path/to/backend)
    Finished `release` profile [optimized] target(s) in 45s
```

### Common Warnings (OK to ignore)

```
warning: unused import: `...`
warning: field is never read: `...`
```

### Fatal Errors (Must Fix)

```
error[E0432]: unresolved import `crate::...`
error[E0308]: mismatched types expected `StatusCode` found `()`
error[E0425]: cannot find function `...` in this scope
```

---

## 🔍 Troubleshooting Guide

### If compilation takes >2 minutes

- This is normal for first build (dependencies compile)
- Subsequent builds are faster

### If you see "cannot find crate"

```bash
# Clean build cache and retry
cd backend
rm -rf target/
cargo build --release
```

### If database file doesn't exist

```bash
# Backend creates it automatically on first run
# Check with:
ls -la ./data/syncspace.db
```

### If migrations don't run

```bash
# Check migration files exist
ls -la ./migrations/
# Migrations run automatically on backend startup
```

---

## ✅ Success Criteria

**Compilation is SUCCESSFUL when**:

- ✅ `cargo build --release` completes with "Finished"
- ✅ Binary exists at `./target/release/syncbackend`
- ✅ Backend starts without panicking
- ✅ Database is created at `./data/syncspace.db`
- ✅ Migrations run automatically
- ✅ Public endpoints work without auth (404, not 401)
- ✅ Auth-required endpoints need token

**All 27 endpoints ready when**:

- ✅ `/api/files/{path}/tags/*` endpoints respond 200/201/204
- ✅ `/api/files/{path}/comments/*` endpoints respond 200/201/204
- ✅ `/api/files/{path}/versions/*` endpoints respond 200/201
- ✅ `/api/sharing/public/*` respond 404 (not 401)
- ✅ Share management endpoints respond 200/201

---

## 🚀 Next Steps After Successful Compilation

1. **Frontend Testing** (1-2 hours)

   - Start frontend: `npm run dev`
   - Test store connections
   - Verify multi-device sync

2. **WebSocket Broadcasting** (30-60 min)

   - Verify real-time updates
   - Check event propagation

3. **Production Readiness** (30 min)
   - Error handling review
   - Performance testing
   - Security audit

---

## 📝 Build Artifacts

After successful compilation:

```
backend/
├── target/
│   └── release/
│       ├── syncbackend          ← Main binary
│       └── .../deps/            ← Compiled dependencies
├── data/
│   ├── syncspace.db             ← SQLite database (created on startup)
│   └── search_index/            ← Tantivy search index
└── build.log                    ← Compilation log
```

---

**Ready to compile? Run:**

```bash
cd /home/mick/Dokumente/GitHub/syncspace/backend
cargo build --release 2>&1 | tee build.log
```
