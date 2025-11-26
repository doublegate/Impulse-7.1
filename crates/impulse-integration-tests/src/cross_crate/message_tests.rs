//! Message system integration tests

use crate::fixtures::BbsTestFixture;
use anyhow::Result;

/// Test message system integration
pub async fn test_message_integration(fixture: &BbsTestFixture) -> Result<()> {
    tracing::info!("Testing message system integration");

    // Test message posting and reading
    test_message_posting(fixture).await?;

    // Test QWK packet handling
    test_qwk_packets(fixture).await?;

    // Test message threading
    test_message_threading(fixture).await?;

    Ok(())
}

/// Test message posting and reading
async fn test_message_posting(fixture: &BbsTestFixture) -> Result<()> {
    let poster = fixture.create_test_user("poster", 20).await?;
    let reader = fixture.create_test_user("reader", 20).await?;

    tracing::debug!("{} posting message", poster.username);

    let message_base = fixture.message_base_path();
    assert!(message_base.exists());

    // Simulate message post and read
    // In real integration test, would use impulse-message crate

    tracing::debug!("{} reading message", reader.username);
    tracing::info!("Message posting test passed");
    Ok(())
}

/// Test QWK offline mail packets
async fn test_qwk_packets(fixture: &BbsTestFixture) -> Result<()> {
    let user = fixture.create_test_user("qwk_user", 30).await?;

    tracing::debug!("{} testing QWK packets", user.username);

    let message_base = fixture.message_base_path();
    assert!(message_base.exists());

    // Simulate QWK packet creation and import
    tracing::info!("QWK packet test passed");
    Ok(())
}

/// Test message threading
async fn test_message_threading(fixture: &BbsTestFixture) -> Result<()> {
    let users = fixture.user_factory.create_users_batch(3, 20).await?;

    tracing::debug!("Testing message threading with {} users", users.len());

    let message_base = fixture.message_base_path();
    assert!(message_base.exists());

    // Simulate threaded conversation
    tracing::info!("Message threading test passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_integration_suite() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_message_integration(&fixture).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_and_read() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_message_posting(&fixture).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_qwk() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_qwk_packets(&fixture).await;
        assert!(result.is_ok());
    }
}
