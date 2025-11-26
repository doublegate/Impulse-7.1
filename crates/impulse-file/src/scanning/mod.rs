//! Virus scanning functionality
//!
//! This module provides virus scanning integration with support for:
//! - ClamAV integration via Unix socket
//! - File quarantine management
//! - SysOp notifications
//! - Mock scanner for testing

mod traits;

pub mod clamav;
pub mod mock;
pub mod notify;
pub mod quarantine;

pub use clamav::ClamAvScanner;
pub use mock::MockScanner;
pub use notify::{LogNotificationSender, NotificationSender, VirusNotification};
pub use quarantine::QuarantineManager;
pub use traits::{ScanResult, VirusScanner};
