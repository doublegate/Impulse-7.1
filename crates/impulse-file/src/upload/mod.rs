//! File upload functionality
//!
//! This module provides complete file upload handling including:
//! - Upload validation (size, extension, quota, permissions)
//! - Virus scanning integration
//! - Duplicate detection
//! - FILE_ID.DIZ extraction
//! - Atomic file storage
//! - Database record creation
//! - Statistics tracking

pub mod cleanup;
pub mod config;
pub mod metadata;
pub mod processor;
pub mod stats;
pub mod storage;

pub use cleanup::UploadRollback;
pub use config::UploadConfig;
pub use metadata::{MetadataBuilder, create_file_entry};
pub use processor::UploadProcessor;
pub use stats::{AreaStats, UploadStats};
pub use storage::{FileStorage, PendingUpload};
