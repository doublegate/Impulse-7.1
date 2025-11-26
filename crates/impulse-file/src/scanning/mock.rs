//! Mock virus scanner for testing

use crate::error::Result;
use crate::scanning::{ScanResult, VirusScanner};
use async_trait::async_trait;
use std::path::Path;

/// Mock scanner for testing
///
/// Always returns a configurable result without actually scanning files.
pub struct MockScanner {
    /// Whether files should be reported as clean
    is_clean: bool,

    /// Threat name to report if not clean
    threat_name: String,

    /// Whether scanner is available
    available: bool,
}

impl MockScanner {
    /// Create a new mock scanner
    pub fn new(is_clean: bool) -> Self {
        Self {
            is_clean,
            threat_name: "Test.Virus".to_string(),
            available: true,
        }
    }

    /// Create a mock scanner that reports infected files
    pub fn infected(threat_name: impl Into<String>) -> Self {
        Self {
            is_clean: false,
            threat_name: threat_name.into(),
            available: true,
        }
    }

    /// Create a mock scanner that reports clean files
    pub fn clean() -> Self {
        Self::new(true)
    }

    /// Create an unavailable mock scanner
    pub fn unavailable() -> Self {
        Self {
            is_clean: true,
            threat_name: String::new(),
            available: false,
        }
    }

    /// Set availability
    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }

    /// Set threat name
    pub fn set_threat_name(&mut self, name: impl Into<String>) {
        self.threat_name = name.into();
    }
}

#[async_trait]
impl VirusScanner for MockScanner {
    async fn scan_file(&self, _path: &Path) -> Result<ScanResult> {
        if self.is_clean {
            Ok(ScanResult::clean(10))
        } else {
            Ok(ScanResult::infected(self.threat_name.clone(), 10))
        }
    }

    async fn is_available(&self) -> bool {
        self.available
    }

    fn name(&self) -> &str {
        "MockScanner"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_mock_scanner_clean() {
        let scanner = MockScanner::clean();
        let result = scanner.scan_file(&PathBuf::from("/test")).await.unwrap();

        assert!(result.is_clean);
        assert!(result.threat_name.is_none());
    }

    #[tokio::test]
    async fn test_mock_scanner_infected() {
        let scanner = MockScanner::infected("Trojan.Test");
        let result = scanner.scan_file(&PathBuf::from("/test")).await.unwrap();

        assert!(!result.is_clean);
        assert_eq!(result.threat_name, Some("Trojan.Test".to_string()));
    }

    #[tokio::test]
    async fn test_mock_scanner_available() {
        let scanner = MockScanner::clean();
        assert!(scanner.is_available().await);

        let scanner = MockScanner::unavailable();
        assert!(!scanner.is_available().await);
    }

    #[tokio::test]
    async fn test_mock_scanner_set_available() {
        let mut scanner = MockScanner::clean();
        assert!(scanner.is_available().await);

        scanner.set_available(false);
        assert!(!scanner.is_available().await);
    }

    #[test]
    fn test_mock_scanner_name() {
        let scanner = MockScanner::clean();
        assert_eq!(scanner.name(), "MockScanner");
    }
}
