//! Terminal rendering and ANSI handling for Impulse BBS
//!
//! This crate provides ANSI escape sequence rendering, color management,
//! and terminal screen control for BBS display.
//!
//! # Features
//!
//! - ANSI color codes (16 colors, 256 colors, RGB)
//! - Cursor movement and positioning
//! - Screen clearing and scrolling
//! - Text attributes (bold, blink, underline, reverse)
//! - ANSI file rendering (.ANS files)
//! - Terminal capability detection
//!
//! # Example
//!
//! ```
//! use impulse_terminal::{AnsiRenderer, Color};
//!
//! let mut renderer = AnsiRenderer::new();
//! renderer.set_foreground(Color::BrightCyan);
//! renderer.set_background(Color::Blue);
//! renderer.write_text("Hello, BBS!");
//! let output = renderer.get_output();
//! ```

mod ansi;
mod color;
mod error;
mod renderer;

pub use ansi::{AnsiCode, AnsiSequence};
pub use color::{AnsiColor, Color};
pub use error::{Result, TerminalError};
pub use renderer::AnsiRenderer;
