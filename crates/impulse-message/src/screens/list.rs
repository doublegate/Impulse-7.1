//! Message list screen

use crate::error::Result;
use crate::traits::MessageBase;
use crate::types::{MessageHeader, SearchCriteria};

/// Message list screen configuration
#[derive(Debug, Clone)]
pub struct MessageListConfig {
    /// Messages per page
    pub messages_per_page: u32,
    /// Show message numbers
    pub show_numbers: bool,
    /// Show read indicators
    pub show_read_status: bool,
    /// Show private indicators
    pub show_private_status: bool,
}

impl Default for MessageListConfig {
    fn default() -> Self {
        Self {
            messages_per_page: 20,
            show_numbers: true,
            show_read_status: true,
            show_private_status: true,
        }
    }
}

/// Message list screen
pub struct MessageListScreen {
    /// Current page (0-based)
    current_page: u32,
    /// Configuration
    config: MessageListConfig,
    /// Current messages on page
    messages: Vec<MessageHeader>,
    /// Total message count
    total_count: u32,
}

impl MessageListScreen {
    /// Create a new message list screen
    pub fn new(config: MessageListConfig) -> Self {
        Self {
            current_page: 0,
            config,
            messages: Vec::new(),
            total_count: 0,
        }
    }

    /// Create with default configuration
    pub fn default_config() -> Self {
        Self::new(MessageListConfig::default())
    }

    /// Load a page of messages
    pub async fn load_page<M: MessageBase>(&mut self, base: &M, page: u32) -> Result<()> {
        let start = page * self.config.messages_per_page + 1;
        self.messages = base
            .list_messages(start, self.config.messages_per_page)
            .await?;
        self.total_count = base.message_count().await?;
        self.current_page = page;
        Ok(())
    }

    /// Search and display results
    pub async fn search<M: MessageBase>(
        &mut self,
        base: &M,
        criteria: &SearchCriteria,
    ) -> Result<Vec<u32>> {
        base.search(criteria).await
    }

    /// Get next page
    pub async fn next_page<M: MessageBase>(&mut self, base: &M) -> Result<bool> {
        let max_page = self.max_page();
        if self.current_page < max_page {
            self.load_page(base, self.current_page + 1).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get previous page
    pub async fn prev_page<M: MessageBase>(&mut self, base: &M) -> Result<bool> {
        if self.current_page > 0 {
            self.load_page(base, self.current_page - 1).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Jump to specific page
    pub async fn goto_page<M: MessageBase>(&mut self, base: &M, page: u32) -> Result<bool> {
        let max_page = self.max_page();
        if page <= max_page {
            self.load_page(base, page).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get maximum page number
    pub fn max_page(&self) -> u32 {
        if self.total_count == 0 {
            0
        } else {
            (self.total_count - 1) / self.config.messages_per_page
        }
    }

    /// Get current page number
    pub fn current_page(&self) -> u32 {
        self.current_page
    }

    /// Get messages on current page
    pub fn messages(&self) -> &[MessageHeader] {
        &self.messages
    }

    /// Get total message count
    pub fn total_count(&self) -> u32 {
        self.total_count
    }

    /// Render message list as text
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str(
            "╔════════════════════════════════════════════════════════════════════════╗\n",
        );
        output.push_str(&format!(
            "║  Messages - Page {}/{}  ({} total)                                \n",
            self.current_page + 1,
            self.max_page() + 1,
            self.total_count
        ));
        output.push_str(
            "╠═══╤═════════════════╤═════════════════╤════════════════════════╤═══════╣\n",
        );
        output.push_str(
            "║ # │ From            │ To              │ Subject                │ Date  ║\n",
        );
        output.push_str(
            "╠═══╪═════════════════╪═════════════════╪════════════════════════╪═══════╣\n",
        );

        // Messages
        for msg in &self.messages {
            let status = if !msg.is_read {
                "✉"
            } else if msg.is_private {
                "🔒"
            } else {
                " "
            };

            output.push_str(&format!(
                "║{} {:3} │ {:15.15} │ {:15.15} │ {:22.22} │ {:5} ║\n",
                status,
                msg.msg_num,
                msg.from,
                msg.to,
                msg.subject,
                msg.date.format("%m/%d")
            ));
        }

        // Footer
        output.push_str(
            "╚═══╧═════════════════╧═════════════════╧════════════════════════╧═══════╝\n",
        );
        output.push_str("  [N]ext  [P]rev  [R]ead  [S]earch  [Q]uit\n");

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_list_config() {
        let config = MessageListConfig::default();
        assert_eq!(config.messages_per_page, 20);
        assert!(config.show_numbers);
    }

    #[test]
    fn test_max_page_calculation() {
        let mut screen = MessageListScreen::default_config();
        screen.total_count = 100;
        screen.config.messages_per_page = 20;
        assert_eq!(screen.max_page(), 4); // 0-4 = 5 pages

        screen.total_count = 20;
        assert_eq!(screen.max_page(), 0); // 0 = 1 page

        screen.total_count = 21;
        assert_eq!(screen.max_page(), 1); // 0-1 = 2 pages
    }
}
