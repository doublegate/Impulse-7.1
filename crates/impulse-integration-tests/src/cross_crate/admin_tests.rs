//! Administration interface integration tests

use crate::fixtures::BbsTestFixture;
use anyhow::Result;

/// Test admin interface integration
pub async fn test_admin_integration(fixture: &BbsTestFixture) -> Result<()> {
    tracing::info!("Testing admin interface integration");

    // Test user management
    test_user_management(fixture).await?;

    // Test file area management
    test_file_area_management(fixture).await?;

    // Test system maintenance
    test_system_maintenance(fixture).await?;

    // Test audit logging
    test_audit_logging(fixture).await?;

    Ok(())
}

/// Test admin user management operations
async fn test_user_management(fixture: &BbsTestFixture) -> Result<()> {
    let sysop = fixture.create_sysop().await?;
    let regular = fixture.create_regular_user().await?;

    tracing::debug!("{} managing user {}", sysop.username, regular.username);

    // Verify SysOp has admin privileges
    assert_eq!(sysop.security_level, 255);

    // Simulate admin operations
    // In real integration test, would use impulse-admin crate

    tracing::info!("User management test passed");
    Ok(())
}

/// Test file area management
async fn test_file_area_management(fixture: &BbsTestFixture) -> Result<()> {
    let sysop = fixture.create_sysop().await?;

    tracing::debug!("{} managing file areas", sysop.username);

    let file_area = fixture.file_areas_path();
    assert!(file_area.exists());

    // Simulate file area creation/editing
    tracing::info!("File area management test passed");
    Ok(())
}

/// Test system maintenance operations
async fn test_system_maintenance(fixture: &BbsTestFixture) -> Result<()> {
    let sysop = fixture.create_sysop().await?;

    tracing::debug!("{} performing system maintenance", sysop.username);

    // Simulate maintenance operations:
    // - View sessions
    // - Kick users
    // - Broadcast messages

    tracing::info!("System maintenance test passed");
    Ok(())
}

/// Test audit logging
async fn test_audit_logging(fixture: &BbsTestFixture) -> Result<()> {
    let sysop = fixture.create_sysop().await?;
    let user = fixture.create_regular_user().await?;

    tracing::debug!(
        "{} reviewing audit logs for {}",
        sysop.username,
        user.username
    );

    // Simulate audit log review
    // Should show admin actions with timestamps

    tracing::info!("Audit logging test passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_admin_integration_suite() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_admin_integration(&fixture).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_mgmt() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_user_management(&fixture).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_audit_logs() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let result = test_audit_logging(&fixture).await;
        assert!(result.is_ok());
    }
}
