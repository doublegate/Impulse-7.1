//! File area manager trait definition

use crate::error::Result;
use crate::types::{FileArea, SearchCriteria, SortBy};
use async_trait::async_trait;
use impulse_types::file::FileEntry;
use impulse_types::security::SecurityLevel;

/// File area manager trait
///
/// Provides async methods for managing file areas and files.
#[async_trait]
pub trait FileAreaManager: Send + Sync {
    /// List all file areas accessible to the given security level
    ///
    /// # Arguments
    ///
    /// * `user_level` - User's security level
    ///
    /// # Returns
    ///
    /// Vector of accessible file areas (excluding hidden areas unless user is operator)
    async fn list_areas(&self, user_level: SecurityLevel) -> Result<Vec<FileArea>>;

    /// Get a specific file area by ID
    ///
    /// # Arguments
    ///
    /// * `area_id` - The file area ID
    ///
    /// # Returns
    ///
    /// The file area, or None if not found
    async fn get_area(&self, area_id: u32) -> Result<Option<FileArea>>;

    /// Get files in a specific area with pagination
    ///
    /// # Arguments
    ///
    /// * `area_id` - The file area ID
    /// * `page` - Page number (0-indexed)
    /// * `page_size` - Number of files per page
    ///
    /// # Returns
    ///
    /// Tuple of (files, total_count)
    async fn get_files(
        &self,
        area_id: u32,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<FileEntry>, u32)>;

    /// Get files in a specific area with pagination and sorting
    ///
    /// # Arguments
    ///
    /// * `area_id` - The file area ID
    /// * `page` - Page number (0-indexed)
    /// * `page_size` - Number of files per page
    /// * `sort_by` - Sort order
    ///
    /// # Returns
    ///
    /// Tuple of (files, total_count)
    async fn get_files_sorted(
        &self,
        area_id: u32,
        page: u32,
        page_size: u32,
        sort_by: SortBy,
    ) -> Result<(Vec<FileEntry>, u32)>;

    /// Search for files matching criteria
    ///
    /// # Arguments
    ///
    /// * `criteria` - Search criteria
    ///
    /// # Returns
    ///
    /// Vector of matching files
    async fn search_files(&self, criteria: &SearchCriteria) -> Result<Vec<FileEntry>>;

    /// Get a specific file by ID
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID
    ///
    /// # Returns
    ///
    /// The file entry, or None if not found
    async fn get_file(&self, file_id: u64) -> Result<Option<FileEntry>>;

    /// Count total files in an area
    ///
    /// # Arguments
    ///
    /// * `area_id` - The file area ID
    ///
    /// # Returns
    ///
    /// Total number of files
    async fn count_files(&self, area_id: u32) -> Result<u32>;

    /// Add a new file area
    ///
    /// # Arguments
    ///
    /// * `area` - The file area to add
    async fn add_area(&mut self, area: FileArea) -> Result<()>;

    /// Add a file to an area
    ///
    /// # Arguments
    ///
    /// * `file` - The file entry to add
    async fn add_file(&mut self, file: FileEntry) -> Result<()>;
}
