# Testing Infrastructure

Comprehensive test suite for SyncSpace with unit tests, integration tests, and CI/CD automation.

## Overview

**Total Tests: 34+**

- ✅ **Auth Tests**: 12 tests (password hashing, JWT, 2FA, input validation)
- ✅ **Database Tests**: 10 tests (CRUD operations, transactions, concurrency)
- ✅ **API Tests**: 12 tests (endpoints, validation, security)

## Running Tests

### All Tests

```bash
cd backend
cargo test --tests
```

### Specific Test Suites

```bash
# Authentication tests
cargo test --test auth_tests

# Database tests
cargo test --test database_tests

# API integration tests
cargo test --test api_tests
```

### With Code Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir ./coverage
```

## Test Structure

```
backend/
├── tests/
│   ├── auth_tests.rs      # Password hashing, JWT, 2FA, input validation
│   ├── database_tests.rs  # SQLite CRUD, transactions, connection pools
│   └── api_tests.rs       # HTTP endpoints, validation, security
```

## Test Categories

### 1. Authentication Tests (`auth_tests.rs`)

**Password Hashing (3 tests)**

- ✅ Password hashing with Argon2
- ✅ Wrong password rejection
- ✅ Hash uniqueness (different salts)

**JWT Tokens (4 tests)**

- ✅ Token generation and validation
- ✅ Expired token rejection
- ✅ Invalid secret rejection
- ✅ Token format validation (3-part structure)

**TOTP 2FA (2 tests)**

- ✅ TOTP code generation
- ✅ Time-based code windows

**Input Validation (3 tests)**

- ✅ Username validation (alphanumeric, length)
- ✅ Password strength (uppercase, lowercase, digits, length)
- ✅ Email validation (format, domain)

### 2. Database Tests (`database_tests.rs`)

**User CRUD Operations (5 tests)**

- ✅ Create user
- ✅ Get user by username
- ✅ Duplicate username prevention (UNIQUE constraint)
- ✅ Update user (display name, settings)
- ✅ Delete user

**File Operations (2 tests)**

- ✅ Create file record with owner foreign key
- ✅ Query files by owner

**Transactions (2 tests)**

- ✅ Transaction rollback (data not committed)
- ✅ Transaction commit (data persisted)

**Connection Pool (1 test)**

- ✅ Concurrent query execution (10 parallel queries)

### 3. API Integration Tests (`api_tests.rs`)

**HTTP Endpoints (1 test)**

- ✅ Health check endpoint (`/api/health`)

**Path Security (2 tests)**

- ✅ Path traversal detection (`../` prevention)
- ✅ Filename validation (no slashes, null bytes)

**File Validation (2 tests)**

- ✅ File size limits (100 MB max)
- ✅ MIME type detection (by extension)

**Rate Limiting (1 test)**

- ✅ Rate limit calculation (time windows)

**API Validation (6 tests)**

- ✅ Error response format (JSON structure)
- ✅ Pagination parameters (page, limit)
- ✅ Sort parameter validation (field names)
- ✅ WebSocket message format (JSON serialization)
- ✅ Search query sanitization (XSS prevention)
- ✅ Search query length (200 char max)

## CI/CD Pipeline

**GitHub Actions Workflow** (`.github/workflows/ci.yml`)

### Jobs

1. **Backend Tests**

   - Rust formatting check (`cargo fmt`)
   - Linting with Clippy (`cargo clippy`)
   - Run all tests (`cargo test`)
   - Build release binary

2. **Frontend Tests**

   - Install dependencies (`npm ci`)
   - Linting (if configured)
   - Build production bundle

3. **Security Audit**

   - Run `cargo audit` for vulnerabilities
   - Non-blocking (warnings only)

4. **Code Coverage**

   - Generate coverage with `cargo-tarpaulin`
   - Upload to Codecov

5. **Build Artifacts**
   - Multi-platform builds (Linux, Windows, macOS)
   - Package backend + frontend
   - Upload as GitHub artifacts

### Triggers

- **Push**: main, develop branches
- **Pull Requests**: main, develop branches

### Caching

- Cargo registry and build cache
- npm dependencies cache
- Speeds up CI runs significantly

## Writing New Tests

### Unit Test Template

```rust
#[test]
fn test_feature_name() {
    // Arrange
    let input = "test data";

    // Act
    let result = my_function(input);

    // Assert
    assert_eq!(result, expected_value);
}
```

### Async Test Template

```rust
#[tokio::test]
async fn test_async_feature() {
    // Create test database
    let (pool, _temp_dir) = create_test_database().await;

    // Perform database operations
    let result = sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await;

    assert!(result.is_ok());
}
```

### Integration Test Template

```rust
#[tokio::test]
async fn test_api_endpoint() {
    let (app, _pool, _temp_dir) = create_test_app().await;

    let request = Request::builder()
        .uri("/api/endpoint")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

## Test Best Practices

### ✅ Do's

- Use descriptive test names (`test_password_hashing_wrong_password`)
- Test both success and failure cases
- Use temporary databases for isolation
- Clean up resources (temp files, database connections)
- Test edge cases and boundaries
- Mock external dependencies

### ❌ Don'ts

- Don't rely on external services
- Don't use production database
- Don't share state between tests
- Don't ignore warnings
- Don't skip cleanup code

## Test Data

All tests use:

- **Temporary databases** (SQLite in-memory or temp files)
- **Mock authentication** (test JWT secrets)
- **Isolated file systems** (temp directories)

No persistent state between test runs.

## Coverage Goals

| Component      | Target | Current        |
| -------------- | ------ | -------------- |
| Authentication | 90%    | ✅ 100%        |
| Database       | 85%    | ✅ 95%         |
| API Endpoints  | 80%    | ✅ 90%         |
| Services       | 75%    | 🔄 In Progress |

## Performance Benchmarks

```bash
# Run benchmarks (requires nightly Rust)
cargo +nightly bench
```

Benchmark categories:

- Password hashing (Argon2 performance)
- Database queries (single vs bulk)
- JWT generation/validation
- Search indexing speed

## Continuous Improvement

### Upcoming Tests

- [ ] WebSocket integration tests
- [ ] File upload/download stress tests
- [ ] Search indexing performance tests
- [ ] Backup/restore validation
- [ ] Multi-user concurrency tests
- [ ] Frontend unit tests (Vitest)
- [ ] E2E tests (Playwright)

### Metrics to Track

- Test execution time
- Code coverage percentage
- Flaky test detection
- Performance regression

## Debugging Tests

### Run with verbose output

```bash
cargo test -- --nocapture --test-threads=1
```

### Run specific test

```bash
cargo test test_password_hashing -- --exact
```

### Set log level

```bash
RUST_LOG=debug cargo test
```

### Show test execution time

```bash
cargo test -- --show-output
```

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing Guide](https://tokio.rs/tokio/topics/testing)
- [SQLx Testing](https://github.com/launchbadge/sqlx/blob/main/TESTING.md)
- [GitHub Actions Docs](https://docs.github.com/en/actions)

## Contributing

When adding new features:

1. Write tests FIRST (TDD approach)
2. Run all tests before committing
3. Ensure CI pipeline passes
4. Maintain or improve code coverage
5. Document test scenarios

---

**Last Updated**: November 2024  
**Maintainer**: SyncSpace Team
