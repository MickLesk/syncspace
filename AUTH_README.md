# SyncSpace Backend v0.2.0 - Authentication & Security

## 🎉 Neue Features

### 🔐 Vollständiges Authentication System

- **User Registration** (`POST /api/auth/register`)
- **Login mit JWT Tokens** (`POST /api/auth/login`)
- **Password Hashing** mit Argon2
- **JWT Token Generation & Validation**
- **Protected Routes** - Alle File-Operationen erfordern Authentication

### 🔒 Two-Factor Authentication (2FA)

- **TOTP-basiert** (Time-based One-Time Password)
- **QR-Code Generation** für Authenticator Apps
- **Setup Endpoint** (`POST /api/auth/2fa/setup`)
- **Enable/Disable** (`POST /api/auth/2fa/enable`, `/disable`)
- **Login mit 2FA** - Automatische Abfrage wenn aktiviert

### 🛡️ Security Features

- **Rate Limiting** - 5 Login-Versuche pro Minute
- **Password Strength** - Argon2 mit Salt
- **Token Expiration** - 24h JWT Lifetime
- **Protected WebSocket** - Auth-Header erforderlich

### 🎨 Modern Frontend (Material 3)

- **Login/Register Page** mit 2FA Support
- **Dark Mode Toggle** mit LocalStorage Persistenz
- **Settings Page** mit 2FA Management
- **Password Change** Dialog
- **Responsive Design** für Mobile/Desktop
- **Toast Notifications** mit Animation
- **Loading Overlays**

## 📋 API Endpoints

### Authentication

```
POST   /api/auth/register           - Create new user
POST   /api/auth/login              - Login (returns JWT token)
GET    /api/auth/me                 - Get current user info
POST   /api/auth/2fa/setup          - Generate 2FA secret
POST   /api/auth/2fa/enable         - Enable 2FA with verification
POST   /api/auth/2fa/disable        - Disable 2FA
PUT    /api/auth/change-password    - Change user password
```

### Protected File Operations (require Authorization header)

```
GET    /api/files/{path}            - List directory
GET    /api/file/{path}             - Download file
POST   /api/upload/{path}           - Upload file
DELETE /api/files/{path}            - Delete file/directory
POST   /api/dirs/{path}             - Create directory
PUT    /api/rename/{old}            - Rename/move file
GET    /api/search?q=term           - Search files
GET    /api/stats                   - File statistics
```

### Configuration & Peers

```
GET    /api/config                  - Get configuration
PUT    /api/config                  - Update configuration
GET    /api/peers                   - List peers
POST   /api/peers                   - Add peer
```

### Real-time

```
GET    /api/ws                      - WebSocket for file events
```

## 🚀 Quick Start

### Backend

```bash
cd backend
cargo run
```

**Default User:**

- Username: `admin`
- Password: `admin`

### Frontend

```bash
cd frontend
python -m http.server 8000
```

Open: **http://localhost:8000/index-new.html**

## 🔑 Authentication Flow

1. **Register/Login** → Receive JWT Token
2. **Store Token** in LocalStorage
3. **Include Token** in all API requests:
   ```javascript
   headers: {
     'Authorization': 'Bearer YOUR_JWT_TOKEN'
   }
   ```
4. **Optional:** Enable 2FA in Settings
5. **Next Login:** Provide TOTP code if 2FA enabled

## 🛠️ Configuration

### JWT Secret

Edit in `backend/src/auth.rs`:

```rust
const JWT_SECRET: &str = "your-secret-key-change-this-in-production";
```

**Production:** Use environment variable!

### Token Expiration

```rust
const TOKEN_EXPIRATION_HOURS: i64 = 24;
```

### Rate Limiting

```rust
// In login handler:
rate_limiter.check_rate_limit(&username, 5, 60)
// 5 attempts per 60 seconds
```

## 📦 Dependencies

### Backend (Cargo.toml)

```toml
jsonwebtoken = "9.2"      # JWT tokens
argon2 = "0.5"            # Password hashing
totp-lite = "2.0"         # TOTP 2FA
rand = "0.8"              # Random number generation
sha2 = "0.10"             # SHA-2 hashing
hex = "0.4"               # Hex encoding
```

### Frontend

- Vanilla JavaScript (no dependencies)
- Material 3 Design System
- Roboto Font (Google Fonts)

## 🗄️ Data Storage

- **Users:** `./users.json` (JSON file with hashed passwords)
- **Files:** `./data/` directory
- **Config:** `./config.json`

## 🔐 Security Best Practices

1. **Change Default Admin Password** immediately
2. **Use HTTPS** in production
3. **Set Strong JWT Secret** via environment variable
4. **Enable 2FA** for all users
5. **Regular Backups** of `users.json` and `data/`
6. **Monitor** `audit.log` for suspicious activity
7. **Update Dependencies** regularly

## 📝 User Management

### Create User (Programmatic)

```rust
let user_db = UserDB::new();
user_db.create_user("username".to_string(), "password".to_string())?;
```

### Password Hashing

```rust
// Automatic with Argon2
let hash = argon2.hash_password(password.as_bytes(), &salt)?;
```

### TOTP Secret Generation

```rust
let secret = generate_totp_secret(); // Base32 encoded
```

## 🧪 Testing

### Register New User

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"secure123"}'
```

### Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}'
```

### Upload File (with Auth)

```bash
curl -X POST http://localhost:8080/api/upload/test.txt \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  --data-binary @test.txt
```

## 🎯 Next Steps

- [ ] Add audit logging (login attempts, file operations)
- [ ] Implement session management (logout endpoint)
- [ ] Add user roles (admin, user, readonly)
- [ ] Email verification for registration
- [ ] Password reset flow
- [ ] Multi-user file permissions
- [ ] Activity dashboard
- [ ] Export audit logs

## 📄 License

MIT

## 🤝 Contributing

PRs welcome! Please test authentication flow before submitting.
