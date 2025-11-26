//! ClamAV virus scanner integration
//!
//! Connects to ClamAV daemon (clamd) via Unix socket or TCP and uses
//! the INSTREAM protocol for file scanning.

use crate::error::{FileError, Result};
use crate::scanning::ScanResult;
use crate::scanning::VirusScanner;
use async_trait::async_trait;
use std::path::Path;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

/// ClamAV scanner implementation
pub struct ClamAvScanner {
    /// Socket path (Unix socket path or tcp://host:port)
    socket_path: String,
}

impl ClamAvScanner {
    /// Create a new ClamAV scanner
    pub fn new(socket_path: impl Into<String>) -> Self {
        Self {
            socket_path: socket_path.into(),
        }
    }

    /// Connect to ClamAV daemon
    async fn connect(&self) -> Result<UnixStream> {
        // For now, only support Unix sockets
        // TCP support can be added later with socket_path.starts_with("tcp://")
        UnixStream::connect(&self.socket_path).await.map_err(|e| {
            FileError::ScannerUnavailable(format!("Failed to connect to ClamAV: {}", e))
        })
    }

    /// Scan file using INSTREAM protocol
    async fn scan_with_instream(&self, path: &Path) -> Result<ScanResult> {
        let start = Instant::now();

        // Connect to ClamAV
        let mut stream = self.connect().await?;

        // Send INSTREAM command
        stream
            .write_all(b"zINSTREAM\0")
            .await
            .map_err(FileError::Io)?;

        // Open file
        let mut file = File::open(path).await?;

        // Stream file in chunks
        const CHUNK_SIZE: usize = 8192;
        let mut buffer = vec![0u8; CHUNK_SIZE];

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break; // EOF
            }

            // Send chunk size (4 bytes, network byte order)
            let size_bytes = (bytes_read as u32).to_be_bytes();
            stream.write_all(&size_bytes).await?;

            // Send chunk data
            stream.write_all(&buffer[..bytes_read]).await?;
        }

        // Send zero-length chunk to signal end
        stream.write_all(&[0, 0, 0, 0]).await?;

        // Read response
        let mut response = String::new();
        stream.read_to_string(&mut response).await?;

        let elapsed = start.elapsed();
        let scan_time_ms = elapsed.as_millis() as u64;

        // Parse response
        self.parse_response(&response, scan_time_ms)
    }

    /// Parse ClamAV response
    fn parse_response(&self, response: &str, scan_time_ms: u64) -> Result<ScanResult> {
        let response = response.trim();

        if response.ends_with("OK") {
            Ok(ScanResult::clean(scan_time_ms))
        } else if response.contains("FOUND") {
            // Extract virus name from "stream: Virus.Name FOUND"
            let parts: Vec<&str> = response.split_whitespace().collect();
            let threat_name = if parts.len() >= 2 {
                parts[1].to_string()
            } else {
                "Unknown".to_string()
            };

            Ok(ScanResult::infected(threat_name, scan_time_ms))
        } else {
            Err(FileError::ScannerUnavailable(format!(
                "Unexpected ClamAV response: {}",
                response
            )))
        }
    }
}

#[async_trait]
impl VirusScanner for ClamAvScanner {
    async fn scan_file(&self, path: &Path) -> Result<ScanResult> {
        self.scan_with_instream(path).await
    }

    async fn is_available(&self) -> bool {
        // Try to connect and send PING
        if let Ok(mut stream) = self.connect().await {
            if stream.write_all(b"zPING\0").await.is_ok() {
                let mut response = String::new();
                if stream.read_to_string(&mut response).await.is_ok() {
                    return response.trim() == "PONG";
                }
            }
        }
        false
    }

    fn name(&self) -> &str {
        "ClamAV"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response_clean() {
        let scanner = ClamAvScanner::new("/var/run/clamav/clamd.ctl");
        let result = scanner.parse_response("stream: OK", 100).unwrap();

        assert!(result.is_clean);
        assert!(result.threat_name.is_none());
        assert_eq!(result.scan_time_ms, 100);
    }

    #[test]
    fn test_parse_response_infected() {
        let scanner = ClamAvScanner::new("/var/run/clamav/clamd.ctl");
        let result = scanner
            .parse_response("stream: Eicar-Test-Signature FOUND", 150)
            .unwrap();

        assert!(!result.is_clean);
        assert_eq!(result.threat_name, Some("Eicar-Test-Signature".to_string()));
        assert_eq!(result.scan_time_ms, 150);
    }

    #[test]
    fn test_parse_response_error() {
        let scanner = ClamAvScanner::new("/var/run/clamav/clamd.ctl");
        let result = scanner.parse_response("ERROR: Invalid command", 100);

        assert!(result.is_err());
    }

    // Note: Integration tests with actual ClamAV require clamd to be running
    // and are better suited for integration test suite
}
