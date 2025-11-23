# Sprint 09: User Authentication System

**Phase:** Phase 2 - Core Features
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 09 implements the complete user authentication system including password hashing with Argon2, login/logout flows, new user registration, and session security. This sprint transforms the basic session framework from Phase 1 into a fully authenticated system where users can securely create accounts and log in.

**Context:** This is the first sprint of Phase 2 (Core Features). Authentication is the gateway to all user-specific features.

**Expected Outcomes:** By the end of this sprint, the BBS will have a production-ready authentication system with secure password storage, rate limiting, and session management.

---

## Objectives

- [ ] Implement password hashing with Argon2id
- [ ] Create login and logout user flows
- [ ] Add new user registration system
- [ ] Implement session token management and security

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-user` crate with AuthService | Code | Authentication service with password hashing and verification |
| Login screen implementation | Code | ANSI login prompt with username/password input |
| New user registration | Code | Interactive sign-up form with validation |
| Session security | Code | Session tokens, timeout handling, rate limiting |

---

## Detailed Tasks

### Task Category 1: Authentication Service Implementation

- [ ] **Task 1.1**: Implement Argon2id password hashing
  - Implementation notes: Use argon2 crate, configure work factors appropriately
  - Files affected: `crates/impulse-user/src/auth/hash.rs`
  - Estimated hours: 3

- [ ] **Task 1.2**: Create verify_password function
  - Implementation notes: Compare password with stored hash, timing-safe comparison
  - Files affected: `crates/impulse-user/src/auth/hash.rs`
  - Estimated hours: 2

- [ ] **Task 1.3**: Implement rate limiting for failed attempts
  - Implementation notes: Track failed logins by IP/username, exponential backoff
  - Files affected: `crates/impulse-user/src/auth/rate_limit.rs`
  - Estimated hours: 4

- [ ] **Task 1.4**: Add account lockout mechanism
  - Implementation notes: Lock account after N failed attempts, unlock after time period
  - Files affected: `crates/impulse-user/src/auth/lockout.rs`
  - Estimated hours: 3

### Task Category 2: Login Screen Implementation

- [ ] **Task 2.1**: Design login ANSI screen
  - Implementation notes: Welcome banner, username/password prompts, error messages
  - Files affected: `assets/screens/login.ans`, `crates/impulse-user/src/screens/login.rs`
  - Estimated hours: 4

- [ ] **Task 2.2**: Implement username input handling
  - Implementation notes: Read input, validate format, check existence
  - Files affected: `crates/impulse-user/src/screens/login.rs`
  - Estimated hours: 3

- [ ] **Task 2.3**: Implement password input (masked)
  - Implementation notes: Display asterisks, no echo, handle backspace
  - Files affected: `crates/impulse-user/src/screens/login.rs`
  - Estimated hours: 4

- [ ] **Task 2.4**: Display login errors clearly
  - Implementation notes: "Invalid credentials", "Account locked", rate limit messages
  - Files affected: `crates/impulse-user/src/screens/login.rs`
  - Estimated hours: 2

- [ ] **Task 2.5**: Handle successful login flow
  - Implementation notes: Create session, load user profile, redirect to main menu
  - Files affected: `crates/impulse-user/src/screens/login.rs`
  - Estimated hours: 3

### Task Category 3: New User Registration

- [ ] **Task 3.1**: Create registration ANSI screen
  - Implementation notes: Sign-up form, terms of service, field labels
  - Files affected: `assets/screens/register.ans`, `crates/impulse-user/src/screens/register.rs`
  - Estimated hours: 4

- [ ] **Task 3.2**: Implement interactive sign-up form
  - Implementation notes: Username, password, password confirm, email (optional), location
  - Files affected: `crates/impulse-user/src/screens/register.rs`
  - Estimated hours: 5

- [ ] **Task 3.3**: Add validation logic
  - Implementation notes: Username uniqueness, password strength, email format
  - Files affected: `crates/impulse-user/src/validation.rs`
  - Estimated hours: 4

- [ ] **Task 3.4**: Implement password strength requirements
  - Implementation notes: Min length, complexity requirements, common password check
  - Files affected: `crates/impulse-user/src/validation.rs`
  - Estimated hours: 3

- [ ] **Task 3.5**: Add email verification (optional)
  - Implementation notes: Send verification code, validate on first login (placeholder for now)
  - Files affected: `crates/impulse-user/src/verification.rs`
  - Estimated hours: 3

### Task Category 4: Session Security

- [ ] **Task 4.1**: Generate secure session tokens
  - Implementation notes: Random tokens using cryptographic RNG
  - Files affected: `crates/impulse-user/src/session/tokens.rs`
  - Estimated hours: 2

- [ ] **Task 4.2**: Store active sessions in database
  - Implementation notes: Session table with user_id, token, IP, timestamps
  - Files affected: `crates/impulse-storage/src/sessions.rs`
  - Estimated hours: 3

- [ ] **Task 4.3**: Implement session timeout
  - Implementation notes: Expire sessions after configured idle time (default 30 min)
  - Files affected: `crates/impulse-user/src/session/timeout.rs`
  - Estimated hours: 3

- [ ] **Task 4.4**: Add session renewal on activity
  - Implementation notes: Update last_activity timestamp on user actions
  - Files affected: `crates/impulse-user/src/session/renewal.rs`
  - Estimated hours: 2

- [ ] **Task 4.5**: Implement logout functionality
  - Implementation notes: Invalidate session token, clear session data, show goodbye message
  - Files affected: `crates/impulse-user/src/screens/logout.rs`
  - Estimated hours: 2

### Task Category 5: Testing and Security

- [ ] **Task 5.1**: Test password hashing performance
  - Implementation notes: Benchmark Argon2 parameters, ensure acceptable latency
  - Files affected: `benches/password_hash_bench.rs`
  - Estimated hours: 2

- [ ] **Task 5.2**: Test rate limiting effectiveness
  - Implementation notes: Simulate brute force attacks, verify lockout
  - Files affected: `tests/rate_limit_test.rs`
  - Estimated hours: 3

- [ ] **Task 5.3**: Test session security
  - Implementation notes: Token generation randomness, timeout enforcement
  - Files affected: `tests/session_security_test.rs`
  - Estimated hours: 3

- [ ] **Task 5.4**: Security audit of authentication flow
  - Implementation notes: Review for timing attacks, injection, common vulnerabilities
  - Files affected: Security audit document
  - Estimated hours: 4

---

## Technical Details

### Architecture Considerations

- Use Argon2id for password hashing (memory-hard, resistant to GPU attacks)
- Store passwords as salted hashes, never plaintext
- Implement constant-time comparison for password verification
- Use cryptographically secure RNG for tokens

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
argon2 = "0.5"
rand = "0.8"
constant_time_eq = "0.3"
tokio = { workspace = true }
sqlx = { workspace = true }

[dev-dependencies]
criterion = "0.5"
```

### Code Patterns

**Password Hashing:**
```rust
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

pub struct PasswordHasher {
    argon2: Argon2<'static>,
}

impl PasswordHasher {
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let hash = self.argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hash)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
```

**Rate Limiting:**
```rust
pub struct RateLimiter {
    attempts: HashMap<String, Vec<Instant>>,
    max_attempts: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn check_rate_limit(&mut self, key: &str) -> Result<()> {
        let now = Instant::now();
        let attempts = self.attempts.entry(key.to_string())
            .or_insert_with(Vec::new);

        // Remove old attempts outside window
        attempts.retain(|t| now.duration_since(*t) < self.window);

        if attempts.len() >= self.max_attempts {
            return Err(anyhow!("Rate limit exceeded"));
        }

        attempts.push(now);
        Ok(())
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 06**: Session management framework required
- **Sprint 04**: Storage layer for user data and sessions

### Blocks Downstream
- **Sprint 10**: Menu system requires authenticated users
- **Sprint 11**: Message reading requires user context
- **All user features**: Authentication is prerequisite

---

## Acceptance Criteria

- [ ] Users can log in with correct credentials
- [ ] Failed logins are rate-limited
- [ ] New users can register successfully
- [ ] Passwords are hashed with Argon2id
- [ ] Sessions expire after configured timeout
- [ ] Account lockout works after repeated failures
- [ ] No timing vulnerabilities in password verification

---

## Testing Requirements

### Unit Tests
- [ ] Password hashing and verification
- [ ] Rate limiting logic
- [ ] Session token generation
- [ ] Validation functions (username, password strength)

### Integration Tests
- [ ] Complete login flow (enter credentials → authenticated)
- [ ] Registration flow (sign up → login)
- [ ] Session timeout and renewal
- [ ] Rate limit enforcement

### Security Tests
- [ ] Timing attack resistance
- [ ] Brute force protection
- [ ] SQL injection in login form
- [ ] Session token randomness

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use Argon2id (recommended by OWASP)
- Default password requirements: 8+ characters, mix of types
- Rate limit: 5 attempts per 15 minutes
- Session timeout: 30 minutes idle, 8 hours absolute

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Argon2 may be too slow on low-end hardware
- **Mitigation**: Benchmark, adjust parameters if needed, document requirements
- **Risk**: Rate limiting may lock out legitimate users
- **Mitigation**: Clear error messages, reasonable limits, admin override
- **Risk**: Session fixation attacks
- **Mitigation**: Regenerate session token on login, validate IP/user-agent

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
