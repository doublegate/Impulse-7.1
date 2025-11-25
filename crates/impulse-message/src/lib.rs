//! Message base management for Impulse 7.1 BBS
//!
//! This crate provides message base reading and management functionality,
//! supporting multiple formats (JAM, Hudson) with threaded conversations,
//! search capabilities, and UI screens.
//!
//! # Features
//!
//! - **Multiple Formats**: JAM and Hudson message base formats
//! - **Threaded Discussions**: Full thread support with parent/child relationships
//! - **Search**: Search messages by from, to, subject, body, and date
//! - **UI Screens**: Message list and read screens for display
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

// Re-export commonly used types
pub use error::{MessageError, Result};
pub use traits::MessageBase;
pub use types::{
    FullMessage, KludgeLine, MessageBaseStats, MessageHeader, MessageThread, SearchCriteria,
};
