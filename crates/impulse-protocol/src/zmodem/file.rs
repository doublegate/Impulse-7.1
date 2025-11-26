//! File metadata handling for Zmodem protocol.
//!
//! This module handles ZFILE frames that contain file information including
//! name, size, modification time, and other metadata.

use super::error::{Result, ZmodemError};
use super::frame::{FrameEncoding, FrameType, ZmodemFrame};
use std::str;

/// Zmodem file information.
///
/// Contains metadata about a file being transferred via Zmodem protocol.
/// This information is sent in the ZFILE frame before file data transfer.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::file::ZmodemFileInfo;
///
/// let file_info = ZmodemFileInfo::new("test.txt", 1024)
///     .with_modification_time(1234567890);
///
/// assert_eq!(file_info.name, "test.txt");
/// assert_eq!(file_info.size, 1024);
/// assert_eq!(file_info.modified_time, Some(1234567890));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZmodemFileInfo {
    /// File name (without path)
    pub name: String,

    /// File size in bytes
    pub size: u64,

    /// Unix timestamp of file modification time
    pub modified_time: Option<u32>,

    /// Unix file mode (permissions)
    pub mode: Option<u32>,

    /// Serial number for resume capability
    pub serial_number: Option<u32>,

    /// Number of files remaining in batch
    pub files_remaining: Option<u32>,

    /// Total bytes remaining in batch
    pub bytes_remaining: Option<u64>,
}

impl ZmodemFileInfo {
    /// Create new file info with required fields.
    ///
    /// # Arguments
    ///
    /// * `name` - File name (without path)
    /// * `size` - File size in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_protocol::zmodem::file::ZmodemFileInfo;
    ///
    /// let info = ZmodemFileInfo::new("document.pdf", 524288);
    /// assert_eq!(info.name, "document.pdf");
    /// assert_eq!(info.size, 524288);
    /// ```
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
            modified_time: None,
            mode: None,
            serial_number: None,
            files_remaining: None,
            bytes_remaining: None,
        }
    }

    /// Set modification time (Unix timestamp).
    pub fn with_modification_time(mut self, time: u32) -> Self {
        self.modified_time = Some(time);
        self
    }

    /// Set file mode (Unix permissions).
    pub fn with_mode(mut self, mode: u32) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set serial number for resume.
    pub fn with_serial_number(mut self, serial: u32) -> Self {
        self.serial_number = Some(serial);
        self
    }

    /// Set number of files remaining in batch.
    pub fn with_files_remaining(mut self, count: u32) -> Self {
        self.files_remaining = Some(count);
        self
    }

    /// Set bytes remaining in batch.
    pub fn with_bytes_remaining(mut self, bytes: u64) -> Self {
        self.bytes_remaining = Some(bytes);
        self
    }

    /// Convert to ZFILE frame.
    ///
    /// Creates a ZFILE frame containing the file information.
    /// The file metadata is encoded in the data payload.
    ///
    /// # Returns
    ///
    /// ZFILE frame ready to be serialized and sent
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_protocol::zmodem::file::ZmodemFileInfo;
    /// use impulse_protocol::zmodem::FrameType;
    ///
    /// let info = ZmodemFileInfo::new("test.txt", 100);
    /// let frame = info.to_zfile_frame();
    ///
    /// assert_eq!(frame.frame_type, FrameType::ZFILE);
    /// assert!(frame.data.is_some());
    /// ```
    pub fn to_zfile_frame(&self) -> ZmodemFrame {
        let data = self.serialize();

        // Flags are typically zero for ZFILE
        let flags = [0u8; 4];

        ZmodemFrame::new(FrameType::ZFILE, FrameEncoding::Bin32, flags, Some(data))
    }

    /// Parse ZFILE frame to extract file information.
    ///
    /// # Arguments
    ///
    /// * `frame` - ZFILE frame to parse
    ///
    /// # Returns
    ///
    /// File information extracted from frame
    ///
    /// # Errors
    ///
    /// Returns error if frame has no data or data is malformed
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_protocol::zmodem::file::ZmodemFileInfo;
    ///
    /// let info = ZmodemFileInfo::new("test.txt", 100);
    /// let frame = info.to_zfile_frame();
    /// let parsed = ZmodemFileInfo::from_zfile_frame(&frame).unwrap();
    ///
    /// assert_eq!(parsed.name, info.name);
    /// assert_eq!(parsed.size, info.size);
    /// ```
    pub fn from_zfile_frame(frame: &ZmodemFrame) -> Result<Self> {
        let data = frame
            .data
            .as_ref()
            .ok_or_else(|| ZmodemError::InvalidFrame("ZFILE frame has no data".to_string()))?;

        Self::parse(data)
    }

    /// Serialize file info to Zmodem format.
    ///
    /// Format: "filename\0size mtime mode serial files bytes\0"
    /// All numeric fields are octal except size which is decimal.
    ///
    /// # Returns
    ///
    /// Serialized file information as bytes
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // File name + null terminator
        result.extend_from_slice(self.name.as_bytes());
        result.push(0);

        // Build info string: size mtime mode serial files bytes
        let mut info_parts = Vec::new();

        // Size (decimal)
        info_parts.push(format!("{}", self.size));

        // Modification time (octal)
        if let Some(mtime) = self.modified_time {
            info_parts.push(format!("{:o}", mtime));
        } else {
            info_parts.push("0".to_string());
        }

        // Mode (octal)
        if let Some(mode) = self.mode {
            info_parts.push(format!("{:o}", mode));
        } else {
            info_parts.push("0".to_string());
        }

        // Serial number (decimal)
        if let Some(serial) = self.serial_number {
            info_parts.push(format!("{}", serial));
        } else {
            info_parts.push("0".to_string());
        }

        // Files remaining (decimal)
        if let Some(files) = self.files_remaining {
            info_parts.push(format!("{}", files));
        }

        // Bytes remaining (decimal)
        if let Some(bytes) = self.bytes_remaining {
            info_parts.push(format!("{}", bytes));
        }

        // Join with spaces
        let info_string = info_parts.join(" ");
        result.extend_from_slice(info_string.as_bytes());

        // Null terminator
        result.push(0);

        result
    }

    /// Parse file info from ZFILE subpacket data.
    ///
    /// This is an alias for [`parse`] for clearer semantics when
    /// parsing data received after a ZFILE frame header.
    ///
    /// # Arguments
    ///
    /// * `data` - Subpacket data following ZFILE frame header
    ///
    /// # Returns
    ///
    /// Parsed file information
    ///
    /// # Errors
    ///
    /// Returns error if data is malformed
    pub fn from_zfile_data(data: &[u8]) -> Result<Self> {
        Self::parse(data)
    }

    /// Parse file info from Zmodem format.
    ///
    /// # Arguments
    ///
    /// * `data` - Serialized file info data
    ///
    /// # Returns
    ///
    /// Parsed file information
    ///
    /// # Errors
    ///
    /// Returns error if data is malformed
    pub fn parse(data: &[u8]) -> Result<Self> {
        // Find first null terminator (end of filename)
        let name_end = data
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| ZmodemError::InvalidFrame("No filename terminator".to_string()))?;

        // Extract filename
        let name = str::from_utf8(&data[..name_end])
            .map_err(|_| ZmodemError::InvalidFrame("Invalid UTF-8 in filename".to_string()))?
            .to_string();

        // Find second null terminator (end of info string)
        let info_start = name_end + 1;
        let info_end = data[info_start..]
            .iter()
            .position(|&b| b == 0)
            .map(|pos| info_start + pos)
            .unwrap_or(data.len());

        // Extract info string
        let info_str = str::from_utf8(&data[info_start..info_end])
            .map_err(|_| ZmodemError::InvalidFrame("Invalid UTF-8 in info string".to_string()))?;

        // Parse info fields: size mtime mode serial files bytes
        let parts: Vec<&str> = info_str.split_whitespace().collect();

        if parts.is_empty() {
            return Err(ZmodemError::InvalidFrame("Missing size field".to_string()));
        }

        // Parse size (decimal)
        let size = parts[0]
            .parse::<u64>()
            .map_err(|_| ZmodemError::InvalidFrame("Invalid size".to_string()))?;

        // Parse optional fields
        let modified_time = if parts.len() > 1 {
            u32::from_str_radix(parts[1], 8).ok()
        } else {
            None
        };

        let mode = if parts.len() > 2 {
            u32::from_str_radix(parts[2], 8).ok()
        } else {
            None
        };

        let serial_number = if parts.len() > 3 {
            parts[3].parse::<u32>().ok().filter(|&s| s != 0)
        } else {
            None
        };

        let files_remaining = if parts.len() > 4 {
            parts[4].parse::<u32>().ok()
        } else {
            None
        };

        let bytes_remaining = if parts.len() > 5 {
            parts[5].parse::<u64>().ok()
        } else {
            None
        };

        Ok(Self {
            name,
            size,
            modified_time,
            mode,
            serial_number,
            files_remaining,
            bytes_remaining,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = ZmodemFileInfo::new("test.txt", 1024);
        assert_eq!(info.name, "test.txt");
        assert_eq!(info.size, 1024);
        assert!(info.modified_time.is_none());
        assert!(info.mode.is_none());
    }

    #[test]
    fn test_with_modification_time() {
        let info = ZmodemFileInfo::new("test.txt", 1024).with_modification_time(1234567890);

        assert_eq!(info.modified_time, Some(1234567890));
    }

    #[test]
    fn test_with_mode() {
        let info = ZmodemFileInfo::new("test.txt", 1024).with_mode(0o644);

        assert_eq!(info.mode, Some(0o644));
    }

    #[test]
    fn test_with_serial_number() {
        let info = ZmodemFileInfo::new("test.txt", 1024).with_serial_number(42);

        assert_eq!(info.serial_number, Some(42));
    }

    #[test]
    fn test_with_files_remaining() {
        let info = ZmodemFileInfo::new("test.txt", 1024).with_files_remaining(5);

        assert_eq!(info.files_remaining, Some(5));
    }

    #[test]
    fn test_with_bytes_remaining() {
        let info = ZmodemFileInfo::new("test.txt", 1024).with_bytes_remaining(10240);

        assert_eq!(info.bytes_remaining, Some(10240));
    }

    #[test]
    fn test_serialize_minimal() {
        let info = ZmodemFileInfo::new("test.txt", 1024);
        let serialized = info.serialize();

        // Should contain: "test.txt\0" + "1024 0 0 0\0"
        assert!(serialized.starts_with(b"test.txt\0"));
        assert!(serialized.ends_with(&[0]));
    }

    #[test]
    fn test_serialize_full() {
        let info = ZmodemFileInfo::new("document.pdf", 524288)
            .with_modification_time(1234567890)
            .with_mode(0o644)
            .with_serial_number(123)
            .with_files_remaining(3)
            .with_bytes_remaining(1048576);

        let serialized = info.serialize();

        assert!(serialized.starts_with(b"document.pdf\0"));
        // Check that size "524288" appears in the serialized data
        let serialized_str = String::from_utf8_lossy(&serialized);
        assert!(serialized_str.contains("524288"));
    }

    #[test]
    fn test_parse_minimal() {
        let data = b"test.txt\x001024 0 0 0\0";
        let info = ZmodemFileInfo::parse(data).unwrap();

        assert_eq!(info.name, "test.txt");
        assert_eq!(info.size, 1024);
    }

    #[test]
    fn test_parse_full() {
        // mtime: 1234567890 in octal = 11145401322
        // mode: 0o644 = 644 (octal)
        let data = b"document.pdf\x00524288 11145401322 644 123 3 1048576\0";
        let info = ZmodemFileInfo::parse(data).unwrap();

        assert_eq!(info.name, "document.pdf");
        assert_eq!(info.size, 524288);
        assert_eq!(info.modified_time, Some(1234567890));
        assert_eq!(info.mode, Some(0o644));
        assert_eq!(info.serial_number, Some(123));
        assert_eq!(info.files_remaining, Some(3));
        assert_eq!(info.bytes_remaining, Some(1048576));
    }

    #[test]
    fn test_roundtrip_minimal() {
        let original = ZmodemFileInfo::new("test.txt", 2048);
        let serialized = original.serialize();
        let parsed = ZmodemFileInfo::parse(&serialized).unwrap();

        assert_eq!(parsed.name, original.name);
        assert_eq!(parsed.size, original.size);
    }

    #[test]
    fn test_roundtrip_full() {
        let original = ZmodemFileInfo::new("archive.zip", 1048576)
            .with_modification_time(1609459200)
            .with_mode(0o755)
            .with_serial_number(999)
            .with_files_remaining(10)
            .with_bytes_remaining(5242880);

        let serialized = original.serialize();
        let parsed = ZmodemFileInfo::parse(&serialized).unwrap();

        assert_eq!(parsed, original);
    }

    #[test]
    fn test_to_zfile_frame() {
        let info = ZmodemFileInfo::new("test.txt", 100);
        let frame = info.to_zfile_frame();

        assert_eq!(frame.frame_type, FrameType::ZFILE);
        assert_eq!(frame.encoding, FrameEncoding::Bin32);
        assert!(frame.data.is_some());
    }

    #[test]
    fn test_from_zfile_frame() {
        let info = ZmodemFileInfo::new("test.txt", 200);
        let frame = info.to_zfile_frame();
        let parsed = ZmodemFileInfo::from_zfile_frame(&frame).unwrap();

        assert_eq!(parsed.name, info.name);
        assert_eq!(parsed.size, info.size);
    }

    #[test]
    fn test_frame_roundtrip() {
        let original = ZmodemFileInfo::new("data.bin", 4096)
            .with_modification_time(1600000000)
            .with_mode(0o600);

        let frame = original.to_zfile_frame();
        let parsed = ZmodemFileInfo::from_zfile_frame(&frame).unwrap();

        assert_eq!(parsed, original);
    }

    #[test]
    fn test_parse_no_filename_terminator() {
        let data = b"test.txt1024 0 0 0\0";
        let result = ZmodemFileInfo::parse(data);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_size() {
        let data = b"test.txt\0invalid 0 0 0\0";
        let result = ZmodemFileInfo::parse(data);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_size() {
        let data = b"test.txt\0\0";
        let result = ZmodemFileInfo::parse(data);

        assert!(result.is_err());
    }

    #[test]
    fn test_special_characters_in_filename() {
        let info = ZmodemFileInfo::new("test-file_2024.txt", 512);
        let serialized = info.serialize();
        let parsed = ZmodemFileInfo::parse(&serialized).unwrap();

        assert_eq!(parsed.name, info.name);
    }

    #[test]
    fn test_large_file_size() {
        let info = ZmodemFileInfo::new("huge.dat", u64::MAX);
        let serialized = info.serialize();
        let parsed = ZmodemFileInfo::parse(&serialized).unwrap();

        assert_eq!(parsed.size, u64::MAX);
    }

    #[test]
    fn test_zero_size() {
        let info = ZmodemFileInfo::new("empty.txt", 0);
        let serialized = info.serialize();
        let parsed = ZmodemFileInfo::parse(&serialized).unwrap();

        assert_eq!(parsed.size, 0);
    }

    #[test]
    fn test_octal_mode_parsing() {
        // Mode 0o755 = 493 decimal, "755" octal string
        let data = b"script.sh\x001024 0 755 0\0";
        let info = ZmodemFileInfo::parse(data).unwrap();

        assert_eq!(info.mode, Some(0o755));
    }
}
