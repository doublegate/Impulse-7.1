//! FILE_ID.DIZ content parser
//!
//! Parses and cleans DIZ file content

/// Common FILE_ID.DIZ filename variants
pub const DIZ_FILENAMES: &[&str] = &[
    "FILE_ID.DIZ",
    "file_id.diz",
    "DESC.SDI",
    "desc.sdi",
    "DESCRIPT.ION",
    "descript.ion",
];

/// Maximum size for DIZ file (32 KB)
pub const MAX_DIZ_SIZE: u64 = 32 * 1024;

/// Clean FILE_ID.DIZ content
///
/// Removes control characters, trims whitespace, and normalizes line endings.
pub fn clean_diz_content(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            // Remove control characters except tab and newline
            line.chars()
                .filter(|c| !c.is_control() || *c == '\t')
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_diz_content() {
        let content = "  Test File  \n  Description  \n\n  Line 3  ";
        let cleaned = clean_diz_content(content);
        assert_eq!(cleaned, "Test File\n  Description\n\n  Line 3");
    }

    #[test]
    fn test_clean_diz_removes_control_chars() {
        let content = "Test\x00File\x01\x02\x03Description";
        let cleaned = clean_diz_content(content);
        assert_eq!(cleaned, "TestFileDescription");
    }

    #[test]
    fn test_clean_diz_preserves_tabs() {
        let content = "Column1\tColumn2\tColumn3";
        let cleaned = clean_diz_content(content);
        assert_eq!(cleaned, "Column1\tColumn2\tColumn3");
    }
}
