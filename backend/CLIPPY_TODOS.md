# Backend - Unfertige Features (basierend auf Clippy Warnungen)

**Status**: 189 Warnungen nach initialer Bereinigung

## 🔐 OAuth Integration (Hohe Priorität)
**Dateien**: `src/api/oauth.rs`, `src/oauth.rs`

- [ ] `OAuthError` und `OAuthProvider` Enums implementieren
- [ ] `AuthResult` struct für OAuth-Callbacks nutzen
- [ ] `OAuthUserResponse` struct für User-Daten nutzen
- [ ] `OAuthCallbackParams` für Callback-Handling implementieren
- [ ] `list_providers()` Funktion fertigstellen
- [ ] Varianten `InvalidState`, `AccountAlreadyLinked`, `UserNotFound` behandeln
- [ ] `access_token_encrypted` und `refresh_token_encrypted` Felder nutzen
- [ ] `DateTime` Import aus oauth.rs verwenden

**Impact**: Kritisch für SSO/OAuth-Login

---

## 🔍 LDAP Integration (Hohe Priorität)
**Dateien**: `src/api/ldap.rs`, `src/ldap_integration.rs`

- [ ] `Deserialize` Import nutzen (Konfiguration)
- [ ] `LdapTestResult` und `SyncResult` Structs implementieren
- [ ] `get_active_config()` Funktion fertigstellen
- [ ] `authenticate()` Funktion komplett implementieren
- [ ] Varianten `AuthenticationFailed`, `UserNotFound`, `ConfigNotFound` behandeln

**Impact**: Kritisch für Enterprise-LDAP-Auth

---

## 🔐 Encryption (Mittlere Priorität)
**Dateien**: `src/api/encryption.rs`, `src/encryption.rs`

- [ ] PUT-Endpoint implementieren
- [ ] `PasswordVerifier` Import nutzen
- [ ] `EncryptedFile` struct für verschlüsselte Dateien nutzen
- [ ] `DecryptRequest` struct für Decrypt-API nutzen
- [ ] `encrypt_file()` Funktion in API integrieren
- [ ] `decrypt_file()` Funktion in API integrieren
- [ ] `password` Felder in Request-Structs verwenden
- [ ] Unused variables `password_hash` und `key_record` implementieren

**Impact**: Wichtig für File-Encryption Feature

---

## 📁 FTP Sync (Mittlere Priorität)
**Dateien**: `src/api/ftp.rs`, `src/ftp_sync.rs`

- [ ] DELETE-Endpoint implementieren
- [ ] PUT-Endpoint implementieren
- [ ] `serde::Deserialize` für Konfiguration nutzen

**Impact**: Wichtig für FTP-Synchronisation

---

## 📧 Email Integration (Niedrige Priorität)
**Dateien**: `src/api/email.rs`, `src/email_integration.rs`

- [ ] PUT-Endpoint implementieren
- [ ] `EmailAttachment` struct für Anhänge nutzen
- [ ] `store_attachment()` Funktion implementieren

**Impact**: Nice-to-have für Email-Benachrichtigungen mit Anhängen

---

## 🖼️ File Preview System (Mittlere Priorität)
**Dateien**: `src/api/preview.rs`, `src/file_preview.rs`

- [ ] `IntoResponse` Import nutzen (Response-Handling)
- [ ] `page` Feld in Query-Parameter nutzen
- [ ] `PreviewMetadata` struct für Metadaten nutzen
- [ ] `store_preview_metadata()` in API integrieren
- [ ] `get_preview()` Funktion implementieren
- [ ] `delete_previews()` Funktion implementieren
- [ ] `preview_type` Variable verwenden

**Impact**: Wichtig für vollständiges Preview-Feature

---

## 🖼️ Thumbnails (Mittlere Priorität)
**Dateien**: `src/api/thumbnails.rs`, `src/thumbnails.rs`

- [ ] `IntoResponse` Import nutzen
- [ ] `ThumbnailInfo` struct für Metadaten nutzen
- [ ] `thumbnail_exists()` Check implementieren
- [ ] `generate_all_thumbnails()` Batch-Generation implementieren
- [ ] `delete_thumbnails()` Cleanup implementieren
- [ ] `generate_thumbnail_background()` Background-Job implementieren
- [ ] Variant `FileNotFound` behandeln

**Impact**: Wichtig für Performance und UX

---

## 🦠 Virus Scan (Mittlere Priorität)
**Dateien**: `src/api/virus_scan.rs`, `src/virus_scan.rs`

- [ ] `state` und `query` Parameter in API verwenden
- [ ] `deep` Feld für Deep-Scan nutzen
- [ ] `ScanResult` struct für Scan-Ergebnisse nutzen
- [ ] `as_str()` Methode für ScanStatus implementieren
- [ ] `ScannerConfig` für Konfiguration nutzen
- [ ] `scan_and_store()` vollständig implementieren
- [ ] `should_skip_file()` Skip-Logik implementieren
- [ ] `init_quarantine_dir()` Initialisierung
- [ ] `quarantine_file()` Quarantäne-Feature
- [ ] `restore_from_quarantine()` Restore-Feature
- [ ] `delete_quarantined()` Cleanup
- [ ] `list_quarantined()` Listing
- [ ] `store_scan_result()` Persistierung
- [ ] `get_scan_history()` History-API
- [ ] `get_latest_scan()` Latest-Scan-API
- [ ] `get_scan_stats()` Statistiken
- [ ] `ScanStats` struct nutzen
- [ ] Variant `DatabaseError` behandeln

**Impact**: Wichtig für Sicherheit, aber optional

---

## 📦 Compression (Niedrige Priorität)
**Dateien**: `src/api/compression.rs`

- [ ] `extension()` Methode für Archive-Extensions implementieren
- [ ] `CompressionStats` struct für Statistiken nutzen

**Impact**: Nice-to-have

---

## ⏱️ Rate Limiting (Niedrige Priorität)
**Dateien**: `src/api/rate_limiting.rs`

- [ ] `params` Variable in Endpoint verwenden
- [ ] `BandwidthUsage` struct für Bandwidth-Tracking nutzen
- [ ] `start_date` und `end_date` Felder verwenden

**Impact**: Nice-to-have für erweiterte Rate-Limiting-Features

---

## 🎫 Guest Access (Niedrige Priorität)
**Dateien**: `src/api/guest.rs`

- [ ] `max_accesses` Feld für Access-Limits nutzen

**Impact**: Nice-to-have

---

## 🛠️ Weitere Kleinigkeiten

### Deprecation Warning
- [ ] **src/api/archives.rs:326** - `zip::DateTime::to_time` durch `OffsetDateTime::try_from()` ersetzen

### Code Quality
- [ ] **src/cron.rs:61** - Doc list item ohne Indentation fixen
- [ ] **src/search.rs:273** - Type alias für komplexen Tupel-Typ erstellen

### Empty Lines After Doc Comments (Formatierung)
Betroffen:
- `src/api/oauth.rs`
- `src/api/ldap.rs`
- `src/oauth.rs`
- `src/ldap_integration.rs`
- `src/file_preview.rs`
- `src/thumbnails.rs`
- `src/virus_scan.rs`

---

## 📊 Prioritäten-Übersicht

### 🔴 Hohe Priorität (Auth-kritisch)
1. OAuth Integration (SSO)
2. LDAP Integration (Enterprise)

### 🟡 Mittlere Priorität (Features)
3. Encryption (File Security)
4. FTP Sync (Sync-Feature)
5. File Preview System (UX)
6. Thumbnails (Performance)
7. Virus Scan (Security)

### 🟢 Niedrige Priorität (Nice-to-have)
8. Email Integration (Notifications)
9. Compression (Stats)
10. Rate Limiting (Advanced)
11. Guest Access (Limits)

---

## 🎯 Nächste Schritte

1. **Phase 1**: OAuth + LDAP komplett fertigstellen (Enterprise-Ready)
2. **Phase 2**: Encryption + FTP Sync (Core Features)
3. **Phase 3**: Preview + Thumbnails + Virus Scan (UX + Security)
4. **Phase 4**: Rest (Nice-to-have)

---

**Aktualisiert**: 2025-12-11
**Clippy Warnungen**: 189 (überwiegend "unused" für unfertige Features)
