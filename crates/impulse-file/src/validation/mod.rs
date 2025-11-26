//! Upload validation
//!
//! This module provides validation for file uploads:
//! - File size limits
//! - Extension allow/block lists
//! - Duplicate detection (SHA-256)
//! - Upload quotas (files and bytes per day)
//! - Area upload permissions

pub mod duplicates;
pub mod extensions;
pub mod permissions;
pub mod quotas;
pub mod size;

pub use duplicates::{calculate_file_hash, check_duplicate};
pub use extensions::{check_extension, extract_extension};
pub use permissions::check_permissions;
pub use quotas::check_quota;
pub use size::{check_size, format_size};
