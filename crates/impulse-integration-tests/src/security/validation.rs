//! Input validation and sanitization testing

use crate::fixtures::BbsTestFixture;
use anyhow::Result;
use std::sync::Arc;

/// Test comprehensive input validation
pub async fn test_input_validation(_fixture: &Arc<BbsTestFixture>) -> Result<()> {
    // Test username validation
    test_username_validation()?;

    // Test password validation
    test_password_validation()?;

    // Test email validation
    test_email_validation()?;

    // Test message subject validation
    test_subject_validation()?;

    // Test filename validation
    test_filename_validation()?;

    Ok(())
}

/// Test username validation rules
fn test_username_validation() -> Result<()> {
    // Valid usernames
    assert!(validate_username("testuser").is_ok());
    assert!(validate_username("user123").is_ok());
    assert!(validate_username("test_user").is_ok());

    // Invalid usernames
    assert!(validate_username("ab").is_err()); // Too short
    assert!(validate_username("a".repeat(21).as_str()).is_err()); // Too long
    assert!(validate_username("test@user").is_err()); // Invalid char
    assert!(validate_username("test user").is_err()); // Space

    Ok(())
}

/// Test password validation rules
fn test_password_validation() -> Result<()> {
    // Valid passwords
    assert!(validate_password("SecurePass123!").is_ok());
    assert!(validate_password("12345678").is_ok()); // Minimum length

    // Invalid passwords
    assert!(validate_password("short").is_err()); // Too short
    assert!(validate_password("").is_err()); // Empty

    Ok(())
}

/// Test email validation rules
fn test_email_validation() -> Result<()> {
    // Valid emails
    assert!(validate_email("user@example.com").is_ok());
    assert!(validate_email("test.user@example.co.uk").is_ok());

    // Invalid emails
    assert!(validate_email("notanemail").is_err());
    assert!(validate_email("@example.com").is_err());
    assert!(validate_email("user@").is_err());

    Ok(())
}

/// Test message subject validation
fn test_subject_validation() -> Result<()> {
    // Valid subjects
    assert!(validate_subject("Test Subject").is_ok());
    assert!(validate_subject("RE: Test").is_ok());

    // Invalid subjects
    assert!(validate_subject("").is_err()); // Empty
    assert!(validate_subject(&"x".repeat(81)).is_err()); // Too long

    Ok(())
}

/// Test filename validation (path traversal prevention)
fn test_filename_validation() -> Result<()> {
    // Valid filenames
    assert!(validate_filename("test.txt").is_ok());
    assert!(validate_filename("document.pdf").is_ok());

    // Invalid filenames (path traversal attempts)
    assert!(validate_filename("../etc/passwd").is_err());
    assert!(validate_filename("..\\windows\\system32").is_err());
    assert!(validate_filename("./../sensitive.dat").is_err());
    assert!(validate_filename("../../backdoor.exe").is_err());

    Ok(())
}

/// Validate username
pub fn validate_username(username: &str) -> Result<()> {
    if username.len() < 3 || username.len() > 20 {
        anyhow::bail!("Username must be 3-20 characters");
    }

    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        anyhow::bail!("Username contains invalid characters");
    }

    Ok(())
}

/// Validate password
pub fn validate_password(password: &str) -> Result<()> {
    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }

    Ok(())
}

/// Validate email address
pub fn validate_email(email: &str) -> Result<()> {
    if !email.contains('@') || !email.contains('.') {
        anyhow::bail!("Invalid email format");
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        anyhow::bail!("Invalid email format");
    }

    Ok(())
}

/// Validate message subject
pub fn validate_subject(subject: &str) -> Result<()> {
    if subject.is_empty() {
        anyhow::bail!("Subject cannot be empty");
    }

    if subject.len() > 80 {
        anyhow::bail!("Subject too long (max 80 characters)");
    }

    Ok(())
}

/// Validate filename (prevent path traversal)
pub fn validate_filename(filename: &str) -> Result<()> {
    if filename.contains("..") || filename.contains("\\..") || filename.contains("/..") {
        anyhow::bail!("Path traversal attempt detected");
    }

    if filename.contains("\\\\") || filename.contains("//") {
        anyhow::bail!("Invalid path separators");
    }

    if filename.starts_with('/') || filename.starts_with('\\') {
        anyhow::bail!("Absolute paths not allowed");
    }

    Ok(())
}

/// Sanitize input by removing control characters
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .take(1024) // Limit length
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_input_validation() {
        let fixture = Arc::new(BbsTestFixture::new().await.unwrap());
        let result = test_input_validation(&fixture).await;
        assert!(result.is_ok(), "Input validation failed");
    }

    #[test]
    fn test_sanitize_input() {
        let dirty = "Test\x00\x01\x02clean\ndata\t";
        let clean = sanitize_input(dirty);
        assert!(!clean.contains('\x00'));
        assert!(!clean.contains('\x01'));
        assert!(clean.contains('\n'));
        assert!(clean.contains('\t'));
    }

    #[test]
    fn test_sanitize_long_input() {
        let long_input = "x".repeat(2000);
        let sanitized = sanitize_input(&long_input);
        assert_eq!(sanitized.len(), 1024);
    }
}
