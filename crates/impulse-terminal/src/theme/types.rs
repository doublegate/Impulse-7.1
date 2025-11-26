//! Core theme types and structures

use crate::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::metadata::ThemeMetadata;

/// A complete theme definition including metadata, colors, screens, and prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme metadata (name, author, version, etc.)
    pub metadata: ThemeMetadata,
    /// Color scheme for the theme
    pub color_scheme: ColorScheme,
    /// Screen template paths (screen_name -> file_path)
    #[serde(default)]
    pub screens: HashMap<String, String>,
    /// Prompt templates (prompt_id -> template_string)
    #[serde(default)]
    pub prompts: HashMap<String, String>,
}

impl Theme {
    /// Create a new theme with the given metadata and color scheme
    pub fn new(metadata: ThemeMetadata, color_scheme: ColorScheme) -> Self {
        Self {
            metadata,
            color_scheme,
            screens: HashMap::new(),
            prompts: HashMap::new(),
        }
    }

    /// Add a screen template to the theme
    pub fn add_screen(&mut self, name: String, path: String) {
        self.screens.insert(name, path);
    }

    /// Add a prompt template to the theme
    pub fn add_prompt(&mut self, id: String, template: String) {
        self.prompts.insert(id, template);
    }

    /// Get a screen path by name
    pub fn get_screen(&self, name: &str) -> Option<&String> {
        self.screens.get(name)
    }

    /// Get a prompt template by id
    pub fn get_prompt(&self, id: &str) -> Option<&String> {
        self.prompts.get(id)
    }

    /// Validate theme has all required components
    pub fn validate(&self) -> Result<(), String> {
        // Validate metadata
        if self.metadata.name.is_empty() {
            return Err("Theme name cannot be empty".to_string());
        }
        if self.metadata.author.is_empty() {
            return Err("Theme author cannot be empty".to_string());
        }
        if self.metadata.version.is_empty() {
            return Err("Theme version cannot be empty".to_string());
        }

        // Color scheme is always valid as it has defaults
        Ok(())
    }
}

/// Color scheme defining all theme colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    /// Primary color (main UI elements)
    pub primary: ThemeAnsiColor,
    /// Secondary color (less prominent UI elements)
    pub secondary: ThemeAnsiColor,
    /// Accent color (highlights, selected items)
    pub accent: ThemeAnsiColor,
    /// Background color
    pub background: ThemeAnsiColor,
    /// Text color (default text)
    pub text: ThemeAnsiColor,
    /// Highlight color (important information)
    pub highlight: ThemeAnsiColor,
    /// Error color (error messages)
    pub error: ThemeAnsiColor,
    /// Success color (success messages)
    pub success: ThemeAnsiColor,
}

impl Default for ColorScheme {
    fn default() -> Self {
        // Classic BBS blue/white theme
        Self {
            primary: ThemeAnsiColor::new(Color::BrightBlue, Color::Black, false, false),
            secondary: ThemeAnsiColor::new(Color::Cyan, Color::Black, false, false),
            accent: ThemeAnsiColor::new(Color::BrightCyan, Color::Black, true, false),
            background: ThemeAnsiColor::new(Color::White, Color::Black, false, false),
            text: ThemeAnsiColor::new(Color::White, Color::Black, false, false),
            highlight: ThemeAnsiColor::new(Color::BrightYellow, Color::Black, true, false),
            error: ThemeAnsiColor::new(Color::BrightRed, Color::Black, true, false),
            success: ThemeAnsiColor::new(Color::BrightGreen, Color::Black, true, false),
        }
    }
}

impl ColorScheme {
    /// Create a new color scheme with all colors specified
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        primary: ThemeAnsiColor,
        secondary: ThemeAnsiColor,
        accent: ThemeAnsiColor,
        background: ThemeAnsiColor,
        text: ThemeAnsiColor,
        highlight: ThemeAnsiColor,
        error: ThemeAnsiColor,
        success: ThemeAnsiColor,
    ) -> Self {
        Self {
            primary,
            secondary,
            accent,
            background,
            text,
            highlight,
            error,
            success,
        }
    }

    /// Get the primary color
    pub fn primary(&self) -> &ThemeAnsiColor {
        &self.primary
    }

    /// Get the secondary color
    pub fn secondary(&self) -> &ThemeAnsiColor {
        &self.secondary
    }

    /// Get the accent color
    pub fn accent(&self) -> &ThemeAnsiColor {
        &self.accent
    }

    /// Get the background color
    pub fn background(&self) -> &ThemeAnsiColor {
        &self.background
    }

    /// Get the text color
    pub fn text(&self) -> &ThemeAnsiColor {
        &self.text
    }

    /// Get the highlight color
    pub fn highlight(&self) -> &ThemeAnsiColor {
        &self.highlight
    }

    /// Get the error color
    pub fn error(&self) -> &ThemeAnsiColor {
        &self.error
    }

    /// Get the success color
    pub fn success(&self) -> &ThemeAnsiColor {
        &self.success
    }
}

/// ANSI color specification for themes (serializable)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThemeAnsiColor {
    /// Foreground color
    pub foreground: ThemeColor,
    /// Background color
    pub background: ThemeColor,
    /// Bold/bright attribute
    #[serde(default)]
    pub bold: bool,
    /// Blink attribute
    #[serde(default)]
    pub blink: bool,
}

impl ThemeAnsiColor {
    /// Create a new ANSI color specification
    pub fn new(foreground: Color, background: Color, bold: bool, blink: bool) -> Self {
        Self {
            foreground: ThemeColor::from_color(foreground),
            background: ThemeColor::from_color(background),
            bold,
            blink,
        }
    }

    /// Convert to runtime Color types
    pub fn to_colors(&self) -> (Color, Color) {
        (self.foreground.to_color(), self.background.to_color())
    }

    /// Generate ANSI escape sequence for this color
    pub fn to_ansi_sequence(&self) -> String {
        let mut codes = Vec::new();

        // Add bold if enabled
        if self.bold {
            codes.push("1".to_string());
        }

        // Add blink if enabled
        if self.blink {
            codes.push("5".to_string());
        }

        // Add foreground color
        codes.push(self.foreground.to_color().foreground_code());

        // Add background color
        codes.push(self.background.to_color().background_code());

        format!("\x1b[{}m", codes.join(";"))
    }
}

/// Serializable color representation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ThemeColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Ansi256(u8),
    Rgb { r: u8, g: u8, b: u8 },
}

impl ThemeColor {
    /// Convert from runtime Color enum
    pub fn from_color(color: Color) -> Self {
        match color {
            Color::Black => ThemeColor::Black,
            Color::Red => ThemeColor::Red,
            Color::Green => ThemeColor::Green,
            Color::Yellow => ThemeColor::Yellow,
            Color::Blue => ThemeColor::Blue,
            Color::Magenta => ThemeColor::Magenta,
            Color::Cyan => ThemeColor::Cyan,
            Color::White => ThemeColor::White,
            Color::BrightBlack => ThemeColor::BrightBlack,
            Color::BrightRed => ThemeColor::BrightRed,
            Color::BrightGreen => ThemeColor::BrightGreen,
            Color::BrightYellow => ThemeColor::BrightYellow,
            Color::BrightBlue => ThemeColor::BrightBlue,
            Color::BrightMagenta => ThemeColor::BrightMagenta,
            Color::BrightCyan => ThemeColor::BrightCyan,
            Color::BrightWhite => ThemeColor::BrightWhite,
            Color::Ansi256(c) => ThemeColor::Ansi256(c),
            Color::Rgb(r, g, b) => ThemeColor::Rgb { r, g, b },
        }
    }

    /// Convert to runtime Color enum
    pub fn to_color(self) -> Color {
        match self {
            ThemeColor::Black => Color::Black,
            ThemeColor::Red => Color::Red,
            ThemeColor::Green => Color::Green,
            ThemeColor::Yellow => Color::Yellow,
            ThemeColor::Blue => Color::Blue,
            ThemeColor::Magenta => Color::Magenta,
            ThemeColor::Cyan => Color::Cyan,
            ThemeColor::White => Color::White,
            ThemeColor::BrightBlack => Color::BrightBlack,
            ThemeColor::BrightRed => Color::BrightRed,
            ThemeColor::BrightGreen => Color::BrightGreen,
            ThemeColor::BrightYellow => Color::BrightYellow,
            ThemeColor::BrightBlue => Color::BrightBlue,
            ThemeColor::BrightMagenta => Color::BrightMagenta,
            ThemeColor::BrightCyan => Color::BrightCyan,
            ThemeColor::BrightWhite => Color::BrightWhite,
            ThemeColor::Ansi256(c) => Color::Ansi256(c),
            ThemeColor::Rgb { r, g, b } => Color::Rgb(r, g, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let metadata = ThemeMetadata {
            name: "Test Theme".to_string(),
            author: "Test Author".to_string(),
            version: "1.0.0".to_string(),
            description: "Test Description".to_string(),
            compatible_bbs_version: "0.1.0".to_string(),
            requires_ansi: true,
            requires_utf8: false,
        };

        let color_scheme = ColorScheme::default();
        let theme = Theme::new(metadata, color_scheme);

        assert_eq!(theme.metadata.name, "Test Theme");
        assert_eq!(theme.screens.len(), 0);
        assert_eq!(theme.prompts.len(), 0);
    }

    #[test]
    fn test_theme_add_screen() {
        let mut theme = Theme::new(
            ThemeMetadata {
                name: "Test".to_string(),
                author: "Author".to_string(),
                version: "1.0".to_string(),
                description: "Desc".to_string(),
                compatible_bbs_version: "0.1.0".to_string(),
                requires_ansi: true,
                requires_utf8: false,
            },
            ColorScheme::default(),
        );

        theme.add_screen("main".to_string(), "screens/main.ans".to_string());
        assert_eq!(
            theme.get_screen("main"),
            Some(&"screens/main.ans".to_string())
        );
    }

    #[test]
    fn test_theme_add_prompt() {
        let mut theme = Theme::new(
            ThemeMetadata {
                name: "Test".to_string(),
                author: "Author".to_string(),
                version: "1.0".to_string(),
                description: "Desc".to_string(),
                compatible_bbs_version: "0.1.0".to_string(),
                requires_ansi: true,
                requires_utf8: false,
            },
            ColorScheme::default(),
        );

        theme.add_prompt("login".to_string(), "Enter password: ".to_string());
        assert_eq!(
            theme.get_prompt("login"),
            Some(&"Enter password: ".to_string())
        );
    }

    #[test]
    fn test_theme_validation() {
        let valid_theme = Theme::new(
            ThemeMetadata {
                name: "Valid".to_string(),
                author: "Author".to_string(),
                version: "1.0".to_string(),
                description: "Desc".to_string(),
                compatible_bbs_version: "0.1.0".to_string(),
                requires_ansi: true,
                requires_utf8: false,
            },
            ColorScheme::default(),
        );
        assert!(valid_theme.validate().is_ok());

        let invalid_theme = Theme::new(
            ThemeMetadata {
                name: "".to_string(),
                author: "Author".to_string(),
                version: "1.0".to_string(),
                description: "Desc".to_string(),
                compatible_bbs_version: "0.1.0".to_string(),
                requires_ansi: true,
                requires_utf8: false,
            },
            ColorScheme::default(),
        );
        assert!(invalid_theme.validate().is_err());
    }

    #[test]
    fn test_color_scheme_default() {
        let scheme = ColorScheme::default();
        assert_eq!(scheme.primary.foreground, ThemeColor::BrightBlue);
        assert_eq!(scheme.text.foreground, ThemeColor::White);
        assert_eq!(scheme.error.foreground, ThemeColor::BrightRed);
    }

    #[test]
    fn test_theme_ansi_color() {
        let color = ThemeAnsiColor::new(Color::BrightCyan, Color::Black, true, false);
        assert_eq!(color.foreground, ThemeColor::BrightCyan);
        assert_eq!(color.background, ThemeColor::Black);
        assert!(color.bold);
        assert!(!color.blink);
    }

    #[test]
    fn test_theme_ansi_color_to_ansi_sequence() {
        let color = ThemeAnsiColor::new(Color::BrightCyan, Color::Black, true, false);
        let sequence = color.to_ansi_sequence();
        assert!(sequence.starts_with("\x1b["));
        assert!(sequence.contains('1')); // bold
        assert!(sequence.contains("96")); // bright cyan
    }

    #[test]
    fn test_theme_color_conversion() {
        let color = Color::BrightMagenta;
        let theme_color = ThemeColor::from_color(color);
        assert_eq!(theme_color, ThemeColor::BrightMagenta);

        let converted_back = theme_color.to_color();
        assert_eq!(converted_back, Color::BrightMagenta);
    }

    #[test]
    fn test_theme_color_rgb() {
        let color = Color::Rgb(255, 128, 0);
        let theme_color = ThemeColor::from_color(color);
        assert_eq!(
            theme_color,
            ThemeColor::Rgb {
                r: 255,
                g: 128,
                b: 0
            }
        );

        let converted_back = theme_color.to_color();
        if let Color::Rgb(r, g, b) = converted_back {
            assert_eq!((r, g, b), (255, 128, 0));
        } else {
            panic!("Expected RGB color");
        }
    }

    #[test]
    fn test_theme_serialization() {
        let theme = Theme::new(
            ThemeMetadata {
                name: "Test".to_string(),
                author: "Author".to_string(),
                version: "1.0".to_string(),
                description: "Desc".to_string(),
                compatible_bbs_version: "0.1.0".to_string(),
                requires_ansi: true,
                requires_utf8: false,
            },
            ColorScheme::default(),
        );

        let json = serde_json::to_string(&theme).unwrap();
        let deserialized: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme.metadata.name, deserialized.metadata.name);
    }
}
