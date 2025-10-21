# 🚀 SyncSpace - Quick Start Guide

## ✅ System Status

**Backend:** ✅ Running on `http://localhost:8080`  
**Frontend:** ✅ Running on `http://localhost:8000`  
**Authentication:** ✅ Enabled with JWT + 2FA Support

---

## 🔑 Initiale Login-Daten

Das System erstellt automatisch einen **Admin-User** beim ersten Start:

```
Username: admin
Password: admin
```

> ⚠️ **WICHTIG:** Ändere das Passwort sofort nach dem ersten Login!

---

## 🎯 So testest du das System:

### Option 1: Web-Interface (Empfohlen)

1. **Öffne Browser:**

   ```
   http://localhost:8000/index-new.html
   ```

2. **Login mit:**

   - Username: `admin`
   - Password: `admin`

3. **Nach dem Login kannst du:**
   - ✅ Dateien hochladen (Drag & Drop)
   - ✅ Verzeichnisse erstellen
   - ✅ Dateien suchen
   - ✅ Dark Mode aktivieren
   - ✅ 2FA einrichten (Settings)
   - ✅ Passwort ändern

---

### Option 2: API Testing (PowerShell)

```powershell
# Im Projektverzeichnis:
.\test-api-simple.ps1
```

Dieser Test prüft alle Endpoints:

- ✅ User Registration
- ✅ Login
- ✅ File Upload/Download
- ✅ Directory Creation
- ✅ Search
- ✅ Statistics
- ✅ 2FA Setup
- ✅ Protected Routes

---

## 📋 Wichtige Features

### 🔐 Authentication Flow

```
1. Register (oder Login mit admin)
   ↓
2. JWT Token erhalten (24h gültig)
   ↓
3. Token wird automatisch gespeichert
   ↓
4. Alle API-Requests nutzen den Token
   ↓
5. Optional: 2FA aktivieren für extra Security
```

### 🎨 Frontend Features

- **Login/Register Page** - Material 3 Design
- **Dark Mode** - Toggle oben rechts
- **File Management** - Upload, Download, Delete, Rename
- **Search** - Suche über alle Dateien
- **Peers** - Synchronisations-Partner verwalten
- **Settings** - Profil, 2FA, Passwort ändern

### 🔒 Security Features

- **JWT Tokens** - 24h Lifetime
- **Argon2 Password Hashing** - Maximale Sicherheit
- **Rate Limiting** - 5 Login-Versuche/Minute
- **2FA/TOTP** - Google Authenticator kompatibel
- **Protected Routes** - Alle File-Ops erfordern Auth

---

## 🧪 Quick Tests

### 1. Manueller Login-Test (cURL)

```bash
# Login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}'

# Response:
# {
#   "token": "eyJ0eXAiOiJKV1Q...",
#   "user": {
#     "id": "...",
#     "username": "admin",
#     "totp_enabled": false
#   },
#   "requires_2fa": false
# }
```

### 2. PowerShell Login-Test

```powershell
$body = @{
    username = "admin"
    password = "admin"
} | ConvertTo-Json

$response = Invoke-RestMethod -Uri "http://localhost:8080/api/auth/login" `
    -Method Post `
    -ContentType "application/json" `
    -Body $body

Write-Host "Token: $($response.token)"
Write-Host "User: $($response.user.username)"
```

### 3. Neuen User erstellen

```powershell
$body = @{
    username = "testuser"
    password = "SecurePassword123!"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:8080/api/auth/register" `
    -Method Post `
    -ContentType "application/json" `
    -Body $body
```

---

## 🔧 Troubleshooting

### Backend startet nicht?

```powershell
cd backend
cargo clean
cargo build
cargo run
```

### Frontend zeigt Fehler?

1. Prüfe ob Backend läuft: `http://localhost:8080/api/stats`
2. Browser Console öffnen (F12)
3. Network Tab prüfen

### Login funktioniert nicht?

1. **Prüfe Backend Log** - Zeigt es "Creating default admin user"?
2. **Lösche users.json** im backend-Verzeichnis und starte neu
3. **Prüfe Browser Console** auf CORS/Network Errors

### 2FA Code wird nicht akzeptiert?

- **Zeit synchronisiert?** TOTP ist zeitbasiert (±30s Toleranz)
- **QR-Code richtig gescannt?** Neu generieren in Settings

---

## 📁 Dateien & Daten

```
backend/
├── data/           # Hier werden deine Dateien gespeichert
├── users.json      # User-Datenbank (Passwörter gehasht)
└── config.json     # System-Konfiguration

frontend/
├── index-new.html  # Haupt-UI (öffnen im Browser)
└── app.js         # JavaScript Logic
```

---

## 🎯 Nächste Schritte

### Sofort nach dem Login:

1. **Passwort ändern**
   - Settings → Account → Change Password
2. **2FA aktivieren**

   - Settings → Security → Setup 2FA
   - QR-Code scannen mit Google Authenticator
   - 6-stelligen Code eingeben → Verify

3. **Dateien hochladen**
   - Files → Drag & Drop Zone
   - Oder "Choose File" Button

### Optional:

- Neuen User anlegen (Register)
- Peers hinzufügen für Sync
- Dark Mode aktivieren

---

## 🆘 Häufige Fragen

**Q: Kann ich mehrere User anlegen?**  
A: Ja! Jeder kann sich über `/api/auth/register` registrieren.

**Q: Wie sicher sind die Passwörter?**  
A: Argon2-gehashed mit Salt, Industrie-Standard.

**Q: Wo werden meine Dateien gespeichert?**  
A: Im `backend/data/` Verzeichnis.

**Q: Kann ich 2FA deaktivieren?**  
A: Ja, in Settings → Security → Disable 2FA.

**Q: Token abgelaufen?**  
A: Einfach neu einloggen, Token ist 24h gültig.

---

## 📞 Support

- **Logs prüfen:** Backend Terminal zeigt alle Requests
- **Browser Console:** F12 → Console für Frontend-Errors
- **API testen:** `.\test-api-simple.ps1` ausführen

---

## ✅ System Check

Prüfe ob alles läuft:

```powershell
# Backend läuft?
Invoke-WebRequest -Uri "http://localhost:8080/api/auth/login" -Method Options

# Frontend läuft?
Invoke-WebRequest -Uri "http://localhost:8000/index-new.html"
```

Wenn beide Befehle **ohne Fehler** durchlaufen, ist alles bereit! 🎉

---

**Viel Erfolg mit SyncSpace!** 🚀
