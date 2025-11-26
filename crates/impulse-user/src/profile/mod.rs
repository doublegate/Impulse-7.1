//! User profile display and viewing
//!
//! This module provides functionality for displaying user profiles with proper
//! privacy enforcement and formatting.

pub mod display;
pub mod view;

pub use display::{ProfileDisplayOptions, format_profile, format_signature};
pub use view::ProfileViewer;
