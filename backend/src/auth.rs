//! Authentication module for SyncSpace
//! 
//! Provides user management, JWT tokens, password hashing, and TOTP 2FA.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use totp_lite::{totp_custom, Sha1};
use uuid::Uuid;

const USERS_FILE: &str = "./users.json";
const JWT_SECRET: &str = "your-secret-key-change-this-in-production"; // TODO: Move to env var
const TOKEN_EXPIRATION_HOURS: i64 = 24;

/// User account structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub totp_secret: Option<String>,
    pub totp_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

/// In-memory user database with persistent JSON storage
#[derive(Clone)]
pub struct UserDB {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
}

impl UserDB {
    pub fn new() -> Self {
        let db = UserDB {
            users: Arc::new(Mutex::new(HashMap::new())),
        };
        db.load_from_disk();
        db
    }

    fn load_from_disk(&self) {
        if let Ok(data) = fs::read_to_string(USERS_FILE) {
            if let Ok(users_vec) = serde_json::from_str::<Vec<User>>(&data) {
                let mut users = self.users.lock().unwrap();
                for user in users_vec {
                    users.insert(user.id, user);
                }
                println!("Loaded {} users from disk", users.len());
            }
        } else {
            println!("No users file found, starting fresh");
        }
    }

    fn save_to_disk(&self) {
        let users = self.users.lock().unwrap();
        let users_vec: Vec<User> = users.values().cloned().collect();
        if let Ok(json) = serde_json::to_string_pretty(&users_vec) {
            let _ = fs::write(USERS_FILE, json);
        }
    }

    pub fn create_user(&self, username: String, password: String) -> Result<User, String> {
        let mut users = self.users.lock().unwrap();

        // Check if username exists
        if users.values().any(|u| u.username == username) {
            return Err("Username already exists".to_string());
        }

        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Password hashing failed: {}", e))?
            .to_string();

        let user = User {
            id: Uuid::new_v4(),
            username,
            password_hash,
            totp_secret: None,
            totp_enabled: false,
            created_at: Utc::now(),
            last_login: None,
        };

        users.insert(user.id, user.clone());
        drop(users);
        self.save_to_disk();

        Ok(user)
    }

    pub fn get_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.values().find(|u| u.username == username).cloned()
    }

    pub fn get_by_id(&self, id: &Uuid) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.get(id).cloned()
    }

    pub fn update_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id, user);
        drop(users);
        self.save_to_disk();
    }

    pub fn verify_password(&self, username: &str, password: &str) -> Result<User, String> {
        let user = self
            .get_by_username(username)
            .ok_or("Invalid username or password")?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| "Invalid password hash".to_string())?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| "Invalid username or password".to_string())?;

        Ok(user)
    }

    pub fn change_password(&self, user_id: Uuid, old_password: &str, new_password: &str) -> Result<(), String> {
        let mut user = self.get_by_id(&user_id).ok_or("User not found")?;

        // Verify old password
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| "Invalid password hash".to_string())?;

        Argon2::default()
            .verify_password(old_password.as_bytes(), &parsed_hash)
            .map_err(|_| "Current password is incorrect".to_string())?;

        // Hash new password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        user.password_hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| format!("Password hashing failed: {}", e))?
            .to_string();

        self.update_user(user);
        Ok(())
    }
}

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

/// Generate JWT token for authenticated user
pub fn generate_token(user: &User) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(TOKEN_EXPIRATION_HOURS))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(|e| format!("Token generation failed: {}", e))
}

/// Verify JWT token and extract claims
pub fn verify_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Token verification failed: {}", e))
}

/// TOTP 2FA Functions
pub fn generate_totp_secret() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
    base32::encode(base32::Alphabet::RFC4648 { padding: false }, &bytes)
}

pub fn generate_totp_code(secret: &str, time_step: u64) -> Result<String, String> {
    let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret)
        .ok_or("Invalid base32 secret")?;

    let code = totp_custom::<Sha1>(30, 6, &secret_bytes, time_step);
    Ok(format!("{:06}", code))
}

pub fn verify_totp_code(secret: &str, code: &str) -> bool {
    let time_step = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Check current time step and ±1 step (30s tolerance)
    for offset in [-1i64, 0, 1] {
        let check_time = (time_step as i64 + offset * 30) as u64;
        if let Ok(expected) = generate_totp_code(secret, check_time) {
            if expected == code {
                return true;
            }
        }
    }
    false
}

/// Warp filter to extract authenticated user from JWT token
/// Axum extractor for authenticated requests
#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header"))?;

        // Verify Bearer token
        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid authorization header"));
        }

        let token = auth_header.trim_start_matches("Bearer ");
        let claims = verify_token(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID"))?;

        // Load user from database
        let db = UserDB::new(); // TODO: Pass via extension layer instead
        db.get_by_id(&user_id)
            .ok_or((StatusCode::UNAUTHORIZED, "User not found"))
    }
}

/// Auth request/response structures
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub totp_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Enable2FARequest {
    pub totp_code: String,
}

#[derive(Debug, Serialize)]
pub struct Setup2FAResponse {
    pub secret: String,
    pub qr_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// Rate limiting structure (simple in-memory)
pub struct RateLimiter {
    attempts: Arc<Mutex<HashMap<String, Vec<DateTime<Utc>>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            attempts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn check_rate_limit(&self, key: &str, max_attempts: usize, window_secs: i64) -> bool {
        let mut attempts = self.attempts.lock().unwrap();
        let now = Utc::now();
        let cutoff = now - Duration::seconds(window_secs);

        let entry = attempts.entry(key.to_string()).or_insert_with(Vec::new);
        entry.retain(|&ts| ts > cutoff);

        if entry.len() >= max_attempts {
            return false;
        }

        entry.push(now);
        true
    }
}
