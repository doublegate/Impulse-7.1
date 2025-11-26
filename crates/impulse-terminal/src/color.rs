//! ANSI color definitions and handling

/// Standard ANSI colors (16-color palette)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    // Normal colors
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // Bright colors
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    // Extended colors
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    /// Get ANSI foreground color code
    pub fn foreground_code(&self) -> String {
        match self {
            Color::Black => "30".to_string(),
            Color::Red => "31".to_string(),
            Color::Green => "32".to_string(),
            Color::Yellow => "33".to_string(),
            Color::Blue => "34".to_string(),
            Color::Magenta => "35".to_string(),
            Color::Cyan => "36".to_string(),
            Color::White => "37".to_string(),
            Color::BrightBlack => "90".to_string(),
            Color::BrightRed => "91".to_string(),
            Color::BrightGreen => "92".to_string(),
            Color::BrightYellow => "93".to_string(),
            Color::BrightBlue => "94".to_string(),
            Color::BrightMagenta => "95".to_string(),
            Color::BrightCyan => "96".to_string(),
            Color::BrightWhite => "97".to_string(),
            Color::Ansi256(c) => format!("38;5;{}", c),
            Color::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        }
    }

    /// Get ANSI background color code
    pub fn background_code(&self) -> String {
        match self {
            Color::Black => "40".to_string(),
            Color::Red => "41".to_string(),
            Color::Green => "42".to_string(),
            Color::Yellow => "43".to_string(),
            Color::Blue => "44".to_string(),
            Color::Magenta => "45".to_string(),
            Color::Cyan => "46".to_string(),
            Color::White => "47".to_string(),
            Color::BrightBlack => "100".to_string(),
            Color::BrightRed => "101".to_string(),
            Color::BrightGreen => "102".to_string(),
            Color::BrightYellow => "103".to_string(),
            Color::BrightBlue => "104".to_string(),
            Color::BrightMagenta => "105".to_string(),
            Color::BrightCyan => "106".to_string(),
            Color::BrightWhite => "107".to_string(),
            Color::Ansi256(c) => format!("48;5;{}", c),
            Color::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
        }
    }

    /// Parse ANSI color from code (0-15 for 16-color palette)
    pub fn from_ansi_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Color::Black),
            1 => Some(Color::Red),
            2 => Some(Color::Green),
            3 => Some(Color::Yellow),
            4 => Some(Color::Blue),
            5 => Some(Color::Magenta),
            6 => Some(Color::Cyan),
            7 => Some(Color::White),
            8 => Some(Color::BrightBlack),
            9 => Some(Color::BrightRed),
            10 => Some(Color::BrightGreen),
            11 => Some(Color::BrightYellow),
            12 => Some(Color::BrightBlue),
            13 => Some(Color::BrightMagenta),
            14 => Some(Color::BrightCyan),
            15 => Some(Color::BrightWhite),
            _ => None,
        }
    }
}

/// ANSI color attribute
#[derive(Debug, Clone, Copy)]
pub struct AnsiColor {
    foreground: Option<Color>,
    background: Option<Color>,
}

impl Default for AnsiColor {
    fn default() -> Self {
        Self {
            foreground: Some(Color::White),
            background: Some(Color::Black),
        }
    }
}

impl AnsiColor {
    /// Create new color attributes
    pub fn new() -> Self {
        Self::default()
    }

    /// Set foreground color
    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Set background color
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Get foreground color
    pub fn foreground(&self) -> Option<Color> {
        self.foreground
    }

    /// Get background color
    pub fn background(&self) -> Option<Color> {
        self.background
    }

    /// Set foreground color
    pub fn set_foreground(&mut self, color: Color) {
        self.foreground = Some(color);
    }

    /// Set background color
    pub fn set_background(&mut self, color: Color) {
        self.background = Some(color);
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreground_codes() {
        assert_eq!(Color::Red.foreground_code(), "31");
        assert_eq!(Color::BrightCyan.foreground_code(), "96");
    }

    #[test]
    fn test_background_codes() {
        assert_eq!(Color::Blue.background_code(), "44");
        assert_eq!(Color::BrightYellow.background_code(), "103");
    }

    #[test]
    fn test_from_ansi_code() {
        assert_eq!(Color::from_ansi_code(1), Some(Color::Red));
        assert_eq!(Color::from_ansi_code(14), Some(Color::BrightCyan));
        assert_eq!(Color::from_ansi_code(99), None);
    }

    #[test]
    fn test_ansi_color_default() {
        let color = AnsiColor::default();
        assert_eq!(color.foreground(), Some(Color::White));
        assert_eq!(color.background(), Some(Color::Black));
    }
}
