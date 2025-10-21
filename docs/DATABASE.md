# SyncSpace Database Schema

## Übersicht

SyncSpace verwendet SQLite mit 8 Haupttabellen für eine vollständige Dateiverwaltung mit Historie, Papierkorb, Favoriten und Shared Links.

## Tabellen

### 1. **users** - Benutzerverwaltung

```sql
- id: UUID (Primary Key)
- username: Unique username
- password_hash: Argon2 hash
- email: Optional email
- display_name: Display name
- avatar_base64: Base64-encoded avatar image
- totp_secret/totp_enabled: 2FA settings
- storage_quota_bytes/storage_used_bytes: Quota management
- default_view/language/theme: User preferences
```

**Features:**

- Base64-encoded avatar storage (kein separates File-System)
- Per-User Quota tracking
- Individualisierbare UI-Präferenzen

### 2. **folders** - Ordnerstruktur

```sql
- id: UUID
- name: Folder name
- path: Full path (e.g., "/Documents/Projects")
- parent_id: Reference to parent folder (hierarchisch)
- owner_id: User owner
- color/icon: Optional UI customization
- is_shared/is_favorite: Quick flags
- is_deleted: Soft delete flag
```

**Features:**

- Hierarchische Struktur mit parent_id
- Soft delete für Papierkorb-Funktion
- Farben und Icons für Personalisierung

### 3. **files** - Dateien

```sql
- id: UUID
- name: Filename
- path: Full path
- folder_id: Parent folder (nullable für root)
- owner_id: User owner
- size_bytes: File size
- mime_type: Content type
- checksum_sha256: SHA256 hash für Deduplication
- storage_path: Actual path on disk
- is_encrypted/is_shared/is_favorite: Flags
- is_deleted: Soft delete
- version: Version number
- previous_version_id: Link to previous version
```

**Features:**

- Deduplication via SHA256 checksum
- Versionierung (optional aktivierbar)
- Soft delete mit Revision Safety
- Separater storage_path für physische Speicherung

### 4. **file_history** - Vollständige Historie

```sql
- id: UUID
- file_id: Reference to file
- user_id: Who performed the action
- action: created, renamed, moved, modified, deleted, restored, shared, unshared
- old_value/new_value: JSON with change details
- ip_address/user_agent: Audit trail
```

**Features:**

- Alle Aktionen werden geloggt
- JSON-basierte old/new values für Flexibilität
- Audit trail mit IP und User-Agent

**Actions:**

- `created`: Neue Datei erstellt
- `renamed`: Name geändert
- `moved`: Verschoben (Ordner geändert)
- `modified`: Inhalt geändert
- `deleted`: Gelöscht (→ Trash)
- `restored`: Aus Trash wiederhergestellt
- `shared`: Share-Link erstellt
- `unshared`: Share-Link entfernt

### 5. **trash** - Papierkorb

```sql
- id: UUID
- item_type: 'file' or 'folder'
- item_id: Reference to file or folder
- original_path/original_name/original_parent_id: Restore information
- deleted_by/deleted_at: Who & when
- auto_delete_at: Automatic cleanup timestamp
- size_bytes: For quota calculation
```

**Features:**

- Unified trash für Files und Folders
- Restore-Informationen gespeichert
- Auto-cleanup nach X Tagen (konfigurierbar)
- Quota-Tracking auch im Trash

### 6. **favorites** - Favoriten

```sql
- id: UUID
- user_id: User
- item_type: 'file' or 'folder'
- item_id: Reference
- sort_order: User-defined ordering
```

**Features:**

- Pro-User Favoriten
- Files und Folders
- Benutzerdefinierte Sortierung

### 7. **shared_links** - Öffentliche Links

```sql
- id: UUID (= public link ID)
- item_type: 'file' or 'folder'
- item_id: Reference
- created_by: Creator
- password_hash: Optional password protection
- is_public: Public or requires login
- allow_download/allow_upload: Permissions
- expires_at: Optional expiry date
- max_downloads: Optional download limit
- download_count: Usage tracking
```

**Features:**

- UUID-basierte public links
- Optional passwortgeschützt
- Ablaufzeit konfigurierbar
- Download-Limits
- Upload in Folder-Shares möglich

### 8. **settings** - Globale Einstellungen

```sql
- key: Setting key
- value: Setting value
- value_type: 'string', 'integer', 'boolean', 'json'
- description: Human-readable description
- category: 'general', 'security', 'storage', 'features'
```

**Default Settings:**

- `revision_safety_enabled`: true - Soft delete aktiv
- `auto_trash_cleanup_days`: 30 - Trash auto-cleanup nach 30 Tagen
- `max_upload_size_mb`: 1024 - Max 1GB Upload
- `enable_file_versioning`: false - Versionierung optional
- `enable_deduplication`: true - Deduplication aktiv
- `max_file_versions`: 5 - Max 5 Versionen pro File
- `require_2fa`: false - 2FA optional
- `session_timeout_hours`: 24 - JWT Token Lifetime
- `enable_shared_links`: true - Shared Links erlaubt
- `default_user_quota_gb`: 10 - Default 10GB pro User

## Features

### 🔒 Revision Safety

Wenn aktiviert (`revision_safety_enabled`):

- Dateien werden nicht gelöscht, sondern als `is_deleted = 1` markiert
- Verschiebung in `trash` Tabelle mit Restore-Informationen
- Optional auto-cleanup nach X Tagen
- Vollständige Historie bleibt erhalten

### 📦 Deduplication

Wenn aktiviert (`enable_deduplication`):

- SHA256 checksum bei Upload berechnen
- Gleiche Dateien teilen sich `storage_path`
- Quota nur einmal berechnen
- Bei letztem Delete physisch löschen

### 🔄 Versionierung

Wenn aktiviert (`enable_file_versioning`):

- Bei Dateiänderung neue Version mit `version++`
- `previous_version_id` zeigt auf alte Version
- Max X Versionen behalten (konfigurierbar)
- Alte Versionen im Trash oder permanent löschen

### 📊 Quota Management

- Pro-User Quota in `storage_quota_bytes`
- Aktuelle Nutzung in `storage_used_bytes`
- Bei Upload prüfen: `used + new_size <= quota`
- Trash-Dateien zählen zum Quota (bis auto-delete)

## Migration

Die Migration läuft automatisch beim ersten Start:

1. `syncspace.db` wird erstellt
2. Schema aus `migrations/001_initial_schema.sql` wird ausgeführt
3. Default settings werden eingefügt
4. Alte `users.json` kann optional importiert werden

## Performance

**Indizes:**

- Alle Foreign Keys haben Indices
- Häufig gefilterte Spalten (is_deleted, path, checksum)
- Composite indices wo sinnvoll

**Optimierungen:**

- SQLite mit WAL mode (besseres Concurrency)
- Connection pooling (max 10 connections)
- Prepared statements via sqlx
- Lazy loading für große Listen

## API Integration

Die Datenbank-Operationen sind in `database.rs` gekapselt:

```rust
// Beispiel: User erstellen
let user = User::create(&pool, "username".into(), "hash".into()).await?;

// File history loggen
FileHistory::log_action(&pool, &file_id, &user_id, "renamed",
    Some(old_name_json), Some(new_name_json)).await?;

// Settings abrufen
let revision_safety = Setting::get_bool(&pool, "revision_safety_enabled").await?;
```

## Nächste Schritte

1. ✅ Schema erstellt
2. ⏳ Auth zu DB migrieren (statt users.json)
3. ⏳ File operations zu DB migrieren
4. ⏳ API endpoints für History, Trash, Favorites, Shared Links
5. ⏳ Frontend für neue Features

## Entwicklung

**Tests hinzufügen:**

```bash
cd backend
cargo test --features sqlite
```

**Migration neu ausführen:**

```bash
rm syncspace.db
cargo run
```

**Schema inspizieren:**

```bash
sqlite3 syncspace.db
.schema
.tables
```
