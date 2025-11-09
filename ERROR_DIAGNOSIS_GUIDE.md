# 🔧 Common Backend Compilation & Runtime Errors

**Quick Reference for Fixing Issues During Compilation**

---

## 📋 Error Categories & Solutions

### 1️⃣ Missing Imports Errors

**Symptom**: `error[E0432]: unresolved import`

```rust
error[E0432]: unresolved import `crate::services::sharing`
```

**Solution**:

1. Check if module exists: `backend/src/services/all_services_impl.rs`
2. Add to `backend/src/services/mod.rs`:

```rust
pub mod sharing {
    pub use super::all_services_impl::sharing::*;
}
```

---

### 2️⃣ Type Mismatch Errors

**Symptom**: `error[E0308]: mismatched types`

```rust
error[E0308]: mismatched types
expected enum `Result<Json<...>, StatusCode>`
   found enum `Result<Json<...>, Error>`
```

**Solution**: Ensure all handler returns are:

```rust
async fn handler(...) -> Result<Json<T>, StatusCode> {
    // All errors must map to StatusCode
    service_call()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
```

---

### 3️⃣ Missing Trait Bounds

**Symptom**: `error[E0599]: no method named`

```rust
error[E0599]: no method named `bind` found for struct `...`
```

**Solution**: Ensure FromRow derive on database structs:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MyStruct {
    pub field: String,
}
```

---

### 4️⃣ Async/Await Errors

**Symptom**: `error[E0308]: Future not awaited`

```rust
error[E0308]: Expected `...` found Future
```

**Solution**: Add `.await` to async function calls:

```rust
// Before ❌
let result = database_query();

// After ✅
let result = database_query().await;
```

---

### 5️⃣ Path Extraction Errors

**Symptom**: `error: failed to match route`

```
failed to match route for GET /files/test.txt/tags
```

**Solution**: Use wildcard path correctly:

```rust
// In router definition:
.route("/files/*path/tags", get(list_file_tags))

// In handler:
async fn list_file_tags(
    Path(path): Path<String>,  // Extracts the wildcard as String
    // ...
) { }
```

---

### 6️⃣ Status Code Errors

**Symptom**: `error[E0308]: mismatched types - found struct StatusCode`

```
expected one of `200`, `201`, `204`
   found StatusCode
```

**Solution**: Use full path for StatusCode:

```rust
use axum::http::StatusCode;

// Then use:
StatusCode::OK              // 200
StatusCode::CREATED         // 201
StatusCode::NO_CONTENT      // 204
StatusCode::NOT_FOUND       // 404
StatusCode::FORBIDDEN       // 403
StatusCode::GONE            // 410
StatusCode::INTERNAL_SERVER_ERROR  // 500
StatusCode::NOT_IMPLEMENTED // 501
```

---

### 7️⃣ Database Query Errors

**Symptom**: `error: the row returned from query doesn't match`

```
expected X columns, query returned Y columns
```

**Solution**: Check SQL query vs struct fields:

```rust
// Table has: id, name, color, owner_id, created_at
// Query: SELECT * FROM tags

// Struct must match:
#[derive(FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub owner_id: String,
    pub created_at: String,
}
```

---

### 8️⃣ Router Registration Errors

**Symptom**: `error: couldn't compile`

```
routes appear to conflict
```

**Solution**: Ensure specific routes BEFORE generic:

```rust
// CORRECT ORDER ✅
.merge(file_versions::file_versions_router())  // /files/{path}/versions/*
.merge(tags::file_tags_router())               // /files/{path}/tags/*
.merge(files::router())                        // /files/* (catch-all LAST)

// WRONG ORDER ❌
.merge(files::router())                        // Catches everything first!
.merge(tags::file_tags_router())               // Never reached
```

---

### 9️⃣ Serialization Errors

**Symptom**: `error[E0599]: the trait bound ... is not satisfied`

```
T: Serialize is not satisfied
```

**Solution**: Add derive macro:

```rust
// Before ❌
pub struct Response {
    pub data: String,
}

// After ✅
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub data: String,
}
```

---

### 🔟 Feature Flag Errors

**Symptom**: `error: feature "xxx" not found`

```
feature "ws" not found
```

**Solution**: Check Cargo.toml features:

```toml
# In Cargo.toml
[dependencies]
axum = { version = "0.8", features = ["multipart", "ws", "macros"] }
                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

---

## 🚀 Runtime Errors

### Error: "Database connection failed"

**Cause**: SQLite database not created  
**Fix**: Backend creates on startup automatically

```bash
# Check if file exists
ls -la ./data/syncspace.db
# If missing, start backend (it will create it)
./target/release/syncbackend
```

### Error: "Migration failed"

**Cause**: Table already exists or migration error  
**Fix**: Migrations use `IF NOT EXISTS`, should be safe to re-run

```bash
# Delete database to reset (DEV ONLY!)
rm ./data/syncspace.db
# Restart backend
./target/release/syncbackend
```

### Error: "Public endpoint returns 401"

**Cause**: Auth middleware applied to public route  
**Fix**: Ensure public_router() registered BEFORE auth middleware:

```rust
// In mod.rs BEFORE the auth middleware Router:
.merge(sharing::public_router())  // NO AUTH
// Then inside the protected Router:
.merge(sharing::router())         // WITH AUTH
```

### Error: "Port already in use"

**Cause**: Backend already running on port 8080  
**Fix**: Kill existing process or use different port

```bash
# Find process using port 8080
lsof -i :8080
# Kill it
kill -9 <PID>
```

---

## ✅ Quick Fixes Checklist

If compilation fails, try these in order:

- [ ] Clean build: `cargo clean && cargo build --release`
- [ ] Check imports: All `use` statements correct?
- [ ] Check types: All `Result<Json<T>, StatusCode>`?
- [ ] Check async: All `.await` calls present?
- [ ] Check derives: All `#[derive(..., FromRow)]` present?
- [ ] Check routes: Specific routes before catch-all?
- [ ] Check status codes: Using `axum::http::StatusCode`?
- [ ] Check SQL: Query columns match struct fields?

---

## 🔍 Debugging Commands

```bash
# Show full compilation output
cargo build --release 2>&1 | head -100

# Show specific error details
cargo build --release 2>&1 | grep -A 5 "error\[E"

# Verbose build
RUST_LOG=debug cargo build --release

# Check specific file for errors
cargo check -p syncbackend 2>&1 | grep "backend/src/api/sharing.rs"

# Run backend with detailed error output
RUST_BACKTRACE=1 ./target/release/syncbackend
```

---

## 📚 Reference Links

**Error Codes**:

- E0308: Type mismatch
- E0432: Missing import
- E0599: Missing method
- E0425: Missing symbol

**Useful Docs**:

- Axum routing: https://docs.rs/axum/
- SQLx database: https://docs.rs/sqlx/
- Tokio async: https://tokio.rs/

---

## 💡 Pro Tips

1. **Read the FULL error message**, not just the first line
2. **Look at the line number** - might be an issue on a previous line
3. **Check imports** - most errors are missing imports
4. **Use cargo check** - faster than full build for checking
5. **Build incrementally** - comment out suspicious code first

---

**Stuck? Try:**

```bash
cd backend
cargo clean
cargo build --release 2>&1 | tee build_errors.log
# Then read build_errors.log carefully
```
