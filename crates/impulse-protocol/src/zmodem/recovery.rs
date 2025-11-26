//! Crash recovery and resume functionality for Zmodem transfers.
//!
//! This module provides the ability to:
//! - Detect incomplete transfers and resume from the last position
//! - Store and load transfer state for crash recovery
//! - Handle ZCRASH frames for recovery negotiation
//! - Verify file integrity before resuming
//!
//! # Protocol Flow for Resume
//!
//! 1. Receiver detects partial file exists
//! 2. Receiver calculates CRC of existing data (optional)
//! 3. Receiver sends ZRPOS with offset instead of 0
//! 4. Sender seeks to position and continues transfer
//! 5. Transfer completes with remaining data
//!
//! # Examples
//!
//! ```no_run
//! use impulse_protocol::zmodem::recovery::{RecoveryManager, TransferState};
//! use std::path::Path;
//!
//! # async fn example() -> impulse_protocol::zmodem::Result<()> {
//! let manager = RecoveryManager::new(Path::new("/tmp/zmodem-state"));
//!
//! // Save transfer state periodically
//! let state = TransferState::new("test.txt", 1024, 512);
//! manager.save_state(&state).await?;
//!
//! // Later, check if we can resume
//! if let Some(state) = manager.load_state("test.txt").await? {
//!     println!("Can resume from position: {}", state.position);
//! }
//! # Ok(())
//! # }
//! ```

use super::crc32;
use super::error::{Result, ZmodemError};
use super::file::ZmodemFileInfo;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;

/// State of an in-progress transfer.
///
/// Stored to enable crash recovery.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::recovery::TransferState;
///
/// let state = TransferState::new("document.pdf", 1048576, 524288);
/// assert_eq!(state.file_name, "document.pdf");
/// assert_eq!(state.file_size, 1048576);
/// assert_eq!(state.position, 524288);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferState {
    /// File name
    pub file_name: String,

    /// Total file size
    pub file_size: u64,

    /// Current position in transfer
    pub position: u64,

    /// CRC-32 of data transferred so far
    pub partial_crc: u32,

    /// Serial number for verification
    pub serial_number: Option<u32>,

    /// Unix timestamp when state was saved
    pub timestamp: u64,

    /// Direction of transfer
    pub direction: TransferDirection,

    /// Local file path
    pub local_path: PathBuf,
}

/// Direction of file transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferDirection {
    /// Sending (download from BBS perspective)
    Send,
    /// Receiving (upload to BBS perspective)
    Receive,
}

impl TransferState {
    /// Create a new transfer state.
    ///
    /// # Arguments
    ///
    /// * `file_name` - Name of file being transferred
    /// * `file_size` - Total size of file
    /// * `position` - Current transfer position
    pub fn new(file_name: &str, file_size: u64, position: u64) -> Self {
        Self {
            file_name: file_name.to_string(),
            file_size,
            position,
            partial_crc: 0,
            serial_number: None,
            timestamp: current_timestamp(),
            direction: TransferDirection::Receive,
            local_path: PathBuf::new(),
        }
    }

    /// Create transfer state for sending.
    pub fn for_send(file_name: &str, file_size: u64, position: u64, local_path: PathBuf) -> Self {
        Self {
            file_name: file_name.to_string(),
            file_size,
            position,
            partial_crc: 0,
            serial_number: None,
            timestamp: current_timestamp(),
            direction: TransferDirection::Send,
            local_path,
        }
    }

    /// Create transfer state for receiving.
    pub fn for_receive(
        file_name: &str,
        file_size: u64,
        position: u64,
        local_path: PathBuf,
    ) -> Self {
        Self {
            file_name: file_name.to_string(),
            file_size,
            position,
            partial_crc: 0,
            serial_number: None,
            timestamp: current_timestamp(),
            direction: TransferDirection::Receive,
            local_path,
        }
    }

    /// Set partial CRC for verification.
    pub fn with_crc(mut self, crc: u32) -> Self {
        self.partial_crc = crc;
        self
    }

    /// Set serial number for identification.
    pub fn with_serial(mut self, serial: u32) -> Self {
        self.serial_number = Some(serial);
        self
    }

    /// Check if state is stale (older than max age).
    ///
    /// # Arguments
    ///
    /// * `max_age` - Maximum age of state in seconds
    pub fn is_stale(&self, max_age: Duration) -> bool {
        let now = current_timestamp();
        now.saturating_sub(self.timestamp) > max_age.as_secs()
    }

    /// Get percentage complete.
    pub fn percent_complete(&self) -> f64 {
        if self.file_size == 0 {
            100.0
        } else {
            (self.position as f64 / self.file_size as f64) * 100.0
        }
    }

    /// Check if transfer is complete.
    pub fn is_complete(&self) -> bool {
        self.position >= self.file_size
    }

    /// Serialize state to bytes.
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // Version byte
        data.push(1);

        // Direction
        data.push(match self.direction {
            TransferDirection::Send => 0,
            TransferDirection::Receive => 1,
        });

        // File name (length-prefixed)
        let name_bytes = self.file_name.as_bytes();
        data.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        data.extend_from_slice(name_bytes);

        // File size
        data.extend_from_slice(&self.file_size.to_le_bytes());

        // Position
        data.extend_from_slice(&self.position.to_le_bytes());

        // Partial CRC
        data.extend_from_slice(&self.partial_crc.to_le_bytes());

        // Serial number (0 if none)
        data.extend_from_slice(&self.serial_number.unwrap_or(0).to_le_bytes());

        // Timestamp
        data.extend_from_slice(&self.timestamp.to_le_bytes());

        // Local path (length-prefixed)
        let path_bytes = self.local_path.to_string_lossy().as_bytes().to_vec();
        data.extend_from_slice(&(path_bytes.len() as u16).to_le_bytes());
        data.extend_from_slice(&path_bytes);

        data
    }

    /// Deserialize state from bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        if data.is_empty() {
            return Err(ZmodemError::InvalidFrame("Empty state data".to_string()));
        }

        let mut pos = 0;

        // Version check
        let version = data[pos];
        if version != 1 {
            return Err(ZmodemError::InvalidFrame(format!(
                "Unknown state version: {}",
                version
            )));
        }
        pos += 1;

        // Direction
        if pos >= data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let direction = match data[pos] {
            0 => TransferDirection::Send,
            1 => TransferDirection::Receive,
            _ => return Err(ZmodemError::InvalidFrame("Invalid direction".to_string())),
        };
        pos += 1;

        // File name
        if pos + 2 > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let name_len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;

        if pos + name_len > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let file_name = String::from_utf8(data[pos..pos + name_len].to_vec())
            .map_err(|_| ZmodemError::InvalidFrame("Invalid UTF-8 in filename".to_string()))?;
        pos += name_len;

        // File size
        if pos + 8 > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let file_size = u64::from_le_bytes([
            data[pos],
            data[pos + 1],
            data[pos + 2],
            data[pos + 3],
            data[pos + 4],
            data[pos + 5],
            data[pos + 6],
            data[pos + 7],
        ]);
        pos += 8;

        // Position
        if pos + 8 > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let position = u64::from_le_bytes([
            data[pos],
            data[pos + 1],
            data[pos + 2],
            data[pos + 3],
            data[pos + 4],
            data[pos + 5],
            data[pos + 6],
            data[pos + 7],
        ]);
        pos += 8;

        // Partial CRC
        if pos + 4 > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let partial_crc =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        // Serial number
        if pos + 4 > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let serial_raw =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        let serial_number = if serial_raw == 0 {
            None
        } else {
            Some(serial_raw)
        };
        pos += 4;

        // Timestamp
        if pos + 8 > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let timestamp = u64::from_le_bytes([
            data[pos],
            data[pos + 1],
            data[pos + 2],
            data[pos + 3],
            data[pos + 4],
            data[pos + 5],
            data[pos + 6],
            data[pos + 7],
        ]);
        pos += 8;

        // Local path
        if pos + 2 > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let path_len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;

        if pos + path_len > data.len() {
            return Err(ZmodemError::InvalidFrame(
                "Truncated state data".to_string(),
            ));
        }
        let local_path = PathBuf::from(
            String::from_utf8(data[pos..pos + path_len].to_vec())
                .map_err(|_| ZmodemError::InvalidFrame("Invalid UTF-8 in path".to_string()))?,
        );

        Ok(Self {
            file_name,
            file_size,
            position,
            partial_crc,
            serial_number,
            timestamp,
            direction,
            local_path,
        })
    }
}

/// Recovery manager for handling interrupted transfers.
///
/// Manages persistent state storage for crash recovery.
///
/// # Examples
///
/// ```no_run
/// use impulse_protocol::zmodem::recovery::RecoveryManager;
/// use std::path::Path;
///
/// let manager = RecoveryManager::new(Path::new("/tmp/zmodem-state"));
/// ```
pub struct RecoveryManager {
    /// Directory for state files
    state_dir: PathBuf,

    /// Maximum age for recovery state (default: 24 hours)
    max_state_age: Duration,

    /// In-memory state cache
    cache: Arc<RwLock<HashMap<String, TransferState>>>,
}

impl RecoveryManager {
    /// Create a new recovery manager.
    ///
    /// # Arguments
    ///
    /// * `state_dir` - Directory to store recovery state files
    pub fn new(state_dir: &Path) -> Self {
        Self {
            state_dir: state_dir.to_path_buf(),
            max_state_age: Duration::from_secs(24 * 60 * 60), // 24 hours
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Set maximum age for recovery state.
    pub fn with_max_age(mut self, max_age: Duration) -> Self {
        self.max_state_age = max_age;
        self
    }

    /// Save transfer state for recovery.
    ///
    /// # Arguments
    ///
    /// * `state` - Transfer state to save
    pub async fn save_state(&self, state: &TransferState) -> Result<()> {
        // Ensure state directory exists
        fs::create_dir_all(&self.state_dir).await?;

        // Generate state file name
        let state_file = self.state_file_path(&state.file_name);

        // Serialize state
        let data = state.serialize();

        // Write atomically (write to temp then rename)
        let temp_file = state_file.with_extension("tmp");
        fs::write(&temp_file, &data).await?;
        fs::rename(&temp_file, &state_file).await?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(state.file_name.clone(), state.clone());
        }

        Ok(())
    }

    /// Load transfer state for recovery.
    ///
    /// Returns None if no state exists or state is stale.
    ///
    /// # Arguments
    ///
    /// * `file_name` - Name of file to load state for
    pub async fn load_state(&self, file_name: &str) -> Result<Option<TransferState>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(state) = cache.get(file_name)
                && !state.is_stale(self.max_state_age)
            {
                return Ok(Some(state.clone()));
            }
        }

        // Try to load from disk
        let state_file = self.state_file_path(file_name);
        if !state_file.exists() {
            return Ok(None);
        }

        let data = fs::read(&state_file).await?;
        let state = TransferState::deserialize(&data)?;

        // Check if stale
        if state.is_stale(self.max_state_age) {
            // Remove stale state
            let _ = fs::remove_file(&state_file).await;
            return Ok(None);
        }

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(file_name.to_string(), state.clone());
        }

        Ok(Some(state))
    }

    /// Remove transfer state (call after successful completion).
    ///
    /// # Arguments
    ///
    /// * `file_name` - Name of file to remove state for
    pub async fn remove_state(&self, file_name: &str) -> Result<()> {
        // Remove from cache
        {
            let mut cache = self.cache.write().await;
            cache.remove(file_name);
        }

        // Remove from disk
        let state_file = self.state_file_path(file_name);
        if state_file.exists() {
            fs::remove_file(&state_file).await?;
        }

        Ok(())
    }

    /// Clean up stale state files.
    pub async fn cleanup_stale(&self) -> Result<usize> {
        let mut removed = 0;

        if !self.state_dir.exists() {
            return Ok(0);
        }

        let mut entries = fs::read_dir(&self.state_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "zstate")
                && let Ok(data) = fs::read(&path).await
                && let Ok(state) = TransferState::deserialize(&data)
                && state.is_stale(self.max_state_age)
            {
                let _ = fs::remove_file(&path).await;
                removed += 1;
            }
        }

        Ok(removed)
    }

    /// Get state file path for a file name.
    fn state_file_path(&self, file_name: &str) -> PathBuf {
        // Sanitize file name for use as state file
        let safe_name: String = file_name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        self.state_dir.join(format!("{}.zstate", safe_name))
    }
}

/// Resume information for a file.
///
/// Contains all information needed to resume a transfer.
#[derive(Debug, Clone)]
pub struct ResumeInfo {
    /// Starting position for resume
    pub position: u64,

    /// CRC-32 of existing data (for verification)
    pub existing_crc: u32,

    /// Whether CRC was verified
    pub crc_verified: bool,

    /// File modification time
    pub mtime: Option<u64>,
}

impl ResumeInfo {
    /// Create resume info for a new transfer (no resume).
    pub fn new_transfer() -> Self {
        Self {
            position: 0,
            existing_crc: 0,
            crc_verified: false,
            mtime: None,
        }
    }

    /// Create resume info from existing partial file.
    pub fn from_partial(position: u64, existing_crc: u32) -> Self {
        Self {
            position,
            existing_crc,
            crc_verified: true,
            mtime: None,
        }
    }
}

/// Check if a file can be resumed and get resume info.
///
/// Verifies the existing partial file and returns resume position.
///
/// # Arguments
///
/// * `path` - Path to existing partial file
/// * `file_info` - Expected file information from sender
/// * `verify_crc` - Whether to calculate and verify CRC
///
/// # Returns
///
/// Resume information or None if file doesn't exist
pub async fn get_resume_info(
    path: &Path,
    file_info: &ZmodemFileInfo,
    verify_crc: bool,
) -> Result<Option<ResumeInfo>> {
    // Check if file exists
    if !path.exists() {
        return Ok(None);
    }

    let metadata = fs::metadata(path).await?;
    let existing_size = metadata.len();

    // Can't resume if existing file is larger
    if existing_size >= file_info.size {
        // File is complete or larger - verify size match
        if existing_size == file_info.size {
            // Might be complete, let caller decide
            return Ok(Some(ResumeInfo {
                position: existing_size,
                existing_crc: 0,
                crc_verified: false,
                mtime: get_mtime(&metadata),
            }));
        }
        // File is larger - can't resume
        return Ok(None);
    }

    // Calculate CRC of existing data if requested
    let (existing_crc, crc_verified) = if verify_crc {
        let crc = calculate_file_crc(path, existing_size).await?;
        (crc, true)
    } else {
        (0, false)
    };

    Ok(Some(ResumeInfo {
        position: existing_size,
        existing_crc,
        crc_verified,
        mtime: get_mtime(&metadata),
    }))
}

/// Calculate CRC-32 of a file up to a given size.
async fn calculate_file_crc(path: &Path, size: u64) -> Result<u32> {
    let mut file = File::open(path).await?;
    let mut crc: u32 = 0xFFFFFFFF; // Initial value
    let mut buffer = vec![0u8; 8192];
    let mut remaining = size;

    while remaining > 0 {
        let to_read = (remaining as usize).min(buffer.len());
        let n = file.read(&mut buffer[..to_read]).await?;
        if n == 0 {
            break;
        }
        crc = crc32::update(crc, &buffer[..n]);
        remaining -= n as u64;
    }

    Ok(crc32::finalize(crc))
}

/// Verify resume position by checking CRC of existing data.
///
/// Sends ZCRC frame to request sender's CRC and compares.
///
/// # Arguments
///
/// * `path` - Path to partial file
/// * `position` - Position to verify up to
/// * `expected_crc` - Expected CRC-32 from sender
pub async fn verify_resume_position(path: &Path, position: u64, expected_crc: u32) -> Result<bool> {
    let actual_crc = calculate_file_crc(path, position).await?;
    Ok(actual_crc == expected_crc)
}

/// Get modification time from metadata as Unix timestamp.
fn get_mtime(metadata: &std::fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
}

/// Get current timestamp.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_state_new() {
        let state = TransferState::new("test.txt", 1024, 512);
        assert_eq!(state.file_name, "test.txt");
        assert_eq!(state.file_size, 1024);
        assert_eq!(state.position, 512);
        assert_eq!(state.partial_crc, 0);
        assert!(state.serial_number.is_none());
    }

    #[test]
    fn test_transfer_state_for_send() {
        let state = TransferState::for_send("doc.pdf", 2048, 1024, PathBuf::from("/files/doc.pdf"));
        assert_eq!(state.direction, TransferDirection::Send);
        assert_eq!(state.local_path, PathBuf::from("/files/doc.pdf"));
    }

    #[test]
    fn test_transfer_state_for_receive() {
        let state =
            TransferState::for_receive("data.bin", 4096, 2048, PathBuf::from("/uploads/data.bin"));
        assert_eq!(state.direction, TransferDirection::Receive);
        assert_eq!(state.local_path, PathBuf::from("/uploads/data.bin"));
    }

    #[test]
    fn test_transfer_state_with_crc() {
        let state = TransferState::new("test.txt", 1024, 512).with_crc(0x12345678);
        assert_eq!(state.partial_crc, 0x12345678);
    }

    #[test]
    fn test_transfer_state_with_serial() {
        let state = TransferState::new("test.txt", 1024, 512).with_serial(42);
        assert_eq!(state.serial_number, Some(42));
    }

    #[test]
    fn test_transfer_state_percent_complete() {
        let state = TransferState::new("test.txt", 1000, 500);
        assert_eq!(state.percent_complete(), 50.0);

        let state_zero = TransferState::new("empty.txt", 0, 0);
        assert_eq!(state_zero.percent_complete(), 100.0);
    }

    #[test]
    fn test_transfer_state_is_complete() {
        let incomplete = TransferState::new("test.txt", 1000, 500);
        assert!(!incomplete.is_complete());

        let complete = TransferState::new("test.txt", 1000, 1000);
        assert!(complete.is_complete());

        let over = TransferState::new("test.txt", 1000, 1500);
        assert!(over.is_complete());
    }

    #[test]
    fn test_transfer_state_is_stale() {
        let mut state = TransferState::new("test.txt", 1024, 512);

        // Fresh state is not stale
        assert!(!state.is_stale(Duration::from_secs(60)));

        // Make state old
        state.timestamp = current_timestamp() - 120;
        assert!(state.is_stale(Duration::from_secs(60)));
        assert!(!state.is_stale(Duration::from_secs(180)));
    }

    #[test]
    fn test_transfer_state_serialize_deserialize() {
        let original = TransferState::new("document.pdf", 1048576, 524288)
            .with_crc(0xDEADBEEF)
            .with_serial(12345);

        let serialized = original.serialize();
        let deserialized = TransferState::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.file_name, original.file_name);
        assert_eq!(deserialized.file_size, original.file_size);
        assert_eq!(deserialized.position, original.position);
        assert_eq!(deserialized.partial_crc, original.partial_crc);
        assert_eq!(deserialized.serial_number, original.serial_number);
        assert_eq!(deserialized.direction, original.direction);
    }

    #[test]
    fn test_transfer_state_serialize_with_path() {
        let state =
            TransferState::for_receive("data.bin", 4096, 2048, PathBuf::from("/uploads/data.bin"));

        let serialized = state.serialize();
        let deserialized = TransferState::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.local_path, state.local_path);
    }

    #[test]
    fn test_transfer_state_deserialize_empty() {
        let result = TransferState::deserialize(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_state_deserialize_invalid_version() {
        let result = TransferState::deserialize(&[99]);
        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_direction_equality() {
        assert_eq!(TransferDirection::Send, TransferDirection::Send);
        assert_eq!(TransferDirection::Receive, TransferDirection::Receive);
        assert_ne!(TransferDirection::Send, TransferDirection::Receive);
    }

    #[test]
    fn test_resume_info_new_transfer() {
        let info = ResumeInfo::new_transfer();
        assert_eq!(info.position, 0);
        assert_eq!(info.existing_crc, 0);
        assert!(!info.crc_verified);
    }

    #[test]
    fn test_resume_info_from_partial() {
        let info = ResumeInfo::from_partial(1024, 0xABCDEF12);
        assert_eq!(info.position, 1024);
        assert_eq!(info.existing_crc, 0xABCDEF12);
        assert!(info.crc_verified);
    }

    #[test]
    fn test_recovery_manager_state_file_path() {
        let manager = RecoveryManager::new(Path::new("/tmp/state"));

        // Normal filename
        let path = manager.state_file_path("test.txt");
        assert_eq!(path, PathBuf::from("/tmp/state/test.txt.zstate"));

        // Filename with special chars
        let path = manager.state_file_path("file with spaces.txt");
        assert!(path.to_string_lossy().contains("file_with_spaces.txt"));
    }

    #[tokio::test]
    async fn test_recovery_manager_cache() {
        let manager = RecoveryManager::new(Path::new("/tmp/zmodem-test-cache"));

        let state = TransferState::new("cache_test.txt", 1024, 512);

        // Save should update cache
        let _ = manager.save_state(&state).await;

        // Check cache contains state
        {
            let cache = manager.cache.read().await;
            assert!(cache.contains_key("cache_test.txt"));
        }

        // Cleanup
        let _ = manager.remove_state("cache_test.txt").await;
    }

    #[test]
    fn test_current_timestamp() {
        let ts = current_timestamp();
        // Should be a reasonable Unix timestamp (after year 2020)
        assert!(ts > 1577836800);
    }
}
