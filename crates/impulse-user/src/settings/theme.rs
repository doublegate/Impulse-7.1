//! Theme selection and management
//!
//! This module provides theme definitions and selection logic.

use serde::{Deserialize, Serialize};

/// Available BBS themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Theme {
    /// Classic BBS (ANSI colors, traditional)
    #[default]
    Classic,
    /// Modern dark theme
    Dark,
    /// Modern light theme
    Light,
    /// Cyberpunk neon theme
    Cyberpunk,
    /// Retro green monochrome
    RetroGreen,
    /// Retro amber monochrome
    RetroAmber,
}

impl Theme {
    /// Get the theme name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Classic => "Classic BBS",
            Theme::Dark => "Modern Dark",
            Theme::Light => "Modern Light",
            Theme::Cyberpunk => "Cyberpunk",
            Theme::RetroGreen => "Retro Green",
            Theme::RetroAmber => "Retro Amber",
        }
    }

    /// Get the theme description
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Theme::Classic => "Traditional BBS colors (ANSI)",
            Theme::Dark => "Modern dark theme with high contrast",
            Theme::Light => "Modern light theme for bright terminals",
            Theme::Cyberpunk => "Neon colors for a futuristic look",
            Theme::RetroGreen => "Monochrome green like old terminals",
            Theme::RetroAmber => "Monochrome amber like old terminals",
        }
    }

    /// Get all available themes
    #[must_use]
    pub fn all() -> Vec<Theme> {
        vec![
            Theme::Classic,
            Theme::Dark,
            Theme::Light,
            Theme::Cyberpunk,
            Theme::RetroGreen,
            Theme::RetroAmber,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_name() {
        assert_eq!(Theme::Classic.name(), "Classic BBS");
        assert_eq!(Theme::Cyberpunk.name(), "Cyberpunk");
    }

    #[test]
    fn test_theme_description() {
        assert!(Theme::Classic.description().contains("Traditional"));
        assert!(Theme::Dark.description().contains("dark"));
    }

    #[test]
    fn test_all_themes() {
        let themes = Theme::all();
        assert_eq!(themes.len(), 6);
        assert!(themes.contains(&Theme::Classic));
        assert!(themes.contains(&Theme::Cyberpunk));
    }

    #[test]
    fn test_default_theme() {
        assert_eq!(Theme::default(), Theme::Classic);
    }
}
