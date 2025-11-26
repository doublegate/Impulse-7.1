//! Batch transfer management for Ymodem.
//!
//! This module handles multiple file transfers in a single Ymodem session.

use super::metadata::FileMetadata;
use std::path::PathBuf;

/// A file in a Ymodem batch transfer.
#[derive(Debug, Clone)]
pub struct BatchFile {
    /// File metadata.
    pub metadata: FileMetadata,
    /// File path (for sender).
    pub path: Option<PathBuf>,
    /// File data (for receiver or in-memory transfers).
    pub data: Option<Vec<u8>>,
}

impl BatchFile {
    /// Create a new batch file with metadata.
    pub fn new(metadata: FileMetadata) -> Self {
        Self {
            metadata,
            path: None,
            data: None,
        }
    }

    /// Create a batch file from a path.
    pub fn from_path(path: PathBuf, metadata: FileMetadata) -> Self {
        Self {
            metadata,
            path: Some(path),
            data: None,
        }
    }

    /// Create a batch file with in-memory data.
    pub fn from_data(metadata: FileMetadata, data: Vec<u8>) -> Self {
        Self {
            metadata,
            path: None,
            data: Some(data),
        }
    }

    /// Get the file name.
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// Get the file size if known.
    pub fn size(&self) -> Option<u64> {
        self.metadata.size.or_else(|| {
            // Try to get size from data if available
            self.data.as_ref().map(|d| d.len() as u64)
        })
    }
}

/// A batch of files for Ymodem transfer.
#[derive(Debug, Clone, Default)]
pub struct YmodemBatch {
    /// Files in the batch.
    files: Vec<BatchFile>,
    /// Current file index (for iteration).
    current: usize,
}

impl YmodemBatch {
    /// Create a new empty batch.
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            current: 0,
        }
    }

    /// Add a file to the batch.
    pub fn add_file(&mut self, file: BatchFile) {
        self.files.push(file);
    }

    /// Add a file with metadata only.
    pub fn add_metadata(&mut self, metadata: FileMetadata) {
        self.files.push(BatchFile::new(metadata));
    }

    /// Add a file from a path.
    pub fn add_from_path(&mut self, path: PathBuf, metadata: FileMetadata) {
        self.files.push(BatchFile::from_path(path, metadata));
    }

    /// Add a file with in-memory data.
    pub fn add_from_data(&mut self, metadata: FileMetadata, data: Vec<u8>) {
        self.files.push(BatchFile::from_data(metadata, data));
    }

    /// Get the number of files in the batch.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Check if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Get a file by index.
    pub fn get(&self, index: usize) -> Option<&BatchFile> {
        self.files.get(index)
    }

    /// Get a mutable reference to a file by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut BatchFile> {
        self.files.get_mut(index)
    }

    /// Get all files.
    pub fn files(&self) -> &[BatchFile] {
        &self.files
    }

    /// Reset the batch iterator.
    pub fn reset(&mut self) {
        self.current = 0;
    }

    /// Get the next file in the batch.
    pub fn next_file(&mut self) -> Option<&BatchFile> {
        if self.current < self.files.len() {
            let file = &self.files[self.current];
            self.current += 1;
            Some(file)
        } else {
            None
        }
    }

    /// Check if there are more files.
    pub fn has_more(&self) -> bool {
        self.current < self.files.len()
    }

    /// Get the current file index.
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Calculate total batch size (if all file sizes are known).
    pub fn total_size(&self) -> Option<u64> {
        let mut total = 0u64;
        for file in &self.files {
            total += file.size()?;
        }
        Some(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_file_new() {
        let metadata = FileMetadata::new("test.txt");
        let file = BatchFile::new(metadata);
        assert_eq!(file.name(), "test.txt");
        assert!(file.path.is_none());
        assert!(file.data.is_none());
    }

    #[test]
    fn test_batch_file_from_path() {
        let metadata = FileMetadata::with_size("test.txt", 100);
        let path = PathBuf::from("/path/to/test.txt");
        let file = BatchFile::from_path(path.clone(), metadata);

        assert_eq!(file.name(), "test.txt");
        assert_eq!(file.path, Some(path));
        assert_eq!(file.size(), Some(100));
    }

    #[test]
    fn test_batch_file_from_data() {
        let data = vec![0x42; 256];
        let metadata = FileMetadata::new("test.bin");
        let file = BatchFile::from_data(metadata, data.clone());

        assert_eq!(file.name(), "test.bin");
        assert_eq!(file.data, Some(data));
        assert_eq!(file.size(), Some(256)); // Gets size from data
    }

    #[test]
    fn test_batch_new() {
        let batch = YmodemBatch::new();
        assert_eq!(batch.len(), 0);
        assert!(batch.is_empty());
    }

    #[test]
    fn test_batch_add_file() {
        let mut batch = YmodemBatch::new();
        let file = BatchFile::new(FileMetadata::new("test1.txt"));
        batch.add_file(file);

        assert_eq!(batch.len(), 1);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_batch_add_metadata() {
        let mut batch = YmodemBatch::new();
        batch.add_metadata(FileMetadata::new("test1.txt"));
        batch.add_metadata(FileMetadata::new("test2.txt"));

        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn test_batch_add_from_path() {
        let mut batch = YmodemBatch::new();
        batch.add_from_path(
            PathBuf::from("/path/to/file.txt"),
            FileMetadata::with_size("file.txt", 123),
        );

        assert_eq!(batch.len(), 1);
        assert_eq!(batch.get(0).unwrap().name(), "file.txt");
    }

    #[test]
    fn test_batch_add_from_data() {
        let mut batch = YmodemBatch::new();
        let data = vec![0x55; 512];
        batch.add_from_data(FileMetadata::new("data.bin"), data);

        assert_eq!(batch.len(), 1);
        let file = batch.get(0).unwrap();
        assert_eq!(file.name(), "data.bin");
        assert_eq!(file.size(), Some(512));
    }

    #[test]
    fn test_batch_get() {
        let mut batch = YmodemBatch::new();
        batch.add_metadata(FileMetadata::new("file1.txt"));
        batch.add_metadata(FileMetadata::new("file2.txt"));

        assert_eq!(batch.get(0).unwrap().name(), "file1.txt");
        assert_eq!(batch.get(1).unwrap().name(), "file2.txt");
        assert!(batch.get(2).is_none());
    }

    #[test]
    fn test_batch_iteration() {
        let mut batch = YmodemBatch::new();
        batch.add_metadata(FileMetadata::new("file1.txt"));
        batch.add_metadata(FileMetadata::new("file2.txt"));
        batch.add_metadata(FileMetadata::new("file3.txt"));

        assert!(batch.has_more());
        assert_eq!(batch.current_index(), 0);

        let file1 = batch.next_file().unwrap();
        assert_eq!(file1.name(), "file1.txt");
        assert_eq!(batch.current_index(), 1);

        let file2 = batch.next_file().unwrap();
        assert_eq!(file2.name(), "file2.txt");
        assert_eq!(batch.current_index(), 2);

        let file3 = batch.next_file().unwrap();
        assert_eq!(file3.name(), "file3.txt");
        assert_eq!(batch.current_index(), 3);

        assert!(!batch.has_more());
        assert!(batch.next_file().is_none());
    }

    #[test]
    fn test_batch_reset() {
        let mut batch = YmodemBatch::new();
        batch.add_metadata(FileMetadata::new("file1.txt"));
        batch.add_metadata(FileMetadata::new("file2.txt"));

        batch.next_file();
        batch.next_file();
        assert_eq!(batch.current_index(), 2);

        batch.reset();
        assert_eq!(batch.current_index(), 0);
        assert!(batch.has_more());
    }

    #[test]
    fn test_batch_total_size() {
        let mut batch = YmodemBatch::new();
        batch.add_metadata(FileMetadata::with_size("file1.txt", 100));
        batch.add_metadata(FileMetadata::with_size("file2.txt", 200));
        batch.add_metadata(FileMetadata::with_size("file3.txt", 300));

        assert_eq!(batch.total_size(), Some(600));
    }

    #[test]
    fn test_batch_total_size_unknown() {
        let mut batch = YmodemBatch::new();
        batch.add_metadata(FileMetadata::with_size("file1.txt", 100));
        batch.add_metadata(FileMetadata::new("file2.txt")); // No size

        assert_eq!(batch.total_size(), None); // Can't calculate if any size unknown
    }

    #[test]
    fn test_batch_files() {
        let mut batch = YmodemBatch::new();
        batch.add_metadata(FileMetadata::new("file1.txt"));
        batch.add_metadata(FileMetadata::new("file2.txt"));

        let files = batch.files();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].name(), "file1.txt");
        assert_eq!(files[1].name(), "file2.txt");
    }

    #[test]
    fn test_batch_default() {
        let batch = YmodemBatch::default();
        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);
    }
}
