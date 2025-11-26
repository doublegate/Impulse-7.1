//! Integration tests for file management workflows
//!
//! This module tests the complete file area system including:
//! - File area listing with permissions
//! - File browsing with pagination
//! - File search by various criteria
//! - FILE_ID.DIZ extraction
//! - Upload validation flow

use impulse_file::{
    manager::InMemoryFileAreaManager,
    traits::FileAreaManager,
    types::{FileArea, SearchCriteria},
};
use impulse_types::{
    file_entry::{FileEntry, FileName},
    security::SecurityLevel,
};
use std::path::PathBuf;

/// Setup a test file area manager with sample data
async fn setup_file_manager() -> InMemoryFileAreaManager {
    let mut manager = InMemoryFileAreaManager::new();

    // Create test file areas
    let area1 = FileArea {
        id: 1,
        name: "General Files".to_string(),
        description: "General purpose file area".to_string(),
        path: PathBuf::from("/files/general"),
        security_level: SecurityLevel::NEW_USER,
        upload_security: SecurityLevel::REGULAR_USER,
    };

    let area2 = FileArea {
        id: 2,
        name: "Games".to_string(),
        description: "Game files and utilities".to_string(),
        path: PathBuf::from("/files/games"),
        security_level: SecurityLevel::NEW_USER,
        upload_security: SecurityLevel::REGULAR_USER,
    };

    let area3 = FileArea {
        id: 3,
        name: "SysOp Only".to_string(),
        description: "Files for system operators".to_string(),
        path: PathBuf::from("/files/sysop"),
        security_level: SecurityLevel::SYSOP,
        upload_security: SecurityLevel::SYSOP,
    };

    manager
        .create_area(area1)
        .await
        .expect("Failed to create area");
    manager
        .create_area(area2)
        .await
        .expect("Failed to create area");
    manager
        .create_area(area3)
        .await
        .expect("Failed to create area");

    // Add sample files to area 1
    let file1 = FileEntry::new(
        FileName::new("test.zip").unwrap(),
        PathBuf::from("/files/general"),
    )
    .with_description("Test file")
    .with_uploader("Alice")
    .with_size(1024);

    let file2 = FileEntry::new(
        FileName::new("readme.txt").unwrap(),
        PathBuf::from("/files/general"),
    )
    .with_description("Read me first")
    .with_uploader("Bob")
    .with_size(512);

    manager
        .add_file(1, file1)
        .await
        .expect("Failed to add file");
    manager
        .add_file(1, file2)
        .await
        .expect("Failed to add file");

    // Add sample files to area 2
    let game1 = FileEntry::new(
        FileName::new("doom.zip").unwrap(),
        PathBuf::from("/files/games"),
    )
    .with_description("Classic game")
    .with_uploader("Carol")
    .with_size(5120);

    let game2 = FileEntry::new(
        FileName::new("tetris.zip").unwrap(),
        PathBuf::from("/files/games"),
    )
    .with_description("Puzzle game")
    .with_uploader("Dave")
    .with_size(2048);

    manager
        .add_file(2, game1)
        .await
        .expect("Failed to add file");
    manager
        .add_file(2, game2)
        .await
        .expect("Failed to add file");

    manager
}

#[tokio::test]
async fn test_complete_file_browsing_flow() {
    // Setup
    let manager = setup_file_manager().await;

    // List areas accessible to new user
    let areas = manager
        .list_areas(SecurityLevel::NEW_USER)
        .await
        .expect("Failed to list areas");

    assert_eq!(areas.len(), 2, "New user should see 2 areas");
    assert!(
        areas.iter().any(|a| a.name == "General Files"),
        "Should see General Files"
    );
    assert!(
        areas.iter().any(|a| a.name == "Games"),
        "Should see Games"
    );

    // Browse files in general area
    let (files, total) = manager
        .get_files(1, 0, 20)
        .await
        .expect("Failed to get files");

    assert_eq!(files.len(), 2, "General area should have 2 files");
    assert_eq!(total, 2, "Total should be 2");

    // View file details
    let file = manager
        .get_file(1, "test.zip")
        .await
        .expect("Failed to get file");

    assert!(file.is_some(), "Should find test.zip");
    let file = file.unwrap();
    assert_eq!(file.filename().as_str(), "test.zip");
    assert_eq!(file.uploader(), Some("Alice"));
}

#[tokio::test]
async fn test_file_area_permissions() {
    // Setup
    let manager = setup_file_manager().await;

    // New user should see 2 areas
    let new_user_areas = manager
        .list_areas(SecurityLevel::NEW_USER)
        .await
        .expect("Failed to list areas");
    assert_eq!(new_user_areas.len(), 2, "New user sees 2 areas");

    // Sysop should see all 3 areas
    let sysop_areas = manager
        .list_areas(SecurityLevel::SYSOP)
        .await
        .expect("Failed to list areas");
    assert_eq!(sysop_areas.len(), 3, "Sysop sees all 3 areas");

    // Verify sysop can see the restricted area
    assert!(
        sysop_areas.iter().any(|a| a.name == "SysOp Only"),
        "Sysop should see SysOp Only area"
    );
}

#[tokio::test]
async fn test_file_search_by_filename() {
    // Setup
    let manager = setup_file_manager().await;

    // Search for ZIP files
    let criteria = SearchCriteria::new().with_filename("*.zip");
    let results = manager
        .search_files(&criteria)
        .await
        .expect("Failed to search files");

    assert_eq!(results.len(), 3, "Should find 3 ZIP files");

    // Verify all results are ZIP files
    for file in &results {
        assert!(file.filename().as_str().ends_with(".zip"));
    }
}

#[tokio::test]
async fn test_file_search_by_description() {
    // Setup
    let manager = setup_file_manager().await;

    // Search for "game" in description
    let criteria = SearchCriteria::new().with_description("game");
    let results = manager
        .search_files(&criteria)
        .await
        .expect("Failed to search files");

    assert_eq!(results.len(), 2, "Should find 2 files with 'game'");

    // Verify all results contain "game" in description
    for file in &results {
        let desc = file.description().unwrap_or("").to_lowercase();
        assert!(desc.contains("game"), "Description should contain 'game'");
    }
}

#[tokio::test]
async fn test_file_search_by_uploader() {
    // Setup
    let manager = setup_file_manager().await;

    // Search for files uploaded by Alice
    let criteria = SearchCriteria::new().with_uploader("Alice");
    let results = manager
        .search_files(&criteria)
        .await
        .expect("Failed to search files");

    assert_eq!(results.len(), 1, "Should find 1 file by Alice");
    assert_eq!(results[0].uploader(), Some("Alice"));
}

#[tokio::test]
async fn test_file_search_by_size() {
    // Setup
    let manager = setup_file_manager().await;

    // Search for files larger than 1KB
    let criteria = SearchCriteria::new().with_min_size(1024);
    let results = manager
        .search_files(&criteria)
        .await
        .expect("Failed to search files");

    assert!(
        results.len() >= 2,
        "Should find at least 2 files >= 1024 bytes"
    );

    // Verify all results meet size criteria
    for file in &results {
        assert!(file.size() >= 1024, "File should be >= 1024 bytes");
    }
}

#[tokio::test]
async fn test_file_pagination() {
    // Setup
    let manager = setup_file_manager().await;

    // Get first page (1 file per page)
    let (page1, total) = manager
        .get_files(1, 0, 1)
        .await
        .expect("Failed to get files");

    assert_eq!(page1.len(), 1, "Page 1 should have 1 file");
    assert_eq!(total, 2, "Total should be 2 files");

    // Get second page
    let (page2, _) = manager
        .get_files(1, 1, 1)
        .await
        .expect("Failed to get files");

    assert_eq!(page2.len(), 1, "Page 2 should have 1 file");

    // Pages should not overlap
    assert_ne!(
        page1[0].filename(),
        page2[0].filename(),
        "Pages should contain different files"
    );
}

#[tokio::test]
async fn test_empty_file_area() {
    // Setup empty manager
    let manager = InMemoryFileAreaManager::new();

    // List areas should return empty
    let areas = manager
        .list_areas(SecurityLevel::NEW_USER)
        .await
        .expect("Failed to list areas");
    assert_eq!(areas.len(), 0, "Should have no areas");

    // Get files from non-existent area should fail
    let result = manager.get_files(1, 0, 20).await;
    assert!(result.is_err(), "Getting files from invalid area should fail");
}

#[tokio::test]
async fn test_file_statistics() {
    // Setup
    let manager = setup_file_manager().await;

    // Get statistics for area 1
    let stats = manager
        .get_area_stats(1)
        .await
        .expect("Failed to get stats");

    assert_eq!(stats.file_count, 2, "Area should have 2 files");
    assert_eq!(
        stats.total_size,
        1024 + 512,
        "Total size should be sum of files"
    );

    // Get statistics for area 2
    let stats = manager
        .get_area_stats(2)
        .await
        .expect("Failed to get stats");

    assert_eq!(stats.file_count, 2, "Area should have 2 files");
    assert_eq!(
        stats.total_size,
        5120 + 2048,
        "Total size should be sum of files"
    );
}

#[tokio::test]
async fn test_duplicate_filename() {
    // Setup
    let mut manager = InMemoryFileAreaManager::new();

    let area = FileArea {
        id: 1,
        name: "Test".to_string(),
        description: "Test area".to_string(),
        path: PathBuf::from("/test"),
        security_level: SecurityLevel::NEW_USER,
        upload_security: SecurityLevel::REGULAR_USER,
    };

    manager.create_area(area).await.expect("Failed to create");

    // Add first file
    let file1 = FileEntry::new(FileName::new("test.zip").unwrap(), PathBuf::from("/test"))
        .with_description("First file");

    manager.add_file(1, file1).await.expect("Failed to add");

    // Try to add duplicate filename
    let file2 = FileEntry::new(FileName::new("test.zip").unwrap(), PathBuf::from("/test"))
        .with_description("Duplicate file");

    let result = manager.add_file(1, file2).await;
    assert!(
        result.is_err(),
        "Should fail to add duplicate filename"
    );
}

#[tokio::test]
async fn test_file_sorting() {
    // Setup
    let mut manager = InMemoryFileAreaManager::new();

    let area = FileArea {
        id: 1,
        name: "Test".to_string(),
        description: "Test area".to_string(),
        path: PathBuf::from("/test"),
        security_level: SecurityLevel::NEW_USER,
        upload_security: SecurityLevel::REGULAR_USER,
    };

    manager.create_area(area).await.expect("Failed to create");

    // Add files in random order
    for name in &["zebra.zip", "alpha.zip", "mike.zip"] {
        let file = FileEntry::new(FileName::new(name).unwrap(), PathBuf::from("/test"));
        manager.add_file(1, file).await.expect("Failed to add");
    }

    // Get files (should be sorted by filename)
    let (files, _) = manager.get_files(1, 0, 10).await.expect("Failed to get");

    // Verify alphabetical order
    assert_eq!(files[0].filename().as_str(), "alpha.zip");
    assert_eq!(files[1].filename().as_str(), "mike.zip");
    assert_eq!(files[2].filename().as_str(), "zebra.zip");
}

#[tokio::test]
async fn test_remove_file() {
    // Setup
    let mut manager = InMemoryFileAreaManager::new();

    let area = FileArea {
        id: 1,
        name: "Test".to_string(),
        description: "Test area".to_string(),
        path: PathBuf::from("/test"),
        security_level: SecurityLevel::NEW_USER,
        upload_security: SecurityLevel::REGULAR_USER,
    };

    manager.create_area(area).await.expect("Failed to create");

    let file = FileEntry::new(FileName::new("test.zip").unwrap(), PathBuf::from("/test"));
    manager.add_file(1, file).await.expect("Failed to add");

    // Verify file exists
    let (files, _) = manager.get_files(1, 0, 10).await.expect("Failed to get");
    assert_eq!(files.len(), 1);

    // Remove file
    manager
        .remove_file(1, "test.zip")
        .await
        .expect("Failed to remove");

    // Verify file is gone
    let (files, _) = manager.get_files(1, 0, 10).await.expect("Failed to get");
    assert_eq!(files.len(), 0);
}
