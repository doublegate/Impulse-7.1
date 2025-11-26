//! Session management for Impulse BBS
//!
//! This crate provides session tracking and lifecycle management for BBS users.
//! It handles connection state, user identification, timeouts, and concurrent session limits.
//!
//! # Features
//!
//! - Session creation and tracking
//! - Automatic timeout handling
//! - User identification and authentication state
//! - Terminal capability tracking
//! - Concurrent session management
//! - Activity monitoring
//!
//! # Example
//!
//! ```no_run
//! use impulse_session::{SessionManager, SessionConfig, Result};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = SessionConfig::default()
//!         .with_idle_timeout(Duration::from_secs(900))
//!         .with_max_sessions_per_user(3);
//!
//!     let manager = SessionManager::new(config);
//!
//!     // Create a new session
//!     let session_id = manager.create_session("192.168.1.1:1234").await?;
//!     println!("Created session: {}", session_id);
//!
//!     Ok(())
//! }
//! ```

mod config;
mod connection;
mod error;
mod manager;
mod session;

#[cfg(feature = "websocket")]
mod websocket;

pub use config::{ConflictPolicy, SessionConfig};
pub use connection::{Connection, ConnectionError, ConnectionType};
pub use error::{Result, SessionError};
pub use manager::SessionManager;
pub use session::{Session, SessionId, SessionState};

#[cfg(feature = "websocket")]
pub use websocket::{BbsMessage, NotificationLevel, SessionEvent, WebSocketConnection};
