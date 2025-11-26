//! File area management functionality for administrators

pub mod create;
pub mod edit;
pub mod security;

// Re-export commonly used types
pub use create::NewFileArea;
pub use edit::FileAreaEditRequest;

use crate::access::{AdminAccessControl, AdminPermission};
use crate::audit::AuditLogger;
use crate::error::{AdminError, AdminResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory file area storage (in production, this would be a database)
type FileAreaStore = Arc<RwLock<Vec<FileAreaRecord>>>;

/// Complete file area record stored in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAreaRecord {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub path: PathBuf,
    pub min_security_upload: u8,
    pub min_security_download: u8,
    pub max_file_size_mb: i32,
    pub file_count: i32,
    pub total_size_kb: i64,
}

/// File area manager for administrative operations
#[derive(Debug, Clone)]
pub struct FileAreaManager {
    areas: FileAreaStore,
    audit: AuditLogger,
    access_control: AdminAccessControl,
}

impl FileAreaManager {
    /// Creates a new file area manager
    pub fn new(access_control: AdminAccessControl, audit: AuditLogger) -> Self {
        Self {
            areas: Arc::new(RwLock::new(Vec::new())),
            audit,
            access_control,
        }
    }

    /// Creates a file area manager with pre-populated areas (for testing)
    pub fn with_areas(
        access_control: AdminAccessControl,
        audit: AuditLogger,
        areas: Vec<FileAreaRecord>,
    ) -> Self {
        Self {
            areas: Arc::new(RwLock::new(areas)),
            audit,
            access_control,
        }
    }

    /// Returns a reference to the access control
    pub fn access_control(&self) -> &AdminAccessControl {
        &self.access_control
    }

    /// Returns a reference to the audit logger
    pub fn audit(&self) -> &AuditLogger {
        &self.audit
    }

    /// Gets a file area by ID
    pub async fn get_area(&self, area_id: i32) -> AdminResult<FileAreaRecord> {
        let areas = self.areas.read().await;
        areas
            .iter()
            .find(|a| a.id == area_id)
            .cloned()
            .ok_or(AdminError::FileAreaNotFound(area_id))
    }

    /// Lists all file areas
    pub async fn list_areas(&self, admin_user_id: i32) -> AdminResult<Vec<FileAreaRecord>> {
        self.access_control
            .require_permission(AdminPermission::ManageFileAreas)?;

        let areas = self.areas.read().await;
        let result = areas.clone();

        self.audit
            .log_action(
                admin_user_id,
                "list_file_areas",
                None::<String>,
                Some(format!("count={}", result.len())),
            )
            .await;

        Ok(result)
    }

    /// Adds a file area (for testing)
    #[cfg(test)]
    pub async fn add_area(&self, area: FileAreaRecord) {
        let mut areas = self.areas.write().await;
        areas.push(area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    async fn test_file_area_manager_creation() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        assert!(
            manager
                .access_control()
                .has_permission(AdminPermission::ManageFileAreas)
        );
    }

    #[tokio::test]
    async fn test_get_area() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let area = create_test_area(1, "general");
        let manager = FileAreaManager::with_areas(access, audit, vec![area.clone()]);

        let result = manager.get_area(1).await;
        assert!(result.is_ok());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.id, 1);
        assert_eq!(retrieved.name, "general");
    }

    #[tokio::test]
    async fn test_get_area_not_found() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let result = manager.get_area(999).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AdminError::FileAreaNotFound(999)
        ));
    }

    #[tokio::test]
    async fn test_list_areas() {
        let access = AdminAccessControl::new(200, 200);
        let audit = AuditLogger::new();
        let areas = vec![
            create_test_area(1, "general"),
            create_test_area(2, "uploads"),
        ];
        let manager = FileAreaManager::with_areas(access, audit, areas);

        let result = manager.list_areas(1).await;
        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn test_list_areas_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let audit = AuditLogger::new();
        let manager = FileAreaManager::new(access, audit);

        let result = manager.list_areas(1).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_file_area_record_serialization() {
        let area = create_test_area(1, "general");
        let json = serde_json::to_string(&area).unwrap();
        let deserialized: FileAreaRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(area.id, deserialized.id);
        assert_eq!(area.name, deserialized.name);
    }
}
