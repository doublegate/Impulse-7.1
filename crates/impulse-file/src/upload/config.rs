//! Upload configuration

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Upload configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadConfig {
    /// Maximum file size in bytes (default: 50 MB)
    pub max_file_size: u64,

    /// Maximum files per user per day (default: 10)
    pub max_files_per_day: u32,

    /// Maximum bytes per user per day (default: 100 MB)
    pub max_bytes_per_day: u64,

    /// Allowed file extensions (empty = allow all)
    pub allowed_extensions: HashSet<String>,

    /// Blocked file extensions (checked after allowed)
    pub blocked_extensions: HashSet<String>,

    /// Enable virus scanning
    pub enable_virus_scan: bool,

    /// ClamAV socket path (Unix socket or tcp://host:port)
    pub clamav_socket: String,

    /// Quarantine directory for infected files
    pub quarantine_dir: String,

    /// Enable duplicate detection
    pub enable_duplicate_check: bool,
}

impl Default for UploadConfig {
    fn default() -> Self {
        Self {
            max_file_size: 50 * 1024 * 1024, // 50 MB
            max_files_per_day: 10,
            max_bytes_per_day: 100 * 1024 * 1024, // 100 MB
            allowed_extensions: HashSet::new(),
            blocked_extensions: ["exe", "com", "bat", "cmd", "scr", "pif", "vbs", "js"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            enable_virus_scan: true,
            clamav_socket: "/var/run/clamav/clamd.ctl".to_string(),
            quarantine_dir: "/var/bbs/quarantine".to_string(),
            enable_duplicate_check: true,
        }
    }
}

impl UploadConfig {
    /// Create a new upload configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum file size
    pub fn with_max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = size;
        self
    }

    /// Set maximum files per day
    pub fn with_max_files_per_day(mut self, count: u32) -> Self {
        self.max_files_per_day = count;
        self
    }

    /// Set maximum bytes per day
    pub fn with_max_bytes_per_day(mut self, bytes: u64) -> Self {
        self.max_bytes_per_day = bytes;
        self
    }

    /// Add allowed extension
    pub fn allow_extension(mut self, ext: impl Into<String>) -> Self {
        self.allowed_extensions.insert(ext.into().to_lowercase());
        self
    }

    /// Add blocked extension
    pub fn block_extension(mut self, ext: impl Into<String>) -> Self {
        self.blocked_extensions.insert(ext.into().to_lowercase());
        self
    }

    /// Set ClamAV socket path
    pub fn with_clamav_socket(mut self, socket: impl Into<String>) -> Self {
        self.clamav_socket = socket.into();
        self
    }

    /// Disable virus scanning
    pub fn disable_virus_scan(mut self) -> Self {
        self.enable_virus_scan = false;
        self
    }

    /// Disable duplicate checking
    pub fn disable_duplicate_check(mut self) -> Self {
        self.enable_duplicate_check = false;
        self
    }

    /// Check if extension is allowed
    pub fn is_extension_allowed(&self, ext: &str) -> bool {
        let ext_lower = ext.to_lowercase();

        // If blocked, reject immediately
        if self.blocked_extensions.contains(&ext_lower) {
            return false;
        }

        // If allowed list is empty, accept all (except blocked)
        if self.allowed_extensions.is_empty() {
            return true;
        }

        // Check if in allowed list
        self.allowed_extensions.contains(&ext_lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UploadConfig::default();
        assert_eq!(config.max_file_size, 50 * 1024 * 1024);
        assert_eq!(config.max_files_per_day, 10);
        assert!(config.enable_virus_scan);
        assert!(config.enable_duplicate_check);
    }

    #[test]
    fn test_builder_pattern() {
        let config = UploadConfig::new()
            .with_max_file_size(10 * 1024 * 1024)
            .with_max_files_per_day(5)
            .allow_extension("zip")
            .block_extension("exe");

        assert_eq!(config.max_file_size, 10 * 1024 * 1024);
        assert_eq!(config.max_files_per_day, 5);
        assert!(config.allowed_extensions.contains("zip"));
        assert!(config.blocked_extensions.contains("exe"));
    }

    #[test]
    fn test_extension_allowed_with_empty_list() {
        let config = UploadConfig {
            allowed_extensions: HashSet::new(),
            blocked_extensions: ["exe"].iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        };

        assert!(config.is_extension_allowed("zip"));
        assert!(config.is_extension_allowed("txt"));
        assert!(!config.is_extension_allowed("exe"));
    }

    #[test]
    fn test_extension_allowed_with_allowlist() {
        let config = UploadConfig {
            allowed_extensions: ["zip", "txt"].iter().map(|s| s.to_string()).collect(),
            blocked_extensions: HashSet::new(),
            ..Default::default()
        };

        assert!(config.is_extension_allowed("zip"));
        assert!(config.is_extension_allowed("txt"));
        assert!(!config.is_extension_allowed("exe"));
    }

    #[test]
    fn test_extension_blocked_overrides_allowed() {
        let config = UploadConfig {
            allowed_extensions: ["zip", "exe"].iter().map(|s| s.to_string()).collect(),
            blocked_extensions: ["exe"].iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        };

        assert!(config.is_extension_allowed("zip"));
        assert!(!config.is_extension_allowed("exe")); // Blocked wins
    }

    #[test]
    fn test_extension_case_insensitive() {
        let config = UploadConfig {
            allowed_extensions: ["zip"].iter().map(|s| s.to_string()).collect(),
            blocked_extensions: HashSet::new(),
            ..Default::default()
        };

        assert!(config.is_extension_allowed("ZIP"));
        assert!(config.is_extension_allowed("Zip"));
        assert!(config.is_extension_allowed("zip"));
    }
}
