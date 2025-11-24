//! Shared data types for Impulse 7.1 BBS
//!
//! This crate provides the fundamental data structures, error types, and constants
//! used throughout the Impulse 7.1 BBS modernization project. All types support
//! serialization via Serde for JSON and binary formats.
//!
//! # Core Modules
//!
//! - [`error`] - Unified error handling framework
//! - [`user`] - User account data structures
//! - [`message`] - Message board data structures
//! - [`file`] - File area data structures
//! - [`config`] - BBS configuration structures
//! - [`session`] - User session data structures
//!
//! # Pascal Compatibility Modules
//!
//! - [`user_flags`] - User permission and preference flags
//! - [`message_enums`] - Message board enumeration types
//! - [`board_flags`] - Board and conference flags
//! - [`menu_flags`] - Menu and command flags
//! - [`protocol_flags`] - File transfer protocol flags
//! - [`pascal_types`] - Pascal type definitions (AR flags, colors, etc.)
//! - [`pascal_user`] - Pascal user record (USER.LST format)
//! - [`pascal_config`] - Pascal system configuration (SYSTAT.DAT format)
//! - [`pascal_message`] - Pascal message system records (*.MIX, *.BRD, BOARDS.DAT formats)
//! - [`pascal_file`] - Pascal file system records (UPLOADS.DAT, *.DIR, VERBOSE.DAT formats)
//! - [`pascal_aux`] - Pascal auxiliary records (NAMES.LST, ZSCAN.DAT, ZLOG.DAT formats)

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Error handling framework
pub mod error;

/// BBS configuration types
pub mod config;

/// User data types
pub mod user;

/// Message data types
pub mod message;

/// File data types
pub mod file;

/// Session data types
pub mod session;

/// User flags and permissions (Pascal compatibility)
pub mod user_flags;

/// Message board enumeration types (Pascal compatibility)
pub mod message_enums;

/// Board and conference flags (Pascal compatibility)
pub mod board_flags;

/// Menu and command flags (Pascal compatibility)
pub mod menu_flags;

/// File transfer protocol flags (Pascal compatibility)
pub mod protocol_flags;

/// Pascal-compatible type definitions
pub mod pascal_types;

/// Pascal-compatible user record (USER.LST format)
pub mod pascal_user;

/// Pascal-compatible system configuration (SYSTAT.DAT format)
pub mod pascal_config;

/// Pascal-compatible message system records (*.MIX, *.BRD, BOARDS.DAT formats)
pub mod pascal_message;

/// Pascal-compatible file system records (UPLOADS.DAT, *.DIR, VERBOSE.DAT formats)
pub mod pascal_file;

/// Pascal-compatible auxiliary records (NAMES.LST, ZSCAN.DAT, ZLOG.DAT formats)
pub mod pascal_aux;

// Re-export commonly used types for convenience
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
