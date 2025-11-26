//! File search screen

use crate::types::SearchCriteria;
use std::fmt;

/// Search screen
///
/// Provides an interface for building search criteria.
pub struct SearchScreen {
    criteria: SearchCriteria,
    results_count: Option<usize>,
}

impl SearchScreen {
    /// Create a new search screen
    pub fn new() -> Self {
        Self {
            criteria: SearchCriteria::new(),
            results_count: None,
        }
    }

    /// Get the current search criteria
    pub fn criteria(&self) -> &SearchCriteria {
        &self.criteria
    }

    /// Set filename pattern
    pub fn set_filename(&mut self, pattern: String) {
        if pattern.is_empty() {
            self.criteria.filename = None;
        } else {
            self.criteria.filename = Some(pattern);
        }
        self.results_count = None; // Clear results
    }

    /// Set description search
    pub fn set_description(&mut self, text: String) {
        if text.is_empty() {
            self.criteria.description = None;
        } else {
            self.criteria.description = Some(text);
        }
        self.results_count = None;
    }

    /// Set uploader filter
    pub fn set_uploader(&mut self, username: String) {
        if username.is_empty() {
            self.criteria.uploader = None;
        } else {
            self.criteria.uploader = Some(username);
        }
        self.results_count = None;
    }

    /// Set minimum file size
    pub fn set_min_size(&mut self, size: Option<u64>) {
        self.criteria.min_size = size;
        self.results_count = None;
    }

    /// Set maximum file size
    pub fn set_max_size(&mut self, size: Option<u64>) {
        self.criteria.max_size = size;
        self.results_count = None;
    }

    /// Toggle new files only filter
    pub fn toggle_new_only(&mut self) {
        self.criteria.new_only = !self.criteria.new_only;
        self.results_count = None;
    }

    /// Toggle available files only filter
    pub fn toggle_available_only(&mut self) {
        self.criteria.available_only = !self.criteria.available_only;
        self.results_count = None;
    }

    /// Set search results count
    pub fn set_results_count(&mut self, count: usize) {
        self.results_count = Some(count);
    }

    /// Clear all search criteria
    pub fn clear(&mut self) {
        self.criteria = SearchCriteria::new();
        self.results_count = None;
    }

    /// Check if any criteria is set
    pub fn has_criteria(&self) -> bool {
        !self.criteria.is_empty()
    }

    /// Render the screen
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str("=== File Search ===\n\n");

        // Current criteria
        output.push_str("Search Criteria:\n");
        output.push_str(&format!("{}\n", "-".repeat(78)));

        if let Some(ref filename) = self.criteria.filename {
            output.push_str(&format!("Filename:     {}\n", filename));
        } else {
            output.push_str("Filename:     (any)\n");
        }

        if let Some(ref description) = self.criteria.description {
            output.push_str(&format!("Description:  {}\n", description));
        } else {
            output.push_str("Description:  (any)\n");
        }

        if let Some(ref uploader) = self.criteria.uploader {
            output.push_str(&format!("Uploader:     {}\n", uploader));
        } else {
            output.push_str("Uploader:     (any)\n");
        }

        // Size range
        match (self.criteria.min_size, self.criteria.max_size) {
            (Some(min), Some(max)) => {
                output.push_str(&format!("Size:         {} - {} bytes\n", min, max));
            }
            (Some(min), None) => {
                output.push_str(&format!("Size:         >= {} bytes\n", min));
            }
            (None, Some(max)) => {
                output.push_str(&format!("Size:         <= {} bytes\n", max));
            }
            (None, None) => {
                output.push_str("Size:         (any)\n");
            }
        }

        // Filters
        output.push_str(&format!(
            "New files:    {}\n",
            if self.criteria.new_only { "Yes" } else { "No" }
        ));
        output.push_str(&format!(
            "Available:    {}\n",
            if self.criteria.available_only {
                "Yes"
            } else {
                "No"
            }
        ));

        output.push_str(&format!("{}\n\n", "-".repeat(78)));

        // Results
        if let Some(count) = self.results_count {
            output.push_str(&format!("Results: {} files found\n\n", count));
        } else if self.has_criteria() {
            output.push_str("Press [Enter] to search\n\n");
        } else {
            output.push_str("Enter search criteria\n\n");
        }

        // Commands
        output.push_str("[F]ilename [D]escription [U]ploader [S]ize\n");
        output.push_str("[N]ew only [A]vailable only [C]lear [Enter]=Search [Q]uit\n");

        output
    }
}

impl Default for SearchScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SearchScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_screen_new() {
        let screen = SearchScreen::new();
        assert!(!screen.has_criteria());
        assert!(screen.results_count.is_none());
    }

    #[test]
    fn test_set_filename() {
        let mut screen = SearchScreen::new();
        screen.set_filename("*.zip".to_string());

        assert_eq!(screen.criteria.filename, Some("*.zip".to_string()));
        assert!(screen.has_criteria());
    }

    #[test]
    fn test_set_filename_empty() {
        let mut screen = SearchScreen::new();
        screen.set_filename("test".to_string());
        screen.set_filename("".to_string());

        assert!(screen.criteria.filename.is_none());
    }

    #[test]
    fn test_set_description() {
        let mut screen = SearchScreen::new();
        screen.set_description("game".to_string());

        assert_eq!(screen.criteria.description, Some("game".to_string()));
        assert!(screen.has_criteria());
    }

    #[test]
    fn test_set_uploader() {
        let mut screen = SearchScreen::new();
        screen.set_uploader("alice".to_string());

        assert_eq!(screen.criteria.uploader, Some("alice".to_string()));
        assert!(screen.has_criteria());
    }

    #[test]
    fn test_set_size_range() {
        let mut screen = SearchScreen::new();
        screen.set_min_size(Some(1000));
        screen.set_max_size(Some(5000));

        assert_eq!(screen.criteria.min_size, Some(1000));
        assert_eq!(screen.criteria.max_size, Some(5000));
        assert!(screen.has_criteria());
    }

    #[test]
    fn test_toggle_new_only() {
        let mut screen = SearchScreen::new();
        assert!(!screen.criteria.new_only);

        screen.toggle_new_only();
        assert!(screen.criteria.new_only);

        screen.toggle_new_only();
        assert!(!screen.criteria.new_only);
    }

    #[test]
    fn test_toggle_available_only() {
        let mut screen = SearchScreen::new();
        assert!(!screen.criteria.available_only);

        screen.toggle_available_only();
        assert!(screen.criteria.available_only);

        screen.toggle_available_only();
        assert!(!screen.criteria.available_only);
    }

    #[test]
    fn test_clear() {
        let mut screen = SearchScreen::new();
        screen.set_filename("*.zip".to_string());
        screen.set_description("game".to_string());
        screen.set_results_count(42);

        assert!(screen.has_criteria());

        screen.clear();
        assert!(!screen.has_criteria());
        assert!(screen.results_count.is_none());
    }

    #[test]
    fn test_set_results_count() {
        let mut screen = SearchScreen::new();
        screen.set_results_count(10);

        assert_eq!(screen.results_count, Some(10));
    }

    #[test]
    fn test_render() {
        let mut screen = SearchScreen::new();
        screen.set_filename("*.zip".to_string());
        screen.set_description("game".to_string());

        let output = screen.render();

        assert!(output.contains("File Search"));
        assert!(output.contains("*.zip"));
        assert!(output.contains("game"));
        assert!(output.contains("Enter] to search"));
    }

    #[test]
    fn test_render_with_results() {
        let mut screen = SearchScreen::new();
        screen.set_filename("*.zip".to_string());
        screen.set_results_count(5);

        let output = screen.render();

        assert!(output.contains("5 files found"));
    }
}
