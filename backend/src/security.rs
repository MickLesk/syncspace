//! Security hardening module for SyncSpace
//! 
//! Provides security headers, CSRF protection, input validation, and security utilities.

use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, StatusCode, Request},
    middleware::Next,
    response::Response,
};
use regex::Regex;
use std::sync::OnceLock;

// ==================== SECURITY HEADERS ====================

/// Security headers middleware
/// Adds CSP, HSTS, X-Frame-Options, etc.
pub async fn security_headers_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // Content Security Policy (CSP)
    // Allows self-hosted resources, inline styles (for DaisyUI), and WebSocket
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static(
            "default-src 'self'; \
             script-src 'self' 'unsafe-inline'; \
             style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; \
             font-src 'self' https://fonts.gstatic.com; \
             img-src 'self' data: blob:; \
             connect-src 'self' ws://localhost:8080 wss://localhost:8080; \
             frame-ancestors 'none'; \
             base-uri 'self'; \
             form-action 'self';"
        ),
    );

    // HTTP Strict Transport Security (HSTS)
    // Force HTTPS for 1 year (31536000 seconds)
    // Note: Only enable in production with HTTPS
    #[cfg(not(debug_assertions))]
    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
    );

    // Prevent clickjacking attacks
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY"),
    );

    // Prevent MIME type sniffing
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );

    // XSS protection (legacy, but still useful)
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );

    // Referrer policy - don't leak URLs
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    // Permissions policy (disable unnecessary features)
    headers.insert(
        "Permissions-Policy",
        HeaderValue::from_static(
            "geolocation=(), microphone=(), camera=(), payment=(), usb=()"
        ),
    );

    response
}

// ==================== CSRF PROTECTION ====================

/// Simple CSRF token validation
/// For production, use a proper CSRF library
/// Currently unused but part of security API for future CSRF implementation
#[allow(dead_code)]
pub fn validate_csrf_token(headers: &HeaderMap, expected_token: &str) -> Result<(), StatusCode> {
    let token = headers
        .get("X-CSRF-Token")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::FORBIDDEN)?;

    if token != expected_token {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(())
}

/// Generate CSRF token (simple implementation)
/// For production, use cryptographically secure random tokens
pub fn generate_csrf_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";
    const TOKEN_LEN: usize = 32;
    let mut rng = rand::rng();

    (0..TOKEN_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// ==================== INPUT VALIDATION ====================

/// Validate file path to prevent directory traversal attacks
pub fn validate_file_path(path: &str) -> Result<String, StatusCode> {
    // Normalize path
    let normalized = path.replace("\\", "/");
    
    // Check for directory traversal patterns
    if normalized.contains("..") || normalized.contains("//") {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Remove leading slash for consistency
    let clean_path = normalized.trim_start_matches('/');

    // Check for absolute paths (Windows and Unix)
    if clean_path.contains(':') || clean_path.starts_with('/') {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check for null bytes
    if clean_path.contains('\0') {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(clean_path.to_string())
}

/// Validate filename (no path separators allowed)
pub fn validate_filename(filename: &str) -> Result<String, StatusCode> {
    // Check for empty filename
    if filename.is_empty() || filename.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check for path separators
    if filename.contains('/') || filename.contains('\\') {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check for special names (Windows reserved)
    let upper = filename.to_uppercase();
    let reserved = [
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    
    if reserved.contains(&upper.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check for null bytes
    if filename.contains('\0') {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Max filename length (255 bytes on most filesystems)
    if filename.len() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(filename.to_string())
}

/// Validate username (alphanumeric, underscore, hyphen)
pub fn validate_username(username: &str) -> Result<String, StatusCode> {
    static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = USERNAME_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9_-]{3,32}$").unwrap()
    });

    if !regex.is_match(username) {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(username.to_string())
}

/// Validate email address (basic validation)
pub fn validate_email(email: &str) -> Result<String, StatusCode> {
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    });

    if !regex.is_match(email) {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(email.to_string())
}

/// Validate password strength
pub fn validate_password_strength(password: &str) -> Result<(), String> {
    // Minimum length
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }

    // Maximum length (prevent DoS via long passwords)
    if password.len() > 128 {
        return Err("Password too long (max 128 characters)".to_string());
    }

    // Check for at least one lowercase letter
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err("Password must contain at least one lowercase letter".to_string());
    }

    // Check for at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain at least one uppercase letter".to_string());
    }

    // Check for at least one digit
    if !password.chars().any(|c| c.is_numeric()) {
        return Err("Password must contain at least one number".to_string());
    }

    // Check for at least one special character
    let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    if !password.chars().any(|c| special_chars.contains(c)) {
        return Err("Password must contain at least one special character".to_string());
    }

    Ok(())
}

/// Sanitize user input for display (prevent XSS)
pub fn sanitize_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('/', "&#x2F;")
}

/// Validate search query (prevent injection attacks)
pub fn validate_search_query(query: &str) -> Result<String, StatusCode> {
    // Max query length
    if query.len() > 500 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Remove potential SQL injection patterns (though we use parameterized queries)
    let dangerous_patterns = [
        "DROP TABLE",
        "DELETE FROM",
        "INSERT INTO",
        "UPDATE ",
        "UNION SELECT",
        "'; --",
        "' OR '1'='1",
    ];

    let upper_query = query.to_uppercase();
    for pattern in &dangerous_patterns {
        if upper_query.contains(pattern) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    Ok(query.to_string())
}

// ==================== RATE LIMITING HELPERS ====================

/// Check if request is from localhost (for rate limit exemption)
/// Currently unused but prepared for advanced rate limiting features
#[allow(dead_code)]
pub fn is_localhost(headers: &HeaderMap) -> bool {
    if let Some(forwarded) = headers.get("X-Forwarded-For")
        && let Ok(addr) = forwarded.to_str() {
            return addr.starts_with("127.") || addr.starts_with("::1");
        }
    
    // Default to not localhost for safety
    false
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_file_path() {
        // Valid paths
        assert!(validate_file_path("folder/file.txt").is_ok());
        assert!(validate_file_path("file.txt").is_ok());
        
        // Invalid paths (directory traversal)
        assert!(validate_file_path("../etc/passwd").is_err());
        assert!(validate_file_path("folder/../../../etc/passwd").is_err());
        assert!(validate_file_path("folder//file.txt").is_err());
        
        // Absolute paths
        assert!(validate_file_path("/etc/passwd").is_err());
        assert!(validate_file_path("C:/Windows/System32").is_err());
    }

    #[test]
    fn test_validate_filename() {
        // Valid filenames
        assert!(validate_filename("file.txt").is_ok());
        assert!(validate_filename("my-document.pdf").is_ok());
        
        // Invalid filenames
        assert!(validate_filename("folder/file.txt").is_err());
        assert!(validate_filename("").is_err());
        assert!(validate_filename("CON").is_err());
        assert!(validate_filename("AUX").is_err());
    }

    #[test]
    fn test_validate_username() {
        // Valid usernames
        assert!(validate_username("john_doe").is_ok());
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("test-user").is_ok());
        
        // Invalid usernames
        assert!(validate_username("ab").is_err()); // Too short
        assert!(validate_username("user@domain").is_err()); // Invalid char
        assert!(validate_username("a".repeat(33).as_str()).is_err()); // Too long
    }

    #[test]
    fn test_validate_email() {
        // Valid emails
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.user@domain.co.uk").is_ok());
        
        // Invalid emails
        assert!(validate_email("not-an-email").is_err());
        assert!(validate_email("@example.com").is_err());
        assert!(validate_email("user@").is_err());
    }

    #[test]
    fn test_validate_password_strength() {
        // Valid passwords
        assert!(validate_password_strength("SecurePass123!").is_ok());
        assert!(validate_password_strength("MyP@ssw0rd").is_ok());
        
        // Invalid passwords
        assert!(validate_password_strength("short").is_err()); // Too short
        assert!(validate_password_strength("alllowercase123!").is_err()); // No uppercase
        assert!(validate_password_strength("ALLUPPERCASE123!").is_err()); // No lowercase
        assert!(validate_password_strength("NoNumbers!").is_err()); // No digits
        assert!(validate_password_strength("NoSpecial123").is_err()); // No special chars
    }

    #[test]
    fn test_sanitize_html() {
        assert_eq!(
            sanitize_html("<script>alert('XSS')</script>"),
            "&lt;script&gt;alert(&#x27;XSS&#x27;)&lt;&#x2F;script&gt;"
        );
        assert_eq!(sanitize_html("Normal text"), "Normal text");
    }

    #[test]
    fn test_validate_search_query() {
        // Valid queries
        assert!(validate_search_query("normal search").is_ok());
        assert!(validate_search_query("file:*.txt size:>1MB").is_ok());
        
        // Invalid queries (SQL injection attempts)
        assert!(validate_search_query("'; DROP TABLE users; --").is_err());
        assert!(validate_search_query("' OR '1'='1").is_err());
        
        // Too long
        assert!(validate_search_query(&"a".repeat(501)).is_err());
    }
}
