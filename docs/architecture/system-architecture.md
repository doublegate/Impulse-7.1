# Impulse BBS - Rust Architecture Specification
## Technical Architecture Document v1.0

---

## Table of Contents

1. [System Architecture Overview](#system-architecture-overview)
2. [Crate Structure & Organization](#crate-structure--organization)
3. [Core Subsystems](#core-subsystems)
4. [Data Models & Type System](#data-models--type-system)
5. [Async Runtime & Concurrency](#async-runtime--concurrency)
6. [Protocol Implementations](#protocol-implementations)
7. [Storage Layer](#storage-layer)
8. [Security Architecture](#security-architecture)
9. [Testing Strategy](#testing-strategy)
10. [Deployment Architecture](#deployment-architecture)

---

## 1. System Architecture Overview

### 1.1 High-Level Component Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                     Impulse BBS System                          │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        │
│  │   Telnet/    │  │     SSH      │  │  HTTP/REST   │        │
│  │  Serial Port │  │   Server     │  │     API      │        │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘        │
│         │                  │                  │                 │
│         └──────────────────┴──────────────────┘                │
│                            │                                    │
│                 ┌──────────▼──────────┐                        │
│                 │  Session Manager    │                        │
│                 │   (Async/Tokio)     │                        │
│                 └──────────┬──────────┘                        │
│                            │                                    │
│         ┌──────────────────┼──────────────────┐               │
│         │                  │                  │                │
│  ┌──────▼───────┐  ┌──────▼──────┐  ┌───────▼────────┐       │
│  │ Terminal I/O │  │   Message    │  │  File Transfer │       │
│  │  Subsystem   │  │   Subsystem  │  │   Subsystem    │       │
│  │  (ANSI/VT)   │  │ (JAM/Hudson) │  │  (Zmodem/FTP)  │       │
│  └──────┬───────┘  └──────┬───────┘  └───────┬────────┘       │
│         │                  │                  │                 │
│         └──────────────────┴──────────────────┘                │
│                            │                                    │
│                 ┌──────────▼──────────┐                        │
│                 │   Storage Layer     │                        │
│                 │  (SQLite/Postgres)  │                        │
│                 └─────────────────────┘                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Design Principles

1. **Safety First**: Leverage Rust's ownership system to eliminate entire classes of bugs
2. **Async by Default**: Use Tokio for all I/O operations to support multiple simultaneous connections
3. **Modular Architecture**: Each subsystem is a separate crate with well-defined interfaces
4. **Testability**: Every component designed with unit and integration testing in mind
5. **Performance**: Target sub-50ms response times for typical operations
6. **Backward Compatibility**: Support legacy data formats with clean migration paths

---

## 2. Crate Structure & Organization

### 2.1 Workspace Layout

```
impulse-bbs/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── impulse-core/             # Core types, traits, and utilities
│   ├── impulse-session/          # Session management and event loop
│   ├── impulse-terminal/         # Terminal I/O and ANSI rendering
│   ├── impulse-message/          # Message base implementations
│   ├── impulse-files/            # File area and transfer protocols
│   ├── impulse-user/             # User management and authentication
│   ├── impulse-config/           # Configuration loading and validation
│   ├── impulse-storage/          # Database abstraction layer
│   ├── impulse-protocol/         # File transfer protocols (Zmodem, etc.)
│   ├── impulse-door/             # External door game interface
│   ├── impulse-telnet/           # Telnet server implementation
│   ├── impulse-ssh/              # SSH server implementation
│   ├── impulse-api/              # REST API server
│   ├── impulse-legacy/           # Legacy format parsers/converters
│   └── impulse-cli/              # Command-line tools (impconfig, etc.)
├── impulse-server/               # Main BBS server binary
├── tests/                        # Integration tests
├── benches/                      # Performance benchmarks
├── docs/                         # Documentation
└── scripts/                      # Build and deployment scripts
```

### 2.2 Dependency Graph

```
impulse-server
├── impulse-session
│   ├── impulse-core
│   ├── impulse-terminal
│   ├── impulse-message
│   ├── impulse-files
│   ├── impulse-user
│   └── impulse-storage
├── impulse-telnet
│   └── impulse-session
├── impulse-ssh
│   └── impulse-session
└── impulse-api
    └── impulse-session

impulse-cli
├── impulse-config
├── impulse-legacy
└── impulse-storage
```

### 2.3 Core Crate Descriptions

#### `impulse-core`
**Purpose**: Foundational types, traits, and utilities used across all crates  
**Key Components**:
- `Error` - Unified error type using `thiserror`
- `Result<T>` - Convenience alias for `std::result::Result<T, Error>`
- Common traits: `Serializable`, `Deserializable`, `Validatable`
- Utility functions: date/time handling, string manipulation

**Dependencies**: Minimal (serde, thiserror, chrono)

#### `impulse-session`
**Purpose**: Manages user sessions, command routing, and event loops  
**Key Components**:
- `SessionManager` - Coordinates multiple active sessions
- `Session` - Represents a single user connection
- `CommandRouter` - Routes user input to appropriate handlers
- `EventLoop` - Async event processing

**Dependencies**: tokio, impulse-core, impulse-terminal, impulse-user

#### `impulse-terminal`
**Purpose**: Terminal I/O, ANSI rendering, menu system  
**Key Components**:
- `TerminalDriver` - Abstract terminal interface
- `AnsiRenderer` - Parses and renders ANSI art
- `MenuSystem` - Loads and displays menu hierarchies
- `ThemeManager` - Handles theme switching

**Dependencies**: crossterm, nom (parsing), impulse-core

---

## 3. Core Subsystems

### 3.1 Terminal I/O Subsystem

#### Architecture

The terminal subsystem abstracts over different connection types (telnet, SSH, serial) to provide a unified interface for user interaction.

```rust
/// Core trait for terminal operations
#[async_trait]
pub trait TerminalDriver: Send + Sync {
    /// Write data to the terminal
    async fn write(&mut self, data: &[u8]) -> Result<usize>;
    
    /// Read data from the terminal (non-blocking)
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    
    /// Clear the screen
    async fn clear_screen(&mut self) -> Result<()>;
    
    /// Move cursor to position
    async fn move_cursor(&mut self, x: u16, y: u16) -> Result<()>;
    
    /// Detect terminal capabilities (ANSI, RIP, Avatar, ASCII)
    async fn detect_capabilities(&mut self) -> Result<TerminalCapabilities>;
}

/// Terminal capabilities detected at connection
#[derive(Debug, Clone)]
pub struct TerminalCapabilities {
    pub supports_ansi: bool,
    pub supports_color: bool,
    pub supports_rip: bool,
    pub width: u16,
    pub height: u16,
    pub terminal_type: String, // e.g., "xterm-256color"
}
```

#### ANSI Rendering Pipeline

```rust
/// ANSI art file with metadata
pub struct AnsiArt {
    pub name: String,
    pub content: Vec<u8>,
    pub width: u16,
    pub height: u16,
    pub sauce_record: Option<SauceRecord>, // SAUCE metadata standard
}

/// Renders ANSI art to terminal
pub struct AnsiRenderer {
    driver: Box<dyn TerminalDriver>,
    current_state: RenderState,
}

impl AnsiRenderer {
    /// Render ANSI file with optional line delay (for classic "scroll" effect)
    pub async fn render_file(
        &mut self,
        art: &AnsiArt,
        line_delay_ms: u64,
    ) -> Result<()> {
        // Parse ANSI sequences
        // Handle ESC codes for color, positioning
        // Implement rate limiting for slower connections
        // Support pause/abort on keypress
    }
}
```

### 3.2 Message Subsystem

#### Architecture

Support for multiple message base formats through a trait-based design.

```rust
/// Abstract message base interface
#[async_trait]
pub trait MessageBase: Send + Sync {
    /// Open/initialize the message base
    async fn open(&mut self) -> Result<()>;
    
    /// Close and flush pending writes
    async fn close(&mut self) -> Result<()>;
    
    /// Read a message by number
    async fn read_message(&self, msg_num: u32) -> Result<Message>;
    
    /// Post a new message
    async fn post_message(&mut self, msg: &Message) -> Result<u32>;
    
    /// Reply to an existing message
    async fn reply_to(&mut self, parent_id: u32, msg: &Message) -> Result<u32>;
    
    /// Get message count
    async fn message_count(&self) -> Result<u32>;
    
    /// Search messages
    async fn search(&self, query: &SearchQuery) -> Result<Vec<u32>>;
}

/// Unified message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u32,
    pub area: String,           // Message area/conference
    pub from: String,           // Author name
    pub to: String,             // Recipient (or "All")
    pub subject: String,
    pub date_written: DateTime<Utc>,
    pub body: String,
    pub reply_to: Option<u32>,  // Thread linkage
    pub flags: MessageFlags,    // Private, deleted, etc.
}

/// Concrete implementations
pub struct JamMessageBase { /* ... */ }
pub struct HudsonMessageBase { /* ... */ }
pub struct SqliteMessageBase { /* ... */ } // Modern alternative
```

#### JAM Format Implementation

The JAM (Joint Area Message) format uses a linked-list structure on disk. Our implementation will:

1. **Use `mmap` for performance**: Memory-map the JAM files for fast access
2. **Maintain indices in memory**: Build B-tree indices on startup for fast lookups
3. **Implement write-ahead logging**: Ensure data integrity during crashes
4. **Support concurrent readers**: Multiple sessions can read simultaneously

```rust
/// JAM message base implementation
pub struct JamMessageBase {
    header_file: File,          // .JHR - message headers
    text_file: File,            // .JDT - message text
    index_file: File,           // .JDX - message indices
    last_read_file: File,       // .JLR - last-read pointers
    
    // In-memory indices
    id_index: BTreeMap<u32, u64>,     // msg_id -> header_offset
    thread_index: HashMap<u32, Vec<u32>>, // parent_id -> reply_ids
}
```

### 3.3 File Transfer Subsystem

#### Protocol Abstraction

```rust
/// File transfer protocol trait
#[async_trait]
pub trait FileTransferProtocol: Send + Sync {
    /// Get protocol name (e.g., "Zmodem", "Xmodem-1K")
    fn name(&self) -> &str;
    
    /// Send a file to the remote end
    async fn send_file(
        &mut self,
        path: &Path,
        progress: Box<dyn ProgressCallback>,
    ) -> Result<TransferStats>;
    
    /// Receive a file from the remote end
    async fn receive_file(
        &mut self,
        dest_path: &Path,
        progress: Box<dyn ProgressCallback>,
    ) -> Result<TransferStats>;
    
    /// Supports batch transfers?
    fn supports_batch(&self) -> bool;
}

/// Transfer statistics
#[derive(Debug, Clone)]
pub struct TransferStats {
    pub bytes_transferred: u64,
    pub duration: Duration,
    pub bytes_per_second: u64,
    pub errors: u32,
    pub retries: u32,
}
```

#### Zmodem Implementation Strategy

Zmodem is complex, involving:
- **CRC-16 and CRC-32 checksums**
- **Sliding window acknowledgments**
- **Crash recovery** (resume partial transfers)
- **File metadata transmission** (size, date, permissions)

```rust
/// Internal Zmodem implementation
pub struct ZmodemProtocol {
    stream: Box<dyn AsyncRead + AsyncWrite + Unpin>,
    config: ZmodemConfig,
    state: ZmodemState,
}

#[derive(Debug, Clone)]
pub struct ZmodemConfig {
    pub block_size: usize,       // Typically 1024 or 8192
    pub window_size: usize,      // Number of unacked blocks
    pub use_crc32: bool,         // CRC-32 vs CRC-16
    pub escape_control: bool,    // Escape control characters
}

impl ZmodemProtocol {
    /// Send ZRQINIT - request receiver info
    async fn send_init_request(&mut self) -> Result<()> { /* ... */ }
    
    /// Send ZFILE - file header
    async fn send_file_header(&mut self, info: &FileInfo) -> Result<()> { /* ... */ }
    
    /// Send ZDATA - file data blocks
    async fn send_data_blocks(&mut self, data: &[u8]) -> Result<()> { /* ... */ }
    
    /// Handle ZACK - acknowledgment
    async fn handle_ack(&mut self, position: u64) -> Result<()> { /* ... */ }
}
```

---

## 4. Data Models & Type System

### 4.1 User Model

```rust
/// User account structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub real_name: String,
    
    // Authentication
    pub password_hash: String,  // Argon2id hash
    pub salt: Vec<u8>,
    
    // Profile
    pub email: Option<String>,
    pub location: String,
    pub signature: String,
    
    // Access control
    pub security_level: u8,     // 0-255, 255 = SysOp
    pub flags: UserFlags,       // Banned, validated, etc.
    
    // Statistics
    pub date_joined: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub total_calls: u32,
    pub upload_bytes: u64,
    pub download_bytes: u64,
    pub posts: u32,
    
    // Preferences
    pub theme: String,
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub color_enabled: bool,
    pub hot_keys: bool,         // Single-key menu navigation
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct UserFlags: u32 {
        const ACTIVE       = 0b00000001;
        const VALIDATED    = 0b00000010;
        const BANNED       = 0b00000100;
        const SYSOP        = 0b00001000;
        const CO_SYSOP     = 0b00010000;
        const ANSI         = 0b00100000;
        const DELETED      = 0b01000000;
    }
}
```

### 4.2 File Record Model

```rust
/// File entry in file database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    pub id: u64,
    pub filename: String,
    pub area: String,           // Which file area
    pub description: String,    // Short description
    pub extended_desc: Vec<String>, // Multi-line description
    
    // File metadata
    pub size_bytes: u64,
    pub date_uploaded: DateTime<Utc>,
    pub uploader: String,       // Username
    pub download_count: u32,
    
    // Storage
    pub physical_path: PathBuf,
    pub hash_sha256: String,    // For integrity checking
    
    // Validation
    pub file_id_diz: Option<String>, // Extracted FILE_ID.DIZ content
    pub scan_status: ScanStatus,     // Virus scan result
    
    // Access control
    pub min_security: u8,       // Minimum security level to download
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ScanStatus {
    NotScanned,
    Clean,
    Suspicious,
    Infected,
}
```

### 4.3 Configuration Model

```rust
/// System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    // System identity
    pub system_name: String,
    pub sysop_name: String,
    pub location: String,
    
    // Network
    pub telnet_port: u16,
    pub ssh_port: u16,
    pub max_connections: usize,
    
    // Paths
    pub data_dir: PathBuf,
    pub file_dir: PathBuf,
    pub text_dir: PathBuf,
    pub menu_dir: PathBuf,
    
    // Features
    pub new_user_registration: bool,
    pub guest_access: bool,
    pub max_upload_size_mb: u64,
    
    // Timeouts
    pub login_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    
    // Database
    pub database_url: String,   // SQLite or PostgreSQL connection string
}
```

---

## 5. Async Runtime & Concurrency

### 5.1 Tokio-Based Architecture

All I/O operations use Tokio's async runtime:

```rust
/// Main server entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let config = SystemConfig::load("impulse.toml")?;
    
    // Initialize storage layer
    let storage = StorageLayer::new(&config.database_url).await?;
    
    // Create session manager
    let session_mgr = Arc::new(SessionManager::new(config.clone(), storage));
    
    // Spawn servers
    let telnet_handle = tokio::spawn(
        telnet_server::run(config.telnet_port, session_mgr.clone())
    );
    
    let ssh_handle = tokio::spawn(
        ssh_server::run(config.ssh_port, session_mgr.clone())
    );
    
    // Wait for shutdown signal
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Shutdown signal received");
        }
        _ = telnet_handle => {},
        _ = ssh_handle => {},
    }
    
    // Graceful shutdown
    session_mgr.shutdown().await?;
    
    Ok(())
}
```

### 5.2 Session Management

Each user connection spawns an independent async task:

```rust
/// Manages all active sessions
pub struct SessionManager {
    config: SystemConfig,
    storage: Arc<StorageLayer>,
    sessions: Arc<RwLock<HashMap<SessionId, Arc<Session>>>>,
    shutdown: Arc<Notify>,
}

impl SessionManager {
    /// Spawn a new session for an incoming connection
    pub async fn spawn_session(
        &self,
        stream: Box<dyn AsyncRead + AsyncWrite + Unpin + Send>,
    ) -> Result<SessionId> {
        let session_id = SessionId::generate();
        let session = Arc::new(Session::new(
            session_id,
            stream,
            self.storage.clone(),
            self.config.clone(),
        ));
        
        // Store session
        self.sessions.write().await.insert(session_id, session.clone());
        
        // Spawn session event loop
        let sessions = self.sessions.clone();
        let shutdown = self.shutdown.clone();
        tokio::spawn(async move {
            if let Err(e) = session.run(shutdown).await {
                error!("Session error: {}", e);
            }
            
            // Remove from active sessions
            sessions.write().await.remove(&session_id);
        });
        
        Ok(session_id)
    }
}
```

### 5.3 Concurrency Safety

**Key Strategies**:
1. **Arc for shared ownership**: Session manager, storage layer, config
2. **RwLock for concurrent reads**: Session list, user cache
3. **Mutex for exclusive writes**: Message posting, file uploads
4. **Message passing (channels)**: Inter-session communication, broadcasts

```rust
/// Example: Broadcasting a system message to all sessions
pub struct SessionManager {
    // ...
    broadcast_tx: broadcast::Sender<SystemEvent>,
}

impl SessionManager {
    /// Send a system-wide notification
    pub async fn broadcast(&self, event: SystemEvent) {
        let _ = self.broadcast_tx.send(event);
    }
}

/// In each session
impl Session {
    async fn run(&self, shutdown: Arc<Notify>) {
        let mut broadcast_rx = self.manager.broadcast_tx.subscribe();
        
        loop {
            tokio::select! {
                // User input
                result = self.read_input() => { /* ... */ }
                
                // Broadcast messages
                Ok(event) = broadcast_rx.recv() => {
                    self.handle_system_event(event).await;
                }
                
                // Shutdown signal
                _ = shutdown.notified() => {
                    break;
                }
            }
        }
    }
}
```

---

## 6. Protocol Implementations

### 6.1 Telnet Server

```rust
/// Telnet server using tokio
pub async fn run(port: u16, session_mgr: Arc<SessionManager>) -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Telnet server listening on port {}", port);
    
    loop {
        let (socket, addr) = listener.accept().await?;
        info!("Telnet connection from {}", addr);
        
        let mgr = session_mgr.clone();
        tokio::spawn(async move {
            // Perform telnet negotiation
            let mut telnet = TelnetStream::new(socket);
            telnet.negotiate().await?;
            
            // Spawn session
            mgr.spawn_session(Box::new(telnet)).await?;
            
            Ok::<_, Error>(())
        });
    }
}

/// Telnet protocol wrapper
pub struct TelnetStream {
    stream: TcpStream,
    state: TelnetState,
}

impl TelnetStream {
    /// Perform IAC (Interpret As Command) negotiation
    async fn negotiate(&mut self) -> Result<()> {
        // WILL ECHO - server will echo
        self.send_command(TelnetCommand::Will(TelnetOption::Echo)).await?;
        
        // DO TERMINAL_TYPE - query terminal type
        self.send_command(TelnetCommand::Do(TelnetOption::TerminalType)).await?;
        
        // WILL SUPPRESS_GO_AHEAD - full duplex
        self.send_command(TelnetCommand::Will(TelnetOption::SuppressGoAhead)).await?;
        
        // Read responses
        // ...
        
        Ok(())
    }
}
```

### 6.2 SSH Server

Use the `russh` crate for SSH protocol handling:

```rust
use russh::server::{Server, Session, Handler};
use russh::ChannelId;

pub async fn run(port: u16, session_mgr: Arc<SessionManager>) -> Result<()> {
    let config = russh::server::Config {
        keys: vec![load_host_key()?],
        ..Default::default()
    };
    
    let server = ImpulseSSHServer { session_mgr };
    server.run_on_address(Arc::new(config), ("0.0.0.0", port)).await?;
    
    Ok(())
}

struct ImpulseSSHServer {
    session_mgr: Arc<SessionManager>,
}

#[async_trait]
impl Server for ImpulseSSHServer {
    type Handler = ImpulseSSHHandler;
    
    fn new_client(&mut self, peer_addr: Option<SocketAddr>) -> Self::Handler {
        ImpulseSSHHandler {
            session_mgr: self.session_mgr.clone(),
            peer_addr,
        }
    }
}

struct ImpulseSSHHandler {
    session_mgr: Arc<SessionManager>,
    peer_addr: Option<SocketAddr>,
}

#[async_trait]
impl Handler for ImpulseSSHHandler {
    async fn channel_open_session(
        self,
        channel: ChannelId,
        session: Session,
    ) -> Result<(Self, bool, Session), Self::Error> {
        // Spawn BBS session
        // ...
        Ok((self, true, session))
    }
    
    async fn auth_password(
        self,
        user: &str,
        password: &str,
    ) -> Result<(Self, Auth), Self::Error> {
        // Authenticate against user database
        // ...
    }
}
```

---

## 7. Storage Layer

### 7.1 Database Abstraction

```rust
/// Abstract storage interface
#[async_trait]
pub trait Storage: Send + Sync {
    // User operations
    async fn get_user(&self, id: u32) -> Result<Option<User>>;
    async fn get_user_by_name(&self, username: &str) -> Result<Option<User>>;
    async fn create_user(&self, user: &User) -> Result<u32>;
    async fn update_user(&self, user: &User) -> Result<()>;
    
    // File operations
    async fn get_file(&self, id: u64) -> Result<Option<FileRecord>>;
    async fn search_files(&self, query: &FileSearchQuery) -> Result<Vec<FileRecord>>;
    async fn add_file(&self, file: &FileRecord) -> Result<u64>;
    
    // Configuration
    async fn get_config(&self) -> Result<SystemConfig>;
    async fn set_config(&self, config: &SystemConfig) -> Result<()>;
}
```

### 7.2 SQLite Implementation

Primary database for single-server deployments:

```rust
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn get_user(&self, id: u32) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    // ... other implementations
}
```

### 7.3 Migration from Legacy Formats

```rust
/// Import tool for Impulse 7.1 data files
pub struct LegacyImporter {
    storage: Arc<dyn Storage>,
}

impl LegacyImporter {
    /// Import users from USERS.DAT
    pub async fn import_users(&self, path: &Path) -> Result<usize> {
        let mut file = File::open(path)?;
        let mut count = 0;
        
        loop {
            // Read Pascal record structure
            let record = match read_pascal_user_record(&mut file) {
                Ok(r) => r,
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            };
            
            // Convert to modern User struct
            let user = convert_pascal_user(&record)?;
            
            // Store in database
            self.storage.create_user(&user).await?;
            count += 1;
        }
        
        info!("Imported {} users", count);
        Ok(count)
    }
    
    /// Convert Pascal string format to Rust String
    fn read_pascal_string(reader: &mut impl Read) -> Result<String> {
        let mut len_buf = [0u8; 1];
        reader.read_exact(&mut len_buf)?;
        let len = len_buf[0] as usize;
        
        let mut str_buf = vec![0u8; len];
        reader.read_exact(&mut str_buf)?;
        
        // Handle CP437 encoding
        let s = encoding_rs::IBM866.decode(&str_buf).0.to_string();
        Ok(s)
    }
}
```

---

## 8. Security Architecture

### 8.1 Authentication

```rust
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;

/// Authentication service
pub struct AuthService {
    storage: Arc<dyn Storage>,
    argon2: Argon2<'static>,
}

impl AuthService {
    /// Hash a password for storage
    pub fn hash_password(&self, password: &str) -> Result<(String, Vec<u8>)> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self.argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        
        Ok((hash, salt.as_str().as_bytes().to_vec()))
    }
    
    /// Verify a password against stored hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    /// Authenticate a user
    pub async fn authenticate(&self, username: &str, password: &str) -> Result<Option<User>> {
        let user = self.storage.get_user_by_name(username).await?;
        
        match user {
            Some(u) if self.verify_password(password, &u.password_hash)? => Ok(Some(u)),
            _ => Ok(None),
        }
    }
}
```

### 8.2 Rate Limiting

```rust
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;

/// Rate limiter for login attempts
pub struct LoginRateLimiter {
    limiter: RateLimiter<
        String,                    // Key: IP address or username
        governor::state::InMemoryState,
        governor::clock::DefaultClock,
    >,
}

impl LoginRateLimiter {
    pub fn new() -> Self {
        // 5 attempts per minute
        let quota = Quota::per_minute(NonZeroU32::new(5).unwrap());
        
        Self {
            limiter: RateLimiter::keyed(quota),
        }
    }
    
    /// Check if attempt is allowed
    pub fn check_attempt(&self, key: &str) -> bool {
        self.limiter.check_key(&key.to_string()).is_ok()
    }
}
```

### 8.3 Input Validation

```rust
/// Validate username format
pub fn validate_username(username: &str) -> Result<()> {
    if username.len() < 3 || username.len() > 30 {
        return Err(Error::ValidationError("Username must be 3-30 characters".into()));
    }
    
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(Error::ValidationError("Username contains invalid characters".into()));
    }
    
    Ok(())
}

/// Sanitize user input for display
pub fn sanitize_for_ansi(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\r')
        .collect()
}
```

---

## 9. Testing Strategy

### 9.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_username() {
        assert!(validate_username("validuser").is_ok());
        assert!(validate_username("ab").is_err()); // Too short
        assert!(validate_username("user@domain").is_err()); // Invalid char
    }
    
    #[tokio::test]
    async fn test_user_authentication() {
        let storage = MockStorage::new();
        let auth = AuthService::new(Arc::new(storage));
        
        // Create test user
        let (hash, salt) = auth.hash_password("testpass123").unwrap();
        // ... store in mock storage ...
        
        // Test valid password
        let result = auth.authenticate("testuser", "testpass123").await.unwrap();
        assert!(result.is_some());
        
        // Test invalid password
        let result = auth.authenticate("testuser", "wrongpass").await.unwrap();
        assert!(result.is_none());
    }
}
```

### 9.2 Integration Tests

```rust
// tests/integration_test.rs

use impulse_bbs::*;
use tokio::net::TcpStream;

#[tokio::test]
async fn test_full_login_flow() {
    // Start test server
    let server = TestServer::start().await;
    
    // Connect via telnet
    let stream = TcpStream::connect(server.addr()).await.unwrap();
    let mut client = TelnetClient::new(stream);
    
    // Read welcome screen
    let welcome = client.read_until_prompt().await.unwrap();
    assert!(welcome.contains("Impulse BBS"));
    
    // Enter username
    client.send("testuser\r\n").await.unwrap();
    
    // Enter password
    client.send("testpass\r\n").await.unwrap();
    
    // Verify login success
    let response = client.read_until_prompt().await.unwrap();
    assert!(response.contains("Main Menu"));
    
    // Cleanup
    server.shutdown().await;
}
```

### 9.3 Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_ansi_parser_doesnt_crash(s in "\\PC*") {
        let renderer = AnsiRenderer::new(MockTerminal::new());
        // Should never panic, regardless of input
        let _ = renderer.parse_ansi(s.as_bytes());
    }
    
    #[test]
    fn test_username_validation_consistent(s in "[a-zA-Z0-9_-]{3,30}") {
        // Valid usernames should always pass
        assert!(validate_username(&s).is_ok());
    }
}
```

---

## 10. Deployment Architecture

### 10.1 Containerized Deployment

```dockerfile
# Dockerfile
FROM rust:1.75-slim as builder

WORKDIR /usr/src/impulse
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/impulse/target/release/impulse-server /usr/local/bin/
COPY ./config/impulse.toml /etc/impulse/

EXPOSE 23 22 8080
VOLUME ["/var/lib/impulse"]

CMD ["impulse-server", "--config", "/etc/impulse/impulse.toml"]
```

### 10.2 Kubernetes Deployment

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: impulse-bbs
spec:
  replicas: 1
  selector:
    matchLabels:
      app: impulse-bbs
  template:
    metadata:
      labels:
        app: impulse-bbs
    spec:
      containers:
      - name: impulse
        image: impulse-bbs:latest
        ports:
        - containerPort: 23
          name: telnet
        - containerPort: 22
          name: ssh
        - containerPort: 8080
          name: http
        volumeMounts:
        - name: data
          mountPath: /var/lib/impulse
        - name: config
          mountPath: /etc/impulse
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: impulse-data
      - name: config
        configMap:
          name: impulse-config
```

### 10.3 Monitoring & Observability

```rust
use prometheus::{Registry, Counter, Histogram, IntGauge};

/// Metrics collector
pub struct Metrics {
    pub active_sessions: IntGauge,
    pub total_logins: Counter,
    pub login_duration: Histogram,
    pub message_posts: Counter,
    pub file_downloads: Counter,
}

impl Metrics {
    pub fn new(registry: &Registry) -> Result<Self> {
        let active_sessions = IntGauge::new(
            "impulse_active_sessions",
            "Number of active user sessions"
        )?;
        
        let total_logins = Counter::new(
            "impulse_total_logins",
            "Total number of logins"
        )?;
        
        // Register with Prometheus
        registry.register(Box::new(active_sessions.clone()))?;
        registry.register(Box::new(total_logins.clone()))?;
        
        Ok(Self {
            active_sessions,
            total_logins,
            // ...
        })
    }
}
```

---

## Conclusion

This architecture provides a solid foundation for modernizing Impulse BBS while maintaining its essential character. The modular design, async runtime, and comprehensive testing strategy ensure that the Rust implementation will be safer, faster, and more maintainable than the original Pascal codebase.

Key architectural decisions:
- **Tokio for async I/O** - Enables efficient handling of multiple concurrent sessions
- **Trait-based abstractions** - Allows multiple implementations (SQLite, PostgreSQL, legacy formats)
- **Type-safe storage** - Rust's type system prevents entire classes of data corruption bugs
- **Modern security** - Argon2id password hashing, rate limiting, input validation
- **Cloud-ready** - Container and Kubernetes support for modern deployment

The architecture is designed to evolve: start with core functionality, then layer on advanced features like federation, web UI, and IoT support.

---

**Document Version**: 1.0  
**Last Updated**: 2025-01-21  
**Author**: Impulse Modernization Team - Architecture Division  
**Status**: Design Phase
