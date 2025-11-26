//! User settings and preferences management
//!
//! This module provides functionality for managing user settings and preferences,
//! including password changes, theme selection, and terminal configuration.

pub mod manager;
pub mod password;
pub mod theme;

pub use manager::SettingsManager;
pub use password::{PasswordStrength, validate_password};
pub use theme::Theme;
