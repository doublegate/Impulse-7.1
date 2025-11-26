//! File area selection screen

use crate::types::FileArea;
use std::fmt;

/// Area selection screen
///
/// Displays a list of file areas with descriptions and file counts.
pub struct AreaSelectionScreen {
    areas: Vec<FileArea>,
    selected_index: usize,
    title: String,
}

impl AreaSelectionScreen {
    /// Create a new area selection screen
    pub fn new(areas: Vec<FileArea>) -> Self {
        Self {
            areas,
            selected_index: 0,
            title: "File Areas".to_string(),
        }
    }

    /// Set custom title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Get selected area index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Get selected area
    pub fn selected_area(&self) -> Option<&FileArea> {
        self.areas.get(self.selected_index)
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.selected_index < self.areas.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Jump to specific index
    pub fn select(&mut self, index: usize) {
        if index < self.areas.len() {
            self.selected_index = index;
        }
    }

    /// Render the screen
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Title
        output.push_str(&format!("=== {} ===\n\n", self.title));

        // Areas list
        for (i, area) in self.areas.iter().enumerate() {
            let marker = if i == self.selected_index { ">" } else { " " };

            output.push_str(&format!(
                "{} {:2}. {:20} ({:4} files) - {}\n",
                marker,
                i + 1,
                truncate(&area.name, 20),
                area.file_count,
                truncate(&area.description, 50)
            ));
        }

        output.push_str("\n[Enter]=Select [Q]=Quit\n");

        output
    }
}

impl Default for AreaSelectionScreen {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl fmt::Display for AreaSelectionScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Truncate a string to a maximum length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("{:width$}...", &s[..max_len - 3], width = max_len - 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_area(id: u32, name: &str, file_count: u32) -> FileArea {
        let mut area = FileArea::new(id, name.to_string(), format!("{} area", name));
        area.file_count = file_count;
        area
    }

    #[test]
    fn test_area_selection_new() {
        let areas = vec![
            create_test_area(1, "General", 100),
            create_test_area(2, "Games", 50),
        ];

        let screen = AreaSelectionScreen::new(areas);
        assert_eq!(screen.selected_index, 0);
        assert_eq!(screen.areas.len(), 2);
    }

    #[test]
    fn test_move_up_down() {
        let areas = vec![
            create_test_area(1, "Area1", 10),
            create_test_area(2, "Area2", 20),
            create_test_area(3, "Area3", 30),
        ];

        let mut screen = AreaSelectionScreen::new(areas);

        assert_eq!(screen.selected_index, 0);

        screen.move_down();
        assert_eq!(screen.selected_index, 1);

        screen.move_down();
        assert_eq!(screen.selected_index, 2);

        screen.move_down(); // Should not go beyond last
        assert_eq!(screen.selected_index, 2);

        screen.move_up();
        assert_eq!(screen.selected_index, 1);

        screen.move_up();
        assert_eq!(screen.selected_index, 0);

        screen.move_up(); // Should not go below 0
        assert_eq!(screen.selected_index, 0);
    }

    #[test]
    fn test_select() {
        let areas = vec![
            create_test_area(1, "Area1", 10),
            create_test_area(2, "Area2", 20),
            create_test_area(3, "Area3", 30),
        ];

        let mut screen = AreaSelectionScreen::new(areas);

        screen.select(2);
        assert_eq!(screen.selected_index, 2);

        screen.select(0);
        assert_eq!(screen.selected_index, 0);

        screen.select(999); // Out of bounds, should not change
        assert_eq!(screen.selected_index, 0);
    }

    #[test]
    fn test_selected_area() {
        let areas = vec![
            create_test_area(1, "Area1", 10),
            create_test_area(2, "Area2", 20),
        ];

        let mut screen = AreaSelectionScreen::new(areas);

        let area = screen.selected_area().unwrap();
        assert_eq!(area.name, "Area1");

        screen.move_down();
        let area = screen.selected_area().unwrap();
        assert_eq!(area.name, "Area2");
    }

    #[test]
    fn test_render() {
        let areas = vec![
            create_test_area(1, "General", 100),
            create_test_area(2, "Games", 50),
        ];

        let screen = AreaSelectionScreen::new(areas);
        let output = screen.render();

        assert!(output.contains("File Areas"));
        assert!(output.contains("General"));
        assert!(output.contains("Games"));
        assert!(output.contains("100 files"));
        assert!(output.contains("50 files"));
        assert!(output.contains(">")); // Selection marker
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Short", 10), "Short     ");
        assert_eq!(truncate("This is a very long string", 10), "This is...");
        assert_eq!(truncate("Exactly10c", 10), "Exactly10c");
    }
}
