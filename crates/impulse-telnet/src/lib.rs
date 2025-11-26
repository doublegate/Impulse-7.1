//! Telnet protocol implementation for Impulse BBS
//!
//! This crate provides a full-featured Telnet server implementation that
//! supports IAC (Interpret As Command) negotiation, option handling, and
//! terminal emulation for BBS connectivity.
//!
//! # Features
//!
//! - RFC 854 Telnet Protocol
//! - RFC 857 Echo Option
//! - RFC 858 Suppress Go Ahead
//! - RFC 1073 Window Size Negotiation
//! - Async/await based on Tokio
//! - Connection lifecycle management
//!
//! # Example
//!
//! ```no_run
//! use impulse_telnet::{TelnetServer, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let server = TelnetServer::bind("0.0.0.0:23").await?;
//!
//!     loop {
//!         let mut connection = server.accept().await?;
//!         tokio::spawn(async move {
//!             // Handle connection
//!         });
//!     }
//! }
//! ```

mod connection;
mod error;
mod iac;
mod server;

pub use connection::TelnetConnection;
pub use error::{Result, TelnetError};
pub use iac::{IacCommand, TelnetOption};
pub use server::TelnetServer;
