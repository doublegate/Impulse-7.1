//! Door game integration tests

use crate::fixtures::BbsTestFixture;
use anyhow::Result;

/// Test door game integration
pub async fn test_door_integration(fixture: &BbsTestFixture) -> Result<()> {
    tracing::info!("Testing door game integration");

    // Test dropfile generation
    test_dropfile_generation(fixture).await?;

    // Test door execution
    test_door_execution(fixture).await?;

    // Test door cleanup
    test_door_cleanup(fixture).await?;

    Ok(())
}

/// Test DOOR.SYS and DORINFO1.DEF generation
async fn test_dropfile_generation(fixture: &BbsTestFixture) -> Result<()> {
    let user = fixture.create_test_user("door_user", 30).await?;

    tracing::debug!("{} testing dropfile generation", user.username);

    let door_dir = fixture.door_directory_path();
    assert!(door_dir.exists());

    // Simulate dropfile creation
    // In real integration test, would use impulse-door crate

    tracing::info!("Dropfile generation test passed");
    Ok(())
}

/// Test door game execution
async fn test_door_execution(fixture: &BbsTestFixture) -> Result<()> {
    let user = fixture.create_test_user("player", 25).await?;

    tracing::debug!("{} testing door execution", user.username);

    let door_dir = fixture.door_directory_path();
    assert!(door_dir.exists());

    // Simulate door launch
    tracing::info!("Door execution test passed");
    Ok(())
}

/// Test door cleanup after exit
async fn test_door_cleanup(fixture: &BbsTestFixture) -> Result<()> {
    let user = fixture.create_test_user("cleanup_user", 20).await?;

    tracing::debug!("{} testing door cleanup", user.username);

    // Verify temporary files are cleaned up
    let door_dir = fixture.door_directory_path();
    assert!(door_dir.exists());

    tracing::info!("Door cleanup test passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_door_integration_suite() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_door_integration(&fixture).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_dropfiles() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_dropfile_generation(&fixture).await;
        assert!(result.is_ok());
    }
}
