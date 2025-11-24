//! Menu and command flags for the menu system
//!
//! This module defines flag types for menu display and command visibility
//! from the original Pascal RECORDS.PAS file.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Menu display flags (Pascal: `mnuflags` set type)
    ///
    /// Original Pascal definition (RECORDS.PAS lines 712-717):
    /// ```pascal
    /// mnuflags=
    ///  (clrscrbefore,                 { C: clear screen before menu display }
    ///   dontcenter,                   { D: don't center the menu titles! }
    ///   nomenuprompt,                 { N: no menu prompt whatsoever? }
    ///   forcepause,                   { F: force a pause before menu display? }
    ///   pulldown,                     { P: pulldown flag. }
    ///   ...);
    /// ```
    ///
    /// Controls menu display behavior and formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::menu_flags::MenuFlags;
    ///
    /// let mut flags = MenuFlags::CLEAR_SCREEN | MenuFlags::FORCE_PAUSE;
    ///
    /// assert!(flags.clears_screen());
    /// assert!(flags.forces_pause());
    /// assert!(!flags.is_pulldown());
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct MenuFlags: u8 {
        /// Clear screen before menu display
        const CLEAR_SCREEN      = 0b0000_0001;

        /// Don't center the menu titles
        const DONT_CENTER       = 0b0000_0010;

        /// No menu prompt whatsoever
        const NO_MENU_PROMPT    = 0b0000_0100;

        /// Force a pause before menu display
        const FORCE_PAUSE       = 0b0000_1000;

        /// Pulldown menu flag
        const PULLDOWN          = 0b0001_0000;
    }
}

impl Default for MenuFlags {
    fn default() -> Self {
        MenuFlags::empty()
    }
}

impl MenuFlags {
    /// Create from Pascal byte
    pub fn from_pascal_byte(byte: u8) -> Self {
        MenuFlags::from_bits_truncate(byte)
    }

    /// Convert to Pascal byte
    pub fn to_pascal_byte(self) -> u8 {
        self.bits()
    }

    /// Check if screen should be cleared before display
    pub fn clears_screen(self) -> bool {
        self.contains(MenuFlags::CLEAR_SCREEN)
    }

    /// Check if titles should not be centered
    pub fn centers_titles(self) -> bool {
        !self.contains(MenuFlags::DONT_CENTER)
    }

    /// Check if menu prompt should be displayed
    pub fn shows_prompt(self) -> bool {
        !self.contains(MenuFlags::NO_MENU_PROMPT)
    }

    /// Check if pause should be forced before display
    pub fn forces_pause(self) -> bool {
        self.contains(MenuFlags::FORCE_PAUSE)
    }

    /// Check if menu is pulldown style
    pub fn is_pulldown(self) -> bool {
        self.contains(MenuFlags::PULLDOWN)
    }
}

bitflags! {
    /// Command visibility flags (Pascal: `cmdflags` set type)
    ///
    /// Original Pascal definition (RECORDS.PAS lines 736-739):
    /// ```pascal
    /// cmdflags=
    ///  (hidden,                       { H: is command ALWAYS hidden? }
    ///   pull,                         { P: is command flagged as Pulldown Active? }
    ///   unhidden);                    { U: is command ALWAYS visible? }
    /// ```
    ///
    /// Controls command visibility and behavior in menus.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_types::menu_flags::CommandFlags;
    ///
    /// let mut flags = CommandFlags::PULLDOWN;
    ///
    /// assert!(flags.is_pulldown());
    /// assert!(!flags.is_always_hidden());
    /// assert!(!flags.is_always_visible());
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct CommandFlags: u8 {
        /// Command is always hidden
        const ALWAYS_HIDDEN     = 0b0000_0001;

        /// Command is flagged as pulldown active
        const PULLDOWN          = 0b0000_0010;

        /// Command is always visible
        const ALWAYS_VISIBLE    = 0b0000_0100;
    }
}

impl Default for CommandFlags {
    fn default() -> Self {
        CommandFlags::empty()
    }
}

impl CommandFlags {
    /// Create from Pascal byte
    pub fn from_pascal_byte(byte: u8) -> Self {
        CommandFlags::from_bits_truncate(byte)
    }

    /// Convert to Pascal byte
    pub fn to_pascal_byte(self) -> u8 {
        self.bits()
    }

    /// Check if command is always hidden
    pub fn is_always_hidden(self) -> bool {
        self.contains(CommandFlags::ALWAYS_HIDDEN)
    }

    /// Check if command is pulldown active
    pub fn is_pulldown(self) -> bool {
        self.contains(CommandFlags::PULLDOWN)
    }

    /// Check if command is always visible
    pub fn is_always_visible(self) -> bool {
        self.contains(CommandFlags::ALWAYS_VISIBLE)
    }

    /// Check if visibility is conditional (neither always hidden nor always visible)
    pub fn is_conditional(self) -> bool {
        !self.intersects(CommandFlags::ALWAYS_HIDDEN | CommandFlags::ALWAYS_VISIBLE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // MenuFlags tests
    #[test]
    fn test_menu_flags_default() {
        let flags = MenuFlags::default();
        assert!(flags.is_empty());
    }

    #[test]
    fn test_menu_flags_pascal_conversion() {
        let flags = MenuFlags::CLEAR_SCREEN | MenuFlags::PULLDOWN;
        let byte = flags.to_pascal_byte();
        let restored = MenuFlags::from_pascal_byte(byte);
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_menu_flags_clear_screen() {
        let mut flags = MenuFlags::empty();
        assert!(!flags.clears_screen());

        flags.insert(MenuFlags::CLEAR_SCREEN);
        assert!(flags.clears_screen());
    }

    #[test]
    fn test_menu_flags_center_titles() {
        let mut flags = MenuFlags::empty();
        assert!(flags.centers_titles()); // Default is to center

        flags.insert(MenuFlags::DONT_CENTER);
        assert!(!flags.centers_titles());
    }

    #[test]
    fn test_menu_flags_show_prompt() {
        let mut flags = MenuFlags::empty();
        assert!(flags.shows_prompt()); // Default is to show

        flags.insert(MenuFlags::NO_MENU_PROMPT);
        assert!(!flags.shows_prompt());
    }

    #[test]
    fn test_menu_flags_pulldown() {
        let flags = MenuFlags::PULLDOWN | MenuFlags::FORCE_PAUSE;
        assert!(flags.is_pulldown());
        assert!(flags.forces_pause());
    }

    #[test]
    fn test_menu_flags_serialization() {
        let flags = MenuFlags::CLEAR_SCREEN | MenuFlags::PULLDOWN;
        let json = serde_json::to_string(&flags).unwrap();
        let restored: MenuFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, restored);
    }

    // CommandFlags tests
    #[test]
    fn test_command_flags_default() {
        let flags = CommandFlags::default();
        assert!(flags.is_empty());
    }

    #[test]
    fn test_command_flags_pascal_conversion() {
        let flags = CommandFlags::ALWAYS_VISIBLE | CommandFlags::PULLDOWN;
        let byte = flags.to_pascal_byte();
        let restored = CommandFlags::from_pascal_byte(byte);
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_command_flags_visibility() {
        let hidden = CommandFlags::ALWAYS_HIDDEN;
        assert!(hidden.is_always_hidden());
        assert!(!hidden.is_always_visible());
        assert!(!hidden.is_conditional());

        let visible = CommandFlags::ALWAYS_VISIBLE;
        assert!(visible.is_always_visible());
        assert!(!visible.is_always_hidden());
        assert!(!visible.is_conditional());

        let conditional = CommandFlags::empty();
        assert!(conditional.is_conditional());
        assert!(!conditional.is_always_hidden());
        assert!(!conditional.is_always_visible());
    }

    #[test]
    fn test_command_flags_pulldown() {
        let flags = CommandFlags::PULLDOWN;
        assert!(flags.is_pulldown());
        assert!(flags.is_conditional()); // Pulldown can be conditional
    }

    #[test]
    fn test_command_flags_serialization() {
        let flags = CommandFlags::ALWAYS_VISIBLE | CommandFlags::PULLDOWN;
        let json = serde_json::to_string(&flags).unwrap();
        let restored: CommandFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_command_flags_mutually_exclusive() {
        // Test that a command can't be both always hidden and always visible
        // (though the type system doesn't prevent this, it's semantically incorrect)
        let conflicted = CommandFlags::ALWAYS_HIDDEN | CommandFlags::ALWAYS_VISIBLE;
        assert!(conflicted.is_always_hidden());
        assert!(conflicted.is_always_visible());
        assert!(!conflicted.is_conditional()); // Both bits set = not conditional
    }
}
