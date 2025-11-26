//! File area management for Impulse 7.1 BBS
//!
//! This crate provides complete file area browsing functionality,
//! supporting file areas, file lists, search, and FILE_ID.DIZ extraction.
//!
//! # Features
//!
//! - **File Areas**: Organize files into categorized areas
//! - **File Browsing**: List and view files with pagination and sorting
//! - **Search**: Search files by name, description, uploader, date, size
//! - **FILE_ID.DIZ**: Extract and display description files from ZIP archives
//! - **Permissions**: Security level-based access control
//! - **UI Screens**: Area selection, file list, file details, search
//! - **Async**: Fully async API using tokio
//!
//! # Examples
//!
//! ## Listing file areas
//!
//! ```no_run
//! use impulse_file::manager::InMemoryFileAreaManager;
//! use impulse_file::traits::FileAreaManager;
//! use impulse_types::security::SecurityLevel;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let manager = InMemoryFileAreaManager::new();
//! let areas = manager.list_areas(SecurityLevel::NEW_USER).await?;
//! for area in areas {
//!     println!("{}: {}", area.name, area.description);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Browsing files in an area
//!
//! ```no_run
//! use impulse_file::manager::InMemoryFileAreaManager;
//! use impulse_file::traits::FileAreaManager;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let manager = InMemoryFileAreaManager::new();
//! let (files, total) = manager.get_files(1, 0, 20).await?;
//! println!("Showing {} of {} files", files.len(), total);
//! # Ok(())
//! # }
//! ```
//!
//! ## Searching for files
//!
//! ```no_run
//! use impulse_file::manager::InMemoryFileAreaManager;
//! use impulse_file::traits::FileAreaManager;
//! use impulse_file::types::SearchCriteria;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let manager = InMemoryFileAreaManager::new();
//! let criteria = SearchCriteria::new()
//!     .with_filename("*.zip")
//!     .with_description("game");
//! let results = manager.search_files(&criteria).await?;
//! println!("Found {} files", results.len());
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Error types
pub mod error;

/// File area types and structures
pub mod types;

/// File area manager trait
pub mod traits;

/// File area manager implementation
pub mod manager;

/// FILE_ID.DIZ extraction
pub mod diz;

/// File permissions checking
pub mod permissions;

/// Search functionality
pub mod search;

/// UI screens
pub mod screens;

/// Upload functionality
pub mod upload;

/// Virus scanning
pub mod scanning;

/// Upload validation
pub mod validation;

/// File transfer protocol integration
pub mod transfer;

// Re-export commonly used types
pub use error::{FileError, Result};
pub use manager::InMemoryFileAreaManager;
pub use traits::FileAreaManager;
pub use types::{FileArea, FileStatus, SearchCriteria, SortBy};

// Re-export upload types
pub use upload::{PendingUpload, UploadConfig, UploadProcessor, UploadRollback, UploadStats};

// Re-export scanning types
#[cfg(unix)]
pub use scanning::ClamAvScanner;
pub use scanning::{MockScanner, QuarantineManager, ScanResult, VirusScanner};

// Re-export validation functions
pub use validation::{
    check_duplicate, check_extension, check_permissions, check_quota, check_size,
};

// Re-export transfer types
pub use transfer::{
    DownloadManager, DownloadResult, DownloadStats, Protocol, TransferConfig, TransferDirection,
    TransferStatus, UploadManager, UploadResult,
};
