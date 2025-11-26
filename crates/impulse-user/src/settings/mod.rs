//! User settings and preferences management
//!
//! This module provides functionality for managing user settings and preferences,
//! including password changes, theme selection, terminal configuration, and
//! protocol preferences for file transfers.

pub mod manager;
pub mod password;
pub mod protocol;
pub mod theme;

pub use manager::SettingsManager;
pub use password::{PasswordStrength, validate_password};
pub use protocol::ProtocolSettings;
pub use theme::Theme;
