//! File search functionality

use crate::types::SearchCriteria;
use chrono::Utc;
use impulse_types::file::FileEntry;
use wildmatch::WildMatch;

/// File searcher
///
/// Performs file searches based on search criteria.
pub struct FileSearcher<'a> {
    criteria: &'a SearchCriteria,
    filename_matcher: Option<WildMatch>,
}

impl<'a> FileSearcher<'a> {
    /// Create a new file searcher
    pub fn new(criteria: &'a SearchCriteria) -> Self {
        let filename_matcher = criteria
            .filename
            .as_ref()
            .map(|pattern| WildMatch::new(pattern));

        Self {
            criteria,
            filename_matcher,
        }
    }

    /// Check if a file matches the search criteria
    pub fn matches(&self, file: &FileEntry) -> bool {
        // Check filename pattern
        if let Some(ref matcher) = self.filename_matcher {
            if !matcher.matches(&file.filename) {
                return false;
            }
        }

        // Check description
        if let Some(ref desc_search) = self.criteria.description {
            let desc_lower = desc_search.to_lowercase();
            let file_desc_lower = file.description.to_lowercase();
            if !file_desc_lower.contains(&desc_lower) {
                return false;
            }
        }

        // Check uploader
        if let Some(ref uploader) = self.criteria.uploader {
            let uploader_lower = uploader.to_lowercase();
            let file_uploader_lower = file.uploader.to_lowercase();
            if file_uploader_lower != uploader_lower {
                return false;
            }
        }

        // Check size range
        if let Some(min_size) = self.criteria.min_size {
            if file.size_bytes < min_size {
                return false;
            }
        }
        if let Some(max_size) = self.criteria.max_size {
            if file.size_bytes > max_size {
                return false;
            }
        }

        // Check date range
        if let Some(date_from) = self.criteria.date_from {
            if file.upload_date < date_from {
                return false;
            }
        }
        if let Some(date_to) = self.criteria.date_to {
            if file.upload_date > date_to {
                return false;
            }
        }

        // Check area filter
        if let Some(area_id) = self.criteria.area_id {
            if file.area_id != area_id {
                return false;
            }
        }

        // Check new files filter (last 7 days)
        if self.criteria.new_only {
            let seven_days_ago = Utc::now() - chrono::Duration::days(7);
            if file.upload_date < seven_days_ago {
                return false;
            }
        }

        // Check available files filter
        if self.criteria.available_only && !file.is_available() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn create_test_file(filename: &str, description: &str, uploader: &str, size: u64) -> FileEntry {
        FileEntry {
            id: 1,
            filename: filename.to_string(),
            description: description.to_string(),
            uploader: uploader.to_string(),
            uploader_id: 1,
            size_bytes: size,
            upload_date: Utc::now(),
            area_id: 1,
            download_count: 0,
            is_offline: false,
            is_missing: false,
            password: None,
            cost_credits: None,
        }
    }

    #[test]
    fn test_filename_pattern_match() {
        let criteria = SearchCriteria::new().with_filename("*.zip");
        let searcher = FileSearcher::new(&criteria);

        let file1 = create_test_file("test.zip", "Test file", "alice", 1024);
        let file2 = create_test_file("test.txt", "Text file", "alice", 1024);

        assert!(searcher.matches(&file1));
        assert!(!searcher.matches(&file2));
    }

    #[test]
    fn test_wildcard_patterns() {
        let criteria = SearchCriteria::new().with_filename("game*.zip");
        let searcher = FileSearcher::new(&criteria);

        let file1 = create_test_file("game01.zip", "Game", "alice", 1024);
        let file2 = create_test_file("game02.zip", "Game", "alice", 1024);
        let file3 = create_test_file("util.zip", "Utility", "alice", 1024);

        assert!(searcher.matches(&file1));
        assert!(searcher.matches(&file2));
        assert!(!searcher.matches(&file3));
    }

    #[test]
    fn test_description_search() {
        let criteria = SearchCriteria::new().with_description("adventure");
        let searcher = FileSearcher::new(&criteria);

        let file1 = create_test_file("game.zip", "Adventure game pack", "alice", 1024);
        let file2 = create_test_file("util.zip", "System utilities", "alice", 1024);

        assert!(searcher.matches(&file1));
        assert!(!searcher.matches(&file2));
    }

    #[test]
    fn test_description_case_insensitive() {
        let criteria = SearchCriteria::new().with_description("GAME");
        let searcher = FileSearcher::new(&criteria);

        let file = create_test_file("file.zip", "game pack", "alice", 1024);
        assert!(searcher.matches(&file));
    }

    #[test]
    fn test_uploader_filter() {
        let criteria = SearchCriteria::new().with_uploader("alice");
        let searcher = FileSearcher::new(&criteria);

        let file1 = create_test_file("file1.zip", "File 1", "alice", 1024);
        let file2 = create_test_file("file2.zip", "File 2", "bob", 1024);

        assert!(searcher.matches(&file1));
        assert!(!searcher.matches(&file2));
    }

    #[test]
    fn test_uploader_case_insensitive() {
        let criteria = SearchCriteria::new().with_uploader("ALICE");
        let searcher = FileSearcher::new(&criteria);

        let file = create_test_file("file.zip", "File", "alice", 1024);
        assert!(searcher.matches(&file));
    }

    #[test]
    fn test_size_range() {
        let criteria = SearchCriteria::new().with_size_range(Some(1000), Some(5000));
        let searcher = FileSearcher::new(&criteria);

        let file1 = create_test_file("small.zip", "Small", "alice", 500);
        let file2 = create_test_file("medium.zip", "Medium", "alice", 2000);
        let file3 = create_test_file("large.zip", "Large", "alice", 10000);

        assert!(!searcher.matches(&file1)); // Too small
        assert!(searcher.matches(&file2)); // In range
        assert!(!searcher.matches(&file3)); // Too large
    }

    #[test]
    fn test_date_range() {
        let now = Utc::now();
        let yesterday = now - Duration::days(1);
        let tomorrow = now + Duration::days(1);

        let criteria = SearchCriteria::new().with_date_range(Some(yesterday), Some(tomorrow));
        let searcher = FileSearcher::new(&criteria);

        let mut file = create_test_file("file.zip", "File", "alice", 1024);

        // File uploaded now should match
        assert!(searcher.matches(&file));

        // File uploaded 2 days ago should not match
        file.upload_date = now - Duration::days(2);
        assert!(!searcher.matches(&file));

        // File uploaded 2 days from now should not match
        file.upload_date = now + Duration::days(2);
        assert!(!searcher.matches(&file));
    }

    #[test]
    fn test_area_filter() {
        let criteria = SearchCriteria::new().in_area(1);
        let searcher = FileSearcher::new(&criteria);

        let mut file1 = create_test_file("file1.zip", "File 1", "alice", 1024);
        file1.area_id = 1;

        let mut file2 = create_test_file("file2.zip", "File 2", "alice", 1024);
        file2.area_id = 2;

        assert!(searcher.matches(&file1));
        assert!(!searcher.matches(&file2));
    }

    #[test]
    fn test_new_only_filter() {
        let criteria = SearchCriteria::new().new_only();
        let searcher = FileSearcher::new(&criteria);

        let mut file1 = create_test_file("new.zip", "New file", "alice", 1024);
        file1.upload_date = Utc::now(); // Uploaded now

        let mut file2 = create_test_file("old.zip", "Old file", "alice", 1024);
        file2.upload_date = Utc::now() - Duration::days(30); // Uploaded 30 days ago

        assert!(searcher.matches(&file1));
        assert!(!searcher.matches(&file2));
    }

    #[test]
    fn test_available_only_filter() {
        let criteria = SearchCriteria::new().available_only();
        let searcher = FileSearcher::new(&criteria);

        let mut file1 = create_test_file("available.zip", "Available", "alice", 1024);
        file1.is_offline = false;
        file1.is_missing = false;

        let mut file2 = create_test_file("offline.zip", "Offline", "alice", 1024);
        file2.is_offline = true;

        let mut file3 = create_test_file("missing.zip", "Missing", "alice", 1024);
        file3.is_missing = true;

        assert!(searcher.matches(&file1));
        assert!(!searcher.matches(&file2));
        assert!(!searcher.matches(&file3));
    }

    #[test]
    fn test_combined_criteria() {
        let criteria = SearchCriteria::new()
            .with_filename("*.zip")
            .with_description("game")
            .with_uploader("alice")
            .with_size_range(Some(1000), Some(10000));
        let searcher = FileSearcher::new(&criteria);

        let file1 = create_test_file("game.zip", "Adventure game", "alice", 5000);
        assert!(searcher.matches(&file1));

        let file2 = create_test_file("game.txt", "Adventure game", "alice", 5000);
        assert!(!searcher.matches(&file2)); // Wrong extension

        let file3 = create_test_file("game.zip", "Utility pack", "alice", 5000);
        assert!(!searcher.matches(&file3)); // Wrong description

        let file4 = create_test_file("game.zip", "Adventure game", "bob", 5000);
        assert!(!searcher.matches(&file4)); // Wrong uploader

        let file5 = create_test_file("game.zip", "Adventure game", "alice", 500);
        assert!(!searcher.matches(&file5)); // Too small
    }

    #[test]
    fn test_empty_criteria_matches_all() {
        let criteria = SearchCriteria::new();
        let searcher = FileSearcher::new(&criteria);

        let file1 = create_test_file("any.zip", "Any file", "alice", 1024);
        let file2 = create_test_file("other.txt", "Other file", "bob", 2048);

        assert!(searcher.matches(&file1));
        assert!(searcher.matches(&file2));
    }
}
