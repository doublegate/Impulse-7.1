//! File area manager implementation

use crate::error::{FileError, Result};
use crate::search::FileSearcher;
use crate::traits::FileAreaManager;
use crate::types::{FileArea, SearchCriteria, SortBy};
use async_trait::async_trait;
use impulse_types::file::FileEntry;
use impulse_types::security::SecurityLevel;
use indexmap::IndexMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory file area manager
///
/// Stores file areas and files in memory using concurrent data structures.
#[derive(Clone)]
pub struct InMemoryFileAreaManager {
    areas: Arc<RwLock<IndexMap<u32, FileArea>>>,
    files: Arc<RwLock<Vec<FileEntry>>>,
}

impl InMemoryFileAreaManager {
    /// Create a new in-memory file area manager
    pub fn new() -> Self {
        Self {
            areas: Arc::new(RwLock::new(IndexMap::new())),
            files: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Sort files by the specified criteria
    fn sort_files(files: &mut [FileEntry], sort_by: SortBy) {
        files.sort_by(|a, b| match sort_by {
            SortBy::NameAsc => a.filename.cmp(&b.filename),
            SortBy::NameDesc => b.filename.cmp(&a.filename),
            SortBy::DateDesc => b.upload_date.cmp(&a.upload_date),
            SortBy::DateAsc => a.upload_date.cmp(&b.upload_date),
            SortBy::SizeDesc => b.size_bytes.cmp(&a.size_bytes),
            SortBy::SizeAsc => a.size_bytes.cmp(&b.size_bytes),
            SortBy::DownloadsDesc => b.download_count.cmp(&a.download_count),
            SortBy::DownloadsAsc => a.download_count.cmp(&b.download_count),
        });
    }
}

impl Default for InMemoryFileAreaManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FileAreaManager for InMemoryFileAreaManager {
    async fn list_areas(&self, user_level: SecurityLevel) -> Result<Vec<FileArea>> {
        let areas = self.areas.read().await;
        let is_operator = user_level.is_operator();

        Ok(areas
            .values()
            .filter(|area| {
                // Check security level
                if !user_level.can_access(area.security_level) {
                    return false;
                }

                // Show hidden areas only to operators
                if area.hidden && !is_operator {
                    return false;
                }

                true
            })
            .cloned()
            .collect())
    }

    async fn get_area(&self, area_id: u32) -> Result<Option<FileArea>> {
        let areas = self.areas.read().await;
        Ok(areas.get(&area_id).cloned())
    }

    async fn get_files(
        &self,
        area_id: u32,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<FileEntry>, u32)> {
        self.get_files_sorted(area_id, page, page_size, SortBy::default())
            .await
    }

    async fn get_files_sorted(
        &self,
        area_id: u32,
        page: u32,
        page_size: u32,
        sort_by: SortBy,
    ) -> Result<(Vec<FileEntry>, u32)> {
        // Verify area exists
        {
            let areas = self.areas.read().await;
            if !areas.contains_key(&area_id) {
                return Err(FileError::AreaNotFound(area_id));
            }
        }

        let files = self.files.read().await;

        // Filter files for this area
        let mut area_files: Vec<FileEntry> = files
            .iter()
            .filter(|f| f.area_id == area_id)
            .cloned()
            .collect();

        let total = area_files.len() as u32;

        // Sort files
        Self::sort_files(&mut area_files, sort_by);

        // Paginate
        let offset = (page * page_size) as usize;
        let page_files = area_files
            .into_iter()
            .skip(offset)
            .take(page_size as usize)
            .collect();

        Ok((page_files, total))
    }

    async fn search_files(&self, criteria: &SearchCriteria) -> Result<Vec<FileEntry>> {
        let files = self.files.read().await;
        let searcher = FileSearcher::new(criteria);

        Ok(files
            .iter()
            .filter(|file| searcher.matches(file))
            .cloned()
            .collect())
    }

    async fn get_file(&self, file_id: u64) -> Result<Option<FileEntry>> {
        let files = self.files.read().await;
        Ok(files.iter().find(|f| f.id as u64 == file_id).cloned())
    }

    async fn count_files(&self, area_id: u32) -> Result<u32> {
        let files = self.files.read().await;
        Ok(files.iter().filter(|f| f.area_id == area_id).count() as u32)
    }

    async fn add_area(&mut self, area: FileArea) -> Result<()> {
        let mut areas = self.areas.write().await;
        areas.insert(area.area_id, area);
        Ok(())
    }

    async fn add_file(&mut self, file: FileEntry) -> Result<()> {
        // Extract area_id before moving file
        let area_id = file.area_id;

        let mut files = self.files.write().await;
        files.push(file);

        // Update file count in area
        let mut areas = self.areas.write().await;
        if let Some(area) = areas.get_mut(&area_id) {
            area.file_count += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_area(id: u32, name: &str) -> FileArea {
        FileArea::new(id, name.to_string(), format!("{} area", name))
    }

    fn create_test_file(id: u32, area_id: u32, filename: &str) -> FileEntry {
        FileEntry {
            id,
            filename: filename.to_string(),
            description: format!("Description for {}", filename),
            uploader: "testuser".to_string(),
            uploader_id: 1,
            size_bytes: 1024 * (id as u64 + 1),
            upload_date: Utc::now(),
            area_id,
            download_count: id * 2,
            is_offline: false,
            is_missing: false,
            password: None,
            cost_credits: None,
        }
    }

    #[tokio::test]
    async fn test_add_and_list_areas() {
        let mut manager = InMemoryFileAreaManager::new();

        let area1 = create_test_area(1, "General");
        let area2 = create_test_area(2, "Games");

        manager.add_area(area1).await.unwrap();
        manager.add_area(area2).await.unwrap();

        let areas = manager.list_areas(SecurityLevel::NEW_USER).await.unwrap();
        assert_eq!(areas.len(), 2);
        assert_eq!(areas[0].name, "General");
        assert_eq!(areas[1].name, "Games");
    }

    #[tokio::test]
    async fn test_get_area() {
        let mut manager = InMemoryFileAreaManager::new();
        let area = create_test_area(1, "General");
        manager.add_area(area).await.unwrap();

        let found = manager.get_area(1).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "General");

        let not_found = manager.get_area(999).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_add_and_get_files() {
        let mut manager = InMemoryFileAreaManager::new();
        let area = create_test_area(1, "General");
        manager.add_area(area).await.unwrap();

        let file1 = create_test_file(1, 1, "file1.zip");
        let file2 = create_test_file(2, 1, "file2.zip");

        manager.add_file(file1).await.unwrap();
        manager.add_file(file2).await.unwrap();

        let (files, total) = manager.get_files(1, 0, 10).await.unwrap();
        assert_eq!(total, 2);
        assert_eq!(files.len(), 2);
    }

    #[tokio::test]
    async fn test_pagination() {
        let mut manager = InMemoryFileAreaManager::new();
        let area = create_test_area(1, "General");
        manager.add_area(area).await.unwrap();

        // Add 25 files
        for i in 0..25 {
            let file = create_test_file(i, 1, &format!("file{}.zip", i));
            manager.add_file(file).await.unwrap();
        }

        // Get first page (10 files)
        let (files, total) = manager.get_files(1, 0, 10).await.unwrap();
        assert_eq!(total, 25);
        assert_eq!(files.len(), 10);

        // Get second page (10 files)
        let (files, _) = manager.get_files(1, 1, 10).await.unwrap();
        assert_eq!(files.len(), 10);

        // Get third page (5 files)
        let (files, _) = manager.get_files(1, 2, 10).await.unwrap();
        assert_eq!(files.len(), 5);
    }

    #[tokio::test]
    async fn test_sorting() {
        let mut manager = InMemoryFileAreaManager::new();
        let area = create_test_area(1, "General");
        manager.add_area(area).await.unwrap();

        manager
            .add_file(create_test_file(1, 1, "zebra.zip"))
            .await
            .unwrap();
        manager
            .add_file(create_test_file(2, 1, "apple.zip"))
            .await
            .unwrap();
        manager
            .add_file(create_test_file(3, 1, "banana.zip"))
            .await
            .unwrap();

        // Sort by name ascending
        let (files, _) = manager
            .get_files_sorted(1, 0, 10, SortBy::NameAsc)
            .await
            .unwrap();
        assert_eq!(files[0].filename, "apple.zip");
        assert_eq!(files[1].filename, "banana.zip");
        assert_eq!(files[2].filename, "zebra.zip");

        // Sort by name descending
        let (files, _) = manager
            .get_files_sorted(1, 0, 10, SortBy::NameDesc)
            .await
            .unwrap();
        assert_eq!(files[0].filename, "zebra.zip");
        assert_eq!(files[1].filename, "banana.zip");
        assert_eq!(files[2].filename, "apple.zip");
    }

    #[tokio::test]
    async fn test_count_files() {
        let mut manager = InMemoryFileAreaManager::new();
        let area = create_test_area(1, "General");
        manager.add_area(area).await.unwrap();

        assert_eq!(manager.count_files(1).await.unwrap(), 0);

        manager
            .add_file(create_test_file(1, 1, "file1.zip"))
            .await
            .unwrap();
        manager
            .add_file(create_test_file(2, 1, "file2.zip"))
            .await
            .unwrap();

        assert_eq!(manager.count_files(1).await.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_security_level_filtering() {
        let mut manager = InMemoryFileAreaManager::new();

        let area1 = create_test_area(1, "Public").with_security_level(SecurityLevel::NEW_USER);
        let area2 = create_test_area(2, "Members").with_security_level(SecurityLevel::VALIDATED);
        let area3 = create_test_area(3, "VIP").with_security_level(SecurityLevel::PRIVILEGED);

        manager.add_area(area1).await.unwrap();
        manager.add_area(area2).await.unwrap();
        manager.add_area(area3).await.unwrap();

        // New user can only see public area
        let areas = manager.list_areas(SecurityLevel::NEW_USER).await.unwrap();
        assert_eq!(areas.len(), 1);
        assert_eq!(areas[0].name, "Public");

        // Validated user can see public and members areas
        let areas = manager.list_areas(SecurityLevel::VALIDATED).await.unwrap();
        assert_eq!(areas.len(), 2);

        // Privileged user can see all areas
        let areas = manager.list_areas(SecurityLevel::PRIVILEGED).await.unwrap();
        assert_eq!(areas.len(), 3);
    }

    #[tokio::test]
    async fn test_hidden_areas() {
        let mut manager = InMemoryFileAreaManager::new();

        let area1 = create_test_area(1, "Public");
        let area2 = create_test_area(2, "Hidden").hidden();

        manager.add_area(area1).await.unwrap();
        manager.add_area(area2).await.unwrap();

        // Normal user cannot see hidden areas
        let areas = manager.list_areas(SecurityLevel::NEW_USER).await.unwrap();
        assert_eq!(areas.len(), 1);

        // Operators can see hidden areas
        let areas = manager.list_areas(SecurityLevel::SYSOP).await.unwrap();
        assert_eq!(areas.len(), 2);
    }
}
