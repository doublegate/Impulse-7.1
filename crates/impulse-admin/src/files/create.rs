//! File area creation functionality

use super::{FileAreaManager, FileAreaRecord};
use crate::access::AdminPermission;
use crate::error::{AdminError, AdminResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Request to create a new file area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFileArea {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
    pub min_security_upload: u8,
    pub min_security_download: u8,
    pub max_file_size_mb: i32,
}

impl FileAreaManager {
    /// Creates a new file area
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator creating the area
    /// * `area` - New file area details
    pub async fn create_area(&self, admin_user_id: i32, area: NewFileArea) -> AdminResult<i32> {
        self.access_control
            .require_permission(AdminPermission::ManageFileAreas)?;

        // Validate input
        if area.name.is_empty() {
            return Err(AdminError::InvalidInput(
                "Area name cannot be empty".to_string(),
            ));
        }

        if area.description.is_empty() {
            return Err(AdminError::InvalidInput(
                "Area description cannot be empty".to_string(),
            ));
        }

        if area.max_file_size_mb <= 0 {
            return Err(AdminError::InvalidInput(
                "Max file size must be positive".to_string(),
            ));
        }

        if area.min_security_upload < area.min_security_download {
            return Err(AdminError::InvalidInput(
                "Upload security must be >= download security".to_string(),
            ));
        }

        // Check for duplicate name
        let areas = self.areas.read().await;
        if areas.iter().any(|a| a.name == area.name) {
            return Err(AdminError::InvalidInput(format!(
                "File area '{}' already exists",
                area.name
            )));
        }
        drop(areas);

        // Generate new ID
        let mut areas = self.areas.write().await;
        let new_id = areas.iter().map(|a| a.id).max().unwrap_or(0) + 1;

        let new_area = FileAreaRecord {
            id: new_id,
            name: area.name.clone(),
            description: area.description,
            path: area.path.clone(),
            min_security_upload: area.min_security_upload,
            min_security_download: area.min_security_download,
            max_file_size_mb: area.max_file_size_mb,
            file_count: 0,
            total_size_kb: 0,
        };

        areas.push(new_area);

        self.audit
            .log_action(
                admin_user_id,
                "create_file_area",
                Some(new_id.to_string()),
                Some(area.name),
            )
            .await;

        // Create directory (in production, this would actually create the directory)
        // For testing, we just log it
        tracing::info!(
            area_id = new_id,
            path = ?area.path,
            "File area directory would be created"
        );

        Ok(new_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::AdminAccessControl;
    use crate::audit::AuditLogger;

    #[tokio::test]
    async fn test_create_area() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let new_area = NewFileArea {
            name: "General Files".to_string(),
            description: "General file uploads".to_string(),
            path: PathBuf::from("/bbs/files/general"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
        };

        let result = manager.create_area(1, new_area).await;
        assert!(result.is_ok());
        let area_id = result.unwrap();
        assert_eq!(area_id, 1);

        let area = manager.get_area(area_id).await.unwrap();
        assert_eq!(area.name, "General Files");
        assert_eq!(area.min_security_upload, 50);
    }

    #[tokio::test]
    async fn test_create_area_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let new_area = NewFileArea {
            name: "Test".to_string(),
            description: "Test area".to_string(),
            path: PathBuf::from("/test"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
        };

        let result = manager.create_area(1, new_area).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_area_empty_name() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let new_area = NewFileArea {
            name: "".to_string(),
            description: "Test".to_string(),
            path: PathBuf::from("/test"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
        };

        let result = manager.create_area(1, new_area).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_create_area_empty_description() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let new_area = NewFileArea {
            name: "Test".to_string(),
            description: "".to_string(),
            path: PathBuf::from("/test"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
        };

        let result = manager.create_area(1, new_area).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_area_invalid_max_size() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let new_area = NewFileArea {
            name: "Test".to_string(),
            description: "Test area".to_string(),
            path: PathBuf::from("/test"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: -1,
        };

        let result = manager.create_area(1, new_area).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_area_invalid_security_levels() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let new_area = NewFileArea {
            name: "Test".to_string(),
            description: "Test area".to_string(),
            path: PathBuf::from("/test"),
            min_security_upload: 10,
            min_security_download: 50,
            max_file_size_mb: 10,
        };

        let result = manager.create_area(1, new_area).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_area_duplicate_name() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let new_area = NewFileArea {
            name: "Test".to_string(),
            description: "Test area".to_string(),
            path: PathBuf::from("/test1"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
        };

        manager.create_area(1, new_area.clone()).await.unwrap();

        // Try to create with same name
        let result = manager.create_area(1, new_area).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_area_sequential_ids() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let area1 = NewFileArea {
            name: "Area1".to_string(),
            description: "First area".to_string(),
            path: PathBuf::from("/area1"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
        };

        let area2 = NewFileArea {
            name: "Area2".to_string(),
            description: "Second area".to_string(),
            path: PathBuf::from("/area2"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
        };

        let id1 = manager.create_area(1, area1).await.unwrap();
        let id2 = manager.create_area(1, area2).await.unwrap();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[tokio::test]
    async fn test_audit_log_create_area() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit.clone());

        let new_area = NewFileArea {
            name: "Test Area".to_string(),
            description: "Test".to_string(),
            path: PathBuf::from("/test"),
            min_security_upload: 50,
            min_security_download: 0,
            max_file_size_mb: 10,
        };

        manager.create_area(42, new_area).await.unwrap();

        let entries = audit.get_entries_by_action("create_file_area").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }
}
