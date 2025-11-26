//! Administration interface for Impulse BBS
//!
//! This crate provides comprehensive administrative functionality for managing
//! a BBS system, including:
//!
//! - User management (list, edit, delete, ban, view history)
//! - File area management (create, edit, delete, security levels)
//! - System maintenance (view sessions, kick users, broadcast messages)
//! - Access control and permission management
//! - Audit logging for compliance and security
//!
//! # Examples
//!
//! ## Managing Users
//!
//! ```rust
//! use impulse_admin::access::AdminAccessControl;
//! use impulse_admin::audit::AuditLogger;
//! use impulse_admin::users::{UserManager, UserEditRequest};
//!
//! # tokio_test::block_on(async {
//! let access = AdminAccessControl::new(200, 200); // User level 200, SysOp level 200
//! let audit = AuditLogger::new();
//! let manager = UserManager::new(access, audit);
//!
//! // List all users
//! let users = manager.list_users(1, 0, 10).await.unwrap();
//! # });
//! ```
//!
//! ## Managing File Areas
//!
//! ```rust
//! use impulse_admin::access::AdminAccessControl;
//! use impulse_admin::audit::AuditLogger;
//! use impulse_admin::files::{FileAreaManager, NewFileArea};
//! use std::path::PathBuf;
//!
//! # tokio_test::block_on(async {
//! let access = AdminAccessControl::new(200, 200);
//! let audit = AuditLogger::new();
//! let manager = FileAreaManager::new(access, audit);
//!
//! let new_area = NewFileArea {
//!     name: "General Files".to_string(),
//!     description: "General file uploads".to_string(),
//!     path: PathBuf::from("/bbs/files/general"),
//!     min_security_upload: 50,
//!     min_security_download: 0,
//!     max_file_size_mb: 10,
//! };
//!
//! let area_id = manager.create_area(1, new_area).await.unwrap();
//! # });
//! ```
//!
//! ## System Maintenance
//!
//! ```rust
//! use impulse_admin::access::AdminAccessControl;
//! use impulse_admin::audit::AuditLogger;
//! use impulse_admin::system::SystemMaintenance;
//!
//! # tokio_test::block_on(async {
//! let access = AdminAccessControl::new(200, 200);
//! let audit = AuditLogger::new();
//! let maint = SystemMaintenance::new(access, audit);
//!
//! // View active sessions
//! let sessions = maint.view_active_sessions(1).await.unwrap();
//!
//! // Broadcast a message
//! let count = maint.broadcast_message(1, "System maintenance at 10 PM".to_string()).await.unwrap();
//! # });
//! ```

pub mod access;
pub mod audit;
pub mod error;
pub mod files;
pub mod system;
pub mod users;

pub use access::{AdminAccessControl, AdminPermission};
pub use audit::{AuditEntry, AuditLogger};
pub use error::{AdminError, AdminResult};
pub use files::{FileAreaManager, FileAreaRecord};
pub use system::{ActiveSession, SystemMaintenance, SystemMessage};
pub use users::{UserManager, UserRecord};
