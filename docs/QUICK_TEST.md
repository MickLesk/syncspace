# 🧪 Quick Test Guide

## Start Everything

### 1. Backend starten
```powershell
cd C:\Users\LeskowitzMickey\Documents\GitHub\syncspace\backend
cargo run --release
```

**Expected output**:
```
🗄️  Database initialized
👤 Admin user ensured: admin
🔍 Search index initialized
🚀 Server running on http://localhost:8080
```

### 2. Frontend starten
```powershell
cd C:\Users\LeskowitzMickey\Documents\GitHub\syncspace\frontend
npm run dev
```

**Expected output**:
```
ROLLDOWN-VITE v7.1.14 ready in 772 ms
Local:   http://localhost:5173/
```

---

## Quick Test (5 Minuten)

### 1. Login
1. Browser: `http://localhost:5173`
2. Username: `admin`
3. Password: `admin`
4. ✅ Should redirect to Files page

### 2. Upload File
1. Click **"Upload"** button
2. Select any file (z.B. `test.txt`)
3. ✅ File appears in list
4. ✅ Backend logs: `📇 Indexed: test.txt`

### 3. Search File
1. Type filename in search bar
2. Wait 300ms
3. ✅ Search indicator appears
4. ✅ File found in results
5. ✅ Result count displayed

### 4. Download File
1. Click download icon on file
2. ✅ File downloads
3. ✅ Toast: "Downloading..."

### 5. Create Folder
1. Click **"New Folder"**
2. Enter name: `TestFolder`
3. ✅ Folder created
4. ✅ Toast: "Folder created"

### 6. Delete File
1. Click trash icon
2. Confirm deletion
3. ✅ File removed
4. ✅ Backend logs: `🗑️ Removed from index`

---

## Debug Checklist

### Backend nicht erreichbar?
```powershell
# Check if running
Test-NetConnection -ComputerName localhost -Port 8080

# Expected: TcpTestSucceeded : True
```

### Frontend Fehler?
```
F12 → Console Tab
# Check for errors
# Check Network tab for failed requests
```

### Auth funktioniert nicht?
```javascript
// Console:
localStorage.getItem("auth")
// Should show: {"token": "eyJ..."}
```

### Upload funktioniert nicht?
```
Backend Console:
# Should show: 📇 Indexed: filename.txt

Browser Console:
# Should NOT show CORS errors
# Should NOT show 401 Unauthorized
```

---

## Common Issues

### "401 Unauthorized"
**Problem**: Token fehlt oder ungültig  
**Fix**: Neu einloggen, localStorage.clear()

### "CORS Error"
**Problem**: Backend CORS nicht konfiguriert  
**Fix**: Backend sollte automatisch CORS erlauben

### "Failed to load files"
**Problem**: Backend nicht erreichbar  
**Fix**: Backend neu starten

### Search findet nichts
**Problem**: Datei nicht indexiert  
**Fix**: Datei neu hochladen, Backend logs checken

---

## Success Criteria

✅ Login funktioniert  
✅ Files werden gelistet  
✅ Upload läuft durch  
✅ Search findet Dateien  
✅ Download funktioniert  
✅ Delete entfernt Dateien  
✅ Folder creation klappt  

**Wenn alles ✅ → Integration erfolgreich!** 🎉

---

## Next Steps

- [ ] PDF hochladen und Search testen
- [ ] 100+ Dateien uploaden (Performance)
- [ ] 2FA aktivieren
- [ ] Mobile View testen
- [ ] WebSocket Live-Updates testen
