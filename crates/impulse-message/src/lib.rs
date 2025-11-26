//! Message base management for Impulse 7.1 BBS
//!
//! This crate provides complete message base functionality,
//! supporting multiple formats (JAM, Hudson) with threaded conversations,
//! message posting, reply functionality, search capabilities, UI screens,
//! and advanced features including QWK offline mail, FidoNet addressing,
//! and message routing.
//!
//! # Features
//!
//! - **Multiple Formats**: JAM and Hudson message base formats
//! - **Message Writing**: Post new messages and replies
//! - **Threaded Discussions**: Full thread support with parent/child relationships
//! - **Message Quoting**: Quote original messages in replies
//! - **Validation**: Comprehensive message validation and sanitization
//! - **Atomic Writes**: Safe, atomic file operations to prevent corruption
//! - **Search**: Search messages by from, to, subject, body, and date
//! - **UI Screens**: Message list and read screens for display
//! - **QWK Support**: Generate QWK offline mail packets and parse reply packets
//! - **FidoNet Addressing**: Full FidoNet address parsing (zone:net/node.point)
//! - **Message Routing**: Intelligent routing decisions for networked messages
//! - **Import/Export**: Export messages to text/JSON/CSV, import from text/JSON
//! - **Async**: Fully async API using tokio
//!
//! # Examples
//!
//! ## Reading messages from a JAM base
//!
//! ```no_run
//! use impulse_message::formats::JamMessageBase;
//! use impulse_message::traits::MessageBase;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let base = JamMessageBase::new("/path/to/base");
//! let message = base.read_message(1).await?;
//! println!("From: {}", message.header.from);
//! println!("Subject: {}", message.header.subject);
//! println!("Body: {}", message.body);
//! # Ok(())
//! # }
//! ```
//!
//! ## Posting a new message
//!
//! ```no_run
//! use impulse_message::formats::JamMessageBase;
//! use impulse_message::traits::MessageBase;
//! use impulse_message::types::NewMessage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut base = JamMessageBase::new("/path/to/base");
//! let message = NewMessage::new("Alice", "Bob", "Hello!")
//!     .with_body("This is my first message.");
//!
//! let msg_num = base.post_message(message).await?;
//! println!("Posted message #{}", msg_num);
//! # Ok(())
//! # }
//! ```
//!
//! ## Replying to a message with quoting
//!
//! ```no_run
//! use impulse_message::formats::JamMessageBase;
//! use impulse_message::traits::MessageBase;
//! use impulse_message::reply::ReplyBuilder;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut base = JamMessageBase::new("/path/to/base");
//!
//! // Read original message
//! let original = base.read_message(1).await?;
//!
//! // Build reply with quoting
//! let reply = ReplyBuilder::new(original)
//!     .quote_original(true)
//!     .build("Bob", "Thanks for your message!");
//!
//! // Post the reply
//! let msg_num = base.post_message(reply).await?;
//! println!("Posted reply #{}", msg_num);
//! # Ok(())
//! # }
//! ```
//!
//! ## Searching messages
//!
//! ```no_run
//! use impulse_message::formats::JamMessageBase;
//! use impulse_message::traits::MessageBase;
//! use impulse_message::types::SearchCriteria;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let base = JamMessageBase::new("/path/to/base");
//! let criteria = SearchCriteria::new()
//!     .with_from("Alice")
//!     .with_subject("test");
//!
//! let results = base.search(&criteria).await?;
//! for msg_num in results {
//!     let msg = base.read_message(msg_num).await?;
//!     println!("Found: {}", msg.header.subject);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Displaying message list
//!
//! ```no_run
//! use impulse_message::formats::JamMessageBase;
//! use impulse_message::screens::MessageListScreen;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let base = JamMessageBase::new("/path/to/base");
//! let mut screen = MessageListScreen::default_config();
//! screen.load_page(&base, 0).await?;
//! println!("{}", screen.render());
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Error types
pub mod error;

/// Message base types and structures
pub mod types;

/// Message base trait definition
pub mod traits;

/// Message format implementations
pub mod formats;

/// UI screens
pub mod screens;

/// Message validation
pub mod validation;

/// Content sanitization
pub mod sanitize;

/// Atomic file operations
pub mod atomic;

/// Message quoting
pub mod quote;

/// Reply functionality
pub mod reply;

/// QWK offline mail packet support
pub mod qwk;

/// Message addressing (FidoNet)
pub mod addressing;

/// Message routing
pub mod routing;

/// Message export
pub mod export;

/// Message import
pub mod import;

// Re-export commonly used types
pub use error::{MessageError, Result};
pub use reply::ReplyBuilder;
pub use sanitize::MessageSanitizer;
pub use traits::MessageBase;
pub use types::{
    FullMessage, KludgeLine, MessageBaseStats, MessageHeader, MessageThread, NewMessage,
    SearchCriteria, ValidationLimits,
};
pub use validation::MessageValidator;
