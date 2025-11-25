//! Message content sanitization

use crate::types::NewMessage;

/// Message sanitizer
pub struct MessageSanitizer {
    /// Allow ANSI color codes
    allow_ansi_colors: bool,
    /// Strip control characters
    strip_control_chars: bool,
}

impl MessageSanitizer {
    /// Create a new sanitizer with default settings
    pub fn new() -> Self {
        Self {
            allow_ansi_colors: true,
            strip_control_chars: true,
        }
    }

    /// Allow ANSI color codes
    pub fn allow_ansi_colors(mut self, allow: bool) -> Self {
        self.allow_ansi_colors = allow;
        self
    }

    /// Strip control characters
    pub fn strip_control_chars(mut self, strip: bool) -> Self {
        self.strip_control_chars = strip;
        self
    }

    /// Sanitize a message
    ///
    /// # Arguments
    /// * `message` - The message to sanitize
    ///
    /// # Returns
    /// A sanitized copy of the message
    pub fn sanitize(&self, message: &NewMessage) -> NewMessage {
        let mut sanitized = message.clone();

        // Sanitize subject
        sanitized.subject = self.sanitize_text(&sanitized.subject, false);

        // Sanitize body (allowing some ANSI codes)
        sanitized.body = self.sanitize_text(&sanitized.body, self.allow_ansi_colors);

        // Trim fields
        sanitized.from = sanitized.from.trim().to_string();
        sanitized.to = sanitized.to.trim().to_string();
        sanitized.subject = sanitized.subject.trim().to_string();
        sanitized.body = sanitized.body.trim().to_string();

        sanitized
    }

    /// Sanitize text content
    fn sanitize_text(&self, text: &str, allow_ansi: bool) -> String {
        if !self.strip_control_chars && allow_ansi {
            return text.to_string();
        }

        let mut result = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' && allow_ansi {
                // Check for ANSI escape sequence
                if chars.peek() == Some(&'[') {
                    // This is an ANSI escape sequence
                    result.push(ch);
                    result.push(chars.next().unwrap()); // '['

                    // Copy the sequence up to 'm' (color code end)
                    while let Some(&next_ch) = chars.peek() {
                        result.push(chars.next().unwrap());
                        if next_ch == 'm' || next_ch == 'H' || next_ch == 'J' {
                            break;
                        }
                        // Safety limit
                        if result.len() > 100 {
                            break;
                        }
                    }
                }
            } else if self.strip_control_chars && ch.is_control() {
                // Keep common control characters
                if ch == '\n' || ch == '\r' || ch == '\t' {
                    result.push(ch);
                }
                // Strip other control characters
            } else {
                result.push(ch);
            }
        }

        result
    }
}

impl Default for MessageSanitizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_basic() {
        let sanitizer = MessageSanitizer::new();
        let message =
            NewMessage::new("Alice", "Bob", "Test Subject").with_body("This is a test message.");

        let sanitized = sanitizer.sanitize(&message);
        assert_eq!(sanitized.from, "Alice");
        assert_eq!(sanitized.to, "Bob");
        assert_eq!(sanitized.subject, "Test Subject");
        assert_eq!(sanitized.body, "This is a test message.");
    }

    #[test]
    fn test_sanitize_trim_whitespace() {
        let sanitizer = MessageSanitizer::new();
        let message = NewMessage::new("  Alice  ", "  Bob  ", "  Test Subject  ")
            .with_body("  This is a test message.  ");

        let sanitized = sanitizer.sanitize(&message);
        assert_eq!(sanitized.from, "Alice");
        assert_eq!(sanitized.to, "Bob");
        assert_eq!(sanitized.subject, "Test Subject");
        // Body keeps internal spaces but trims ends
        assert_eq!(sanitized.body, "This is a test message.");
    }

    #[test]
    fn test_sanitize_control_characters() {
        let sanitizer = MessageSanitizer::new();
        let message = NewMessage::new("Alice", "Bob", "Test Subject")
            .with_body("This\x00is\x01a\x02test\x03message.");

        let sanitized = sanitizer.sanitize(&message);
        // Control characters should be stripped
        assert_eq!(sanitized.body, "Thisisatestmessage.");
    }

    #[test]
    fn test_sanitize_keep_newlines() {
        let sanitizer = MessageSanitizer::new();
        let message = NewMessage::new("Alice", "Bob", "Test Subject")
            .with_body("Line 1\nLine 2\rLine 3\r\nLine 4");

        let sanitized = sanitizer.sanitize(&message);
        // Newlines and carriage returns should be kept
        assert!(sanitized.body.contains('\n'));
        assert!(sanitized.body.contains('\r'));
    }

    #[test]
    fn test_sanitize_ansi_colors() {
        let sanitizer = MessageSanitizer::new().allow_ansi_colors(true);
        let message = NewMessage::new("Alice", "Bob", "Test Subject")
            .with_body("This is \x1b[31mred\x1b[0m text.");

        let sanitized = sanitizer.sanitize(&message);
        // ANSI codes should be preserved
        assert!(sanitized.body.contains("\x1b[31m"));
        assert!(sanitized.body.contains("\x1b[0m"));
    }

    #[test]
    fn test_sanitize_strip_ansi() {
        let sanitizer = MessageSanitizer::new().allow_ansi_colors(false);
        let message = NewMessage::new("Alice", "Bob", "Test Subject")
            .with_body("This is \x1b[31mred\x1b[0m text.");

        let sanitized = sanitizer.sanitize(&message);
        // ANSI codes should be stripped (ESC is a control character)
        assert!(!sanitized.body.contains('\x1b'));
    }

    #[test]
    fn test_sanitize_no_strip_control() {
        let sanitizer = MessageSanitizer::new().strip_control_chars(false);
        let message = NewMessage::new("Alice", "Bob", "Test Subject")
            .with_body("This\x00has\x01control\x02chars");

        let sanitized = sanitizer.sanitize(&message);
        // Control characters should be kept
        assert!(sanitized.body.contains('\x00'));
        assert!(sanitized.body.contains('\x01'));
        assert!(sanitized.body.contains('\x02'));
    }
}
