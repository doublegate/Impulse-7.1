# Sprint 24: Phase 3 Integration & Testing

**Phase:** Phase 3 - Feature Completion
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 24 is the comprehensive integration and testing sprint for Phase 3, ensuring all advanced features work together seamlessly. Includes full system testing, security audit, and performance stress testing.

**Context:** Final sprint of Phase 3. Validates feature-complete BBS system.

**Expected Outcomes:** BBS passes rigorous testing and is ready for Phase 4 polish and documentation.

---

## Objectives

- [ ] Full system integration testing
- [ ] Comprehensive security audit
- [ ] Performance stress testing with 50+ concurrent users
- [ ] Critical and high-priority bug fixing

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| Integration test suite | Tests | All features tested together |
| Security audit report | Report | No critical vulnerabilities |
| Performance benchmarks | Report | Handles load gracefully |
| Bug fixes | Code | All critical/high bugs resolved |

---

## Detailed Tasks

### Task Category 1: Integration Testing

- [ ] **Task 1.1**: Complete user journeys
  - Estimated hours: 8

- [ ] **Task 1.2**: Test all feature combinations
  - Estimated hours: 12

- [ ] **Task 1.3**: File transfer integration
  - Estimated hours: 6

- [ ] **Task 1.4**: Door game integration
  - Estimated hours: 5

- [ ] **Task 1.5**: Admin functionality
  - Estimated hours: 5

### Task Category 2: Security Audit

- [ ] **Task 2.1**: Input validation review
  - Estimated hours: 8

- [ ] **Task 2.2**: Authentication security review
  - Estimated hours: 6

- [ ] **Task 2.3**: Rate limiting verification
  - Estimated hours: 4

- [ ] **Task 2.4**: File upload security
  - Estimated hours: 5

- [ ] **Task 2.5**: Penetration testing
  - Estimated hours: 8

### Task Category 3: Stress Testing

- [ ] **Task 3.1**: Simulate 50+ concurrent users
  - Estimated hours: 8

- [ ] **Task 3.2**: Measure resource usage
  - Estimated hours: 4

- [ ] **Task 3.3**: Identify bottlenecks
  - Estimated hours: 6

- [ ] **Task 3.4**: Long-duration testing (24 hours)
  - Estimated hours: 4

### Task Category 4: Bug Fixing

- [ ] **Task 4.1**: Fix all critical bugs
  - Estimated hours: 16

- [ ] **Task 4.2**: Fix all high-priority bugs
  - Estimated hours: 20

- [ ] **Task 4.3**: Defer low-priority issues
  - Estimated hours: 2

---

## Acceptance Criteria

- [ ] All major features work together
- [ ] No critical security vulnerabilities
- [ ] System handles 50+ concurrent users
- [ ] All tests passing
- [ ] Performance acceptable under load

---

## Phase 3 Milestone Achievement

Upon completion, Phase 3 delivers:
- ✅ Complete file transfer protocols (Zmodem, Xmodem, Ymodem)
- ✅ Theme system
- ✅ Door game interface
- ✅ Advanced messaging (QWK, network addressing)
- ✅ Full administration interface
- ✅ Feature-complete BBS system

**Next Phase**: Phase 4 - Polish & Launch (Sprints 25-32)

---

## Technical Details

### Architecture Considerations

- **Test Pyramid Strategy**: Unit tests (foundation), integration tests (feature verification), end-to-end tests (user journeys)
- **Test Fixtures**: Shared test database, mock file systems, simulated network connections
- **Test Isolation**: Each test gets clean database state via transactions or migrations
- **Parallel Test Execution**: Tokio test runtime with proper isolation
- **Load Generation**: Simulate realistic user behavior patterns
- **Performance Profiling**: CPU, memory, I/O metrics collection
- **Security Testing**: Automated fuzzing, penetration testing, vulnerability scanning

### Dependencies

**Crate-Level Dependencies:**
```toml
[dev-dependencies]
tokio-test = "0.4"
proptest = "1.4"
criterion = "0.5"
mockall = "0.12"
wiremock = "0.6"
fake = "2.9"
sqlx = { workspace = true, features = ["runtime-tokio", "postgres", "migrate"] }
tempfile = "3.8"
tracing-subscriber = { workspace = true, features = ["env-filter"] }
```

**External Testing Tools:**
- **wrk** or **k6**: HTTP load testing
- **Artillery**: Advanced load testing scenarios
- **OWASP ZAP**: Security vulnerability scanning
- **cargo-audit**: Dependency vulnerability checking
- **cargo-fuzz**: Fuzzing for input validation

**Pascal Units Being Tested:**
- All Phase 3 features: PROTOCOL.PAS, THEME.PAS, DOORS.PAS, MESSAGES.PAS, ADMIN.PAS
- Cross-feature integration: Login → messaging → file transfer → door games
- System-level: Multi-node support, concurrent access, resource cleanup

### Code Examples

**Integration Test Framework Setup:**
```rust
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use tokio::sync::RwLock;
use tempfile::TempDir;

/// Test fixture providing clean BBS environment for each test
pub struct BbsTestFixture {
    pub db: PgPool,
    pub temp_dir: TempDir,
    pub config: BbsConfig,
    pub user_factory: UserFactory,
}

impl BbsTestFixture {
    /// Create new test fixture with isolated database
    pub async fn new() -> anyhow::Result<Self> {
        // Create temporary directory for file storage
        let temp_dir = TempDir::new()?;

        // Setup test database with migrations
        let db_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/impulse_test".to_string());

        let db = PgPool::connect(&db_url).await?;

        // Run migrations to clean state
        sqlx::migrate!("./migrations").run(&db).await?;

        // Configure test BBS instance
        let config = BbsConfig {
            data_dir: temp_dir.path().to_path_buf(),
            max_nodes: 10,
            max_login_attempts: 3,
            session_timeout_minutes: 30,
            ..Default::default()
        };

        let user_factory = UserFactory::new(db.clone());

        Ok(Self {
            db,
            temp_dir,
            config,
            user_factory,
        })
    }

    /// Start BBS server for integration testing
    pub async fn start_server(&self) -> anyhow::Result<BbsServer> {
        let server = BbsServer::new(self.config.clone(), self.db.clone()).await?;
        server.start().await?;
        Ok(server)
    }

    /// Create test user with specific permissions
    pub async fn create_test_user(
        &self,
        username: &str,
        security_level: u8,
    ) -> anyhow::Result<User> {
        self.user_factory.create_user(username, security_level).await
    }

    /// Cleanup test data
    pub async fn cleanup(&self) -> anyhow::Result<()> {
        sqlx::query!("TRUNCATE TABLE users, messages, files, sessions CASCADE")
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

/// Factory for creating test users with various configurations
pub struct UserFactory {
    db: PgPool,
}

impl UserFactory {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create_user(
        &self,
        username: &str,
        security_level: u8,
    ) -> anyhow::Result<User> {
        use fake::{Fake, Faker};

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password_hash, email, security_level,
                               location, phone, birthdate)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            username,
            "$argon2id$v=19$m=19456,t=2,p=1$test$test", // Test hash
            format!("{}@test.local", username),
            security_level as i16,
            "Test Location",
            "555-0000",
            chrono::NaiveDate::from_ymd_opt(1990, 1, 1),
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    pub async fn create_sysop(&self) -> anyhow::Result<User> {
        self.create_user("sysop", 255).await
    }

    pub async fn create_regular_user(&self) -> anyhow::Result<User> {
        self.create_user(&format!("user_{}", Faker.fake::<u32>()), 10).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixture_isolation() {
        let fixture1 = BbsTestFixture::new().await.unwrap();
        let fixture2 = BbsTestFixture::new().await.unwrap();

        let user1 = fixture1.create_test_user("test1", 10).await.unwrap();
        let user2 = fixture2.create_test_user("test2", 10).await.unwrap();

        // Verify fixtures are isolated
        assert_ne!(user1.id, user2.id);

        fixture1.cleanup().await.unwrap();
        fixture2.cleanup().await.unwrap();
    }
}
```

**End-to-End User Journey Testing:**
```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Simulates complete user journey through BBS
pub struct UserJourneyTest {
    fixture: Arc<BbsTestFixture>,
}

impl UserJourneyTest {
    pub async fn new() -> anyhlo::Result<Self> {
        let fixture = Arc::new(BbsTestFixture::new().await?);
        Ok(Self { fixture })
    }

    /// Test: New user registration → login → read messages → upload file → play door game
    pub async fn test_complete_user_journey(&self) -> anyhow::Result<()> {
        let server = self.fixture.start_server().await?;

        // Connect to BBS
        let mut stream = TcpStream::connect("127.0.0.1:2323").await?;

        // Step 1: Register new user
        self.send_and_expect(
            &mut stream,
            "N",  // New user
            "Enter desired username:"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "testuser",
            "Enter password:"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "SecurePass123!",
            "Re-enter password:"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "SecurePass123!",
            "Registration complete"
        ).await?;

        // Step 2: Navigate to message area
        self.send_and_expect(
            &mut stream,
            "M",  // Messages
            "Message Areas"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "1",  // Select first message area
            "Messages 1-"
        ).await?;

        // Step 3: Post a message
        self.send_and_expect(
            &mut stream,
            "P",  // Post message
            "To (username):"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "All",
            "Subject:"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "Test Message",
            "Enter message text:"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "This is a test message.\r\n/S",  // Save
            "Message posted"
        ).await?;

        // Step 4: Upload a file
        self.send_and_expect(
            &mut stream,
            "F",  // Files
            "File Areas"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "U",  // Upload
            "Select protocol:"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "Z",  // Zmodem
            "Ready to receive"
        ).await?;

        // Simulate Zmodem upload
        let test_file_data = b"Test file content";
        self.send_zmodem_file(&mut stream, "test.txt", test_file_data).await?;

        // Step 5: Play door game
        self.send_and_expect(
            &mut stream,
            "D",  // Doors
            "Available Doors"
        ).await?;

        self.send_and_expect(
            &mut stream,
            "1",  // Select first door
            "Starting door..."
        ).await?;

        // Wait for door to load
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Exit door
        self.send_and_expect(
            &mut stream,
            "Q",  // Quit door
            "Returning to BBS"
        ).await?;

        // Step 6: Logout
        self.send_and_expect(
            &mut stream,
            "G",  // Goodbye
            "Thank you for calling"
        ).await?;

        // Verify user stats were updated in database
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            "testuser"
        )
        .fetch_one(&self.fixture.db)
        .await?;

        assert_eq!(user.login_count, 1);
        assert!(user.last_login.is_some());
        assert_eq!(user.total_uploads, 1);

        server.shutdown().await?;
        Ok(())
    }

    async fn send_and_expect(
        &self,
        stream: &mut TcpStream,
        input: &str,
        expected_output: &str,
    ) -> anyhow::Result<()> {
        // Send input
        stream.write_all(input.as_bytes()).await?;
        stream.write_all(b"\r\n").await?;

        // Read response
        let mut buffer = vec![0u8; 4096];
        let n = stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);

        // Verify expected output
        if !response.contains(expected_output) {
            anyhow::bail!(
                "Expected '{}' in response, got: {}",
                expected_output,
                response
            );
        }

        Ok(())
    }

    async fn send_zmodem_file(
        &self,
        stream: &mut TcpStream,
        filename: &str,
        data: &[u8],
    ) -> anyhow::Result<()> {
        // Simplified Zmodem protocol for testing
        // In real implementation, would use full Zmodem state machine

        // Send ZRQINIT header
        stream.write_all(b"**\x18B00000000000000\r\n").await?;

        // Send file header
        let file_header = format!("**\x18B0100000023be50\r\n{}\0", filename);
        stream.write_all(file_header.as_bytes()).await?;

        // Send file data in 1024-byte blocks
        stream.write_all(data).await?;

        // Send EOF
        stream.write_all(b"**\x18B0800000000be50\r\n").await?;

        Ok(())
    }
}

#[cfg(test)]
mod journey_tests {
    use super::*;

    #[tokio::test]
    async fn complete_user_journey() {
        let test = UserJourneyTest::new().await.unwrap();
        test.test_complete_user_journey().await.unwrap();
    }
}
```

**Concurrent User Stress Testing:**
```rust
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::task::JoinSet;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Load generator for stress testing concurrent users
pub struct LoadGenerator {
    fixture: Arc<BbsTestFixture>,
    metrics: Arc<LoadMetrics>,
}

#[derive(Default)]
pub struct LoadMetrics {
    pub total_connections: AtomicU64,
    pub successful_logins: AtomicU64,
    pub failed_logins: AtomicU64,
    pub messages_posted: AtomicU64,
    pub files_uploaded: AtomicU64,
    pub total_latency_ms: AtomicU64,
    pub errors: AtomicU64,
}

impl LoadMetrics {
    pub fn print_summary(&self) {
        let total = self.total_connections.load(Ordering::Relaxed);
        let success = self.successful_logins.load(Ordering::Relaxed);
        let failed = self.failed_logins.load(Ordering::Relaxed);
        let messages = self.messages_posted.load(Ordering::Relaxed);
        let files = self.files_uploaded.load(Ordering::Relaxed);
        let latency = self.total_latency_ms.load(Ordering::Relaxed);
        let errors = self.errors.load(Ordering::Relaxed);

        println!("Load Test Results:");
        println!("  Total Connections: {}", total);
        println!("  Successful Logins: {} ({:.2}%)", success, (success as f64 / total as f64) * 100.0);
        println!("  Failed Logins: {}", failed);
        println!("  Messages Posted: {}", messages);
        println!("  Files Uploaded: {}", files);
        println!("  Average Latency: {} ms", latency / total.max(1));
        println!("  Errors: {}", errors);
    }
}

impl LoadGenerator {
    pub async fn new() -> anyhow::Result<Self> {
        let fixture = Arc::new(BbsTestFixture::new().await?);
        let metrics = Arc::new(LoadMetrics::default());
        Ok(Self { fixture, metrics })
    }

    /// Simulate N concurrent users for duration
    pub async fn run_load_test(
        &self,
        num_users: usize,
        duration_secs: u64,
    ) -> anyhow::Result<()> {
        let server = self.fixture.start_server().await?;

        // Create users
        let mut users = Vec::new();
        for i in 0..num_users {
            let user = self.fixture
                .create_test_user(&format!("loadtest_{}", i), 10)
                .await?;
            users.push(user);
        }

        // Spawn user simulation tasks
        let mut tasks = JoinSet::new();

        for user in users {
            let metrics = Arc::clone(&self.metrics);
            let duration = duration_secs;

            tasks.spawn(async move {
                Self::simulate_user(user, metrics, duration).await
            });
        }

        // Wait for all tasks to complete
        while let Some(result) = tasks.join_next().await {
            if let Err(e) = result {
                eprintln!("Task error: {}", e);
                self.metrics.errors.fetch_add(1, Ordering::Relaxed);
            }
        }

        server.shutdown().await?;
        self.metrics.print_summary();

        Ok(())
    }

    /// Simulate single user behavior
    async fn simulate_user(
        user: User,
        metrics: Arc<LoadMetrics>,
        duration_secs: u64,
    ) -> anyhow::Result<()> {
        let start_time = std::time::Instant::now();
        let end_time = start_time + std::time::Duration::from_secs(duration_secs);

        while std::time::Instant::now() < end_time {
            metrics.total_connections.fetch_add(1, Ordering::Relaxed);

            // Connect and login
            let login_start = std::time::Instant::now();
            let mut stream = match TcpStream::connect("127.0.0.1:2323").await {
                Ok(s) => s,
                Err(_) => {
                    metrics.failed_logins.fetch_add(1, Ordering::Relaxed);
                    continue;
                }
            };

            // Send credentials
            stream.write_all(user.username.as_bytes()).await?;
            stream.write_all(b"\r\n").await?;
            stream.write_all(b"password\r\n").await?;

            let mut buffer = vec![0u8; 1024];
            let n = stream.read(&mut buffer).await?;

            if String::from_utf8_lossy(&buffer[..n]).contains("Welcome") {
                metrics.successful_logins.fetch_add(1, Ordering::Relaxed);
            } else {
                metrics.failed_logins.fetch_add(1, Ordering::Relaxed);
                continue;
            }

            // Random user actions
            let action = rand::random::<u8>() % 3;

            match action {
                0 => {
                    // Post message
                    stream.write_all(b"M\r\n1\r\nP\r\n").await?;
                    stream.write_all(b"All\r\nTest\r\nMessage\r\n/S\r\n").await?;
                    metrics.messages_posted.fetch_add(1, Ordering::Relaxed);
                }
                1 => {
                    // Browse files
                    stream.write_all(b"F\r\n1\r\nL\r\n").await?;
                }
                2 => {
                    // Check who's online
                    stream.write_all(b"W\r\n").await?;
                }
                _ => {}
            }

            // Logout
            stream.write_all(b"G\r\n").await?;

            // Record latency
            let latency = login_start.elapsed().as_millis() as u64;
            metrics.total_latency_ms.fetch_add(latency, Ordering::Relaxed);

            // Random think time (100-500ms)
            let think_time = 100 + (rand::random::<u64>() % 400);
            tokio::time::sleep(tokio::time::Duration::from_millis(think_time)).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;

    #[tokio::test]
    async fn stress_test_50_concurrent_users() {
        let generator = LoadGenerator::new().await.unwrap();
        generator.run_load_test(50, 60).await.unwrap();

        // Verify metrics
        assert!(generator.metrics.total_connections.load(Ordering::Relaxed) > 0);
        assert!(generator.metrics.successful_logins.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    async fn endurance_test_24_hours() {
        // Reduced to 1 hour for CI/CD
        let generator = LoadGenerator::new().await.unwrap();
        generator.run_load_test(10, 3600).await.unwrap();
    }
}

// Criterion benchmarks for performance profiling
fn benchmark_login_throughput(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("login_throughput", |b| {
        b.to_async(&runtime).iter(|| async {
            let generator = LoadGenerator::new().await.unwrap();
            generator.run_load_test(black_box(10), black_box(5)).await.unwrap();
        });
    });
}

criterion_group!(benches, benchmark_login_throughput);
criterion_main!(benches);
```

**Security Audit Test Suite:**
```rust
use proptest::prelude::*;

/// Security testing framework for input validation and attack prevention
pub struct SecurityAuditSuite {
    fixture: Arc<BbsTestFixture>,
}

impl SecurityAuditSuite {
    pub async fn new() -> anyhow::Result<Self> {
        let fixture = Arc::new(BbsTestFixture::new().await?);
        Ok(Self { fixture })
    }

    /// Test SQL injection vulnerabilities
    pub async fn test_sql_injection(&self) -> anyhow::Result<()> {
        let server = self.fixture.start_server().await?;

        // Common SQL injection payloads
        let payloads = vec![
            "' OR '1'='1",
            "'; DROP TABLE users; --",
            "admin'--",
            "1' UNION SELECT NULL, NULL, NULL--",
            "' OR 1=1 LIMIT 1 OFFSET 1--",
        ];

        for payload in payloads {
            let mut stream = TcpStream::connect("127.0.0.1:2323").await?;

            // Try username injection
            stream.write_all(payload.as_bytes()).await?;
            stream.write_all(b"\r\n").await?;
            stream.write_all(b"password\r\n").await?;

            let mut buffer = vec![0u8; 1024];
            let n = stream.read(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer[..n]);

            // Verify no SQL errors leaked
            assert!(!response.contains("SQL"));
            assert!(!response.contains("syntax error"));
            assert!(!response.contains("Welcome")); // Should not login
        }

        server.shutdown().await?;
        Ok(())
    }

    /// Test rate limiting on login attempts
    pub async fn test_rate_limiting(&self) -> anyhow::Result<()> {
        let server = self.fixture.start_server().await?;

        // Attempt 100 rapid logins
        let mut stream = TcpStream::connect("127.0.0.1:2323").await?;

        for i in 0..100 {
            stream.write_all(b"testuser\r\n").await?;
            stream.write_all(b"wrongpassword\r\n").await?;

            let mut buffer = vec![0u8; 1024];
            let n = stream.read(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer[..n]);

            // After 3 attempts, should be rate limited
            if i >= 3 {
                assert!(response.contains("Too many attempts")
                    || response.contains("Rate limit"));
            }
        }

        server.shutdown().await?;
        Ok(())
    }

    /// Test file upload security (path traversal, malware)
    pub async fn test_file_upload_security(&self) -> anyhow::Result<()> {
        let server = self.fixture.start_server().await?;
        let user = self.fixture.create_test_user("uploader", 10).await?;

        // Test path traversal attempts
        let malicious_filenames = vec![
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32\\config\\sam",
            "./../sensitive.dat",
            "../../backdoor.exe",
        ];

        for filename in malicious_filenames {
            let result = self.attempt_file_upload(filename, b"malicious").await;

            // Upload should be rejected
            assert!(result.is_err() ||
                result.unwrap().contains("Invalid filename"));
        }

        // Test EICAR malware test string
        let eicar = b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";
        let result = self.attempt_file_upload("test.exe", eicar).await;

        // Should be detected by virus scanner
        assert!(result.is_err() ||
            result.unwrap().contains("Virus detected"));

        server.shutdown().await?;
        Ok(())
    }

    /// Property-based testing for input validation
    pub async fn test_input_validation_properties(&self) -> anyhow::Result<()> {
        proptest!(|(
            username in "[a-zA-Z0-9_]{1,20}",
            password in ".{8,128}",
            message_subject in ".{1,80}",
        )| {
            // Valid inputs should be accepted
            assert!(validate_username(&username).is_ok());
            assert!(validate_password(&password).is_ok());
            assert!(validate_subject(&message_subject).is_ok());
        });

        proptest!(|(
            username in ".*",
            password in ".*",
        )| {
            // All inputs should be sanitized (no crashes)
            let _ = sanitize_input(&username);
            let _ = sanitize_input(&password);
        });

        Ok(())
    }

    /// Test authentication token security
    pub async fn test_session_token_security(&self) -> anyhow::Result<()> {
        let server = self.fixture.start_server().await?;
        let user = self.fixture.create_test_user("tokentest", 10).await?;

        // Login and get session token
        let mut stream = TcpStream::connect("127.0.0.1:2323").await?;
        stream.write_all(b"tokentest\r\npassword\r\n").await?;

        // Extract session token from response
        let mut buffer = vec![0u8; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);

        // Verify token properties:
        // - Should be cryptographically random
        // - Should be at least 128 bits
        // - Should not be predictable

        if let Some(token) = extract_session_token(&response) {
            assert!(token.len() >= 32); // At least 128 bits base64
            assert!(is_cryptographically_random(&token));
        }

        server.shutdown().await?;
        Ok(())
    }

    async fn attempt_file_upload(
        &self,
        filename: &str,
        data: &[u8],
    ) -> anyhow::Result<String> {
        let mut stream = TcpStream::connect("127.0.0.1:2323").await?;

        // Login
        stream.write_all(b"uploader\r\npassword\r\n").await?;

        // Navigate to file upload
        stream.write_all(b"F\r\nU\r\nZ\r\n").await?;

        // Send filename
        stream.write_all(filename.as_bytes()).await?;
        stream.write_all(b"\r\n").await?;

        // Send data
        stream.write_all(data).await?;

        // Read response
        let mut buffer = vec![0u8; 4096];
        let n = stream.read(&mut buffer).await?;

        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }
}

fn validate_username(username: &str) -> anyhow::Result<()> {
    if username.len() < 3 || username.len() > 20 {
        anyhow::bail!("Invalid length");
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        anyhow::bail!("Invalid characters");
    }
    Ok(())
}

fn validate_password(password: &str) -> anyhow::Result<()> {
    if password.len() < 8 {
        anyhow::bail!("Too short");
    }
    Ok(())
}

fn validate_subject(subject: &str) -> anyhow::Result<()> {
    if subject.len() > 80 {
        anyhow::bail!("Too long");
    }
    Ok(())
}

fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .take(1024)
        .collect()
}

fn extract_session_token(response: &str) -> Option<String> {
    // Extract token from response (implementation specific)
    response.lines()
        .find(|line| line.contains("Session:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .map(|s| s.to_string())
}

fn is_cryptographically_random(token: &str) -> bool {
    // Basic entropy check
    let unique_chars: std::collections::HashSet<_> = token.chars().collect();
    unique_chars.len() > token.len() / 2
}

#[cfg(test)]
mod security_tests {
    use super::*;

    #[tokio::test]
    async fn sql_injection_prevention() {
        let suite = SecurityAuditSuite::new().await.unwrap();
        suite.test_sql_injection().await.unwrap();
    }

    #[tokio::test]
    async fn rate_limiting_enforcement() {
        let suite = SecurityAuditSuite::new().await.unwrap();
        suite.test_rate_limiting().await.unwrap();
    }

    #[tokio::test]
    async fn file_upload_validation() {
        let suite = SecurityAuditSuite::new().await.unwrap();
        suite.test_file_upload_security().await.unwrap();
    }

    #[tokio::test]
    async fn session_token_randomness() {
        let suite = SecurityAuditSuite::new().await.unwrap();
        suite.test_session_token_security().await.unwrap();
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 06**: User system provides authentication and session management
- **Sprint 19**: Protocol implementation for file transfer testing
- **Sprint 20**: Theme system for UI testing
- **Sprint 21**: Door games for integration testing
- **Sprint 22**: Advanced messaging for QWK packet testing
- **Sprint 23**: Admin interface for management testing

### Blocks Downstream
- **Sprint 25**: Performance optimization (uses stress test results)
- **Sprint 26-27**: Documentation (uses test coverage reports)
- **Sprint 30**: Beta testing (requires passing integration tests)

---

## Testing Requirements

### Unit Tests
- [ ] Test fixture isolation
- [ ] User factory creates valid users
- [ ] Load metrics calculation accuracy
- [ ] Input validation functions

### Integration Tests
- [ ] Complete user journey (registration → logout)
- [ ] Multi-user concurrent access
- [ ] File transfer protocol integration
- [ ] Door game launch and cleanup
- [ ] Admin operations with audit logging
- [ ] Message posting and retrieval
- [ ] Theme switching during session

### Security Tests
- [ ] SQL injection prevention across all inputs
- [ ] XSS prevention in message display
- [ ] CSRF protection on admin actions
- [ ] Rate limiting enforcement (login, API, uploads)
- [ ] File upload validation (path traversal, malware)
- [ ] Session token security (randomness, expiration)
- [ ] Password hashing verification (Argon2id)
- [ ] Authorization checks (security level enforcement)

### Performance Tests
- [ ] 50+ concurrent users sustained for 5 minutes
- [ ] Response time < 100ms under load
- [ ] Memory usage stable over 24 hours
- [ ] Database connection pool efficiency
- [ ] File system I/O performance
- [ ] CPU utilization under load < 80%

### Load Test Scenarios
- [ ] Gradual ramp-up: 1 → 50 users over 5 minutes
- [ ] Sustained load: 50 users for 30 minutes
- [ ] Spike test: 0 → 100 users instantly
- [ ] Soak test: 10 users for 24 hours
- [ ] Stress test: Increase until failure point

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use `tokio-test` for async test harness
- Isolate tests with database transactions or test databases
- Generate realistic test data with `fake` crate
- Property-based testing with `proptest` for input validation
- Load testing with custom async generator (not HTTP-based wrk/k6)
- Security testing with OWASP ZAP for automated vulnerability scanning
- Benchmark critical paths with `criterion`
- Maintain >80% code coverage for all features

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Integration tests may be flaky due to timing issues
- **Mitigation**: Use deterministic timeouts; retry transient failures; isolate network I/O
- **Risk**: Load tests may overwhelm development machines
- **Mitigation**: Run on dedicated test infrastructure; limit concurrent users in CI/CD
- **Risk**: Security vulnerabilities may be discovered late
- **Mitigation**: Run automated security scans daily; manual penetration testing; bug bounty program
- **Risk**: Performance bottlenecks may require architectural changes
- **Mitigation**: Profile early; identify critical paths; optimize before Phase 4

---

## Progress Log

### Week 1
- *Date*: Progress notes will be added here as sprint progresses

### Week 2
- *Date*: Progress notes will be added here as sprint progresses

### Week 3
- *Date*: Progress notes will be added here as sprint progresses

### Sprint Completion
- **Completed**: TBD
- **Velocity**: TBD
- **Burndown**: TBD
