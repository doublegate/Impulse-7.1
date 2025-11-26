//! Protocol implementations for Impulse BBS.
//!
//! This crate provides implementations of various file transfer and communication protocols
//! used in BBS systems.
//!
//! # Protocols
//!
//! ## Zmodem
//!
//! A robust file transfer protocol with error correction and resume capabilities.
//! See the [`zmodem`] module for details.
//!
//! # Examples
//!
//! ```
//! use impulse_protocol::zmodem::{FrameType, FrameEncoding, ZmodemFrame};
//!
//! let frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
//! let serialized = frame.serialize();
//! ```

pub mod zmodem;
