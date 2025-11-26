//! QWK offline mail packet support
//!
//! QWK is a standard format for offline mail packets used by BBS systems.
//! This module provides complete support for generating QWK packets for download
//! and parsing QWK reply packets uploaded by users.
//!
//! # QWK Format Overview
//!
//! A QWK packet is a ZIP archive containing:
//! - `CONTROL.DAT` - BBS configuration and message area information
//! - `DOOR.ID` - BBS identification and version information
//! - `MESSAGES.DAT` - Message data in 128-byte blocks
//!
//! Reply packets (.REP files) have a similar structure with user messages.
//!
//! # Examples
//!
//! ## Generating a QWK packet
//!
//! ```no_run
//! use impulse_message::qwk::{QwkConfig, QwkPacketGenerator};
//! use impulse_message::types::FullMessage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = QwkConfig {
//!     bbs_name: "My BBS".to_string(),
//!     bbs_location: "Cyberspace".to_string(),
//!     sysop_name: "Sysop".to_string(),
//!     phone: "555-1234".to_string(),
//!     default_conference: 1,
//! };
//!
//! let mut generator = QwkPacketGenerator::new(config);
//! // Add messages...
//! generator.generate("mail.qwk").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Parsing a reply packet
//!
//! ```no_run
//! use impulse_message::qwk::QwkReplyParser;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut parser = QwkReplyParser::open("reply.rep")?;
//! let messages = parser.parse_messages()?;
//!
//! for msg in messages {
//!     println!("To: {}", msg.to);
//!     println!("Subject: {}", msg.subject);
//!     println!("Body: {}", msg.body);
//! }
//! # Ok(())
//! # }
//! ```

pub mod compress;
pub mod error;
pub mod generate;
pub mod header;
pub mod parse;

pub use compress::{QwkCompressor, QwkDecompressor};
pub use error::{QwkError, Result};
pub use generate::{QwkConfig, QwkPacketGenerator};
pub use header::{MessageStatus, QwkMessageHeader};
pub use parse::{ParsedReply, QwkReplyParser};
