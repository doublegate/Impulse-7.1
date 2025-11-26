//! ANSI terminal renderer

use crate::ansi::{AnsiCode, AnsiSequence};
use crate::color::{AnsiColor, Color};

/// ANSI terminal renderer
pub struct AnsiRenderer {
    /// Output buffer
    buffer: String,
    /// Current color state
    color: AnsiColor,
    /// Bold attribute
    bold: bool,
    /// Underline attribute
    underline: bool,
    /// Blink attribute
    blink: bool,
    /// Reverse attribute
    reverse: bool,
}

impl Default for AnsiRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsiRenderer {
    /// Create a new renderer
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            color: AnsiColor::default(),
            bold: false,
            underline: false,
            blink: false,
            reverse: false,
        }
    }

    /// Get the rendered output
    pub fn get_output(&self) -> &str {
        &self.buffer
    }

    /// Get owned output and reset buffer
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.buffer)
    }

    /// Clear the output buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Reset all attributes to defaults
    pub fn reset(&mut self) {
        self.buffer.push_str(&AnsiSequence::reset());
        self.color.reset();
        self.bold = false;
        self.underline = false;
        self.blink = false;
        self.reverse = false;
    }

    /// Set foreground color
    pub fn set_foreground(&mut self, color: Color) {
        self.color.set_foreground(color);
        self.apply_color();
    }

    /// Set background color
    pub fn set_background(&mut self, color: Color) {
        self.color.set_background(color);
        self.apply_color();
    }

    /// Enable bold
    pub fn set_bold(&mut self, enabled: bool) {
        self.bold = enabled;
        if enabled {
            let seq = AnsiSequence::new()
                .add_code(AnsiCode::Bold.as_str())
                .build();
            self.buffer.push_str(&seq);
        } else {
            // Bold off is not universally supported, reset instead
            self.reset();
            self.apply_color();
        }
    }

    /// Enable underline
    pub fn set_underline(&mut self, enabled: bool) {
        self.underline = enabled;
        if enabled {
            let seq = AnsiSequence::new()
                .add_code(AnsiCode::Underline.as_str())
                .build();
            self.buffer.push_str(&seq);
        }
    }

    /// Enable blink
    pub fn set_blink(&mut self, enabled: bool) {
        self.blink = enabled;
        if enabled {
            let seq = AnsiSequence::new()
                .add_code(AnsiCode::Blink.as_str())
                .build();
            self.buffer.push_str(&seq);
        }
    }

    /// Enable reverse video
    pub fn set_reverse(&mut self, enabled: bool) {
        self.reverse = enabled;
        if enabled {
            let seq = AnsiSequence::new()
                .add_code(AnsiCode::Reverse.as_str())
                .build();
            self.buffer.push_str(&seq);
        }
    }

    /// Write raw text (no ANSI escaping)
    pub fn write_text(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    /// Write a line with CRLF
    pub fn write_line(&mut self, text: &str) {
        self.buffer.push_str(text);
        self.buffer.push_str("\r\n");
    }

    /// Clear screen
    pub fn clear_screen(&mut self) {
        self.buffer.push_str(&AnsiSequence::clear_screen());
        self.buffer.push_str(&AnsiSequence::move_cursor(1, 1));
    }

    /// Move cursor to position (1-indexed)
    pub fn move_cursor(&mut self, row: u16, col: u16) {
        self.buffer.push_str(&AnsiSequence::move_cursor(row, col));
    }

    /// Move cursor up
    pub fn cursor_up(&mut self, n: u16) {
        self.buffer.push_str(&AnsiSequence::cursor_up(n));
    }

    /// Move cursor down
    pub fn cursor_down(&mut self, n: u16) {
        self.buffer.push_str(&AnsiSequence::cursor_down(n));
    }

    /// Move cursor forward (right)
    pub fn cursor_forward(&mut self, n: u16) {
        self.buffer.push_str(&AnsiSequence::cursor_forward(n));
    }

    /// Move cursor backward (left)
    pub fn cursor_backward(&mut self, n: u16) {
        self.buffer.push_str(&AnsiSequence::cursor_backward(n));
    }

    /// Save cursor position
    pub fn save_cursor(&mut self) {
        self.buffer.push_str(&AnsiSequence::save_cursor());
    }

    /// Restore cursor position
    pub fn restore_cursor(&mut self) {
        self.buffer.push_str(&AnsiSequence::restore_cursor());
    }

    /// Hide cursor
    pub fn hide_cursor(&mut self) {
        self.buffer.push_str(&AnsiSequence::hide_cursor());
    }

    /// Show cursor
    pub fn show_cursor(&mut self) {
        self.buffer.push_str(&AnsiSequence::show_cursor());
    }

    /// Erase current line
    pub fn erase_line(&mut self) {
        self.buffer.push_str(&AnsiSequence::erase_line());
    }

    /// Erase to end of line
    pub fn erase_to_end_of_line(&mut self) {
        self.buffer.push_str(&AnsiSequence::erase_to_end_of_line());
    }

    /// Apply current color state
    fn apply_color(&mut self) {
        let mut seq = AnsiSequence::new();

        if let Some(fg) = self.color.foreground() {
            seq = seq.add_code(fg.foreground_code());
        }
        if let Some(bg) = self.color.background() {
            seq = seq.add_code(bg.background_code());
        }

        let output = seq.build();
        if !output.is_empty() {
            self.buffer.push_str(&output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = AnsiRenderer::new();
        assert_eq!(renderer.get_output(), "");
    }

    #[test]
    fn test_write_text() {
        let mut renderer = AnsiRenderer::new();
        renderer.write_text("Hello");
        assert!(renderer.get_output().contains("Hello"));
    }

    #[test]
    fn test_write_line() {
        let mut renderer = AnsiRenderer::new();
        renderer.write_line("Test");
        assert!(renderer.get_output().contains("Test\r\n"));
    }

    #[test]
    fn test_color_setting() {
        let mut renderer = AnsiRenderer::new();
        renderer.set_foreground(Color::Red);
        let output = renderer.get_output();
        assert!(output.contains("\x1b["));
    }

    #[test]
    fn test_clear() {
        let mut renderer = AnsiRenderer::new();
        renderer.write_text("Test");
        renderer.clear();
        assert_eq!(renderer.get_output(), "");
    }

    #[test]
    fn test_take_output() {
        let mut renderer = AnsiRenderer::new();
        renderer.write_text("Test");
        let output = renderer.take_output();
        assert_eq!(output, "Test");
        assert_eq!(renderer.get_output(), "");
    }

    #[test]
    fn test_cursor_movement() {
        let mut renderer = AnsiRenderer::new();
        renderer.move_cursor(10, 20);
        assert!(renderer.get_output().contains("\x1b[10;20H"));
    }
}
