//! User preferences and display settings
//!
//! This module provides the `UserPreferences` type for storing user-specific
//! preferences such as terminal settings, display options, and UI behavior.

use serde::{Deserialize, Serialize};

/// User preferences and display settings
///
/// Stores all user-configurable preferences including terminal dimensions,
/// display options (ANSI, color, Avatar), and UI behavior (pause, one-key mode).
///
/// # Examples
///
/// ```
/// use impulse_types::user_prefs::UserPreferences;
///
/// let prefs = UserPreferences::default();
/// assert_eq!(prefs.line_length, 80);
/// assert_eq!(prefs.page_length, 24);
/// assert!(prefs.ansi_enabled);
/// assert!(prefs.color_enabled);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Terminal line length in columns (default: 80)
    pub line_length: u8,

    /// Terminal page length in rows (default: 24)
    pub page_length: u8,

    /// ANSI escape codes enabled
    pub ansi_enabled: bool,

    /// Color output enabled
    pub color_enabled: bool,

    /// Pause after each screen
    pub pause_enabled: bool,

    /// One-key command mode (no Enter required)
    pub one_key_mode: bool,

    /// Avatar graphics enabled
    pub avatar_enabled: bool,

    /// Novice mode (extra help prompts)
    pub novice_mode: bool,

    /// Clear screen before messages
    pub clear_screen: bool,

    /// Expert mode (minimal prompts)
    pub expert_mode: bool,

    /// Use hot keys (single key commands)
    pub hot_keys: bool,

    /// Skip full-screen editor
    pub skip_full_screen_editor: bool,
}

impl Default for UserPreferences {
    /// Default preferences for new users
    ///
    /// # Default Values
    ///
    /// - Line length: 80 columns
    /// - Page length: 24 rows
    /// - ANSI: enabled
    /// - Color: enabled
    /// - Pause: enabled
    /// - One-key mode: disabled
    /// - Avatar: disabled
    /// - Novice mode: enabled
    /// - Clear screen: enabled
    /// - Expert mode: disabled
    /// - Hot keys: disabled
    /// - Skip full-screen editor: disabled
    fn default() -> Self {
        UserPreferences {
            line_length: 80,
            page_length: 24,
            ansi_enabled: true,
            color_enabled: true,
            pause_enabled: true,
            one_key_mode: false,
            avatar_enabled: false,
            novice_mode: true,
            clear_screen: true,
            expert_mode: false,
            hot_keys: false,
            skip_full_screen_editor: false,
        }
    }
}

impl UserPreferences {
    /// Create new user preferences with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let prefs = UserPreferences::new();
    /// assert_eq!(prefs.line_length, 80);
    /// assert!(prefs.ansi_enabled);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create preferences for a basic terminal (no ANSI, no color)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let prefs = UserPreferences::basic_terminal();
    /// assert!(!prefs.ansi_enabled);
    /// assert!(!prefs.color_enabled);
    /// assert!(!prefs.avatar_enabled);
    /// ```
    #[must_use]
    pub fn basic_terminal() -> Self {
        UserPreferences {
            ansi_enabled: false,
            color_enabled: false,
            avatar_enabled: false,
            ..Default::default()
        }
    }

    /// Create preferences for an expert user (minimal prompts)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let prefs = UserPreferences::expert();
    /// assert!(prefs.expert_mode);
    /// assert!(!prefs.novice_mode);
    /// assert!(prefs.hot_keys);
    /// assert!(!prefs.pause_enabled);
    /// ```
    #[must_use]
    pub fn expert() -> Self {
        UserPreferences {
            expert_mode: true,
            novice_mode: false,
            hot_keys: true,
            one_key_mode: true,
            pause_enabled: false,
            ..Default::default()
        }
    }

    /// Validate terminal dimensions
    ///
    /// Returns `true` if line_length and page_length are within acceptable ranges.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let mut prefs = UserPreferences::new();
    /// assert!(prefs.validate_dimensions());
    ///
    /// prefs.line_length = 20; // Too narrow
    /// assert!(!prefs.validate_dimensions());
    /// ```
    #[must_use]
    pub fn validate_dimensions(&self) -> bool {
        (40..=255).contains(&self.line_length) && (10..=100).contains(&self.page_length)
    }

    /// Set terminal dimensions
    ///
    /// # Arguments
    ///
    /// * `columns` - Line length (40-255)
    /// * `rows` - Page length (10-100)
    ///
    /// Returns `true` if dimensions are valid and set.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let mut prefs = UserPreferences::new();
    /// assert!(prefs.set_dimensions(132, 43));
    /// assert_eq!(prefs.line_length, 132);
    /// assert_eq!(prefs.page_length, 43);
    ///
    /// assert!(!prefs.set_dimensions(20, 5)); // Too small
    /// ```
    pub fn set_dimensions(&mut self, columns: u8, rows: u8) -> bool {
        if (40..=255).contains(&columns) && (10..=100).contains(&rows) {
            self.line_length = columns;
            self.page_length = rows;
            true
        } else {
            false
        }
    }

    /// Enable full graphics mode (ANSI, color, Avatar)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let mut prefs = UserPreferences::basic_terminal();
    /// prefs.enable_graphics();
    /// assert!(prefs.ansi_enabled);
    /// assert!(prefs.color_enabled);
    /// assert!(prefs.avatar_enabled);
    /// ```
    pub fn enable_graphics(&mut self) {
        self.ansi_enabled = true;
        self.color_enabled = true;
        self.avatar_enabled = true;
    }

    /// Disable all graphics (basic ASCII mode)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let mut prefs = UserPreferences::new();
    /// prefs.disable_graphics();
    /// assert!(!prefs.ansi_enabled);
    /// assert!(!prefs.color_enabled);
    /// assert!(!prefs.avatar_enabled);
    /// ```
    pub fn disable_graphics(&mut self) {
        self.ansi_enabled = false;
        self.color_enabled = false;
        self.avatar_enabled = false;
    }

    /// Check if user has full graphics capabilities
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let prefs = UserPreferences::new();
    /// assert!(prefs.has_graphics());
    ///
    /// let basic = UserPreferences::basic_terminal();
    /// assert!(!basic.has_graphics());
    /// ```
    #[must_use]
    pub fn has_graphics(&self) -> bool {
        self.ansi_enabled && self.color_enabled
    }

    /// Check if user wants minimal UI (expert mode)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let expert = UserPreferences::expert();
    /// assert!(expert.wants_minimal_ui());
    ///
    /// let novice = UserPreferences::new();
    /// assert!(!novice.wants_minimal_ui());
    /// ```
    #[must_use]
    pub fn wants_minimal_ui(&self) -> bool {
        self.expert_mode || (self.hot_keys && !self.novice_mode)
    }

    /// Get the number of displayable lines (page_length - 1 for prompts)
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::user_prefs::UserPreferences;
    ///
    /// let prefs = UserPreferences::new();
    /// assert_eq!(prefs.displayable_lines(), 23); // 24 - 1
    /// ```
    #[must_use]
    pub fn displayable_lines(&self) -> u8 {
        self.page_length.saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let prefs = UserPreferences::default();
        assert_eq!(prefs.line_length, 80);
        assert_eq!(prefs.page_length, 24);
        assert!(prefs.ansi_enabled);
        assert!(prefs.color_enabled);
        assert!(prefs.pause_enabled);
        assert!(!prefs.one_key_mode);
        assert!(!prefs.avatar_enabled);
        assert!(prefs.novice_mode);
        assert!(prefs.clear_screen);
        assert!(!prefs.expert_mode);
    }

    #[test]
    fn test_basic_terminal() {
        let prefs = UserPreferences::basic_terminal();
        assert!(!prefs.ansi_enabled);
        assert!(!prefs.color_enabled);
        assert!(!prefs.avatar_enabled);
        assert_eq!(prefs.line_length, 80);
    }

    #[test]
    fn test_expert() {
        let prefs = UserPreferences::expert();
        assert!(prefs.expert_mode);
        assert!(!prefs.novice_mode);
        assert!(prefs.hot_keys);
        assert!(prefs.one_key_mode);
        assert!(!prefs.pause_enabled);
    }

    #[test]
    fn test_validate_dimensions() {
        let mut prefs = UserPreferences::new();
        assert!(prefs.validate_dimensions());

        prefs.line_length = 39; // Too narrow
        assert!(!prefs.validate_dimensions());

        prefs.line_length = 255; // At max (will pass)
        assert!(prefs.validate_dimensions());

        prefs.line_length = 80;
        prefs.page_length = 9; // Too short
        assert!(!prefs.validate_dimensions());

        prefs.page_length = 100; // At max (will pass)
        assert!(prefs.validate_dimensions());
    }

    #[test]
    fn test_set_dimensions() {
        let mut prefs = UserPreferences::new();

        assert!(prefs.set_dimensions(132, 43));
        assert_eq!(prefs.line_length, 132);
        assert_eq!(prefs.page_length, 43);

        assert!(!prefs.set_dimensions(20, 5)); // Too small
        assert_eq!(prefs.line_length, 132); // Should not change
        assert_eq!(prefs.page_length, 43);
    }

    #[test]
    fn test_enable_graphics() {
        let mut prefs = UserPreferences::basic_terminal();
        prefs.enable_graphics();
        assert!(prefs.ansi_enabled);
        assert!(prefs.color_enabled);
        assert!(prefs.avatar_enabled);
    }

    #[test]
    fn test_disable_graphics() {
        let mut prefs = UserPreferences::new();
        prefs.disable_graphics();
        assert!(!prefs.ansi_enabled);
        assert!(!prefs.color_enabled);
        assert!(!prefs.avatar_enabled);
    }

    #[test]
    fn test_has_graphics() {
        let prefs = UserPreferences::new();
        assert!(prefs.has_graphics());

        let basic = UserPreferences::basic_terminal();
        assert!(!basic.has_graphics());

        let mut partial = UserPreferences::new();
        partial.ansi_enabled = false;
        assert!(!partial.has_graphics()); // Needs both ANSI and color
    }

    #[test]
    fn test_wants_minimal_ui() {
        let expert = UserPreferences::expert();
        assert!(expert.wants_minimal_ui());

        let novice = UserPreferences::new();
        assert!(!novice.wants_minimal_ui());

        let mut hot_keys = UserPreferences::new();
        hot_keys.hot_keys = true;
        hot_keys.novice_mode = false;
        assert!(hot_keys.wants_minimal_ui());
    }

    #[test]
    fn test_displayable_lines() {
        let prefs = UserPreferences::new();
        assert_eq!(prefs.displayable_lines(), 23);

        let mut custom = UserPreferences::new();
        custom.page_length = 50;
        assert_eq!(custom.displayable_lines(), 49);
    }

    #[test]
    fn test_serialization() {
        let prefs = UserPreferences::expert();
        let json = serde_json::to_string(&prefs).unwrap();
        let deserialized: UserPreferences = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, prefs);
    }
}
