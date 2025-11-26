//! File metadata for Ymodem protocol.
//!
//! Ymodem uses block 0 to transmit file metadata including filename, size,
//! and modification time.

use std::time::SystemTime;

/// File metadata for Ymodem transfers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    /// File name (without path).
    pub name: String,
    /// File size in bytes.
    pub size: Option<u64>,
    /// File modification time.
    pub mod_time: Option<SystemTime>,
    /// File mode/permissions (Unix).
    pub mode: Option<u32>,
    /// Serial number (optional).
    pub serial: Option<u32>,
}

impl FileMetadata {
    /// Create new file metadata with just a name.
    ///
    /// # Arguments
    ///
    /// * `name` - File name (without path)
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            size: None,
            mod_time: None,
            mode: None,
            serial: None,
        }
    }

    /// Create file metadata with name and size.
    pub fn with_size<S: Into<String>>(name: S, size: u64) -> Self {
        Self {
            name: name.into(),
            size: Some(size),
            mod_time: None,
            mode: None,
            serial: None,
        }
    }

    /// Set file size.
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Set modification time.
    pub fn mod_time(mut self, time: SystemTime) -> Self {
        self.mod_time = Some(time);
        self
    }

    /// Set file mode/permissions.
    pub fn mode(mut self, mode: u32) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set serial number.
    pub fn serial(mut self, serial: u32) -> Self {
        self.serial = Some(serial);
        self
    }

    /// Encode metadata into block 0 format.
    ///
    /// Format: `filename\0size mod_time mode serial\0`
    ///
    /// Where:
    /// - filename: null-terminated string
    /// - size: decimal number
    /// - mod_time: octal Unix timestamp
    /// - mode: octal file mode
    /// - serial: decimal serial number
    ///
    /// All fields after filename are space-separated and collectively null-terminated.
    ///
    /// # Returns
    ///
    /// A vector containing the encoded metadata, padded to 128 bytes.
    pub fn encode(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Add filename (null-terminated)
        data.extend_from_slice(self.name.as_bytes());
        data.push(0);

        // Build metadata string
        let mut metadata_parts = Vec::new();

        if let Some(size) = self.size {
            metadata_parts.push(format!("{}", size));
        }

        if let Some(time) = self.mod_time
            && let Ok(duration) = time.duration_since(SystemTime::UNIX_EPOCH)
        {
            metadata_parts.push(format!("{:o}", duration.as_secs()));
        }

        if let Some(mode) = self.mode {
            metadata_parts.push(format!("{:o}", mode));
        }

        if let Some(serial) = self.serial {
            metadata_parts.push(format!("{}", serial));
        }

        if !metadata_parts.is_empty() {
            let metadata_str = metadata_parts.join(" ");
            data.extend_from_slice(metadata_str.as_bytes());
        }

        // Null-terminate metadata string
        data.push(0);

        // Pad to 128 bytes (standard Ymodem block 0 size)
        if data.len() < 128 {
            data.resize(128, 0);
        } else if data.len() > 128 {
            // Truncate if too long (shouldn't happen with reasonable filenames)
            data.truncate(128);
            data[127] = 0; // Ensure last byte is null
        }

        data
    }

    /// Decode metadata from block 0 data.
    ///
    /// # Arguments
    ///
    /// * `data` - Block 0 data (typically 128 bytes)
    ///
    /// # Returns
    ///
    /// * `Ok(Some(metadata))` - Successfully decoded metadata
    /// * `Ok(None)` - Empty block 0 (end of batch)
    /// * `Err(String)` - Decode error
    pub fn decode(data: &[u8]) -> Result<Option<Self>, String> {
        // Find first null byte (end of filename)
        let filename_end = data
            .iter()
            .position(|&b| b == 0)
            .ok_or("No null terminator found")?;

        // Check for empty block 0 (end of batch)
        if filename_end == 0 {
            return Ok(None);
        }

        // Extract filename
        let name = String::from_utf8(data[..filename_end].to_vec())
            .map_err(|e| format!("Invalid UTF-8 in filename: {}", e))?;

        // Find second null byte (end of metadata)
        let metadata_start = filename_end + 1;
        let metadata_end = data[metadata_start..]
            .iter()
            .position(|&b| b == 0)
            .map(|pos| metadata_start + pos)
            .unwrap_or(data.len());

        // Extract metadata string
        let metadata_str = if metadata_end > metadata_start {
            String::from_utf8_lossy(&data[metadata_start..metadata_end])
        } else {
            return Ok(Some(Self::new(name))); // Only filename, no metadata
        };

        // Parse metadata fields (space-separated)
        let parts: Vec<&str> = metadata_str.split_whitespace().collect();
        let mut metadata = Self::new(name);

        // Parse size (decimal)
        if !parts.is_empty()
            && let Ok(size) = parts[0].parse::<u64>()
        {
            metadata.size = Some(size);
        }

        // Parse mod_time (octal Unix timestamp)
        if parts.len() > 1
            && let Ok(timestamp) = u64::from_str_radix(parts[1], 8)
        {
            metadata.mod_time =
                Some(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp));
        }

        // Parse mode (octal)
        if parts.len() > 2
            && let Ok(mode) = u32::from_str_radix(parts[2], 8)
        {
            metadata.mode = Some(mode);
        }

        // Parse serial (decimal)
        if parts.len() > 3
            && let Ok(serial) = parts[3].parse::<u32>()
        {
            metadata.serial = Some(serial);
        }

        Ok(Some(metadata))
    }

    /// Create an empty block 0 to signal end of batch.
    pub fn end_of_batch() -> Vec<u8> {
        vec![0u8; 128]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let metadata = FileMetadata::new("test.txt");
        assert_eq!(metadata.name, "test.txt");
        assert!(metadata.size.is_none());
        assert!(metadata.mod_time.is_none());
        assert!(metadata.mode.is_none());
        assert!(metadata.serial.is_none());
    }

    #[test]
    fn test_with_size() {
        let metadata = FileMetadata::with_size("test.txt", 12345);
        assert_eq!(metadata.name, "test.txt");
        assert_eq!(metadata.size, Some(12345));
    }

    #[test]
    fn test_builder() {
        let time = SystemTime::now();
        let metadata = FileMetadata::new("test.txt")
            .size(12345)
            .mod_time(time)
            .mode(0o644)
            .serial(42);

        assert_eq!(metadata.name, "test.txt");
        assert_eq!(metadata.size, Some(12345));
        assert_eq!(metadata.mod_time, Some(time));
        assert_eq!(metadata.mode, Some(0o644));
        assert_eq!(metadata.serial, Some(42));
    }

    #[test]
    fn test_encode_name_only() {
        let metadata = FileMetadata::new("test.txt");
        let encoded = metadata.encode();

        assert_eq!(encoded.len(), 128);
        assert_eq!(&encoded[..9], b"test.txt\0");
        assert_eq!(encoded[9], 0); // Null terminator for metadata
    }

    #[test]
    fn test_encode_with_size() {
        let metadata = FileMetadata::with_size("test.txt", 12345);
        let encoded = metadata.encode();

        assert_eq!(encoded.len(), 128);
        assert_eq!(&encoded[..9], b"test.txt\0");
        // Should contain "12345\0"
        let metadata_str = String::from_utf8_lossy(&encoded[9..]);
        assert!(metadata_str.contains("12345"));
    }

    #[test]
    fn test_encode_full_metadata() {
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1000000);
        let metadata = FileMetadata::new("test.txt")
            .size(12345)
            .mod_time(time)
            .mode(0o644)
            .serial(42);

        let encoded = metadata.encode();
        assert_eq!(encoded.len(), 128);

        // Verify filename
        assert_eq!(&encoded[..9], b"test.txt\0");

        // Verify metadata contains expected values
        let metadata_str = String::from_utf8_lossy(&encoded[9..]);
        assert!(metadata_str.contains("12345")); // size
        assert!(metadata_str.contains("3641100")); // time in octal
        assert!(metadata_str.contains("644")); // mode in octal
        assert!(metadata_str.contains("42")); // serial
    }

    #[test]
    fn test_decode_name_only() {
        let mut data = vec![0u8; 128];
        data[..9].copy_from_slice(b"test.txt\0");
        data[9] = 0;

        let metadata = FileMetadata::decode(&data).unwrap().unwrap();
        assert_eq!(metadata.name, "test.txt");
        assert!(metadata.size.is_none());
    }

    #[test]
    fn test_decode_with_size() {
        let original = FileMetadata::with_size("test.txt", 12345);
        let encoded = original.encode();
        let decoded = FileMetadata::decode(&encoded).unwrap().unwrap();

        assert_eq!(decoded.name, "test.txt");
        assert_eq!(decoded.size, Some(12345));
    }

    #[test]
    fn test_decode_full_metadata() {
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1000000);
        let original = FileMetadata::new("test.txt")
            .size(12345)
            .mod_time(time)
            .mode(0o644)
            .serial(42);

        let encoded = original.encode();
        let decoded = FileMetadata::decode(&encoded).unwrap().unwrap();

        assert_eq!(decoded.name, "test.txt");
        assert_eq!(decoded.size, Some(12345));
        assert_eq!(decoded.mod_time, Some(time));
        assert_eq!(decoded.mode, Some(0o644));
        assert_eq!(decoded.serial, Some(42));
    }

    #[test]
    fn test_decode_empty_block() {
        let data = vec![0u8; 128];
        let result = FileMetadata::decode(&data).unwrap();
        assert!(result.is_none()); // Empty block = end of batch
    }

    #[test]
    fn test_end_of_batch() {
        let eob = FileMetadata::end_of_batch();
        assert_eq!(eob.len(), 128);
        assert!(eob.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_round_trip() {
        let time = SystemTime::now();
        let original = FileMetadata::new("myfile.dat")
            .size(98765)
            .mod_time(time)
            .mode(0o755)
            .serial(123);

        let encoded = original.encode();
        let decoded = FileMetadata::decode(&encoded).unwrap().unwrap();

        assert_eq!(decoded.name, original.name);
        assert_eq!(decoded.size, original.size);
        // Time may differ slightly due to precision loss in encoding
        assert!(decoded.mod_time.is_some());
        assert_eq!(decoded.mode, original.mode);
        assert_eq!(decoded.serial, original.serial);
    }

    #[test]
    fn test_long_filename() {
        let long_name = "a".repeat(100);
        let metadata = FileMetadata::with_size(&long_name, 12345);
        let encoded = metadata.encode();

        // Should still be 128 bytes
        assert_eq!(encoded.len(), 128);

        // Decode should work (filename will be truncated)
        let decoded = FileMetadata::decode(&encoded).unwrap().unwrap();
        assert!(!decoded.name.is_empty());
    }
}
