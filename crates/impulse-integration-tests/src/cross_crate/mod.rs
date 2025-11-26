//! Cross-crate integration testing
//!
//! Tests interactions between multiple Impulse crates to ensure
//! they work together correctly.

mod admin_tests;
mod door_tests;
mod message_tests;
mod protocol_tests;

pub use admin_tests::*;
pub use door_tests::*;
pub use message_tests::*;
pub use protocol_tests::*;

use crate::fixtures::BbsTestFixture;
use anyhow::Result;

/// Run all cross-crate integration tests
pub async fn run_all_cross_crate_tests() -> Result<()> {
    tracing::info!("Running cross-crate integration tests");

    let fixture = BbsTestFixture::new().await?;

    // Protocol integration tests
    test_protocol_integration(&fixture).await?;

    // Door game integration tests
    test_door_integration(&fixture).await?;

    // Message system integration tests
    test_message_integration(&fixture).await?;

    // Admin interface integration tests
    test_admin_integration(&fixture).await?;

    tracing::info!("All cross-crate integration tests passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Comprehensive test, run explicitly
    async fn test_all_cross_crate_integration() {
        let result = run_all_cross_crate_tests().await;
        assert!(result.is_ok(), "Cross-crate integration tests failed");
    }
}
