# 🗺️ Backend Features Implementation Roadmap

## Phase 1: Core Advanced Features (Week 1)

### 1. File Compression & Extraction ⏱️ 8h
**Priority**: HIGH
**Dependencies**: `zip`, `tar`, `flate2` crates

#### Implementation Plan:
```rust
// POST /api/compress
// Request: { paths: ["file1.txt", "file2.txt"], archive_name: "backup.zip", format: "zip" }
// Response: { archive_path: "backup.zip", size: 12345, files_count: 2 }

async fn compress_files(req: CompressRequest) -> Result<CompressResponse> {
    // 1. Validate all paths exist
    // 2. Create temp ZIP file
    // 3. Add files to archive with relative paths
    // 4. Move to DATA_DIR
    // 5. Broadcast file change event
    // 6. Log activity
}

// POST /api/extract/{archive_path}
// Request: { destination: "extracted/" }
// Response: { files_extracted: 42, destination: "extracted/" }

async fn extract_archive(archive_path: String, req: ExtractRequest) -> Result<ExtractResponse> {
    // 1. Validate archive exists and is valid
    // 2. Create destination folder
    // 3. Extract all files
    // 4. Preserve directory structure
    // 5. Broadcast events for each file
    // 6. Log activity
}
```

**Testing**:
- [ ] Create ZIP with single file
- [ ] Create ZIP with multiple files
- [ ] Create ZIP with directory structure
- [ ] Extract ZIP to existing folder
- [ ] Extract ZIP to new folder
- [ ] Handle corrupted archives
- [ ] Large file support (1GB+)

---

### 2. Share Links with Expiry ⏱️ 6h
**Priority**: HIGH
**Dependencies**: None (using existing Warp + Serde)

#### Implementation Plan:
```rust
// POST /api/share
// Request: { path: "document.pdf", duration_hours: 24, password: "secret", max_downloads: 10 }
// Response: { link_id: "abc123", url: "/share/abc123", expires_at: "2025-10-21T..." }

async fn create_share_link(
    req: CreateShareLinkRequest,
    db: Arc<Mutex<HashMap<String, ShareLink>>>,
) -> Result<ShareLinkResponse> {
    // 1. Validate path exists
    // 2. Create ShareLink with expiry
    // 3. Optional password hashing
    // 4. Save to share_links.json
    // 5. Return link URL
}

// GET /share/{link_id}?password=secret
// Response: File download or error

async fn download_share_link(
    link_id: String,
    password: Option<String>,
    db: Arc<Mutex<HashMap<String, ShareLink>>>,
) -> Result<Response> {
    // 1. Load link from DB
    // 2. Validate not expired
    // 3. Validate not exhausted (max_downloads)
    // 4. Validate password if set
    // 5. Increment download_count
    // 6. Serve file
    // 7. Log download
}

// GET /api/shares (list all active share links)
// DELETE /api/share/{link_id} (revoke share link)
```

**Testing**:
- [ ] Create share link with default expiry (24h)
- [ ] Create share link with custom expiry
- [ ] Create password-protected link
- [ ] Download with valid link
- [ ] Download with expired link (should fail)
- [ ] Download with wrong password (should fail)
- [ ] Max downloads limit enforcement
- [ ] List all active links
- [ ] Revoke share link

---

### 3. Trash/Recycle Bin ⏱️ 6h
**Priority**: HIGH
**Dependencies**: None

#### Implementation Plan:
```rust
// POST /api/trash/{path}
// Move file to .trash/ folder instead of permanent delete

async fn move_to_trash(path: String, user: String) -> Result<TrashItem> {
    // 1. Get file metadata (size, is_dir)
    // 2. Generate unique trash filename (timestamp + original)
    // 3. Move file to ./data/.trash/
    // 4. Create TrashItem record
    // 5. Save to trash_items.json
    // 6. Log activity
}

// GET /api/trash (list trash contents)
async fn list_trash() -> Result<Vec<TrashItem>> {
    // Load from trash_items.json
}

// POST /api/restore/{trash_id}
// Restore file from trash to original location

async fn restore_from_trash(trash_id: String) -> Result<()> {
    // 1. Find trash item by ID
    // 2. Check if original path available
    // 3. Move file back to original location
    // 4. Remove from trash_items.json
    // 5. Broadcast file change event
    // 6. Log activity
}

// DELETE /api/trash/empty
// Permanently delete all trash items

async fn empty_trash() -> Result<u32> {
    // 1. Load all trash items
    // 2. Delete all files in .trash/
    // 3. Clear trash_items.json
    // 4. Log activity
    // 5. Return count of deleted items
}

// Background task: Auto-cleanup trash older than 30 days
tokio::spawn(async move {
    loop {
        tokio::time::sleep(Duration::hours(24)).await;
        cleanup_old_trash_items(30).await;
    }
});
```

**Testing**:
- [ ] Move single file to trash
- [ ] Move directory to trash (recursive)
- [ ] List trash contents
- [ ] Restore file to original path
- [ ] Restore when original path occupied
- [ ] Empty entire trash
- [ ] Auto-cleanup old items (simulate 30+ days)

---

## Phase 2: Performance & Search (Week 2)

### 4. Thumbnail Generation ⏱️ 10h
**Priority**: MEDIUM
**Dependencies**: `image`, `tempfile` crates

#### Implementation Plan:
```rust
// GET /api/thumbnail/{path}?size=150
// Generate or serve cached thumbnail

async fn get_thumbnail(
    path: String,
    size: u32, // 150, 300, 600
) -> Result<Response> {
    // 1. Check cache: .thumbnails/{size}/{hash}.jpg
    // 2. If cached, serve immediately
    // 3. If not cached:
    //    a. Load original image
    //    b. Resize to requested size
    //    c. Save to cache
    //    d. Serve thumbnail
    // 4. For videos: extract frame at 10% duration
}

// Background task: Pre-generate thumbnails on upload
async fn pregenerate_thumbnails(file_path: String) {
    if can_generate_thumbnail(&file_path) {
        for size in [150, 300] {
            generate_thumbnail(&file_path, size).await;
        }
    }
}
```

**Supported Formats**:
- Images: JPG, PNG, GIF, WebP
- Videos: MP4, WebM (first frame extraction)

**Cache Strategy**:
- Store in `./data/.thumbnails/{size}/`
- Filename: SHA256(path) + ".jpg"
- Max cache size: 1GB (LRU eviction)

**Testing**:
- [ ] Generate 150x150 thumbnail
- [ ] Generate 300x300 thumbnail
- [ ] Serve cached thumbnail (faster)
- [ ] Regenerate if original modified
- [ ] Video frame extraction
- [ ] Cache eviction when full

---

### 5. Full-Text Search & Indexing ⏱️ 12h
**Priority**: MEDIUM
**Dependencies**: `tantivy` or custom indexer

#### Implementation Plan:
```rust
// POST /api/search/index
// Rebuild full search index (admin only)

async fn rebuild_search_index() -> Result<SearchIndexStats> {
    // 1. Scan all files in DATA_DIR
    // 2. For text-searchable files:
    //    a. Read content
    //    b. Tokenize and index
    //    c. Store in search index
    // 3. Return stats (files indexed, time taken)
}

// GET /api/search?q=query&content=true
// Search filenames AND file contents

async fn search_with_content(query: String, content_search: bool) -> Result<Vec<SearchIndexResult>> {
    if content_search {
        // 1. Search in indexed content
        // 2. Return matches with context (3 lines before/after)
        // 3. Highlight matched terms
    } else {
        // Existing filename-only search
    }
}
```

**Indexing Strategy**:
- Index files: TXT, MD, JSON, Code files
- Update index on file modification (WebSocket event)
- Incremental indexing (only changed files)
- Store index in `./search_index/`

**Testing**:
- [ ] Index all text files
- [ ] Search in file contents
- [ ] Match highlighting
- [ ] Context preview
- [ ] Incremental update on file change
- [ ] Performance with 10,000+ files

---

### 6. File Versioning ⏱️ 12h
**Priority**: LOW
**Dependencies**: None

#### Implementation Plan:
```rust
// Automatic versioning on file upload

async fn upload_with_versioning(path: String, content: Vec<u8>) -> Result<FileVersion> {
    // 1. Calculate SHA256 checksum
    // 2. Check if file changed (compare checksum)
    // 3. If changed:
    //    a. Copy current file to .versions/{path}/v{n}
    //    b. Increment version number
    //    c. Create FileVersion record
    //    d. Save new content
    // 4. Return version info
}

// GET /api/versions/{path}
// List all versions of a file

async fn list_file_versions(path: String) -> Result<Vec<FileVersion>> {
    // Load from .versions/{path}/versions.json
}

// GET /api/version/{path}/{version_id}
// Download specific version

async fn download_version(path: String, version_id: String) -> Result<Response> {
    // Serve file from .versions/{path}/v{n}
}

// POST /api/restore-version/{path}/{version_id}
// Restore old version as current

async fn restore_version(path: String, version_id: String) -> Result<FileVersion> {
    // 1. Load version file
    // 2. Create new version of current file
    // 3. Replace current with old version
    // 4. Update version records
}
```

**Storage**:
- `.versions/{file_path}/`
  - `v1` - First version
  - `v2` - Second version
  - `versions.json` - Metadata
- Max versions per file: 10 (configurable)
- Auto-cleanup: Delete old versions when limit reached

**Testing**:
- [ ] Upload file (creates v1)
- [ ] Upload again (creates v2)
- [ ] List versions
- [ ] Download specific version
- [ ] Restore old version
- [ ] Version limit enforcement
- [ ] Large file versioning (1GB+)

---

## Phase 3: Advanced Features (Week 3)

### 7. Activity Log & Audit Trail ⏱️ 4h
**Priority**: MEDIUM
**Dependencies**: None

#### Implementation Plan:
Already implemented in `features.rs`:
- `ActivityLogEntry` struct
- `append_activity_log()` function
- `read_activity_log()` function

**Integration**:
Add logging to all file operations:
```rust
// In upload handler:
append_activity_log(&ActivityLogEntry::new(
    user.username.clone(),
    "upload".to_string(),
    path.clone(),
)).await;

// In delete handler:
append_activity_log(&ActivityLogEntry::new(
    user.username.clone(),
    "delete".to_string(),
    path.clone(),
)).await;

// Similar for: rename, download, share, trash, restore
```

**API Endpoints**:
```rust
// GET /api/activity?limit=100&user=admin&action=delete
async fn get_activity_log(
    limit: usize,
    user_filter: Option<String>,
    action_filter: Option<String>,
) -> Result<Vec<ActivityLogEntry>>
```

**Testing**:
- [ ] Log upload action
- [ ] Log delete action
- [ ] Log rename action
- [ ] Filter by user
- [ ] Filter by action type
- [ ] Limit results
- [ ] Pagination

---

### 8. Duplicate File Detection ⏱️ 8h
**Priority**: LOW
**Dependencies**: SHA256 from `sha2` crate

#### Implementation Plan:
```rust
// POST /api/scan-duplicates
// Background job to find duplicate files

async fn scan_duplicates() -> Result<Vec<DuplicateGroup>> {
    // 1. Scan all files in DATA_DIR
    // 2. Calculate SHA256 for each file
    // 3. Group by checksum
    // 4. Return groups with 2+ files
}

// GET /api/duplicates
// Get cached duplicate scan results

// DELETE /api/duplicates/resolve
// Request: { keep: "path/to/keep.txt", delete: ["path/to/dup1.txt", "path/to/dup2.txt"] }
// Delete duplicate files, keep one
```

**Optimization**:
- Skip files < 1KB (too small to matter)
- Compare file sizes first (fast pre-filter)
- Progressive hashing (first 8KB, then full file)
- Cache checksums in `./checksums.json`
- Background scanning (don't block API)

**Testing**:
- [ ] Detect exact duplicates
- [ ] Group by checksum
- [ ] Delete duplicates (keep one)
- [ ] Performance with 10,000+ files
- [ ] Large file support (1GB+)
- [ ] Incremental scanning

---

### 9. Bandwidth Control & Rate Limiting ⏱️ 6h
**Priority**: LOW
**Dependencies**: Token bucket algorithm

#### Implementation Plan:
```rust
struct BandwidthLimiter {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl BandwidthLimiter {
    fn try_consume(&mut self, bytes: usize) -> bool {
        self.refill();
        let tokens_needed = bytes as f64 / 1024.0; // KB
        if self.tokens >= tokens_needed {
            self.tokens -= tokens_needed;
            true
        } else {
            false
        }
    }
}

// Apply to upload/download routes
async fn upload_with_rate_limit(
    content: Vec<u8>,
    limiter: Arc<Mutex<BandwidthLimiter>>,
) -> Result<()> {
    let mut limiter = limiter.lock().unwrap();
    if !limiter.try_consume(content.len()) {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    // Proceed with upload
}
```

**Configuration**:
```json
{
  "bandwidth_limits": {
    "upload_mbps": 10,
    "download_mbps": 50,
    "per_user": true
  }
}
```

**Testing**:
- [ ] Upload with limit (10 MB/s)
- [ ] Download with limit (50 MB/s)
- [ ] Rate limit enforcement
- [ ] Per-user limits
- [ ] Burst allowance
- [ ] Refill rate

---

### 10. WebDAV Support ⏱️ 16h
**Priority**: LOW
**Dependencies**: `warp` WebDAV middleware or custom implementation

#### Implementation Plan:
```rust
// WebDAV endpoints at /webdav/*

// PROPFIND - List directory
// GET - Download file
// PUT - Upload file
// DELETE - Delete file
// MKCOL - Create directory
// COPY - Copy file
// MOVE - Move/rename file
// LOCK - Lock file
// UNLOCK - Unlock file

// Example PROPFIND handler:
async fn webdav_propfind(path: String, depth: u8) -> Result<Response> {
    // 1. List directory entries
    // 2. Build XML response with file properties
    // 3. Return with proper WebDAV headers
}
```

**Benefits**:
- Mount as network drive on Windows/Mac/Linux
- Standard protocol support
- Native OS integration
- No custom client needed

**Testing**:
- [ ] Mount on Windows (net use)
- [ ] Mount on macOS (Finder)
- [ ] Mount on Linux (davfs2)
- [ ] Create files via WebDAV
- [ ] Read files via WebDAV
- [ ] Delete files via WebDAV
- [ ] Folder operations

---

## Phase 4: Polish & Optimization (Week 4)

### Additional Enhancements

#### 11. Smart File Locking
- Prevent concurrent edits
- Lock timeout (30 minutes)
- Force unlock (admin)
- Lock status indicator

#### 12. Advanced Metadata
- EXIF data for images
- ID3 tags for audio
- Video metadata
- Custom file tags

#### 13. Real-time Collaboration
- WebSocket-based sync
- Operational Transformation
- Presence indicators
- Collaborative text editing

---

## Testing Strategy

### Unit Tests
```bash
cargo test
```

### Integration Tests
```rust
#[tokio::test]
async fn test_compress_and_extract() {
    // Create test files
    // Compress to ZIP
    // Extract to new location
    // Verify all files match
}
```

### Load Tests
```bash
# Apache Bench
ab -n 1000 -c 10 http://localhost:8080/api/files/

# Custom load test
for i in {1..1000}; do
    curl -X POST http://localhost:8080/api/upload/test_$i.txt \
         -H "Authorization: Bearer $TOKEN" \
         -d "test content"
done
```

---

## Deployment Checklist

### Pre-production
- [ ] All tests passing
- [ ] Documentation complete
- [ ] Security audit
- [ ] Performance benchmarks
- [ ] Backup strategy
- [ ] Monitoring setup

### Production
- [ ] HTTPS enabled
- [ ] Rate limiting configured
- [ ] Logging enabled
- [ ] Metrics collection
- [ ] Error tracking
- [ ] Auto-restart on crash

---

## Success Metrics

### Performance Targets
- Upload: 100 MB/s
- Download: 200 MB/s
- Search: < 100ms for 10,000 files
- Thumbnail generation: < 200ms
- API response: < 50ms (95th percentile)

### Capacity Targets
- Files: 1,000,000+
- Total size: 10 TB+
- Concurrent users: 100+
- WebSocket connections: 1000+

---

## Timeline Summary

| Phase | Features | Duration | Priority |
|-------|----------|----------|----------|
| 1 | Compression, Share Links, Trash | 1 week | HIGH |
| 2 | Thumbnails, Search, Versioning | 1 week | MEDIUM |
| 3 | Activity Log, Duplicates, Bandwidth | 1 week | LOW |
| 4 | WebDAV, Advanced Features | 1 week | LOW |

**Total**: 4 weeks for complete implementation

---

**Last Updated**: 2025-10-20
**Version**: 0.2.0
