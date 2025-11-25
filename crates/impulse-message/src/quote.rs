//! Message quoting functionality for replies

/// Quote a message body for inclusion in a reply
///
/// Adds "> " prefix to each line and optionally adds attribution header.
///
/// # Arguments
/// * `original_text` - The original message body
/// * `from` - The original sender's name (for attribution)
/// * `include_attribution` - Whether to include "On [date] [from] wrote:" header
///
/// # Returns
/// The quoted text with prefix
pub fn quote_message(original_text: &str, from: Option<&str>, include_attribution: bool) -> String {
    let mut result = String::new();

    // Add attribution header if requested
    if include_attribution {
        if let Some(sender) = from {
            result.push_str(&format!(
                "On {}, {} wrote:\n",
                chrono::Utc::now().format("%Y-%m-%d"),
                sender
            ));
        }
    }

    // Quote each line
    for line in original_text.lines() {
        result.push_str("> ");
        result.push_str(line);
        result.push('\n');
    }

    result
}

/// Quote selected lines from a message
///
/// # Arguments
/// * `original_text` - The original message body
/// * `start_line` - Starting line number (0-based)
/// * `end_line` - Ending line number (exclusive)
///
/// # Returns
/// The quoted text with prefix
pub fn quote_lines(original_text: &str, start_line: usize, end_line: usize) -> String {
    let lines: Vec<&str> = original_text.lines().collect();
    let mut result = String::new();

    for (idx, line) in lines.iter().enumerate() {
        if idx >= start_line && idx < end_line {
            result.push_str("> ");
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Remove quote prefixes from text
///
/// Useful for extracting original content from quoted messages.
///
/// # Arguments
/// * `quoted_text` - The quoted text with "> " prefixes
///
/// # Returns
/// The text with quote prefixes removed
pub fn unquote_message(quoted_text: &str) -> String {
    let mut result = String::new();

    for line in quoted_text.lines() {
        let trimmed = line.trim_start();
        if let Some(stripped) = trimmed.strip_prefix("> ") {
            result.push_str(stripped);
        } else if let Some(stripped) = trimmed.strip_prefix('>') {
            result.push_str(stripped);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }

    // Remove trailing newline
    if result.ends_with('\n') {
        result.pop();
    }

    result
}

/// Get the quote depth of a line
///
/// Counts the number of "> " prefixes at the start of a line.
///
/// # Arguments
/// * `line` - The line to check
///
/// # Returns
/// The quote depth (0 = not quoted, 1 = quoted once, etc.)
pub fn quote_depth(line: &str) -> usize {
    let mut depth = 0;
    let mut remaining = line.trim_start();

    while let Some(stripped) = remaining.strip_prefix("> ") {
        depth += 1;
        remaining = stripped.trim_start();
    }

    depth
}

/// Wrap quoted text to a maximum line width
///
/// # Arguments
/// * `quoted_text` - The quoted text
/// * `max_width` - Maximum line width (including quote prefix)
///
/// # Returns
/// The wrapped quoted text
pub fn wrap_quoted_text(quoted_text: &str, max_width: usize) -> String {
    let mut result = String::new();

    for line in quoted_text.lines() {
        let depth = quote_depth(line);
        let prefix = "> ".repeat(depth);
        let prefix_len = prefix.len();

        // Calculate available width for text
        let available_width = if max_width > prefix_len {
            max_width - prefix_len
        } else {
            10 // Minimum width
        };

        // Remove quote prefix to get actual text
        let text = unquote_message(line);

        // Wrap text
        let mut current_line = String::new();
        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line.len() + 1 + word.len() <= available_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                // Line is full, output it
                result.push_str(&prefix);
                result.push_str(&current_line);
                result.push('\n');
                current_line = word.to_string();
            }
        }

        // Output remaining text
        if !current_line.is_empty() {
            result.push_str(&prefix);
            result.push_str(&current_line);
            result.push('\n');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_message() {
        let original = "This is line 1.\nThis is line 2.\nThis is line 3.";
        let quoted = quote_message(original, Some("Alice"), false);

        assert_eq!(
            quoted,
            "> This is line 1.\n> This is line 2.\n> This is line 3.\n"
        );
    }

    #[test]
    fn test_quote_with_attribution() {
        let original = "Hello, world!";
        let quoted = quote_message(original, Some("Alice"), true);

        assert!(quoted.contains("Alice wrote:"));
        assert!(quoted.contains("> Hello, world!"));
    }

    #[test]
    fn test_quote_lines() {
        let original = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let quoted = quote_lines(original, 1, 4);

        assert_eq!(quoted, "> Line 2\n> Line 3\n> Line 4\n");
    }

    #[test]
    fn test_unquote_message() {
        let quoted = "> This is line 1.\n> This is line 2.\n> This is line 3.";
        let unquoted = unquote_message(quoted);

        assert_eq!(
            unquoted,
            "This is line 1.\nThis is line 2.\nThis is line 3."
        );
    }

    #[test]
    fn test_quote_depth() {
        assert_eq!(quote_depth("Normal line"), 0);
        assert_eq!(quote_depth("> Quoted once"), 1);
        assert_eq!(quote_depth("> > Quoted twice"), 2);
        assert_eq!(quote_depth("> > > Quoted thrice"), 3);
    }

    #[test]
    fn test_wrap_quoted_text() {
        let quoted = "> This is a very long line that should be wrapped at a certain width to make it more readable.";
        let wrapped = wrap_quoted_text(quoted, 40);

        // Should be split into multiple lines
        let lines: Vec<&str> = wrapped.lines().collect();
        assert!(lines.len() > 1);

        // Each line should start with "> "
        for line in lines {
            assert!(line.starts_with("> "));
        }
    }

    #[test]
    fn test_nested_quotes() {
        assert_eq!(quote_depth("> Original quote"), 1);
        assert_eq!(quote_depth("> > Nested quote"), 2);
    }
}
