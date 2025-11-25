# Security Architecture

**Project:** Impulse 7.1 BBS Modernization
**Last Updated:** 2025-11-22
**Target Audience:** Security Engineers, Developers, System Administrators

---

## Table of Contents

1. [Security Overview](#security-overview)
2. [Threat Model](#threat-model)
3. [Authentication and Authorization](#authentication-and-authorization)
4. [Input Validation](#input-validation)
5. [Cryptography](#cryptography)
6. [Network Security](#network-security)
7. [Data Protection](#data-protection)
8. [Secrets Management](#secrets-management)
9. [Audit Logging](#audit-logging)
10. [Dependency Security](#dependency-security)
11. [Security Testing](#security-testing)
12. [Incident Response](#incident-response)

---

## Security Overview

### Security Principles

1. **Defense in Depth**: Multiple layers of security controls
2. **Least Privilege**: Minimal access rights by default
3. **Secure by Default**: Safe configuration out-of-the-box
4. **Fail Securely**: Errors default to deny access
5. **Privacy by Design**: User data protection built-in
6. **Separation of Duties**: No single point of privilege
7. **Audit Everything**: Comprehensive security logging

### Security Goals

| Goal | Target | Verification |
|------|--------|--------------|
| **Confidentiality** | No unauthorized data access | Access control tests, encryption validation |
| **Integrity** | No unauthorized data modification | Checksums, digital signatures |
| **Availability** | 99.9% uptime | Load testing, DDoS mitigation |
| **Authentication** | Strong password requirements | Password policy enforcement |
| **Authorization** | Role-based access control | Permission matrix testing |
| **Accountability** | Complete audit trail | Log analysis, forensics capability |

### Security Layers

```
┌─────────────────────────────────────────┐
│  Network Layer (Firewall, TLS/SSH)     │
├─────────────────────────────────────────┤
│  Transport Layer (Rate Limiting)        │
├─────────────────────────────────────────┤
│  Application Layer (Input Validation)   │
├─────────────────────────────────────────┤
│  Authentication Layer (User Identity)   │
├─────────────────────────────────────────┤
│  Authorization Layer (Access Control)   │
├─────────────────────────────────────────┤
│  Data Layer (Encryption at Rest)        │
└─────────────────────────────────────────┘
```

---

## Threat Model

### Attack Surface Analysis

**External Attack Surface:**
- Telnet/SSH ports (2323, 2222)
- WebSocket endpoint (8080)
- HTTP/HTTPS interfaces (if enabled)
- Publicly accessible files

**Internal Attack Surface:**
- User-generated content (messages, file uploads)
- Door game integration
- Admin interfaces
- Database access

### Threat Categories

#### 1. Network-Based Threats

| Threat | Risk Level | Mitigation |
|--------|-----------|------------|
| **DDoS Attack** | High | Rate limiting, connection limits, firewall |
| **Port Scanning** | Medium | Fail2ban, minimal port exposure |
| **Man-in-the-Middle** | High | TLS 1.3, SSH, certificate pinning |
| **Packet Sniffing** | Medium | Encryption (TLS/SSH) for all connections |

#### 2. Authentication Threats

| Threat | Risk Level | Mitigation |
|--------|-----------|------------|
| **Brute Force** | High | Rate limiting, account lockout, CAPTCHA |
| **Credential Stuffing** | High | Password hashing (bcrypt), breach detection |
| **Session Hijacking** | Medium | Secure session tokens, timeout enforcement |
| **Password Cracking** | High | Strong hash algorithm (bcrypt cost 12+) |

#### 3. Injection Attacks

| Threat | Risk Level | Mitigation |
|--------|-----------|------------|
| **Command Injection** | Critical | Input sanitization, parameterized commands |
| **ANSI Injection** | Medium | ANSI parser validation, escape filtering |
| **Path Traversal** | High | Path canonicalization, whitelist validation |
| **SQL Injection** | High | Prepared statements, ORM usage |

#### 4. Data Security Threats

| Threat | Risk Level | Mitigation |
|--------|-----------|------------|
| **Data Breach** | Critical | Encryption at rest, access controls |
| **Privacy Violation** | High | Data minimization, GDPR compliance |
| **Unauthorized Access** | High | Role-based access control (RBAC) |
| **Data Corruption** | Medium | Checksums, backups, integrity validation |

#### 5. Application Logic Threats

| Threat | Risk Level | Mitigation |
|--------|-----------|------------|
| **Privilege Escalation** | Critical | Strict permission checks, audit logging |
| **Business Logic Bypass** | High | Server-side validation, state verification |
| **Resource Exhaustion** | Medium | Resource limits, garbage collection |
| **Malicious File Upload** | High | File type validation, virus scanning |

---

## Authentication and Authorization

### Password Security

**Password Requirements:**
```rust
pub struct PasswordPolicy {
    pub min_length: usize,           // Minimum 8 characters
    pub require_uppercase: bool,     // At least one uppercase letter
    pub require_lowercase: bool,     // At least one lowercase letter
    pub require_digit: bool,         // At least one digit
    pub require_special: bool,       // At least one special character
    pub max_age_days: u32,           // Password expiration
    pub prevent_reuse: usize,        // Remember last N passwords
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special: false,  // Optional for BBS use
            max_age_days: 90,
            prevent_reuse: 5,
        }
    }
}
```

**Password Hashing (bcrypt):**
```rust
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String> {
    // Use cost factor of 12 (2^12 = 4,096 iterations)
    const COST: u32 = 12;

    hash(password, COST)
        .context("Failed to hash password")
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash)
        .context("Failed to verify password")
}

// Example usage
let hash = hash_password("SecureP@ssw0rd")?;
let valid = verify_password("SecureP@ssw0rd", &hash)?;
```

**Account Lockout:**
```rust
pub struct LoginAttemptTracker {
    attempts: HashMap<String, Vec<Instant>>,
    max_attempts: u32,
    lockout_duration: Duration,
}

impl LoginAttemptTracker {
    pub fn record_failed_attempt(&mut self, username: &str) {
        let attempts = self.attempts.entry(username.to_string())
            .or_insert_with(Vec::new);

        attempts.push(Instant::now());

        // Remove old attempts outside lockout window
        attempts.retain(|&instant| {
            instant.elapsed() < self.lockout_duration
        });
    }

    pub fn is_locked_out(&self, username: &str) -> bool {
        if let Some(attempts) = self.attempts.get(username) {
            attempts.len() >= self.max_attempts as usize
        } else {
            false
        }
    }

    pub fn clear_attempts(&mut self, username: &str) {
        self.attempts.remove(username);
    }
}
```

### Role-Based Access Control (RBAC)

**Security Levels:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Guest = 0,           // Limited read-only access
    User = 10,           // Standard user privileges
    Moderator = 50,      // Message/file moderation
    CoSysop = 100,       // Co-System Operator
    Sysop = 255,         // Full system access
}

impl SecurityLevel {
    pub fn can_post_messages(&self) -> bool {
        *self >= SecurityLevel::User
    }

    pub fn can_upload_files(&self) -> bool {
        *self >= SecurityLevel::User
    }

    pub fn can_moderate(&self) -> bool {
        *self >= SecurityLevel::Moderator
    }

    pub fn can_manage_users(&self) -> bool {
        *self >= SecurityLevel::CoSysop
    }

    pub fn can_configure_system(&self) -> bool {
        *self >= SecurityLevel::Sysop
    }
}
```

**Permission Checking:**
```rust
pub trait Authorizable {
    fn has_permission(&self, permission: Permission) -> bool;
    fn require_permission(&self, permission: Permission) -> Result<()>;
}

impl Authorizable for User {
    fn has_permission(&self, permission: Permission) -> bool {
        match permission {
            Permission::ReadMessages => self.security >= SecurityLevel::Guest,
            Permission::PostMessages => self.security >= SecurityLevel::User,
            Permission::ModerateMessages => self.security >= SecurityLevel::Moderator,
            Permission::ManageUsers => self.security >= SecurityLevel::CoSysop,
            Permission::ConfigureSystem => self.security >= SecurityLevel::Sysop,
        }
    }

    fn require_permission(&self, permission: Permission) -> Result<()> {
        if self.has_permission(permission) {
            Ok(())
        } else {
            Err(anyhow!("Insufficient permissions: {:?}", permission))
        }
    }
}

// Usage example
pub async fn delete_user(current_user: &User, target_user_id: u32) -> Result<()> {
    // Require permission before proceeding
    current_user.require_permission(Permission::ManageUsers)?;

    // Perform deletion
    UserManager::delete(target_user_id).await?;

    // Audit log
    audit_log(AuditEvent::UserDeleted {
        deleted_by: current_user.id,
        deleted_user: target_user_id,
    })?;

    Ok(())
}
```

---

## Input Validation

### Validation Framework

```rust
pub trait Validator<T> {
    fn validate(&self, value: &T) -> Result<()>;
}

// String length validator
pub struct LengthValidator {
    min: usize,
    max: usize,
}

impl Validator<String> for LengthValidator {
    fn validate(&self, value: &String) -> Result<()> {
        if value.len() < self.min {
            return Err(anyhow!("Input too short (min: {})", self.min));
        }
        if value.len() > self.max {
            return Err(anyhow!("Input too long (max: {})", self.max));
        }
        Ok(())
    }
}

// Regex pattern validator
pub struct RegexValidator {
    pattern: Regex,
    description: String,
}

impl Validator<String> for RegexValidator {
    fn validate(&self, value: &String) -> Result<()> {
        if self.pattern.is_match(value) {
            Ok(())
        } else {
            Err(anyhow!("Invalid format: {}", self.description))
        }
    }
}

// Username validator (alphanumeric, underscore, hyphen)
pub fn validate_username(username: &str) -> Result<()> {
    let length_check = LengthValidator { min: 3, max: 30 };
    length_check.validate(&username.to_string())?;

    let pattern_check = RegexValidator {
        pattern: Regex::new(r"^[a-zA-Z0-9_-]+$")?,
        description: "Username must contain only letters, numbers, underscore, and hyphen".to_string(),
    };
    pattern_check.validate(&username.to_string())?;

    // Additional checks
    if username.to_lowercase() == "sysop" || username.to_lowercase() == "admin" {
        return Err(anyhow!("Reserved username"));
    }

    Ok(())
}
```

### ANSI Escape Sequence Validation

```rust
pub struct AnsiValidator {
    max_sequence_length: usize,
    allowed_commands: HashSet<char>,
}

impl AnsiValidator {
    pub fn new() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert('m'); // Color/style
        allowed.insert('H'); // Cursor position
        allowed.insert('J'); // Clear screen
        allowed.insert('K'); // Clear line

        Self {
            max_sequence_length: 20,
            allowed_commands: allowed,
        }
    }

    pub fn validate_and_sanitize(&self, text: &str) -> Result<String> {
        let mut sanitized = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Start of escape sequence
                if chars.peek() == Some(&'[') {
                    chars.next(); // Consume '['

                    let mut sequence = String::from("\x1b[");
                    let mut params = String::new();

                    // Read parameters and command
                    while let Some(&ch) = chars.peek() {
                        if sequence.len() > self.max_sequence_length {
                            return Err(anyhow!("ANSI sequence too long"));
                        }

                        if ch.is_ascii_digit() || ch == ';' {
                            params.push(chars.next().unwrap());
                        } else if ch.is_ascii_alphabetic() {
                            let cmd = chars.next().unwrap();

                            if self.allowed_commands.contains(&cmd) {
                                sequence.push_str(&params);
                                sequence.push(cmd);
                                sanitized.push_str(&sequence);
                            } else {
                                warn!("Blocked disallowed ANSI command: {}", cmd);
                            }
                            break;
                        } else {
                            return Err(anyhow!("Invalid ANSI sequence"));
                        }
                    }
                } else {
                    // Invalid escape, skip
                    warn!("Invalid escape sequence");
                }
            } else {
                // Regular character
                sanitized.push(ch);
            }
        }

        Ok(sanitized)
    }
}
```

### Path Traversal Prevention

```rust
use std::path::{Path, PathBuf};

pub struct SecurePath {
    base_dir: PathBuf,
}

impl SecurePath {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Resolve path safely, preventing traversal outside base_dir
    pub fn resolve(&self, user_path: &str) -> Result<PathBuf> {
        // Canonicalize to resolve .. and symlinks
        let requested = Path::new(user_path);

        // Reject absolute paths
        if requested.is_absolute() {
            return Err(anyhow!("Absolute paths not allowed"));
        }

        // Build full path
        let full_path = self.base_dir.join(requested);

        // Canonicalize (resolve .. and symlinks)
        let canonical = full_path.canonicalize()
            .context("Path does not exist or cannot be resolved")?;

        // Verify still within base_dir
        if !canonical.starts_with(&self.base_dir) {
            return Err(anyhow!("Path traversal attempt detected"));
        }

        Ok(canonical)
    }
}

// Usage
let secure = SecurePath::new(PathBuf::from("/var/lib/impulse/files"));

// Safe: resolves to /var/lib/impulse/files/uploads/file.txt
let safe_path = secure.resolve("uploads/file.txt")?;

// Blocked: attempts to escape base directory
let blocked = secure.resolve("../../etc/passwd");
// Returns: Err("Path traversal attempt detected")
```

---

## Cryptography

### TLS Configuration

```rust
use rustls::{ServerConfig, NoClientAuth};
use rustls::internal::pemfile::{certs, rsa_private_keys};

pub fn create_tls_config(
    cert_path: &Path,
    key_path: &Path,
) -> Result<Arc<ServerConfig>> {
    // Load certificates
    let cert_file = File::open(cert_path)?;
    let mut cert_reader = BufReader::new(cert_file);
    let certs = certs(&mut cert_reader)
        .map_err(|_| anyhow!("Failed to load certificates"))?;

    // Load private key
    let key_file = File::open(key_path)?;
    let mut key_reader = BufReader::new(key_file);
    let mut keys = rsa_private_keys(&mut key_reader)
        .map_err(|_| anyhow!("Failed to load private key"))?;

    if keys.is_empty() {
        return Err(anyhow!("No private key found"));
    }

    // Create TLS config
    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(certs, keys.remove(0))?;

    // Set protocol versions (TLS 1.2 and 1.3 only)
    config.versions = vec![
        rustls::ProtocolVersion::TLSv1_3,
        rustls::ProtocolVersion::TLSv1_2,
    ];

    Ok(Arc::new(config))
}
```

### SSH Host Key Management

```rust
use ssh2::Session;

pub fn load_or_generate_host_key(key_path: &Path) -> Result<Vec<u8>> {
    if key_path.exists() {
        // Load existing key
        info!("Loading SSH host key from: {}", key_path.display());
        std::fs::read(key_path)
            .context("Failed to read host key")
    } else {
        // Generate new key
        info!("Generating new SSH host key");
        let key = generate_ssh_host_key()?;

        // Save for future use (secure permissions)
        let mut file = File::create(key_path)?;
        file.write_all(&key)?;

        // Set restrictive permissions (0600)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(key_path, perms)?;
        }

        Ok(key)
    }
}

fn generate_ssh_host_key() -> Result<Vec<u8>> {
    use openssl::rsa::Rsa;
    use openssl::pkey::PKey;

    // Generate 4096-bit RSA key
    let rsa = Rsa::generate(4096)?;
    let pkey = PKey::from_rsa(rsa)?;

    // Export to PEM format
    let key_pem = pkey.private_key_to_pem_pkcs8()?;
    Ok(key_pem)
}
```

### Data Encryption at Rest

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

pub struct DataEncryption {
    cipher: Aes256Gcm,
}

impl DataEncryption {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        use rand::Rng;

        // Generate random nonce
        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; 12] = rng.gen();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = self.cipher.encrypt(nonce, data)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>> {
        if encrypted.len() < 12 {
            return Err(anyhow!("Invalid encrypted data (too short)"));
        }

        // Extract nonce
        let nonce = Nonce::from_slice(&encrypted[..12]);

        // Decrypt
        let plaintext = self.cipher.decrypt(nonce, &encrypted[12..])
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }
}

// Usage for sensitive user data
pub fn encrypt_user_data(user: &User, key: &[u8; 32]) -> Result<Vec<u8>> {
    let encryption = DataEncryption::new(key);

    // Serialize user data
    let data = serde_json::to_vec(user)?;

    // Encrypt
    encryption.encrypt(&data)
}
```

---

## Network Security

### Rate Limiting

```rust
use std::net::IpAddr;
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct RateLimiter {
    limits: HashMap<IpAddr, Vec<Instant>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            limits: HashMap::new(),
            max_requests,
            window,
        }
    }

    pub fn check_rate_limit(&mut self, addr: IpAddr) -> bool {
        let now = Instant::now();

        let requests = self.limits.entry(addr)
            .or_insert_with(Vec::new);

        // Remove old requests outside window
        requests.retain(|&instant| now.duration_since(instant) < self.window);

        // Check if under limit
        if requests.len() < self.max_requests {
            requests.push(now);
            true  // Allowed
        } else {
            false  // Rate limited
        }
    }
}

// Per-endpoint rate limiting
pub async fn handle_connection(
    stream: TcpStream,
    rate_limiter: Arc<Mutex<RateLimiter>>,
) -> Result<()> {
    let addr = stream.peer_addr()?.ip();

    // Check rate limit
    let mut limiter = rate_limiter.lock().await;
    if !limiter.check_rate_limit(addr) {
        warn!("Rate limit exceeded for {}", addr);
        return Err(anyhow!("Rate limit exceeded"));
    }
    drop(limiter);

    // Process connection
    handle_client(stream).await
}
```

### Connection Limits

```rust
use tokio::sync::Semaphore;

pub struct ConnectionManager {
    semaphore: Arc<Semaphore>,
    max_connections: usize,
}

impl ConnectionManager {
    pub fn new(max_connections: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_connections)),
            max_connections,
        }
    }

    pub async fn accept_connection<F, Fut>(
        &self,
        handler: F,
    ) -> Result<()>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        // Acquire permit (blocks if at limit)
        let permit = self.semaphore.acquire().await?;

        // Handle connection
        let result = handler().await;

        // Permit automatically released on drop
        drop(permit);

        result
    }

    pub fn available_slots(&self) -> usize {
        self.semaphore.available_permits()
    }
}
```

---

## Data Protection

### Data Classification

| Classification | Examples | Protection |
|----------------|----------|------------|
| **Public** | BBS name, file list | No encryption |
| **Internal** | User list, message count | Access control |
| **Confidential** | User messages, file uploads | Encryption at rest, access control |
| **Restricted** | Passwords, PII | Hashing (bcrypt), encryption, audit |

### GDPR Compliance

```rust
pub struct UserDataExport {
    pub user_id: u32,
    pub username: String,
    pub personal_info: PersonalInfo,
    pub activity: UserActivity,
    pub messages: Vec<Message>,
    pub files: Vec<FileUpload>,
}

/// Export all user data (GDPR Article 20: Right to data portability)
pub async fn export_user_data(user_id: u32) -> Result<UserDataExport> {
    let user = UserManager::load(user_id).await?;

    let export = UserDataExport {
        user_id: user.id,
        username: user.name.clone(),
        personal_info: user.personal_info.clone(),
        activity: fetch_user_activity(user_id).await?,
        messages: fetch_user_messages(user_id).await?,
        files: fetch_user_uploads(user_id).await?,
    };

    // Audit log
    audit_log(AuditEvent::DataExported { user_id })?;

    Ok(export)
}

/// Delete all user data (GDPR Article 17: Right to erasure)
pub async fn delete_user_data(user_id: u32, requestor: &User) -> Result<()> {
    // Verify authorization
    if requestor.id != user_id && !requestor.has_permission(Permission::ManageUsers) {
        return Err(anyhow!("Unauthorized"));
    }

    // Delete from all systems
    UserManager::delete(user_id).await?;
    MessageBase::delete_user_messages(user_id).await?;
    FileManager::delete_user_files(user_id).await?;

    // Audit log (keep for compliance)
    audit_log(AuditEvent::UserDataDeleted {
        user_id,
        deleted_by: requestor.id,
    })?;

    Ok(())
}
```

---

## Secrets Management

### Environment-Based Secrets

```rust
use std::env;

pub struct Secrets {
    pub database_password: String,
    pub jwt_secret: String,
    pub encryption_key: [u8; 32],
    pub ssh_host_key_path: PathBuf,
}

impl Secrets {
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            database_password: env::var("DATABASE_PASSWORD")
                .context("DATABASE_PASSWORD not set")?,

            jwt_secret: env::var("JWT_SECRET")
                .context("JWT_SECRET not set")?,

            encryption_key: Self::load_encryption_key()?,

            ssh_host_key_path: env::var("SSH_HOST_KEY_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/etc/impulse/ssh_host_key")),
        })
    }

    fn load_encryption_key() -> Result<[u8; 32]> {
        let key_hex = env::var("ENCRYPTION_KEY")
            .context("ENCRYPTION_KEY not set")?;

        let key_bytes = hex::decode(key_hex)
            .context("Invalid ENCRYPTION_KEY (must be hex)")?;

        if key_bytes.len() != 32 {
            return Err(anyhow!("ENCRYPTION_KEY must be 32 bytes (64 hex chars)"));
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(&key_bytes);
        Ok(key)
    }
}
```

### Secret Rotation

```bash
#!/bin/bash
# rotate_secrets.sh

set -e

echo "Rotating encryption key..."

# Generate new key
NEW_KEY=$(openssl rand -hex 32)

# Update environment file
sed -i.bak "s/^ENCRYPTION_KEY=.*/ENCRYPTION_KEY=$NEW_KEY/" /etc/impulse/secrets.env

# Trigger re-encryption of data
/usr/local/bin/imp-cli admin --reencrypt-data \
    --old-key "$OLD_KEY" \
    --new-key "$NEW_KEY"

echo "Encryption key rotated successfully"
echo "Backup of old config: /etc/impulse/secrets.env.bak"
```

---

## Audit Logging

### Audit Event Types

```rust
#[derive(Debug, Serialize)]
pub enum AuditEvent {
    // Authentication
    LoginSuccess { user_id: u32, ip: IpAddr },
    LoginFailed { username: String, ip: IpAddr, reason: String },
    Logout { user_id: u32 },

    // Authorization
    PermissionDenied { user_id: u32, permission: String },
    PrivilegeEscalationAttempt { user_id: u32, details: String },

    // Data Access
    UserDataExported { user_id: u32 },
    UserDataDeleted { user_id: u32, deleted_by: u32 },
    SensitiveDataAccessed { user_id: u32, data_type: String },

    // System Changes
    ConfigurationChanged { user_id: u32, setting: String, old_value: String, new_value: String },
    UserCreated { user_id: u32, created_by: u32 },
    UserDeleted { user_id: u32, deleted_by: u32 },
    SecurityLevelChanged { user_id: u32, old_level: u8, new_level: u8, changed_by: u32 },

    // Security Events
    BruteForceDetected { ip: IpAddr, attempts: u32 },
    SuspiciousActivity { user_id: u32, description: String },
    RateLimitExceeded { ip: IpAddr },
}

pub fn audit_log(event: AuditEvent) -> Result<()> {
    let timestamp = Utc::now();

    let log_entry = serde_json::json!({
        "timestamp": timestamp.to_rfc3339(),
        "event_type": format!("{:?}", event),
        "event": event,
    });

    // Write to audit log (append-only)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/var/log/impulse/audit.log")?;

    writeln!(file, "{}", log_entry)?;

    // Also send to SIEM if configured
    if let Ok(siem_endpoint) = env::var("SIEM_ENDPOINT") {
        send_to_siem(&siem_endpoint, &log_entry)?;
    }

    Ok(())
}
```

---

## Dependency Security

### Automated Vulnerability Scanning

```bash
# Install cargo-audit
cargo install cargo-audit

# Check dependencies for known vulnerabilities
cargo audit

# Auto-fix where possible
cargo audit fix

# CI/CD integration
cargo audit --deny warnings
```

**CI/CD Pipeline (.github/workflows/security.yml):**
```yaml
name: Security Audit

on:
  push:
  pull_request:
  schedule:
    - cron: '0 0 * * *'  # Daily

jobs:
  security_audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-audit
        run: cargo install cargo-audit

      - name: Run security audit
        run: cargo audit --deny warnings

      - name: Check for outdated dependencies
        run: cargo outdated --exit-code 1
```

### Dependency Pinning

**Cargo.lock:** Always commit to version control for reproducible builds.

```bash
# Update dependencies conservatively
cargo update --dry-run

# Update specific crate
cargo update -p tokio

# Verify no breaking changes
cargo test --workspace
```

---

## Security Testing

### Penetration Testing

```bash
# Port scanning
nmap -sV -p- localhost

# SQL injection testing (if database used)
sqlmap -u "http://localhost:8080/api/users?id=1"

# Brute force testing
hydra -l admin -P passwords.txt telnet://localhost:2323

# Fuzzing ANSI parser
cargo fuzz run ansi_parser -- -max_total_time=300
```

### Security Code Review Checklist

- [ ] No hardcoded secrets or credentials
- [ ] Input validation on all user inputs
- [ ] Authorization checks before sensitive operations
- [ ] Passwords hashed with bcrypt (cost ≥ 12)
- [ ] TLS/SSH for all network communications
- [ ] SQL queries use prepared statements
- [ ] File paths validated against traversal
- [ ] Rate limiting on authentication endpoints
- [ ] Audit logging for security events
- [ ] Error messages don't leak sensitive info
- [ ] Dependencies scanned for vulnerabilities

---

## Incident Response

### Security Incident Procedure

1. **Detection**: Automated monitoring alerts or user report
2. **Containment**: Isolate affected systems, block attacker
3. **Investigation**: Analyze audit logs, determine scope
4. **Eradication**: Remove malicious code, patch vulnerabilities
5. **Recovery**: Restore from clean backups, verify integrity
6. **Lessons Learned**: Document incident, update procedures

### Emergency Response

```bash
#!/bin/bash
# emergency_lockdown.sh

set -e

echo "EMERGENCY SECURITY LOCKDOWN INITIATED"

# Stop BBS immediately
systemctl stop impulse-bbs

# Block all network access (except SSH for admin)
iptables -P INPUT DROP
iptables -P FORWARD DROP
iptables -P OUTPUT DROP
iptables -A INPUT -i lo -j ACCEPT
iptables -A OUTPUT -o lo -j ACCEPT
iptables -A INPUT -p tcp --dport 22 -j ACCEPT
iptables -A OUTPUT -p tcp --sport 22 -j ACCEPT

# Create forensic backup
tar -czf /backup/forensic_$(date +%Y%m%d_%H%M%S).tar.gz \
    /var/lib/impulse/ \
    /var/log/impulse/ \
    /etc/impulse/

echo "System locked down. Forensic backup created."
echo "Investigate before restoring service."
```

---

## Security Checklist

### Pre-Deployment Security Review

- [ ] All dependencies audited (`cargo audit`)
- [ ] Secrets management configured
- [ ] TLS/SSH certificates valid
- [ ] Firewall rules configured
- [ ] Rate limiting enabled
- [ ] Audit logging operational
- [ ] Backups tested and verified
- [ ] Incident response plan documented
- [ ] Security monitoring configured
- [ ] Penetration testing completed

### Ongoing Security Maintenance

- [ ] Weekly: Review audit logs for anomalies
- [ ] Monthly: Dependency vulnerability scan
- [ ] Quarterly: Security code review
- [ ] Annually: Penetration testing
- [ ] As needed: Secret rotation

---

## Summary

Security is not a feature—it's a fundamental requirement. This architecture implements:

- **Multi-layered defense** with redundant protections
- **Strong authentication** with bcrypt password hashing
- **Comprehensive input validation** preventing injection attacks
- **Encryption** for data in transit (TLS/SSH) and at rest (AES-256)
- **Audit logging** for accountability and forensics
- **Rate limiting** to prevent abuse
- **GDPR compliance** for data protection

**Remember:** Security is an ongoing process, not a one-time implementation. Stay vigilant, keep dependencies updated, and respond quickly to emerging threats.

---

**For additional security resources:**
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [CIS Benchmarks](https://www.cisecurity.org/cis-benchmarks/)

**Security concerns?** Report privately to security@project-domain.com (not public issues).
