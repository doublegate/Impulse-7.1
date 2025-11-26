//! Access control and permission management for administrative functions

use crate::error::{AdminError, AdminResult};
use serde::{Deserialize, Serialize};

/// Administrative permissions for BBS operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdminPermission {
    /// View user list and basic information
    ViewUsers,
    /// Edit user profiles (email, security level, limits)
    EditUsers,
    /// Delete users (requires elevated privileges)
    DeleteUsers,
    /// Ban users from the system
    BanUsers,
    /// Create, edit, and delete file areas
    ManageFileAreas,
    /// View active sessions and user activity
    ViewSessions,
    /// Kick users from active sessions
    KickUsers,
    /// Broadcast system messages to all users
    BroadcastMessages,
    /// System maintenance operations (requires highest level)
    SystemMaintenance,
    /// View administrative audit logs
    ViewLogs,
}

/// Access control for administrative operations based on security levels
#[derive(Debug, Clone)]
pub struct AdminAccessControl {
    user_security_level: u8,
    sysop_level: u8,
}

impl AdminAccessControl {
    /// Creates a new access control instance
    ///
    /// # Arguments
    /// * `user_security_level` - The current user's security level
    /// * `sysop_level` - The minimum security level required for SysOp access (typically 200)
    pub fn new(user_security_level: u8, sysop_level: u8) -> Self {
        Self {
            user_security_level,
            sysop_level,
        }
    }

    /// Checks if the user has a specific permission
    pub fn has_permission(&self, permission: AdminPermission) -> bool {
        // Must meet minimum SysOp level for any admin access
        if self.user_security_level < self.sysop_level {
            return false;
        }

        match permission {
            AdminPermission::ViewUsers => self.user_security_level >= self.sysop_level,
            AdminPermission::EditUsers => self.user_security_level >= self.sysop_level,
            AdminPermission::DeleteUsers => self.user_security_level >= 250, // Elevated
            AdminPermission::BanUsers => self.user_security_level >= self.sysop_level,
            AdminPermission::ManageFileAreas => self.user_security_level >= self.sysop_level,
            AdminPermission::ViewSessions => self.user_security_level >= self.sysop_level,
            AdminPermission::KickUsers => self.user_security_level >= self.sysop_level,
            AdminPermission::BroadcastMessages => self.user_security_level >= self.sysop_level,
            AdminPermission::SystemMaintenance => self.user_security_level == 255, // Highest
            AdminPermission::ViewLogs => self.user_security_level >= self.sysop_level,
        }
    }

    /// Requires that the user has a specific permission, returning an error if not
    pub fn require_permission(&self, permission: AdminPermission) -> AdminResult<()> {
        if !self.has_permission(permission) {
            return Err(AdminError::AccessDenied(format!(
                "Insufficient permissions for {:?} (level {} required, user has {})",
                permission,
                self.required_level_for(permission),
                self.user_security_level
            )));
        }
        Ok(())
    }

    /// Returns the required security level for a specific permission
    fn required_level_for(&self, permission: AdminPermission) -> u8 {
        match permission {
            AdminPermission::DeleteUsers => 250,
            AdminPermission::SystemMaintenance => 255,
            _ => self.sysop_level,
        }
    }

    /// Returns the user's current security level
    pub fn user_level(&self) -> u8 {
        self.user_security_level
    }

    /// Returns the configured SysOp level threshold
    pub fn sysop_level(&self) -> u8 {
        self.sysop_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_sysop_level_denied() {
        let access = AdminAccessControl::new(100, 200);
        assert!(!access.has_permission(AdminPermission::ViewUsers));
        assert!(!access.has_permission(AdminPermission::EditUsers));
    }

    #[test]
    fn test_sysop_level_basic_permissions() {
        let access = AdminAccessControl::new(200, 200);
        assert!(access.has_permission(AdminPermission::ViewUsers));
        assert!(access.has_permission(AdminPermission::EditUsers));
        assert!(access.has_permission(AdminPermission::BanUsers));
        assert!(access.has_permission(AdminPermission::ManageFileAreas));
        assert!(access.has_permission(AdminPermission::ViewSessions));
        assert!(access.has_permission(AdminPermission::KickUsers));
        assert!(access.has_permission(AdminPermission::BroadcastMessages));
        assert!(access.has_permission(AdminPermission::ViewLogs));
    }

    #[test]
    fn test_delete_users_requires_250() {
        let access_200 = AdminAccessControl::new(200, 200);
        assert!(!access_200.has_permission(AdminPermission::DeleteUsers));

        let access_250 = AdminAccessControl::new(250, 200);
        assert!(access_250.has_permission(AdminPermission::DeleteUsers));
    }

    #[test]
    fn test_system_maintenance_requires_255() {
        let access_200 = AdminAccessControl::new(200, 200);
        assert!(!access_200.has_permission(AdminPermission::SystemMaintenance));

        let access_254 = AdminAccessControl::new(254, 200);
        assert!(!access_254.has_permission(AdminPermission::SystemMaintenance));

        let access_255 = AdminAccessControl::new(255, 200);
        assert!(access_255.has_permission(AdminPermission::SystemMaintenance));
    }

    #[test]
    fn test_require_permission_ok() {
        let access = AdminAccessControl::new(200, 200);
        assert!(
            access
                .require_permission(AdminPermission::ViewUsers)
                .is_ok()
        );
    }

    #[test]
    fn test_require_permission_denied() {
        let access = AdminAccessControl::new(100, 200);
        let result = access.require_permission(AdminPermission::ViewUsers);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AdminError::AccessDenied(_)));
    }

    #[test]
    fn test_user_level_getter() {
        let access = AdminAccessControl::new(220, 200);
        assert_eq!(access.user_level(), 220);
    }

    #[test]
    fn test_sysop_level_getter() {
        let access = AdminAccessControl::new(220, 200);
        assert_eq!(access.sysop_level(), 200);
    }

    #[test]
    fn test_permission_serialization() {
        let perm = AdminPermission::ViewUsers;
        let json = serde_json::to_string(&perm).unwrap();
        let deserialized: AdminPermission = serde_json::from_str(&json).unwrap();
        assert_eq!(perm, deserialized);
    }

    #[test]
    fn test_custom_sysop_level() {
        let access = AdminAccessControl::new(180, 150);
        assert!(access.has_permission(AdminPermission::ViewUsers));

        let access2 = AdminAccessControl::new(140, 150);
        assert!(!access2.has_permission(AdminPermission::ViewUsers));
    }
}
