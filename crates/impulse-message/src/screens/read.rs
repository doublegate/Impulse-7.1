//! Message read screen

use crate::error::Result;
use crate::traits::MessageBase;
use crate::types::{FullMessage, MessageThread};

/// Message read screen configuration
#[derive(Debug, Clone)]
pub struct MessageReadConfig {
    /// Wrap width for message body
    pub wrap_width: usize,
    /// Show kludge lines
    pub show_kludges: bool,
    /// Show thread tree
    pub show_thread: bool,
    /// Highlight quoted text
    pub highlight_quotes: bool,
}

impl Default for MessageReadConfig {
    fn default() -> Self {
        Self {
            wrap_width: 79,
            show_kludges: false,
            show_thread: true,
            highlight_quotes: true,
        }
    }
}

/// Message read screen
pub struct MessageReadScreen {
    /// Configuration
    config: MessageReadConfig,
    /// Current message
    message: Option<FullMessage>,
    /// Thread information
    thread: Option<MessageThread>,
}

impl MessageReadScreen {
    /// Create a new message read screen
    pub fn new(config: MessageReadConfig) -> Self {
        Self {
            config,
            message: None,
            thread: None,
        }
    }

    /// Create with default configuration
    pub fn default_config() -> Self {
        Self::new(MessageReadConfig::default())
    }

    /// Load a message
    pub async fn load_message<M: MessageBase>(&mut self, base: &M, msg_num: u32) -> Result<()> {
        self.message = Some(base.read_message(msg_num).await?);
        if self.config.show_thread {
            self.thread = Some(base.get_thread(msg_num).await?);
        }
        Ok(())
    }

    /// Get current message
    pub fn message(&self) -> Option<&FullMessage> {
        self.message.as_ref()
    }

    /// Get thread information
    pub fn thread(&self) -> Option<&MessageThread> {
        self.thread.as_ref()
    }

    /// Word wrap text
    fn wrap_text(text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        for line in text.lines() {
            if line.len() <= width {
                lines.push(line.to_string());
            } else {
                let mut current = String::new();
                for word in line.split_whitespace() {
                    if current.len() + word.len() + 1 > width {
                        if !current.is_empty() {
                            lines.push(current);
                            current = String::new();
                        }
                        if word.len() > width {
                            lines.push(word.to_string());
                        } else {
                            current = word.to_string();
                        }
                    } else {
                        if !current.is_empty() {
                            current.push(' ');
                        }
                        current.push_str(word);
                    }
                }
                if !current.is_empty() {
                    lines.push(current);
                }
            }
        }
        lines
    }

    /// Render message as text
    pub fn render(&self) -> String {
        let mut output = String::new();

        if let Some(msg) = &self.message {
            // Header
            output.push_str(
                "╔════════════════════════════════════════════════════════════════════════╗\n",
            );
            output.push_str(&format!(
                "║ Message #{:<4}                                                         ║\n",
                msg.header.msg_num
            ));
            output.push_str(
                "╠════════════════════════════════════════════════════════════════════════╣\n",
            );
            output.push_str(&format!("║ From: {:<65} ║\n", msg.header.from));
            output.push_str(&format!("║ To:   {:<65} ║\n", msg.header.to));
            output.push_str(&format!("║ Subj: {:<65} ║\n", msg.header.subject));
            output.push_str(&format!(
                "║ Date: {:<65} ║\n",
                msg.header.date.format("%Y-%m-%d %H:%M:%S")
            ));

            // Thread info
            if let Some(thread) = &self.thread {
                if thread.reply_count > 0 || thread.parent_id.is_some() {
                    output.push_str(&format!(
                        "║ Thread: {} replies, depth {}                                          ║\n",
                        thread.reply_count, thread.depth
                    ));
                }
            }

            output.push_str(
                "╠════════════════════════════════════════════════════════════════════════╣\n",
            );

            // Body (wrapped)
            let wrapped = Self::wrap_text(&msg.body, self.config.wrap_width - 4);
            for line in wrapped {
                let display = if self.config.highlight_quotes && line.trim_start().starts_with('>')
                {
                    format!("║ \x1b[36m{:<70}\x1b[0m ║", line) // Cyan for quotes
                } else {
                    format!("║ {:<70} ║", line)
                };
                output.push_str(&display);
                output.push('\n');
            }

            // Kludges (if enabled)
            if self.config.show_kludges && !msg.kludges.is_empty() {
                output.push_str(
                    "╠════════════════════════════════════════════════════════════════════════╣\n",
                );
                output.push_str("║ Kludges:                                                                 ║\n");
                for kludge in &msg.kludges {
                    output.push_str(&format!(
                        "║   {:8}: {:<58} ║\n",
                        kludge.kludge_type, kludge.value
                    ));
                }
            }

            // Footer
            output.push_str(
                "╚════════════════════════════════════════════════════════════════════════╝\n",
            );
            output.push_str("  [N]ext  [P]rev  [R]eply  [L]ist  [Q]uit\n");
        } else {
            output.push_str("No message loaded\n");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text() {
        let text = "This is a very long line that should be wrapped at the specified width";
        let wrapped = MessageReadScreen::wrap_text(text, 20);
        assert!(wrapped.len() > 1);
        assert!(wrapped.iter().all(|line| line.len() <= 20));
    }

    #[test]
    fn test_wrap_text_short() {
        let text = "Short line";
        let wrapped = MessageReadScreen::wrap_text(text, 80);
        assert_eq!(wrapped.len(), 1);
        assert_eq!(wrapped[0], "Short line");
    }

    #[test]
    fn test_wrap_text_multiline() {
        let text = "Line 1\nLine 2\nLine 3";
        let wrapped = MessageReadScreen::wrap_text(text, 80);
        assert_eq!(wrapped.len(), 3);
    }
}
