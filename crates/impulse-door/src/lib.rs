//! Door game interface for Impulse-Next BBS.
//!
//! This crate provides comprehensive support for running BBS door games,
//! including:
//!
//! - Dropfile generation (DOOR.SYS, DORINFO1.DEF, etc.)
//! - Door configuration and management
//! - Native and DOSBox execution support
//! - Session management with time tracking
//! - Async I/O handling
//!
//! # Examples
//!
//! ## Basic door execution
//!
//! ```no_run
//! use impulse_door::{DoorManager, DoorExecutor, DoorSession};
//! use std::sync::Arc;
//! use std::path::PathBuf;
//! use chrono::Utc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create door manager
//! let door_dir = PathBuf::from("/bbs/doors");
//! let node_dir = PathBuf::from("/bbs/nodes");
//! let manager = Arc::new(DoorManager::new(door_dir, node_dir).await?);
//!
//! // Create door executor
//! let executor = DoorExecutor::new(manager);
//!
//! // Create session
//! let mut session = DoorSession {
//!     node_id: 1,
//!     user_name: "John Doe".to_string(),
//!     user_alias: None,
//!     location: "Seattle, WA".to_string(),
//!     security_level: 50,
//!     time_remaining_seconds: 3600,
//!     ansi_enabled: true,
//!     login_time: Utc::now(),
//!     total_calls: 10,
//!     last_call_date: "11/26/25".to_string(),
//!     upload_kb: 100,
//!     download_kb: 200,
//! };
//!
//! // Execute door
//! let result = executor.execute("tradewars", &mut session).await?;
//! println!("Door exited with code: {}", result.exit_code);
//! # Ok(())
//! # }
//! ```
//!
//! ## Dropfile generation
//!
//! ```no_run
//! use impulse_door::{DropfileGenerator, DropfileType, DoorSession};
//! use std::path::PathBuf;
//! use chrono::Utc;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let session = DoorSession {
//!     node_id: 1,
//!     user_name: "Test User".to_string(),
//!     user_alias: None,
//!     location: "Unknown".to_string(),
//!     security_level: 10,
//!     time_remaining_seconds: 3600,
//!     ansi_enabled: true,
//!     login_time: Utc::now(),
//!     total_calls: 1,
//!     last_call_date: "11/26/25".to_string(),
//!     upload_kb: 0,
//!     download_kb: 0,
//! };
//!
//! let output_dir = PathBuf::from("/tmp/node1");
//! let dropfile_path = DropfileGenerator::generate(
//!     DropfileType::DoorSys,
//!     &session,
//!     &output_dir,
//! )?;
//!
//! println!("Created dropfile: {:?}", dropfile_path);
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod dropfiles;
pub mod error;
pub mod executor;
pub mod io;
pub mod manager;
pub mod session;

// Re-export commonly used types
pub use config::{DoorConfig, DosBoxConfig};
pub use dropfiles::{DorinfoDropfile, DoorSysDropfile, DropfileGenerator, DropfileType};
pub use error::{DoorError, Result};
pub use executor::{DoorExecutor, DoorResult};
pub use io::DoorIoHandler;
pub use manager::DoorManager;
pub use session::DoorSession;
