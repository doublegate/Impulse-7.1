//! Common test utilities and helpers for integration tests
//!
//! This module provides shared functionality for integration tests across
//! the Impulse-Next_BBS workspace.

use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Load a test fixture file
///
/// # Arguments
///
/// * `path` - Relative path within the fixtures directory (e.g., "users/valid.json")
///
/// # Panics
///
/// Panics if the fixture file cannot be read
pub fn load_fixture(path: &str) -> Vec<u8> {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(path);

    std::fs::read(&fixture_path)
        .unwrap_or_else(|e| panic!("Failed to load fixture {}: {}", fixture_path.display(), e))
}

/// Load a text fixture file as a String
///
/// # Arguments
///
/// * `path` - Relative path within the fixtures directory
///
/// # Panics
///
/// Panics if the fixture file cannot be read or contains invalid UTF-8
pub fn load_text_fixture(path: &str) -> String {
    String::from_utf8(load_fixture(path))
        .unwrap_or_else(|e| panic!("Invalid UTF-8 in fixture {}: {}", path, e))
}

/// Create a temporary directory for tests
///
/// The directory will be automatically cleaned up when the returned
/// `TempDir` is dropped.
pub fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temporary directory")
}

/// Create a temporary directory with a specific prefix
///
/// # Arguments
///
/// * `prefix` - Prefix for the temporary directory name
pub fn create_temp_dir_with_prefix(prefix: &str) -> TempDir {
    tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .expect("Failed to create temporary directory")
}

/// Get the path to the test fixtures directory
pub fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

/// Get the path to a specific fixture file
///
/// # Arguments
///
/// * `path` - Relative path within the fixtures directory
pub fn fixture_path(path: &str) -> PathBuf {
    fixtures_dir().join(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_temp_dir() {
        let dir = create_temp_dir();
        assert!(dir.path().exists());
        assert!(dir.path().is_dir());
    }

    #[test]
    fn test_create_temp_dir_with_prefix() {
        let dir = create_temp_dir_with_prefix("impulse-test-");
        assert!(dir.path().exists());
        assert!(dir.path().is_dir());

        let name = dir.path().file_name().unwrap().to_str().unwrap();
        assert!(name.starts_with("impulse-test-"));
    }

    #[test]
    fn test_fixtures_dir() {
        let dir = fixtures_dir();
        assert!(dir.ends_with("tests/fixtures"));
    }

    #[test]
    fn test_fixture_path() {
        let path = fixture_path("users/test.json");
        assert!(path.ends_with("tests/fixtures/users/test.json"));
    }
}
