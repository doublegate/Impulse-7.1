//! Virus scanner trait definitions

use crate::error::Result;
use async_trait::async_trait;
use std::path::Path;

/// Virus scan result
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// Whether the file is clean (no threats detected)
    pub is_clean: bool,

    /// Name of detected threat (if any)
    pub threat_name: Option<String>,

    /// Scan time in milliseconds
    pub scan_time_ms: u64,
}

impl ScanResult {
    /// Create a clean scan result
    pub fn clean(scan_time_ms: u64) -> Self {
        Self {
            is_clean: true,
            threat_name: None,
            scan_time_ms,
        }
    }

    /// Create an infected scan result
    pub fn infected(threat_name: String, scan_time_ms: u64) -> Self {
        Self {
            is_clean: false,
            threat_name: Some(threat_name),
            scan_time_ms,
        }
    }
}

/// Virus scanner trait
///
/// Provides an abstraction for virus scanning functionality.
/// Implementations can use ClamAV, other antivirus engines, or
/// mock scanners for testing.
#[async_trait]
pub trait VirusScanner: Send + Sync {
    /// Scan a file for viruses
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to scan
    ///
    /// # Returns
    ///
    /// Scan result indicating whether the file is clean or infected
    async fn scan_file(&self, path: &Path) -> Result<ScanResult>;

    /// Check if the scanner is available
    ///
    /// # Returns
    ///
    /// `true` if the scanner can be used, `false` otherwise
    async fn is_available(&self) -> bool;

    /// Get scanner name/version
    fn name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_result_clean() {
        let result = ScanResult::clean(100);
        assert!(result.is_clean);
        assert!(result.threat_name.is_none());
        assert_eq!(result.scan_time_ms, 100);
    }

    #[test]
    fn test_scan_result_infected() {
        let result = ScanResult::infected("Trojan.Generic".to_string(), 150);
        assert!(!result.is_clean);
        assert_eq!(result.threat_name, Some("Trojan.Generic".to_string()));
        assert_eq!(result.scan_time_ms, 150);
    }
}
