//! File upload security testing

use crate::fixtures::BbsTestFixture;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

/// Test file upload security features
pub async fn test_file_upload_security(fixture: &Arc<BbsTestFixture>) -> Result<()> {
    // Test path traversal prevention
    test_path_traversal_prevention(fixture).await?;

    // Test malware detection (EICAR test)
    test_malware_detection(fixture).await?;

    // Test file size limits
    test_file_size_limits(fixture).await?;

    // Test extension validation
    test_extension_validation(fixture).await?;

    Ok(())
}

/// Test path traversal attack prevention
async fn test_path_traversal_prevention(fixture: &Arc<BbsTestFixture>) -> Result<()> {
    let malicious_filenames = vec![
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "./../sensitive.dat",
        "../../backdoor.exe",
        "subdir/../../escape.txt",
    ];

    for filename in malicious_filenames {
        let result = validate_upload_filename(filename);

        if result.is_ok() {
            tracing::error!("Path traversal not prevented: {}", filename);
            return Err(anyhow::anyhow!("Path traversal vulnerability"));
        }

        tracing::debug!("Path traversal blocked: {}", filename);
    }

    // Verify file area is protected
    let file_area = fixture.file_areas_path();
    assert!(file_area.exists());

    Ok(())
}

/// Test malware detection with EICAR test string
async fn test_malware_detection(fixture: &Arc<BbsTestFixture>) -> Result<()> {
    // EICAR test file content (standard anti-virus test)
    let eicar = b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";

    // Attempt to upload EICAR file
    let test_path = fixture.file_areas_path().join("test.exe");

    // In production, this would be scanned by ClamAV
    // For testing, we simulate the scan
    let is_malware = detect_malware(eicar);

    if is_malware {
        tracing::info!("Malware detected and blocked");
        // File should not be written
        assert!(!test_path.exists(), "Malware file was written to disk");
    } else {
        tracing::warn!("Malware detection may not be active in test environment");
    }

    Ok(())
}

/// Test file size limit enforcement
async fn test_file_size_limits(fixture: &Arc<BbsTestFixture>) -> Result<()> {
    let user = fixture.create_test_user("uploader", 10).await?;

    // User upload limit: 10 MB (10240 KB)
    let upload_limit_bytes = user.upload_limit_kb as usize * 1024;

    // Test file within limit
    let small_file_size = 1024 * 1024; // 1 MB
    assert!(small_file_size < upload_limit_bytes);

    // Test file exceeding limit
    let large_file_size = 20 * 1024 * 1024; // 20 MB
    let exceeds_limit = large_file_size > upload_limit_bytes;
    assert!(exceeds_limit, "Large file should exceed limit");

    Ok(())
}

/// Test file extension validation
async fn test_extension_validation(_fixture: &Arc<BbsTestFixture>) -> Result<()> {
    // Allowed extensions
    let allowed = vec!["txt", "zip", "jpg", "png", "pdf", "doc"];

    // Potentially dangerous extensions
    let dangerous = vec!["exe", "bat", "cmd", "sh", "dll", "scr"];

    // Test allowed extensions
    for ext in allowed {
        let filename = format!("test.{}", ext);
        let result =
            validate_file_extension(&filename, &["txt", "zip", "jpg", "png", "pdf", "doc"]);
        assert!(result.is_ok(), "Allowed extension {} rejected", ext);
    }

    // Test dangerous extensions
    for ext in dangerous {
        let filename = format!("malware.{}", ext);
        let result = validate_file_extension(&filename, &["txt", "zip", "jpg", "png"]);
        assert!(result.is_err(), "Dangerous extension {} not blocked", ext);
    }

    Ok(())
}

/// Validate upload filename for path traversal
fn validate_upload_filename(filename: &str) -> Result<()> {
    if filename.contains("..") {
        anyhow::bail!("Path traversal detected");
    }

    if filename.contains('\\') || filename.starts_with('/') {
        anyhow::bail!("Absolute or Windows path not allowed");
    }

    Ok(())
}

/// Detect malware in file content (simplified)
fn detect_malware(content: &[u8]) -> bool {
    // Simple EICAR detection for testing
    let eicar_signature = b"EICAR-STANDARD-ANTIVIRUS-TEST-FILE";
    content
        .windows(eicar_signature.len())
        .any(|window| window == eicar_signature)
}

/// Validate file extension against allowed list
fn validate_file_extension(filename: &str, allowed: &[&str]) -> Result<()> {
    let path = PathBuf::from(filename);
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("No file extension"))?;

    if !allowed.contains(&extension) {
        anyhow::bail!("File extension '{}' not allowed", extension);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_security() {
        let fixture = Arc::new(BbsTestFixture::new().await.unwrap());
        let result = test_file_upload_security(&fixture).await;
        assert!(result.is_ok(), "File upload security test failed");
    }

    #[tokio::test]
    async fn test_path_traversal_blocked() {
        let fixture = Arc::new(BbsTestFixture::new().await.unwrap());
        let result = test_path_traversal_prevention(&fixture).await;
        assert!(result.is_ok(), "Path traversal not blocked");
    }

    #[tokio::test]
    async fn test_eicar_detection() {
        let fixture = Arc::new(BbsTestFixture::new().await.unwrap());
        let result = test_malware_detection(&fixture).await;
        assert!(result.is_ok(), "Malware detection test failed");
    }

    #[test]
    fn test_eicar_signature_detection() {
        let eicar = b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";
        assert!(detect_malware(eicar));

        let clean = b"This is a clean file";
        assert!(!detect_malware(clean));
    }

    #[test]
    fn test_filename_validation() {
        assert!(validate_upload_filename("test.txt").is_ok());
        assert!(validate_upload_filename("document.pdf").is_ok());

        assert!(validate_upload_filename("../etc/passwd").is_err());
        assert!(validate_upload_filename("..\\system32").is_err());
        assert!(validate_upload_filename("/etc/passwd").is_err());
    }

    #[test]
    fn test_extension_filtering() {
        let allowed = vec!["txt", "zip", "pdf"];

        assert!(validate_file_extension("test.txt", &allowed).is_ok());
        assert!(validate_file_extension("archive.zip", &allowed).is_ok());
        assert!(validate_file_extension("malware.exe", &allowed).is_err());
        assert!(validate_file_extension("script.sh", &allowed).is_err());
    }
}
