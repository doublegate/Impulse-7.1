//! File area security level management

use super::FileAreaManager;
use crate::access::AdminPermission;
use crate::error::{AdminError, AdminResult};

impl FileAreaManager {
    /// Sets the minimum security level required to upload files
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the change
    /// * `area_id` - ID of the file area
    /// * `min_security` - Minimum security level required
    pub async fn set_upload_security(
        &self,
        admin_user_id: i32,
        area_id: i32,
        min_security: u8,
    ) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::ManageFileAreas)?;

        let mut areas = self.areas.write().await;
        let area = areas
            .iter_mut()
            .find(|a| a.id == area_id)
            .ok_or(AdminError::FileAreaNotFound(area_id))?;

        // Validate that upload security is >= download security
        if min_security < area.min_security_download {
            return Err(AdminError::InvalidInput(
                "Upload security must be >= download security".to_string(),
            ));
        }

        area.min_security_upload = min_security;

        self.audit
            .log_action(
                admin_user_id,
                "set_upload_security",
                Some(area_id.to_string()),
                Some(format!("min_security={}", min_security)),
            )
            .await;

        Ok(())
    }

    /// Sets the minimum security level required to download files
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the change
    /// * `area_id` - ID of the file area
    /// * `min_security` - Minimum security level required
    pub async fn set_download_security(
        &self,
        admin_user_id: i32,
        area_id: i32,
        min_security: u8,
    ) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::ManageFileAreas)?;

        let mut areas = self.areas.write().await;
        let area = areas
            .iter_mut()
            .find(|a| a.id == area_id)
            .ok_or(AdminError::FileAreaNotFound(area_id))?;

        // Validate that download security is <= upload security
        if min_security > area.min_security_upload {
            return Err(AdminError::InvalidInput(
                "Download security must be <= upload security".to_string(),
            ));
        }

        area.min_security_download = min_security;

        self.audit
            .log_action(
                admin_user_id,
                "set_download_security",
                Some(area_id.to_string()),
                Some(format!("min_security={}", min_security)),
            )
            .await;

        Ok(())
    }

    /// Sets both upload and download security levels at once
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the change
    /// * `area_id` - ID of the file area
    /// * `upload_security` - Minimum security level required for uploads
    /// * `download_security` - Minimum security level required for downloads
    pub async fn set_security_levels(
        &self,
        admin_user_id: i32,
        area_id: i32,
        upload_security: u8,
        download_security: u8,
    ) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::ManageFileAreas)?;

        if upload_security < download_security {
            return Err(AdminError::InvalidInput(
                "Upload security must be >= download security".to_string(),
            ));
        }

        let mut areas = self.areas.write().await;
        let area = areas
            .iter_mut()
            .find(|a| a.id == area_id)
            .ok_or(AdminError::FileAreaNotFound(area_id))?;

        area.min_security_upload = upload_security;
        area.min_security_download = download_security;

        self.audit
            .log_action(
                admin_user_id,
                "set_security_levels",
                Some(area_id.to_string()),
                Some(format!(
                    "upload={}, download={}",
                    upload_security, download_security
                )),
            )
            .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AdminAccessControl;
    use crate::audit::AuditLogger;
    use crate::files::FileAreaRecord;
    use std::path::PathBuf;

    fn create_test_area(id: i32, name: &str) -> FileAreaRecord {
        FileAreaRecord {
            id,
            name: name.to_string(),
            description: format!("Test area {}", name),
            path: PathBuf::from(format!("/bbs/files/{}", name)),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
            file_count: 0,
            total_size_kb: 0,
        }
    }

    #[tokio::test]
    async fn test_set_upload_security() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let result = manager.set_upload_security(1, 1, 100).await;
        assert!(result.is_ok());

        let updated = manager.get_area(1).await.unwrap();
        assert_eq!(updated.min_security_upload, 100);
    }

    #[tokio::test]
    async fn test_set_upload_security_validation() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let mut area = create_test_area(1, "test");
        area.min_security_download = 50;
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        // Try to set upload security below download security
        let result = manager.set_upload_security(1, 1, 25).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_download_security() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let result = manager.set_download_security(1, 1, 25).await;
        assert!(result.is_ok());

        let updated = manager.get_area(1).await.unwrap();
        assert_eq!(updated.min_security_download, 25);
    }

    #[tokio::test]
    async fn test_set_download_security_validation() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let mut area = create_test_area(1, "test");
        area.min_security_upload = 50;
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        // Try to set download security above upload security
        let result = manager.set_download_security(1, 1, 75).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_security_levels() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let result = manager.set_security_levels(1, 1, 100, 50).await;
        assert!(result.is_ok());

        let updated = manager.get_area(1).await.unwrap();
        assert_eq!(updated.min_security_upload, 100);
        assert_eq!(updated.min_security_download, 50);
    }

    #[tokio::test]
    async fn test_set_security_levels_validation() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        // Try to set upload < download
        let result = manager.set_security_levels(1, 1, 25, 75).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_security_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let result = manager.set_upload_security(1, 999, 50).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AdminError::FileAreaNotFound(999)
        ));
    }

    #[tokio::test]
    async fn test_set_security_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let result = manager.set_upload_security(1, 1, 50).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_audit_log_set_upload_security() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit.clone(), vec![area]);

        manager.set_upload_security(42, 1, 100).await.unwrap();

        let entries = audit.get_entries_by_action("set_upload_security").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }

    #[tokio::test]
    async fn test_audit_log_set_security_levels() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit.clone(), vec![area]);

        manager.set_security_levels(42, 1, 100, 50).await.unwrap();

        let entries = audit.get_entries_by_action("set_security_levels").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }
}
