# SyncSpace - Quick Reference

## 🚀 Backend starten

```powershell
cd C:\Users\LeskowitzMickey\Documents\GitHub\syncspace\backend
cargo run --release
```

**Oder in separatem Fenster:**

```powershell
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd 'C:\Users\LeskowitzMickey\Documents\GitHub\syncspace\backend'; cargo run --release"
```

---

## 🗄️ Datenbank

**Pfad:** `backend/data/syncspace.db`

**Tabellen:**

- `users` - Benutzer (admin/admin immer vorhanden)
- `files` - Dateien mit Checksums & Versioning
- `folders` - Hierarchische Ordner
- `file_history` - Alle Änderungen geloggt
- `trash` - Papierkorb (30 Tage Auto-Cleanup)
- `favorites` - User-spezifische Favoriten
- `shared_links` - Public Links mit Expiry
- `settings` - Globale Konfiguration

**Inspektion:**

```powershell
sqlite3 backend/data/syncspace.db
.tables
.schema users
SELECT * FROM settings;
.quit
```

---

## 🔐 Admin User

**Credentials:**

- Username: `admin`
- Password: `admin`
- Email: `admin@syncspace.local`
- Quota: 10GB

**Login testen:**

```powershell
$body = '{"username":"admin","password":"admin"}'
Invoke-RestMethod -Uri "http://localhost:8080/api/auth/login" -Method POST -ContentType "application/json" -Body $body
```

**Wichtig:** Admin wird automatisch beim Start erstellt und **nie überschrieben**!

---

## 📊 Performance-Optimierungen

**SQLite PRAGMAs (automatisch aktiv):**

```sql
PRAGMA journal_mode=WAL;       -- Write-Ahead Logging
PRAGMA synchronous=NORMAL;     -- Balanced Speed/Safety
PRAGMA temp_store=MEMORY;      -- Temp-Daten im RAM
PRAGMA cache_size=-64000;      -- 64MB Cache
PRAGMA foreign_keys=ON;        -- Referenzielle Integrität
```

**Erwartete Performance:**

- 10k Dateien: <5ms Queries
- 100k Dateien: 10-50ms Queries
- 1M Dateien: 50-150ms Queries (mit FTS5)

---

## 🔍 Volltext-Suche (Roadmap)

### Phase 1: SQLite FTS5 (Empfohlen)

- **Setup:** Keine zusätzliche Software
- **Performance:** 20-50ms für 100k Dokumente
- **Features:** BM25 Ranking, Porter Stemming, Phrase Search
- **Aufwand:** ~6 Stunden

### Phase 2: Tantivy (Optional)

- **Setup:** `tantivy = "0.22"` in Cargo.toml
- **Performance:** <5ms für 100k Dokumente
- **Features:** Fuzzy Search, besseres Ranking, Faceting
- **Aufwand:** ~2-3 Tage

### Phase 3: Meilisearch (Enterprise)

- **Setup:** Docker Container
- **Performance:** <1ms für 100k Dokumente
- **Features:** Beste Typo-Toleranz, Multi-Language
- **Aufwand:** ~1 Woche

**Empfehlung:** Start mit FTS5, später Upgrade zu Tantivy bei Bedarf.

---

## 📁 Projekt-Struktur

```
syncspace/
├── backend/
│   ├── src/
│   │   ├── main.rs           # HTTP Server + Routes
│   │   ├── auth.rs           # JWT Authentication (TODO: auf DB umstellen)
│   │   └── database.rs       # SQLite Pool + Models (367 Zeilen)
│   ├── migrations/
│   │   └── 001_initial_schema.sql  # DB Schema (269 Zeilen)
│   ├── data/
│   │   └── syncspace.db      # SQLite Datenbank (wird erstellt)
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── pages/
│   │   │   └── FilesView.svelte  # Hauptansicht
│   │   └── App.svelte
│   ├── index.html
│   └── package.json
├── docs/
│   ├── DATABASE.md            # DB Schema Dokumentation
│   ├── PERFORMANCE_SEARCH.md  # Performance-Analyse
│   └── ROADMAP_SEARCH.md      # Implementation-Roadmap
└── README.md
```

---

## 🔧 Häufige Commands

### Backend kompilieren:

```powershell
cd backend
cargo build --release          # Produktions-Build (optimiert)
cargo build                    # Debug-Build (schneller)
```

### Datenbank neu erstellen:

```powershell
Remove-Item backend/data/syncspace.db
cd backend
cargo run --release
```

### Backend-Prozess stoppen:

```powershell
Get-Process | Where-Object {$_.ProcessName -like "*syncbackend*"} | Stop-Process -Force
```

### Port prüfen:

```powershell
Get-NetTCPConnection -LocalPort 8080 -ErrorAction SilentlyContinue
```

---

## 🐛 Debugging

### Backend-Logs sehen:

Backend-Fenster beobachten oder:

```powershell
cd backend
cargo run --release 2>&1 | Tee-Object -FilePath logs.txt
```

### Datenbank-Queries debuggen:

```powershell
sqlite3 backend/data/syncspace.db
.headers on
.mode column
SELECT * FROM files LIMIT 10;
```

### API testen:

```powershell
# Login
$token = (Invoke-RestMethod -Uri "http://localhost:8080/api/auth/login" -Method POST -ContentType "application/json" -Body '{"username":"admin","password":"admin"}').token

# Files abrufen
Invoke-RestMethod -Uri "http://localhost:8080/api/files/" -Headers @{Authorization="Bearer $token"}

# Upload
$file = Get-Content "test.txt" -Raw -Encoding Byte
Invoke-RestMethod -Uri "http://localhost:8080/api/upload/test.txt" -Method POST -Headers @{Authorization="Bearer $token"} -Body $file
```

---

## 📚 Dokumentation

- **Datenbank Schema:** `docs/DATABASE.md`
- **Performance & Suche:** `docs/PERFORMANCE_SEARCH.md`
- **Volltext-Suche Roadmap:** `docs/ROADMAP_SEARCH.md`
- **Copilot Instructions:** `.github/copilot-instructions.md`

---

## ✅ Was funktioniert bereits

- ✅ SQLite Datenbank mit 8 Tabellen
- ✅ Admin User (admin/admin) automatisch erstellt
- ✅ JWT Authentication
- ✅ File Upload/Download/Delete
- ✅ Folder Hierarchie
- ✅ Soft Delete (Revision Safety)
- ✅ 10 Default Settings
- ✅ Performance-Optimierungen (WAL, Cache)

## 🔄 In Arbeit

- 🔄 Auth auf Database umstellen (statt users.json)
- 🔄 FTS5 Volltext-Suche
- 🔄 File History Endpoint
- 🔄 Trash/Restore Endpoints
- 🔄 Favorites Endpoints
- 🔄 Shared Links Endpoints

## ⏳ Geplant

- ⏳ Content-Extraktion (PDF, DOCX)
- ⏳ Deduplication (SHA256 Checksums)
- ⏳ File Versioning
- ⏳ Tantivy Search (optional)
- ⏳ Auto-Trash Cleanup Scheduler
- ⏳ Frontend für History/Trash/Favorites

---

## 🎯 Nächste Schritte

1. **Diese Woche:**

   - FTS5 Migration (`002_fulltext_search.sql`)
   - Search Endpoint (`/api/search/fulltext`)
   - Frontend Search-Box

2. **Nächste Woche:**

   - Auth auf Database migrieren
   - File History Endpoint
   - Trash/Restore UI

3. **Diesen Monat:**
   - Content-Extraction für .txt/.md
   - PDF Support evaluieren
   - Performance-Testing mit 10k Files
