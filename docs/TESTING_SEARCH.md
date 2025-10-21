# 🧪 Testing the Search Feature

## Quick Start

Both backend and frontend are running:
- **Backend**: http://localhost:8080
- **Frontend**: http://localhost:5173

## Step-by-Step Testing

### 1. Access the Application

1. Open browser: http://localhost:5173
2. Login with default credentials:
   - Username: `admin`
   - Password: `admin`

### 2. Upload Test Files

**Upload a text file:**
```bash
# Create test file
echo "This is a test file with searchable content about Tantivy search engine" > test.txt

# Upload via curl
curl -X POST http://localhost:8080/api/upload/test.txt \
  -H "Authorization: Bearer YOUR_TOKEN" \
  --data-binary @test.txt
```

**Or use the UI:**
1. Go to Files page
2. Click "Upload" button
3. Select `test.txt`
4. Wait for "✅ Uploaded successfully" message
5. Backend logs should show: `📇 Indexed: test.txt`

### 3. Test Basic Search

1. **In the search bar at top**, type: `test`
2. **Wait 300ms** for debounce
3. **Observe**:
   - Search indicator appears with query
   - Results show below
   - File count displayed
   - Loading spinner briefly visible

### 4. Test Fuzzy Search

1. Type: `tset` (typo of "test")
2. **Expected**: Still finds `test.txt` (2-edit distance)
3. **Result**: Search indicator shows `tset`, results show files

### 5. Test Content Search

1. Type: `Tantivy`
2. **Expected**: Finds `test.txt` even though "Tantivy" is in content, not filename
3. **Result**: File appears in results

### 6. Test Clear Search

1. Click **"Clear search"** button in search indicator
2. **Expected**:
   - Search indicator disappears
   - Returns to normal file list
   - Search bar cleared

### 7. Test Empty Results

1. Type: `zzzznonexistent`
2. **Expected**:
   - Empty state with 🔍 icon
   - Message: "No results found"
   - Subtitle: "Try different keywords"

### 8. Upload PDF and Test

**Create test PDF** (or use existing):
```bash
# If you have a PDF, upload it:
curl -X POST http://localhost:8080/api/upload/document.pdf \
  -H "Authorization: Bearer YOUR_TOKEN" \
  --data-binary @document.pdf
```

**Then search for content inside PDF**:
1. Type text that appears in the PDF
2. **Expected**: PDF file appears in results

### 9. Test with Many Files

**Upload multiple files:**
```bash
for i in {1..20}; do
  echo "Test file number $i with content" > test$i.txt
  curl -X POST http://localhost:8080/api/upload/test$i.txt \
    -H "Authorization: Bearer YOUR_TOKEN" \
    --data-binary @test$i.txt
done
```

**Search for common term:**
1. Type: `content`
2. **Expected**: Multiple results (up to 50)
3. **Result**: All files with "content" appear

### 10. Test Navigation During Search

1. **Start a search**: Type `test`
2. **Click on a folder** (if you have subfolders)
3. **Expected**:
   - Search clears automatically
   - Navigate into folder
   - Normal file list shown

---

## UI Checklist

### Search Bar
- [ ] Appears at top of Files page
- [ ] Has 🔍 placeholder icon
- [ ] Focus styles work (blue border)
- [ ] Text is visible and readable
- [ ] Clear button (X) appears when typing
- [ ] Clear button removes text

### Search Indicator
- [ ] Appears below header when searching
- [ ] Shows query in bold
- [ ] Shows result count
- [ ] Has gradient background (purple to blue)
- [ ] "Clear search" button visible
- [ ] Animates in with slide-down effect

### Search Results
- [ ] Display as grid or list (based on view mode)
- [ ] Show filename correctly
- [ ] Show file size
- [ ] Show file icon
- [ ] Actions (download, rename, delete) work
- [ ] Click file downloads it

### Loading States
- [ ] Shows spinner while searching
- [ ] Shows "🔍 Searching..." text
- [ ] Replaces with results when done

### Empty States
- [ ] Shows 🔍 icon when no results
- [ ] Shows "No results found" message
- [ ] Shows "Try different keywords" subtitle
- [ ] Different from empty folder state (📂 icon)

---

## Performance Testing

### 1. Debouncing
- Type quickly: `t` `e` `s` `t`
- **Expected**: Only 1 API call after 300ms
- **Check**: Browser DevTools Network tab

### 2. Search Speed
- Search for common term
- **Check**: Backend logs for timing
- **Expected**: < 50ms response time

### 3. Large Result Set
- Upload 100+ files
- Search for common term
- **Expected**: Returns max 50 results (API limit)
- **Check**: Fast response, no lag

---

## Backend Verification

### Check Logs

**Upload should log:**
```
📇 Indexed: test.txt
```

**Search should show:**
```rust
// In console if you add debug logging:
Search query: "test", limit: 50, fuzzy: true
Found: 3 results in 8ms
```

**Delete should log:**
```
🗑️ Removed from index: /test.txt
```

### Check Index Files

```bash
cd backend
ls ./data/search_index/
```

**Expected files:**
- `.managed.json` - Index metadata
- Several `.idx` files - Index segments
- `.meta` file - Index configuration

---

## API Testing (Manual)

### Get Auth Token
```bash
# Login
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}'

# Response:
# {"token": "eyJ0eXAiOiJKV1QiLCJhbGc..."}
```

### Test Search Endpoint
```bash
TOKEN="your_token_here"

# Basic search
curl "http://localhost:8080/api/search?q=test&limit=10&fuzzy=true" \
  -H "Authorization: Bearer $TOKEN"

# Response:
# {
#   "results": [...],
#   "total": 3,
#   "query": "test"
# }
```

### Test Without Fuzzy
```bash
curl "http://localhost:8080/api/search?q=test&limit=10&fuzzy=false" \
  -H "Authorization: Bearer $TOKEN"
```

---

## Troubleshooting

### Search Returns Empty
1. **Check if file is indexed**:
   - Upload file in UI
   - Check backend logs for "📇 Indexed"
   - If not, check file type is supported

2. **Check index exists**:
   ```bash
   ls ./data/search_index/
   ```
   - Should have `.managed.json` and `.idx` files

3. **Try exact filename search**:
   - Instead of content, search for exact filename
   - Should always work if file exists

### Frontend Not Connecting
1. **Check backend is running**:
   ```bash
   curl http://localhost:8080/api/config
   ```
   - Should return config JSON

2. **Check CORS**:
   - Backend should allow `http://localhost:5173`
   - Check browser console for CORS errors

3. **Check authentication**:
   - Login first
   - Token stored in localStorage
   - Check Network tab for 401 errors

### Search Too Slow
1. **Reduce result limit**:
   - Change `limit=50` to `limit=20` in FilesView.svelte
   - Line ~62: `&limit=20&fuzzy=true`

2. **Disable fuzzy search**:
   - Change `fuzzy=true` to `fuzzy=false`
   - Faster but less flexible

3. **Check file count**:
   - Large index (1000+ files) may be slower
   - Consider splitting into folders

---

## Expected Results

### Successful Test Run

✅ **Upload**:
- File appears in file list
- Backend logs "📇 Indexed: filename"
- Index files updated in ./data/search_index/

✅ **Search**:
- Results appear after 300ms
- Search indicator shows query and count
- Results include filename, size, icon
- Loading spinner shows briefly

✅ **Clear**:
- "Clear search" button works
- Returns to file list
- Search bar cleared

✅ **Navigation**:
- Clicking folder clears search
- Navigates into folder
- Breadcrumb updates

✅ **Delete**:
- Delete file from UI
- Backend logs "🗑️ Removed from index"
- Search no longer finds deleted file

---

## Browser DevTools Checklist

### Network Tab
- [ ] `GET /api/search?q=...` returns 200
- [ ] Response time < 100ms
- [ ] Only 1 request per search (debounced)
- [ ] Authorization header present

### Console Tab
- [ ] No JavaScript errors
- [ ] Search logs (if added): `🔍 Found N results`
- [ ] No CORS errors
- [ ] No authentication errors

### Application Tab
- [ ] localStorage has `auth_token`
- [ ] localStorage has `filesViewMode`

---

## Success Criteria

**The search feature is working if:**
1. ✅ Can type in search bar
2. ✅ Results appear after 300ms
3. ✅ Search indicator shows query and count
4. ✅ Clear button returns to file list
5. ✅ Fuzzy search finds typos
6. ✅ Content search finds text inside files
7. ✅ Empty state shows when no results
8. ✅ Loading state shows during search
9. ✅ Upload automatically indexes file
10. ✅ Delete removes from index

---

**Happy Testing! 🎉**

If you encounter any issues, check:
- `docs/SEARCH_FEATURE.md` for feature documentation
- `docs/SEARCH_IMPLEMENTATION_COMPLETE.md` for implementation details
- Backend logs for indexing confirmation
- Browser console for frontend errors
