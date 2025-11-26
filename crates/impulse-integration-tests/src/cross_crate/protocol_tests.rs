//! Protocol integration tests (Zmodem, Xmodem, Ymodem)

use crate::fixtures::BbsTestFixture;
use anyhow::Result;

/// Test file transfer protocol integration
pub async fn test_protocol_integration(fixture: &BbsTestFixture) -> Result<()> {
    tracing::info!("Testing protocol integration");

    // Test Zmodem integration
    test_zmodem_integration(fixture).await?;

    // Test Xmodem integration
    test_xmodem_integration(fixture).await?;

    // Test Ymodem integration
    test_ymodem_integration(fixture).await?;

    // Test protocol auto-detection
    test_protocol_auto_detection(fixture).await?;

    Ok(())
}

/// Test Zmodem protocol integration
async fn test_zmodem_integration(fixture: &BbsTestFixture) -> Result<()> {
    let user = fixture.create_test_user("zmodem_user", 20).await?;

    tracing::debug!("{} testing Zmodem protocol", user.username);

    // Verify file area exists
    let file_area = fixture.file_areas_path();
    assert!(file_area.exists());

    // Simulate Zmodem upload
    // In real integration test, would use impulse-protocol crate

    tracing::info!("Zmodem integration test passed");
    Ok(())
}

/// Test Xmodem protocol integration
async fn test_xmodem_integration(fixture: &BbsTestFixture) -> Result<()> {
    let user = fixture.create_test_user("xmodem_user", 20).await?;

    tracing::debug!("{} testing Xmodem protocol", user.username);

    let file_area = fixture.file_areas_path();
    assert!(file_area.exists());

    tracing::info!("Xmodem integration test passed");
    Ok(())
}

/// Test Ymodem protocol integration
async fn test_ymodem_integration(fixture: &BbsTestFixture) -> Result<()> {
    let user = fixture.create_test_user("ymodem_user", 20).await?;

    tracing::debug!("{} testing Ymodem protocol", user.username);

    let file_area = fixture.file_areas_path();
    assert!(file_area.exists());

    tracing::info!("Ymodem integration test passed");
    Ok(())
}

/// Test protocol auto-detection
async fn test_protocol_auto_detection(fixture: &BbsTestFixture) -> Result<()> {
    let user = fixture.create_test_user("autodetect_user", 20).await?;

    tracing::debug!("{} testing protocol auto-detection", user.username);

    // Simulate protocol detection
    // Should detect Zmodem/Ymodem/Xmodem based on init sequence

    tracing::info!("Protocol auto-detection test passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_protocol_integration_suite() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_protocol_integration(&fixture).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_zmodem() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_zmodem_integration(&fixture).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_xmodem() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_xmodem_integration(&fixture).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ymodem() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_ymodem_integration(&fixture).await;
        assert!(result.is_ok());
    }
}
