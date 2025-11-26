//! Theme preview generation

use crate::error::Result;

use super::types::{ColorScheme, Theme, ThemeAnsiColor};

/// Theme preview generator
#[derive(Debug)]
pub struct ThemePreview;

impl ThemePreview {
    /// Create a new theme preview generator
    pub fn new() -> Self {
        Self
    }

    /// Generate a preview of a theme
    pub fn preview_theme(&self, theme: &Theme) -> Result<String> {
        let mut preview = String::new();

        // Add theme header
        preview.push_str("\x1b[1;37m╔══════════════════════════════════════════════════╗\x1b[0m\n");
        preview.push_str(&format!(
            "\x1b[1;37m║ \x1b[1;36mTheme Preview: {:<30}\x1b[1;37m ║\x1b[0m\n",
            theme.metadata.name
        ));
        preview
            .push_str("\x1b[1;37m╚══════════════════════════════════════════════════╝\x1b[0m\n\n");

        // Add metadata
        preview.push_str(&format!(
            "{}Author:{} {}\n",
            Self::color_to_ansi(&theme.color_scheme.secondary),
            "\x1b[0m",
            theme.metadata.author
        ));
        preview.push_str(&format!(
            "{}Version:{} {}\n",
            Self::color_to_ansi(&theme.color_scheme.secondary),
            "\x1b[0m",
            theme.metadata.version
        ));
        preview.push_str(&format!(
            "{}Description:{} {}\n\n",
            Self::color_to_ansi(&theme.color_scheme.secondary),
            "\x1b[0m",
            theme.metadata.description
        ));

        // Add color samples
        preview.push_str(&self.display_color_samples(&theme.color_scheme));

        // Add sample UI elements
        preview.push('\n');
        preview.push_str(&self.display_sample_ui(&theme.color_scheme));

        Ok(preview)
    }

    /// Display color samples from a color scheme
    fn display_color_samples(&self, scheme: &ColorScheme) -> String {
        let mut samples = String::new();

        samples.push_str("\x1b[1;37mColor Scheme:\x1b[0m\n");

        let colors = [
            ("Primary", &scheme.primary),
            ("Secondary", &scheme.secondary),
            ("Accent", &scheme.accent),
            ("Background", &scheme.background),
            ("Text", &scheme.text),
            ("Highlight", &scheme.highlight),
            ("Error", &scheme.error),
            ("Success", &scheme.success),
        ];

        for (name, color) in &colors {
            samples.push_str(&format!(
                "  {:<12} {} ████████ {} Sample text\n",
                format!("{}:", name),
                Self::color_to_ansi(color),
                "\x1b[0m"
            ));
        }

        samples
    }

    /// Display sample UI elements
    fn display_sample_ui(&self, scheme: &ColorScheme) -> String {
        let mut ui = String::new();

        ui.push_str("\x1b[1;37mSample UI Elements:\x1b[0m\n\n");

        // Menu sample
        ui.push_str(&format!(
            "{}╔══════════════════════════════════════════╗{}\n",
            Self::color_to_ansi(&scheme.primary),
            "\x1b[0m"
        ));
        ui.push_str(&format!(
            "{}║          MAIN MENU                       ║{}\n",
            Self::color_to_ansi(&scheme.primary),
            "\x1b[0m"
        ));
        ui.push_str(&format!(
            "{}╠══════════════════════════════════════════╣{}\n",
            Self::color_to_ansi(&scheme.primary),
            "\x1b[0m"
        ));
        ui.push_str(&format!(
            "{}║ {}[M]{} Messages  {}[F]{} Files  {}[U]{} Users  {}║{}\n",
            Self::color_to_ansi(&scheme.secondary),
            Self::color_to_ansi(&scheme.accent),
            Self::color_to_ansi(&scheme.text),
            Self::color_to_ansi(&scheme.accent),
            Self::color_to_ansi(&scheme.text),
            Self::color_to_ansi(&scheme.accent),
            Self::color_to_ansi(&scheme.text),
            Self::color_to_ansi(&scheme.secondary),
            "\x1b[0m"
        ));
        ui.push_str(&format!(
            "{}╚══════════════════════════════════════════╝{}\n\n",
            Self::color_to_ansi(&scheme.primary),
            "\x1b[0m"
        ));

        // Status messages
        ui.push_str(&format!(
            "{}✓{} Operation completed successfully!\n",
            Self::color_to_ansi(&scheme.success),
            "\x1b[0m"
        ));
        ui.push_str(&format!(
            "{}✗{} An error occurred during processing.\n",
            Self::color_to_ansi(&scheme.error),
            "\x1b[0m"
        ));
        ui.push_str(&format!(
            "{}★{} Important notification message.\n\n",
            Self::color_to_ansi(&scheme.highlight),
            "\x1b[0m"
        ));

        // Prompt sample
        ui.push_str(&format!(
            "{}Command:{} ",
            Self::color_to_ansi(&scheme.secondary),
            "\x1b[0m"
        ));

        ui
    }

    /// Convert ThemeAnsiColor to ANSI escape sequence
    fn color_to_ansi(color: &ThemeAnsiColor) -> String {
        color.to_ansi_sequence()
    }
}

impl Default for ThemePreview {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::theme::metadata::ThemeMetadata;

    fn create_test_theme() -> Theme {
        Theme::new(
            ThemeMetadata::new(
                "Test Theme".to_string(),
                "Test Author".to_string(),
                "1.0.0".to_string(),
                "A test theme for preview".to_string(),
            ),
            ColorScheme::default(),
        )
    }

    #[test]
    fn test_theme_preview_creation() {
        let preview = ThemePreview::new();
        assert!(format!("{:?}", preview).contains("ThemePreview"));
    }

    #[test]
    fn test_preview_theme() {
        let theme = create_test_theme();
        let preview = ThemePreview::new();

        let result = preview.preview_theme(&theme);
        assert!(result.is_ok());

        let preview_text = result.unwrap();
        assert!(preview_text.contains("Test Theme"));
        assert!(preview_text.contains("Test Author"));
        assert!(preview_text.contains("Color Scheme"));
    }

    #[test]
    fn test_preview_contains_colors() {
        let theme = create_test_theme();
        let preview = ThemePreview::new();

        let preview_text = preview.preview_theme(&theme).unwrap();

        // Should contain color names
        assert!(preview_text.contains("Primary"));
        assert!(preview_text.contains("Secondary"));
        assert!(preview_text.contains("Accent"));
        assert!(preview_text.contains("Error"));
        assert!(preview_text.contains("Success"));
    }

    #[test]
    fn test_preview_contains_ui_samples() {
        let theme = create_test_theme();
        let preview = ThemePreview::new();

        let preview_text = preview.preview_theme(&theme).unwrap();

        // Should contain UI samples
        assert!(preview_text.contains("MAIN MENU"));
        assert!(preview_text.contains("Messages"));
        assert!(preview_text.contains("Files"));
        assert!(preview_text.contains("Users"));
        assert!(preview_text.contains("Command:"));
    }

    #[test]
    fn test_preview_contains_status_messages() {
        let theme = create_test_theme();
        let preview = ThemePreview::new();

        let preview_text = preview.preview_theme(&theme).unwrap();

        // Should contain status samples
        assert!(preview_text.contains("completed successfully"));
        assert!(preview_text.contains("error occurred"));
        assert!(preview_text.contains("notification"));
    }

    #[test]
    fn test_display_color_samples() {
        let scheme = ColorScheme::default();
        let preview = ThemePreview::new();

        let samples = preview.display_color_samples(&scheme);

        assert!(samples.contains("Primary"));
        assert!(samples.contains("Secondary"));
        assert!(samples.contains("Sample text"));
        assert!(samples.contains("████████")); // Color blocks
    }

    #[test]
    fn test_display_sample_ui() {
        let scheme = ColorScheme::default();
        let preview = ThemePreview::new();

        let ui = preview.display_sample_ui(&scheme);

        assert!(ui.contains("Sample UI Elements"));
        assert!(ui.contains("MAIN MENU"));
        assert!(ui.contains("[M]"));
        assert!(ui.contains("[F]"));
        assert!(ui.contains("[U]"));
    }

    #[test]
    fn test_color_to_ansi() {
        let color = ThemeAnsiColor::new(Color::BrightCyan, Color::Black, true, false);
        let ansi = ThemePreview::color_to_ansi(&color);

        assert!(ansi.starts_with("\x1b["));
        assert!(ansi.ends_with('m'));
    }

    #[test]
    fn test_preview_default_trait() {
        let preview = ThemePreview;
        let theme = create_test_theme();

        let result = preview.preview_theme(&theme);
        assert!(result.is_ok());
    }
}
