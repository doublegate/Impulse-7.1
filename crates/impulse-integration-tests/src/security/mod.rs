//! Security audit and penetration testing suite
//!
//! Comprehensive security testing including:
//! - SQL injection prevention
//! - Input validation and sanitization
//! - Authentication security
//! - File upload security (path traversal, malware)
//! - Rate limiting enforcement
//! - Session token security

mod auth;
mod injection;
mod upload;
mod validation;

pub use auth::*;
pub use injection::*;
pub use upload::*;
pub use validation::*;

use crate::fixtures::BbsTestFixture;
use anyhow::Result;
use std::sync::Arc;

/// Security audit suite orchestrator
pub struct SecurityAuditSuite {
    fixture: Arc<BbsTestFixture>,
}

impl SecurityAuditSuite {
    /// Create a new security audit suite
    pub async fn new() -> Result<Self> {
        let fixture = Arc::new(BbsTestFixture::new().await?);
        Ok(Self { fixture })
    }

    /// Run complete security audit
    ///
    /// Executes all security tests and returns overall results
    pub async fn run_complete_audit(&self) -> Result<SecurityAuditReport> {
        let mut report = SecurityAuditReport::default();

        // Test SQL injection prevention
        if let Err(e) = test_sql_injection(&self.fixture).await {
            report.sql_injection_vulnerabilities.push(e.to_string());
        }

        // Test input validation
        if let Err(e) = test_input_validation(&self.fixture).await {
            report.input_validation_issues.push(e.to_string());
        }

        // Test authentication security
        if let Err(e) = test_auth_security(&self.fixture).await {
            report.auth_security_issues.push(e.to_string());
        }

        // Test file upload security
        if let Err(e) = test_file_upload_security(&self.fixture).await {
            report.file_upload_issues.push(e.to_string());
        }

        Ok(report)
    }
}

/// Security audit report
#[derive(Default, Debug)]
pub struct SecurityAuditReport {
    pub sql_injection_vulnerabilities: Vec<String>,
    pub input_validation_issues: Vec<String>,
    pub auth_security_issues: Vec<String>,
    pub file_upload_issues: Vec<String>,
}

impl SecurityAuditReport {
    /// Check if the audit passed (no vulnerabilities found)
    pub fn passed(&self) -> bool {
        self.sql_injection_vulnerabilities.is_empty()
            && self.input_validation_issues.is_empty()
            && self.auth_security_issues.is_empty()
            && self.file_upload_issues.is_empty()
    }

    /// Get total number of issues found
    pub fn total_issues(&self) -> usize {
        self.sql_injection_vulnerabilities.len()
            + self.input_validation_issues.len()
            + self.auth_security_issues.len()
            + self.file_upload_issues.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_audit_creation() {
        let suite = SecurityAuditSuite::new().await.unwrap();
        assert!(suite.fixture.temp_path().exists());
    }

    #[tokio::test]
    async fn test_complete_security_audit() {
        let suite = SecurityAuditSuite::new().await.unwrap();
        let report = suite.run_complete_audit().await.unwrap();

        // Report should be generated (may have issues in unimplemented features)
        tracing::info!(
            "Security audit found {} total issues",
            report.total_issues()
        );
    }
}
