//! Unit tests for authentication module
//! 
//! Tests password hashing, JWT generation/validation, and 2FA functionality.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &str = "test-secret-key";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    username: String,
    exp: usize,
}

// ============================================================================
// Password Hashing Tests
// ============================================================================

#[test]
fn test_password_hashing() {
    let password = "test_password_123";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    // Hash password
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password");
    
    // Verify password
    let hash_string = hash.to_string();
    let parsed_hash = argon2::password_hash::PasswordHash::new(&hash_string)
        .expect("Failed to parse hash");
    
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    assert!(result.is_ok(), "Password verification should succeed");
}

#[test]
fn test_password_hashing_wrong_password() {
    let password = "correct_password";
    let wrong_password = "wrong_password";
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    // Hash correct password
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password");
    
    // Try to verify with wrong password
    let hash_string = hash.to_string();
    let parsed_hash = argon2::password_hash::PasswordHash::new(&hash_string)
        .expect("Failed to parse hash");
    
    let result = Argon2::default().verify_password(wrong_password.as_bytes(), &parsed_hash);
    assert!(result.is_err(), "Wrong password should fail verification");
}

#[test]
fn test_password_hash_uniqueness() {
    let password = "same_password";
    
    // Hash the same password twice
    let salt1 = SaltString::generate(&mut OsRng);
    let hash1 = Argon2::default()
        .hash_password(password.as_bytes(), &salt1)
        .expect("Failed to hash password");
    
    let salt2 = SaltString::generate(&mut OsRng);
    let hash2 = Argon2::default()
        .hash_password(password.as_bytes(), &salt2)
        .expect("Failed to hash password");
    
    // Hashes should be different due to different salts
    assert_ne!(
        hash1.to_string(),
        hash2.to_string(),
        "Same password with different salts should produce different hashes"
    );
}

// ============================================================================
// JWT Token Tests
// ============================================================================

#[test]
fn test_jwt_generation_and_validation() {
    let user_id = "test-user-123";
    let username = "testuser";
    
    // Create token
    let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .expect("Failed to generate token");
    
    // Validate token
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .expect("Failed to validate token");
    
    assert_eq!(token_data.claims.sub, user_id);
    assert_eq!(token_data.claims.username, username);
}

#[test]
fn test_jwt_expired_token() {
    let user_id = "test-user-123";
    let username = "testuser";
    
    // Create expired token (1 hour ago)
    let exp = (Utc::now() - chrono::Duration::hours(1)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .expect("Failed to generate token");
    
    // Try to validate expired token
    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    );
    
    assert!(result.is_err(), "Expired token should fail validation");
}

#[test]
fn test_jwt_invalid_secret() {
    let user_id = "test-user-123";
    let username = "testuser";
    
    // Create token with one secret
    let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .expect("Failed to generate token");
    
    // Try to validate with different secret
    let wrong_secret = "wrong-secret-key";
    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(wrong_secret.as_bytes()),
        &Validation::default(),
    );
    
    assert!(result.is_err(), "Token with wrong secret should fail validation");
}

#[test]
fn test_jwt_token_format() {
    let user_id = "test-user-123";
    let username = "testuser";
    
    let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .expect("Failed to generate token");
    
    // JWT should have 3 parts separated by dots
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3, "JWT should have 3 parts (header.payload.signature)");
}

// ============================================================================
// TOTP 2FA Tests
// ============================================================================

#[test]
fn test_totp_generation() {
    use totp_lite::{totp_custom, Sha1};
    
    let secret = b"testsecret12345678901234567890"; // 30 bytes
    let time = 59; // Test vector time
    
    let totp = totp_custom::<Sha1>(30, 6, secret, time);
    
    // Should generate a 6-digit code
    assert_eq!(totp.len(), 6, "TOTP should be 6 digits");
    assert!(totp.chars().all(|c| c.is_ascii_digit()), "TOTP should only contain digits");
}

#[test]
fn test_totp_time_based() {
    use totp_lite::{totp_custom, Sha1};
    
    let secret = b"testsecret12345678901234567890";
    
    // Generate codes at different times
    let code1 = totp_custom::<Sha1>(30, 6, secret, 1000);
    let code2 = totp_custom::<Sha1>(30, 6, secret, 1001);
    let code3 = totp_custom::<Sha1>(30, 6, secret, 1030); // 30 seconds later
    
    // Codes at almost same time should be equal
    assert_eq!(code1, code2, "Codes within same 30s window should match");
    
    // Codes at different time windows might differ
    // (Not guaranteed, but statistically likely)
    println!("Code at 1000s: {}", code1);
    println!("Code at 1030s: {}", code3);
}

// ============================================================================
// Input Validation Tests
// ============================================================================

#[test]
fn test_username_validation() {
    fn is_valid_username(username: &str) -> bool {
        let re = regex::Regex::new(r"^[a-zA-Z0-9_-]{3,32}$").unwrap();
        re.is_match(username)
    }
    
    assert!(is_valid_username("testuser"));
    assert!(is_valid_username("test_user"));
    assert!(is_valid_username("test-user"));
    assert!(is_valid_username("user123"));
    
    assert!(!is_valid_username("ab"), "Username too short");
    assert!(!is_valid_username("a".repeat(33).as_str()), "Username too long");
    assert!(!is_valid_username("test user"), "Username with space");
    assert!(!is_valid_username("test@user"), "Username with special char");
    assert!(!is_valid_username("test.user"), "Username with dot");
}

#[test]
fn test_password_strength() {
    fn is_strong_password(password: &str) -> bool {
        password.len() >= 8
            && password.chars().any(|c| c.is_ascii_uppercase())
            && password.chars().any(|c| c.is_ascii_lowercase())
            && password.chars().any(|c| c.is_ascii_digit())
    }
    
    assert!(is_strong_password("Test1234"));
    assert!(is_strong_password("SecurePass123"));
    
    assert!(!is_strong_password("short1A"), "Password too short");
    assert!(!is_strong_password("nouppercase123"), "No uppercase");
    assert!(!is_strong_password("NOLOWERCASE123"), "No lowercase");
    assert!(!is_strong_password("NoDigits"), "No digits");
}

#[test]
fn test_email_validation() {
    fn is_valid_email(email: &str) -> bool {
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(email)
    }
    
    assert!(is_valid_email("test@example.com"));
    assert!(is_valid_email("user.name@example.co.uk"));
    assert!(is_valid_email("user+tag@example.org"));
    
    assert!(!is_valid_email("invalid"), "No @ symbol");
    assert!(!is_valid_email("@example.com"), "No local part");
    assert!(!is_valid_email("test@"), "No domain");
    assert!(!is_valid_email("test@.com"), "Invalid domain");
}
