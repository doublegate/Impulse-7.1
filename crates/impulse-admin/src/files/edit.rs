//! File area editing functionality

use super::FileAreaManager;
use crate::access::AdminPermission;
use crate::error::{AdminError, AdminResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Request to edit file area properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAreaEditRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub path: Option<PathBuf>,
    pub max_file_size_mb: Option<i32>,
}

impl FileAreaManager {
    /// Edits file area properties
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the edit
    /// * `area_id` - ID of the file area to edit
    /// * `changes` - Requested changes to apply
    pub async fn edit_area(
        &self,
        admin_user_id: i32,
        area_id: i32,
        changes: FileAreaEditRequest,
    ) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::ManageFileAreas)?;

        // Validate changes
        if let Some(ref name) = changes.name && name.is_empty() {
            return Err(AdminError::InvalidInput("Area name cannot be empty".to_string()));
        }

        if let Some(ref description) = changes.description && description.is_empty() {
            return Err(AdminError::InvalidInput(
                "Area description cannot be empty".to_string(),
            ));
        }

        if let Some(max_size) = changes.max_file_size_mb && max_size <= 0 {
            return Err(AdminError::InvalidInput(
                "Max file size must be positive".to_string(),
            ));
        }

        // Apply changes
        let mut areas = self.areas.write().await;

        // Check for duplicate name if renaming
        if let Some(ref new_name) = changes.name
            && areas.iter().any(|a| a.id != area_id && a.name == *new_name) {
            return Err(AdminError::InvalidInput(format!(
                "File area '{}' already exists",
                new_name
            )));
        }

        let area = areas
            .iter_mut()
            .find(|a| a.id == area_id)
            .ok_or(AdminError::FileAreaNotFound(area_id))?;

        if let Some(ref new_name) = changes.name {
            area.name = new_name.clone();
        }

        if let Some(ref description) = changes.description {
            area.description = description.clone();
        }

        if let Some(ref path) = changes.path {
            area.path = path.clone();
        }

        if let Some(max_size) = changes.max_file_size_mb {
            area.max_file_size_mb = max_size;
        }

        self.audit
            .log_action(
                admin_user_id,
                "edit_file_area",
                Some(area_id.to_string()),
                Some(format!("{:?}", changes)),
            )
            .await;

        Ok(())
    }

    /// Deletes a file area
    ///
    /// # Arguments
    /// * `admin_user_id` - ID of the administrator performing the deletion
    /// * `area_id` - ID of the file area to delete
    /// * `delete_files` - Whether to delete files on disk (not implemented in this version)
    pub async fn delete_area(
        &self,
        admin_user_id: i32,
        area_id: i32,
        delete_files: bool,
    ) -> AdminResult<()> {
        self.access_control
            .require_permission(AdminPermission::ManageFileAreas)?;

        let mut areas = self.areas.write().await;
        let index = areas
            .iter()
            .position(|a| a.id == area_id)
            .ok_or(AdminError::FileAreaNotFound(area_id))?;

        areas.remove(index);

        self.audit
            .log_action(
                admin_user_id,
                "delete_file_area",
                Some(area_id.to_string()),
                Some(format!("delete_files={}", delete_files)),
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
    async fn test_edit_area_name() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "oldname");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let changes = FileAreaEditRequest {
            name: Some("newname".to_string()),
            description: None,
            path: None,
            max_file_size_mb: None,
        };

        let result = manager.edit_area(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_area(1).await.unwrap();
        assert_eq!(updated.name, "newname");
    }

    #[tokio::test]
    async fn test_edit_area_description() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let changes = FileAreaEditRequest {
            name: None,
            description: Some("New description".to_string()),
            path: None,
            max_file_size_mb: None,
        };

        let result = manager.edit_area(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_area(1).await.unwrap();
        assert_eq!(updated.description, "New description");
    }

    #[tokio::test]
    async fn test_edit_area_path() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let changes = FileAreaEditRequest {
            name: None,
            description: None,
            path: Some(PathBuf::from("/new/path")),
            max_file_size_mb: None,
        };

        let result = manager.edit_area(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_area(1).await.unwrap();
        assert_eq!(updated.path, PathBuf::from("/new/path"));
    }

    #[tokio::test]
    async fn test_edit_area_max_size() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let changes = FileAreaEditRequest {
            name: None,
            description: None,
            path: None,
            max_file_size_mb: Some(20),
        };

        let result = manager.edit_area(1, 1, changes).await;
        assert!(result.is_ok());

        let updated = manager.get_area(1).await.unwrap();
        assert_eq!(updated.max_file_size_mb, 20);
    }

    #[tokio::test]
    async fn test_edit_area_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let changes = FileAreaEditRequest {
            name: Some("test".to_string()),
            description: None,
            path: None,
            max_file_size_mb: None,
        };

        let result = manager.edit_area(1, 999, changes).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AdminError::FileAreaNotFound(999)
        ));
    }

    #[tokio::test]
    async fn test_edit_area_duplicate_name() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let areas = vec![
            create_test_area(1, "area1"),
            create_test_area(2, "area2"),
        ];
        let manager = FileAreaManager::with_areas(access, audit, areas);

        let changes = FileAreaEditRequest {
            name: Some("area2".to_string()),
            description: None,
            path: None,
            max_file_size_mb: None,
        };

        let result = manager.edit_area(1, 1, changes).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_area_empty_name() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let changes = FileAreaEditRequest {
            name: Some("".to_string()),
            description: None,
            path: None,
            max_file_size_mb: None,
        };

        let result = manager.edit_area(1, 1, changes).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_area_invalid_max_size() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let changes = FileAreaEditRequest {
            name: None,
            description: None,
            path: None,
            max_file_size_mb: Some(-1),
        };

        let result = manager.edit_area(1, 1, changes).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_area() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit, vec![area]);

        let result = manager.delete_area(1, 1, false).await;
        assert!(result.is_ok());

        let get_result = manager.get_area(1).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_delete_area_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let result = manager.delete_area(1, 999, false).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_audit_log_edit_area() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit.clone(), vec![area]);

        let changes = FileAreaEditRequest {
            name: Some("newname".to_string()),
            description: None,
            path: None,
            max_file_size_mb: None,
        };

        manager.edit_area(42, 1, changes).await.unwrap();

        let entries = audit.get_entries_by_action("edit_file_area").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }

    #[tokio::test]
    async fn test_audit_log_delete_area() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "test");
        let manager = FileAreaManager::with_areas(access, audit.clone(), vec![area]);

        manager.delete_area(42, 1, true).await.unwrap();

        let entries = audit.get_entries_by_action("delete_file_area").await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].admin_user_id, 42);
    }
}
