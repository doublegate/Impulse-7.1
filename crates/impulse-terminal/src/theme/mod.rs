//! Theme system for BBS terminal customization
//!
//! This module provides comprehensive theming capabilities including:
//! - Theme loading from TOML files
//! - Color scheme management
//! - Theme switching with preview
//! - Hot-reload support
//!
//! # Example
//!
//! ```no_run
//! use impulse_terminal::theme::{ThemeManager, Theme};
//! use std::path::PathBuf;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let theme_dir = PathBuf::from("themes");
//! let mut manager = ThemeManager::new(theme_dir).await?;
//!
//! // List available themes
//! let themes = manager.list_themes().await;
//! for theme in themes {
//!     println!("Theme: {} by {}", theme.name, theme.author);
//! }
//!
//! // Switch to a theme
//! manager.switch_theme("matrix").await?;
//!
//! // Get current theme colors
//! let colors = manager.get_color_scheme().await;
//! # Ok(())
//! # }
//! ```

mod apply;
mod loader;
mod manager;
mod metadata;
mod preview;
mod selector;
mod types;

pub use apply::apply_theme;
pub use loader::ThemeLoader;
pub use manager::ThemeManager;
pub use metadata::ThemeMetadata;
pub use preview::ThemePreview;
pub use selector::ThemeSelector;
pub use types::{ColorScheme, Theme, ThemeAnsiColor};
