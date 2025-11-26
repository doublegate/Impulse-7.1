//! ANSI escape sequence handling

/// ANSI control codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiCode {
    /// Reset all attributes
    Reset,
    /// Bold/bright
    Bold,
    /// Dim
    Dim,
    /// Italic (not widely supported)
    Italic,
    /// Underline
    Underline,
    /// Blink (slow)
    Blink,
    /// Reverse video (swap fg/bg)
    Reverse,
    /// Hidden/invisible
    Hidden,
    /// Strikethrough
    Strikethrough,
}

impl AnsiCode {
    /// Get ANSI code number
    pub fn code(&self) -> u8 {
        match self {
            Self::Reset => 0,
            Self::Bold => 1,
            Self::Dim => 2,
            Self::Italic => 3,
            Self::Underline => 4,
            Self::Blink => 5,
            Self::Reverse => 7,
            Self::Hidden => 8,
            Self::Strikethrough => 9,
        }
    }

    /// Get ANSI code as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Reset => "0",
            Self::Bold => "1",
            Self::Dim => "2",
            Self::Italic => "3",
            Self::Underline => "4",
            Self::Blink => "5",
            Self::Reverse => "7",
            Self::Hidden => "8",
            Self::Strikethrough => "9",
        }
    }
}

/// ANSI escape sequence builder
#[derive(Debug, Clone)]
pub struct AnsiSequence {
    codes: Vec<String>,
}

impl AnsiSequence {
    /// Create new sequence
    pub fn new() -> Self {
        Self { codes: Vec::new() }
    }

    /// Add a code to the sequence
    pub fn add_code(mut self, code: impl Into<String>) -> Self {
        self.codes.push(code.into());
        self
    }

    /// Add multiple codes
    pub fn add_codes(mut self, codes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.codes.extend(codes.into_iter().map(Into::into));
        self
    }

    /// Build the escape sequence
    pub fn build(&self) -> String {
        if self.codes.is_empty() {
            String::new()
        } else {
            format!("\x1b[{}m", self.codes.join(";"))
        }
    }

    /// Create reset sequence
    pub fn reset() -> String {
        "\x1b[0m".to_string()
    }

    /// Create clear screen sequence
    pub fn clear_screen() -> String {
        "\x1b[2J".to_string()
    }

    /// Create move cursor sequence
    pub fn move_cursor(row: u16, col: u16) -> String {
        format!("\x1b[{};{}H", row, col)
    }

    /// Create cursor up sequence
    pub fn cursor_up(n: u16) -> String {
        format!("\x1b[{}A", n)
    }

    /// Create cursor down sequence
    pub fn cursor_down(n: u16) -> String {
        format!("\x1b[{}B", n)
    }

    /// Create cursor forward sequence
    pub fn cursor_forward(n: u16) -> String {
        format!("\x1b[{}C", n)
    }

    /// Create cursor backward sequence
    pub fn cursor_backward(n: u16) -> String {
        format!("\x1b[{}D", n)
    }

    /// Create save cursor position sequence
    pub fn save_cursor() -> String {
        "\x1b[s".to_string()
    }

    /// Create restore cursor position sequence
    pub fn restore_cursor() -> String {
        "\x1b[u".to_string()
    }

    /// Create hide cursor sequence
    pub fn hide_cursor() -> String {
        "\x1b[?25l".to_string()
    }

    /// Create show cursor sequence
    pub fn show_cursor() -> String {
        "\x1b[?25h".to_string()
    }

    /// Create erase line sequence
    pub fn erase_line() -> String {
        "\x1b[2K".to_string()
    }

    /// Create erase to end of line sequence
    pub fn erase_to_end_of_line() -> String {
        "\x1b[K".to_string()
    }
}

impl Default for AnsiSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_code_values() {
        assert_eq!(AnsiCode::Reset.code(), 0);
        assert_eq!(AnsiCode::Bold.code(), 1);
        assert_eq!(AnsiCode::Underline.code(), 4);
    }

    #[test]
    fn test_sequence_builder() {
        let seq = AnsiSequence::new().add_code("1").add_code("31").build();

        assert_eq!(seq, "\x1b[1;31m");
    }

    #[test]
    fn test_reset_sequence() {
        assert_eq!(AnsiSequence::reset(), "\x1b[0m");
    }

    #[test]
    fn test_cursor_movement() {
        assert_eq!(AnsiSequence::move_cursor(10, 20), "\x1b[10;20H");
        assert_eq!(AnsiSequence::cursor_up(5), "\x1b[5A");
        assert_eq!(AnsiSequence::cursor_down(3), "\x1b[3B");
    }

    #[test]
    fn test_clear_screen() {
        assert_eq!(AnsiSequence::clear_screen(), "\x1b[2J");
    }
}
