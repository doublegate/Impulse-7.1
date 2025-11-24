//! Pascal-compatible type definitions
//!
//! This module provides types that match the binary layout of Pascal records
//! from the original Impulse 7.1 BBS system. These types enable reading and
//! writing binary data files in the original format.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// File list display flags (Pascal: `flistflags` set type)
    ///
    /// Original Pascal definition (RECORDS.PAS lines 53-61):
    /// ```pascal
    /// flistflags =
    ///  (fname,      { filename }
    ///   fsize,      { file size }
    ///   fpts,       { file points }
    ///   fdesc,      { file desc. }
    ///   extdesc,    { extended desc. }
    ///   whoul,      { who uploaded it }
    ///   dateul,     { date uploaded }
    ///   numdl);     { num of d/l's. }
    /// ```
    ///
    /// Controls which fields are displayed in file listings.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct FileListFlags: u8 {
        /// Display filename
        const FILENAME          = 0b0000_0001;

        /// Display file size
        const FILE_SIZE         = 0b0000_0010;

        /// Display file points
        const FILE_POINTS       = 0b0000_0100;

        /// Display file description
        const DESCRIPTION       = 0b0000_1000;

        /// Display extended description
        const EXTENDED_DESC     = 0b0001_0000;

        /// Display who uploaded
        const UPLOADER          = 0b0010_0000;

        /// Display upload date
        const UPLOAD_DATE       = 0b0100_0000;

        /// Display download count
        const DOWNLOAD_COUNT    = 0b1000_0000;
    }
}

impl Default for FileListFlags {
    fn default() -> Self {
        // Default: show filename, size, description, and uploader
        FileListFlags::FILENAME
            | FileListFlags::FILE_SIZE
            | FileListFlags::DESCRIPTION
            | FileListFlags::UPLOADER
    }
}

impl FileListFlags {
    /// Create from Pascal byte
    pub fn from_pascal_byte(byte: u8) -> Self {
        FileListFlags::from_bits_truncate(byte)
    }

    /// Convert to Pascal byte
    pub fn to_pascal_byte(self) -> u8 {
        self.bits()
    }
}

bitflags! {
    /// AR (Access Requirement) flags (Pascal: `set of acrq`)
    ///
    /// Original Pascal definition (RECORDS.PAS line 25):
    /// ```pascal
    /// acrq = '@'..'Z';              { AR flags }
    /// ```
    ///
    /// Represents a set of 27 access requirement flags (characters '@' through 'Z').
    /// Each flag can be used to restrict access to specific areas or features.
    ///
    /// # Binary Format
    ///
    /// Stored as a 32-bit integer (4 bytes) in Pascal binary files.
    /// Bits 0-26 correspond to flags '@' through 'Z'.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ArFlags: u32 {
        const AR_AT = 1 << 0;   // '@'
        const AR_A  = 1 << 1;   // 'A'
        const AR_B  = 1 << 2;   // 'B'
        const AR_C  = 1 << 3;   // 'C'
        const AR_D  = 1 << 4;   // 'D'
        const AR_E  = 1 << 5;   // 'E'
        const AR_F  = 1 << 6;   // 'F'
        const AR_G  = 1 << 7;   // 'G'
        const AR_H  = 1 << 8;   // 'H'
        const AR_I  = 1 << 9;   // 'I'
        const AR_J  = 1 << 10;  // 'J'
        const AR_K  = 1 << 11;  // 'K'
        const AR_L  = 1 << 12;  // 'L'
        const AR_M  = 1 << 13;  // 'M'
        const AR_N  = 1 << 14;  // 'N'
        const AR_O  = 1 << 15;  // 'O'
        const AR_P  = 1 << 16;  // 'P'
        const AR_Q  = 1 << 17;  // 'Q'
        const AR_R  = 1 << 18;  // 'R'
        const AR_S  = 1 << 19;  // 'S'
        const AR_T  = 1 << 20;  // 'T'
        const AR_U  = 1 << 21;  // 'U'
        const AR_V  = 1 << 22;  // 'V'
        const AR_W  = 1 << 23;  // 'W'
        const AR_X  = 1 << 24;  // 'X'
        const AR_Y  = 1 << 25;  // 'Y'
        const AR_Z  = 1 << 26;  // 'Z'
    }
}

impl Default for ArFlags {
    fn default() -> Self {
        ArFlags::empty()
    }
}

impl ArFlags {
    /// Create from Pascal 32-bit integer
    pub fn from_pascal_u32(value: u32) -> Self {
        ArFlags::from_bits_truncate(value)
    }

    /// Convert to Pascal 32-bit integer
    pub fn to_pascal_u32(self) -> u32 {
        self.bits()
    }

    /// Create flags from AR string (e.g., "ABC@" sets flags A, B, C, and @)
    pub fn from_ar_string(s: &str) -> Self {
        let mut flags = ArFlags::empty();
        for c in s.chars() {
            if let Some(flag) = Self::flag_from_char(c) {
                flags.insert(flag);
            }
        }
        flags
    }

    /// Convert to AR string (e.g., ArFlags with A, B, C set returns "ABC")
    pub fn to_ar_string(&self) -> String {
        let mut result = String::new();
        for c in b'@'..=b'Z' {
            if let Some(flag) = Self::flag_from_char(c as char) {
                if self.contains(flag) {
                    result.push(c as char);
                }
            }
        }
        result
    }

    /// Get flag for a specific character ('@' through 'Z')
    fn flag_from_char(c: char) -> Option<ArFlags> {
        match c {
            '@' => Some(ArFlags::AR_AT),
            'A' => Some(ArFlags::AR_A),
            'B' => Some(ArFlags::AR_B),
            'C' => Some(ArFlags::AR_C),
            'D' => Some(ArFlags::AR_D),
            'E' => Some(ArFlags::AR_E),
            'F' => Some(ArFlags::AR_F),
            'G' => Some(ArFlags::AR_G),
            'H' => Some(ArFlags::AR_H),
            'I' => Some(ArFlags::AR_I),
            'J' => Some(ArFlags::AR_J),
            'K' => Some(ArFlags::AR_K),
            'L' => Some(ArFlags::AR_L),
            'M' => Some(ArFlags::AR_M),
            'N' => Some(ArFlags::AR_N),
            'O' => Some(ArFlags::AR_O),
            'P' => Some(ArFlags::AR_P),
            'Q' => Some(ArFlags::AR_Q),
            'R' => Some(ArFlags::AR_R),
            'S' => Some(ArFlags::AR_S),
            'T' => Some(ArFlags::AR_T),
            'U' => Some(ArFlags::AR_U),
            'V' => Some(ArFlags::AR_V),
            'W' => Some(ArFlags::AR_W),
            'X' => Some(ArFlags::AR_X),
            'Y' => Some(ArFlags::AR_Y),
            'Z' => Some(ArFlags::AR_Z),
            _ => None,
        }
    }
}

/// Download scan flags (Pascal: `dlnscan = set of 0..96`)
///
/// Represents which file areas should be scanned for new files.
/// Stored as a 128-bit bitset (16 bytes) for 97 possible file areas (0-96).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub struct DownloadScanFlags {
    /// Bitset for file areas 0-96 (stored as 16 bytes in Pascal)
    bits: [u8; 16],
}


impl DownloadScanFlags {
    /// Create from Pascal byte array (16 bytes)
    pub fn from_pascal_bytes(bytes: [u8; 16]) -> Self {
        DownloadScanFlags { bits: bytes }
    }

    /// Convert to Pascal byte array (16 bytes)
    pub fn to_pascal_bytes(&self) -> [u8; 16] {
        self.bits
    }

    /// Check if area is flagged for scanning
    pub fn is_scanning(&self, area: u8) -> bool {
        if area > 96 {
            return false;
        }
        let byte_idx = (area / 8) as usize;
        let bit_idx = area % 8;
        (self.bits[byte_idx] & (1 << bit_idx)) != 0
    }

    /// Set scanning flag for area
    pub fn set_scanning(&mut self, area: u8, enabled: bool) {
        if area > 96 {
            return;
        }
        let byte_idx = (area / 8) as usize;
        let bit_idx = area % 8;
        if enabled {
            self.bits[byte_idx] |= 1 << bit_idx;
        } else {
            self.bits[byte_idx] &= !(1 << bit_idx);
        }
    }
}

/// User color array (Pascal: `clrs = array[FALSE..TRUE, 0..9] of byte`)
///
/// Stores 20 color values (2 rows × 10 columns).
/// First row (index 0): Normal colors
/// Second row (index 1): Highlighted/intense colors
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserColors {
    /// 2D array: [normal/intense][color_slot]
    colors: [[u8; 10]; 2],
}

impl Default for UserColors {
    fn default() -> Self {
        UserColors {
            // Default ANSI color scheme
            colors: [
                [7, 7, 7, 7, 7, 7, 7, 7, 7, 7],           // Normal (white on black)
                [15, 15, 15, 15, 15, 15, 15, 15, 15, 15], // Intense (bright white)
            ],
        }
    }
}

impl UserColors {
    /// Create from Pascal 2D byte array
    pub fn from_pascal_array(array: [[u8; 10]; 2]) -> Self {
        UserColors { colors: array }
    }

    /// Convert to Pascal 2D byte array
    pub fn to_pascal_array(&self) -> [[u8; 10]; 2] {
        self.colors
    }

    /// Get color at specific slot
    pub fn get(&self, intense: bool, slot: u8) -> Option<u8> {
        if slot >= 10 {
            return None;
        }
        let row = if intense { 1 } else { 0 };
        Some(self.colors[row][slot as usize])
    }

    /// Set color at specific slot
    pub fn set(&mut self, intense: bool, slot: u8, color: u8) -> bool {
        if slot >= 10 {
            return false;
        }
        let row = if intense { 1 } else { 0 };
        self.colors[row][slot as usize] = color;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_list_flags() {
        let flags = FileListFlags::FILENAME | FileListFlags::FILE_SIZE;
        assert_eq!(flags.bits(), 0b0000_0011);

        let restored = FileListFlags::from_pascal_byte(flags.to_pascal_byte());
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_ar_flags_from_string() {
        let flags = ArFlags::from_ar_string("ABC");
        assert!(flags.contains(ArFlags::AR_A));
        assert!(flags.contains(ArFlags::AR_B));
        assert!(flags.contains(ArFlags::AR_C));
        assert!(!flags.contains(ArFlags::AR_D));
    }

    #[test]
    fn test_ar_flags_to_string() {
        let flags = ArFlags::AR_A | ArFlags::AR_B | ArFlags::AR_C;
        assert_eq!(flags.to_ar_string(), "ABC");
    }

    #[test]
    fn test_ar_flags_with_at() {
        let flags = ArFlags::from_ar_string("@ABC");
        assert!(flags.contains(ArFlags::AR_AT));
        assert_eq!(flags.to_ar_string(), "@ABC");
    }

    #[test]
    fn test_download_scan_flags() {
        let mut flags = DownloadScanFlags::default();
        assert!(!flags.is_scanning(0));
        assert!(!flags.is_scanning(96));

        flags.set_scanning(0, true);
        flags.set_scanning(96, true);
        assert!(flags.is_scanning(0));
        assert!(flags.is_scanning(96));

        flags.set_scanning(0, false);
        assert!(!flags.is_scanning(0));
        assert!(flags.is_scanning(96));
    }

    #[test]
    fn test_download_scan_flags_out_of_range() {
        let mut flags = DownloadScanFlags::default();
        flags.set_scanning(97, true); // Out of range
        assert!(!flags.is_scanning(97));
    }

    #[test]
    fn test_user_colors_default() {
        let colors = UserColors::default();
        assert_eq!(colors.get(false, 0), Some(7)); // Normal white
        assert_eq!(colors.get(true, 0), Some(15)); // Intense white
    }

    #[test]
    fn test_user_colors_get_set() {
        let mut colors = UserColors::default();
        assert!(colors.set(false, 5, 12)); // Set color slot 5 to 12
        assert_eq!(colors.get(false, 5), Some(12));
        assert_eq!(colors.get(true, 5), Some(15)); // Intense unchanged
    }

    #[test]
    fn test_user_colors_out_of_range() {
        let mut colors = UserColors::default();
        assert!(!colors.set(false, 10, 1)); // Slot 10 out of range
        assert_eq!(colors.get(false, 10), None);
    }

    #[test]
    fn test_user_colors_roundtrip() {
        let original = UserColors::default();
        let array = original.to_pascal_array();
        let restored = UserColors::from_pascal_array(array);
        assert_eq!(original, restored);
    }
}
