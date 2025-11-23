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

// Re-export commonly used types for convenience
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
