# SyncSpace - Performance & Volltext-Suche

## Performance-Analyse: SQLx + SQLite

### Aktuelle Lösung: SQLx mit SQLite

**Vorteile:**

- ✅ **Embedded Database** - Keine separate Datenbank-Installation
- ✅ **Single-File** - Einfaches Backup (nur `syncspace.db` kopieren)
- ✅ **ACID Garantien** - Vollständige Transaktionssicherheit
- ✅ **Type-Safety** - Compile-time SQL-Prüfung mit sqlx
- ✅ **WAL Mode** - Write-Ahead Logging für besseres Concurrency
- ✅ **FTS5 Support** - Integrierte Volltext-Suche

**Performance bei 100k+ Einträgen:**

- SQLite kann **problemlos Millionen Zeilen** verwalten
- Mit richtigen Indizes: **< 1ms** für einfache Queries
- WAL Mode: **~50k Writes/Sekunde** möglich
- FTS5: **~10-50ms** für Volltext-Suche über 100k Dokumente

**Optimierungen implementiert:**

```rust
PRAGMA journal_mode=WAL;       // Write-Ahead Logging
PRAGMA synchronous=NORMAL;     // Schneller, aber sicher
PRAGMA temp_store=MEMORY;      // Temp-Daten im RAM
PRAGMA cache_size=-64000;      // 64MB Cache
PRAGMA foreign_keys=ON;        // Referenzielle Integrität
```

**Bestehende Indizes (27 total):**

- Alle Foreign Keys haben Indizes
- `files.checksum_sha256` (für Deduplication)
- `files.path` (für schnelle Lookup)
- `file_history.file_id` + `file_history.created_at` (für Historie)
- `trash.auto_delete_at` (für Cleanup-Scheduler)

### Alternativen verglichen

#### 1. PostgreSQL (über sqlx)

**Vorteile:**

- Besser für Multi-User Szenarien (100+ gleichzeitige Connections)
- Bessere Replikation und Sharding
- Leistungsfähigere Volltext-Suche (tsvector)

**Nachteile:**

- ❌ Externe Installation erforderlich
- ❌ Komplexeres Setup für Self-Hosted
- ❌ Overkill für <10 gleichzeitige User
- ❌ Backup komplizierter

**Fazit:** Nicht empfohlen für SyncSpace (self-hosted, wenige User)

#### 2. RocksDB / sled

**Vorteile:**

- Sehr schnelle Writes (Key-Value Store)
- Embedded wie SQLite

**Nachteile:**

- ❌ Keine SQL-Queries (nur Key-Value)
- ❌ Komplexere Datenmodellierung
- ❌ Keine ACID über mehrere Collections
- ❌ Keine eingebaute Volltext-Suche

**Fazit:** Nicht geeignet für relationale Daten

#### 3. DuckDB

**Vorteile:**

- Sehr schnelle Analytics-Queries
- Embedded wie SQLite

**Nachteile:**

- ❌ Optimiert für OLAP, nicht OLTP (weniger Write-Performance)
- ❌ Noch junge Technologie

**Fazit:** Nicht geeignet für transaktionale Workloads

### Empfehlung: **SQLite + sqlx beibehalten**

Für SyncSpace ist SQLite die beste Wahl:

1. **Self-Hosted Focus** - Keine externe DB nötig
2. **Wenige User** - 1-10 gleichzeitige Connections
3. **Embedded** - Single-Binary Deployment
4. **Bewährt** - SQLite ist battle-tested seit 20+ Jahren
5. **FTS5** - Eingebaute Volltext-Suche

---

## Volltext-Suche: Optionen

### Option 1: SQLite FTS5 (Empfohlen für Start) ✅

**Implementierung:**

```sql
-- Create virtual table for fulltext search
CREATE VIRTUAL TABLE files_fts USING fts5(
    file_id UNINDEXED,
    filename,
    path,
    content_text,  -- Extracted text content
    tokenize='porter unicode61 remove_diacritics 1'
);

-- Trigger to keep FTS in sync
CREATE TRIGGER files_fts_insert AFTER INSERT ON files BEGIN
    INSERT INTO files_fts(file_id, filename, path, content_text)
    VALUES (new.id, new.name, new.path, '');
END;
```

**Vorteile:**

- ✅ **Embedded** - Keine zusätzliche Software
- ✅ **Schnell** - 10-50ms für 100k Dokumente
- ✅ **Porter Stemming** - Englisch/Deutsch Support
- ✅ **BM25 Ranking** - Relevanz-basierte Sortierung
- ✅ **Phrase Search** - "exact phrase" Suche
- ✅ **Prefix Search** - `word*` Wildcards

**Performance:**

- 100k Einträge: **~20ms** average query time
- 1M Einträge: **~50-100ms** average query time
- Index-Größe: ~30% der Original-Daten

**Nachteile:**

- ❌ Keine Fuzzy Search (Typos)
- ❌ Kein Content-Extraction (nur Filename/Path)

### Option 2: Tantivy (Rust Native) 🔥

**Was ist Tantivy?**

- Pure Rust Volltext-Suche (wie Lucene/Elasticsearch)
- Entwickelt von Quickwit (Search-as-a-Service)
- Sehr schnell: **<5ms** für 100k Dokumente

**Implementierung:**

```rust
use tantivy::schema::*;
use tantivy::{Index, IndexWriter};

// Schema definieren
let mut schema_builder = Schema::builder();
schema_builder.add_text_field("filename", TEXT | STORED);
schema_builder.add_text_field("path", TEXT | STORED);
schema_builder.add_text_field("content", TEXT);
let schema = schema_builder.build();

// Index erstellen
let index = Index::create_in_dir("./data/search_index", schema)?;

// Dokument indizieren
let mut index_writer = index.writer(50_000_000)?; // 50MB buffer
index_writer.add_document(doc!(
    filename => "example.pdf",
    path => "/Documents/example.pdf",
    content => extracted_text
))?;
index_writer.commit()?;

// Suche
let searcher = index.reader()?.searcher();
let query_parser = QueryParser::for_index(&index, vec![filename, content]);
let query = query_parser.parse_query("search term")?;
let results = searcher.search(&query, &TopDocs::with_limit(10))?;
```

**Vorteile:**

- ✅ **Sehr schnell** - <5ms für 100k Dokumente
- ✅ **Fuzzy Search** - Typo-Toleranz
- ✅ **Faceting** - Filter nach Typ/Datum/Größe
- ✅ **Highlighting** - Treffer-Markierung
- ✅ **BM25 Ranking** - Besseres Ranking als SQLite
- ✅ **Pure Rust** - Keine C-Dependencies

**Nachteile:**

- ⚠️ Separate Index-Dateien (zusätzlich zur DB)
- ⚠️ Muss synchronisiert werden (File Change → Index Update)
- ⚠️ ~20% mehr Disk Space

**Performance:**

- 100k Einträge: **<5ms** average query time
- 1M Einträge: **~10-20ms** average query time
- Fuzzy Search: **~10-30ms**

### Option 3: Meilisearch (Externe Service) 🚀

**Was ist Meilisearch?**

- Typo-tolerante Suche (wie Algolia)
- REST API
- Separate Binary/Docker Container

**Setup:**

```bash
# Docker
docker run -d -p 7700:7700 getmeili/meilisearch

# In SyncSpace Backend
POST http://localhost:7700/indexes/files/documents
[
  {
    "id": "file-123",
    "filename": "example.pdf",
    "path": "/Documents/example.pdf"
  }
]

# Suche
GET http://localhost:7700/indexes/files/search?q=example
```

**Vorteile:**

- ✅ **Sehr schnell** - <1ms für 100k Dokumente
- ✅ **Fuzzy Search** - Beste Typo-Toleranz
- ✅ **Faceting** - Sehr flexible Filter
- ✅ **Highlighting** - Sehr gute Treffer-Anzeige
- ✅ **Multi-Language** - 30+ Sprachen

**Nachteile:**

- ❌ **Externe Service** - Nicht embedded
- ❌ Separate Installation (Binary oder Docker)
- ❌ Zusätzlicher Prozess zu managen
- ❌ Mehr RAM Verbrauch (~100-500MB)

**Performance:**

- 100k Einträge: **<1ms** average query time
- 1M Einträge: **~2-5ms** average query time

### Option 4: Qdrant (Vector Search)

**Nur relevant wenn:**

- AI-basierte semantische Suche gewünscht
- "Find similar files" Feature
- Embedding-Modell läuft (z.B. BERT)

**Nicht empfohlen** für klassische Keyword-Suche.

---

## Empfehlung: Stufenweise Implementation

### Phase 1: SQLite FTS5 (Jetzt implementieren) ✅

**Warum:**

- Keine zusätzliche Dependency
- Ausreichend für 90% der Anwendungsfälle
- Einfach zu implementieren (~100 Zeilen Code)

**Implementierung:**

1. `002_fulltext_search.sql` Migration erstellen
2. FTS5 Virtual Table für `files` und `folders`
3. Trigger für Auto-Update
4. `/api/search/fulltext` Endpoint

**Code-Aufwand:** ~2-3 Stunden

### Phase 2: Tantivy (Optional Upgrade)

**Wann:**

- User beschweren sich über Suche
- Content-Extraktion aus PDFs gewünscht
- > 100k Dateien im System

**Implementierung:**

1. `tantivy` zu Cargo.toml
2. Separate `search.rs` Modul
3. Index in `./data/search_index/`
4. Background-Thread für Indexing
5. `/api/search/advanced` Endpoint

**Code-Aufwand:** ~1-2 Tage

### Phase 3: Meilisearch (Falls Scale benötigt)

**Wann:**

- Multi-User System mit 10+ gleichzeitigen Suchen
- Sub-millisecond Response Time erforderlich
- Budget für zusätzlichen Service vorhanden

**Implementierung:**

1. Docker Compose hinzufügen
2. `reqwest` für HTTP Calls
3. Webhook für File Changes → Meilisearch
4. Failover zu SQLite FTS5 falls Meilisearch down

**Code-Aufwand:** ~2-3 Tage

---

## Content-Extraktion für Volltext-Suche

### Datei-Typen für Content-Indexing

**Text-Dateien (direkt lesbar):**

- `.txt`, `.md`, `.json`, `.xml`, `.csv`
- Rust: `std::fs::read_to_string()`

**Office-Dokumente:**

- PDF: `lopdf` crate (reine Text-Extraktion)
- DOCX: `docx-rs` crate
- XLSX: `calamine` crate

**Code-Dateien:**

- `.rs`, `.js`, `.py`, `.java` etc.
- Direkt als Text indizieren

**Binär-Dateien (nicht indexieren):**

- Bilder, Videos, Audio
- Nur Metadaten (Filename, Tags) indexieren

### Asynchrone Indexierung

```rust
// File Upload → Sofort in DB
sqlx::query("INSERT INTO files ...").execute(&pool).await?;

// Background Job → Content Extraction + FTS Update
tokio::spawn(async move {
    if let Ok(content) = extract_text(&file_path).await {
        sqlx::query(
            "INSERT INTO files_fts(file_id, content_text) VALUES (?, ?)"
        )
        .bind(file_id)
        .bind(content)
        .execute(&pool)
        .await?;
    }
});
```

---

## Benchmark: Erwartete Performance

### SQLite FTS5

| Dateien | Index Size | Search Time | Index Time |
| ------- | ---------- | ----------- | ---------- |
| 10k     | ~50 MB     | 5-10ms      | ~2 sec     |
| 100k    | ~500 MB    | 20-50ms     | ~20 sec    |
| 1M      | ~5 GB      | 50-150ms    | ~3 min     |

### Tantivy

| Dateien | Index Size | Search Time | Index Time |
| ------- | ---------- | ----------- | ---------- |
| 10k     | ~30 MB     | 1-3ms       | ~1 sec     |
| 100k    | ~300 MB    | 3-10ms      | ~10 sec    |
| 1M      | ~3 GB      | 10-30ms     | ~2 min     |

### Meilisearch

| Dateien | Index Size | Search Time | Index Time |
| ------- | ---------- | ----------- | ---------- |
| 10k     | ~40 MB     | <1ms        | ~1 sec     |
| 100k    | ~400 MB    | <2ms        | ~8 sec     |
| 1M      | ~4 GB      | 2-5ms       | ~1.5 min   |

---

## Nächste Schritte

1. ✅ SQLite mit Performance-Optimierungen (PRAGMA)
2. ✅ Admin User immer beim Start erstellen
3. 🔄 FTS5 Migration erstellen (`002_fulltext_search.sql`)
4. 🔄 `/api/search/fulltext?q=term` Endpoint
5. ⏳ Content-Extraction für PDFs (optional)
6. ⏳ Tantivy Integration (optional upgrade)

**Empfehlung:** Start mit SQLite FTS5, später auf Tantivy upgraden falls nötig.
