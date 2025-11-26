//! Zmodem file sender implementation.
//!
//! This module implements the sender (download) side of the Zmodem protocol,
//! allowing files to be sent from the BBS to a remote client.
//!
//! # Protocol Flow
//!
//! 1. Initialize session with ZRQINIT/ZRINIT exchange
//! 2. Send file header (ZFILE) with metadata
//! 3. Wait for ZRPOS (position) or ZSKIP (skip file)
//! 4. Send file data in blocks (ZDATA frames)
//! 5. Send EOF (ZEOF) and wait for acknowledgment
//! 6. Repeat for additional files or finish session
//!
//! # Examples
//!
//! ```no_run
//! use impulse_protocol::zmodem::send::{ZmodemSender, SenderConfig};
//! use std::path::Path;
//!
//! # async fn example() -> impulse_protocol::zmodem::Result<()> {
//! // Create sender with TCP stream (or any AsyncRead + AsyncWrite)
//! let stream = tokio::net::TcpStream::connect("127.0.0.1:2323").await?;
//! let config = SenderConfig::default();
//! let mut sender = ZmodemSender::new(stream, config);
//!
//! // Initialize session
//! sender.init().await?;
//!
//! // Send a file
//! let path = Path::new("test.txt");
//! let stats = sender.send_file(path).await?;
//!
//! println!("Sent {} bytes in {:?}", stats.bytes_sent, stats.elapsed());
//!
//! // Finish session
//! sender.finish().await?;
//! # Ok(())
//! # }
//! ```

use super::error::{Result, ZmodemError};
use super::escape::{self, ZCRCG, ZCRCQ, ZCRCW, ZDLE};
use super::file::ZmodemFileInfo;
use super::frame::{FrameEncoding, FrameType, ZmodemFrame};
use super::init::ZmodemInit;
use super::negotiate::{CrcType, NegotiatedParams};
use super::parser::FrameParser;
use super::state::{ZmodemState, ZmodemStateMachine};
use super::{crc16, crc32};
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeekExt, AsyncWrite, AsyncWriteExt};
use tokio::time::timeout;

/// Configuration for Zmodem sender.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::send::SenderConfig;
///
/// let config = SenderConfig {
///     block_size: 1024,
///     timeout_ms: 10000,
///     max_retries: 5,
///     use_crc32: true,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct SenderConfig {
    /// Block size for data transfer (default: 1024 bytes)
    pub block_size: usize,

    /// Timeout in milliseconds for responses (default: 10000ms)
    pub timeout_ms: u64,

    /// Maximum retries before aborting (default: 10)
    pub max_retries: u32,

    /// Use CRC-32 instead of CRC-16 (default: true)
    pub use_crc32: bool,

    /// Escape control characters (default: false)
    pub escape_control: bool,

    /// Escape 8-bit characters (default: false)
    pub escape_8bit: bool,
}

impl Default for SenderConfig {
    fn default() -> Self {
        Self {
            block_size: 1024,
            timeout_ms: 10000,
            max_retries: 10,
            use_crc32: true,
            escape_control: false,
            escape_8bit: false,
        }
    }
}

/// Transfer statistics for a single file.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::send::TransferStats;
///
/// let stats = TransferStats::new(1048576);
/// // ... transfer data ...
/// println!("Transfer rate: {} bytes/sec", stats.bytes_per_second());
/// ```
#[derive(Debug, Clone)]
pub struct TransferStats {
    /// Bytes successfully sent
    pub bytes_sent: u64,

    /// Total bytes to send
    pub bytes_total: u64,

    /// Transfer start time
    pub start_time: Instant,

    /// Transfer end time (if complete)
    pub end_time: Option<Instant>,

    /// Number of retransmissions
    pub retries: u32,
}

impl TransferStats {
    /// Create new transfer statistics.
    pub fn new(bytes_total: u64) -> Self {
        Self {
            bytes_sent: 0,
            bytes_total,
            start_time: Instant::now(),
            end_time: None,
            retries: 0,
        }
    }

    /// Get elapsed time.
    pub fn elapsed(&self) -> Duration {
        self.end_time
            .unwrap_or_else(Instant::now)
            .duration_since(self.start_time)
    }

    /// Get transfer rate in bytes per second.
    pub fn bytes_per_second(&self) -> f64 {
        let elapsed_secs = self.elapsed().as_secs_f64();
        if elapsed_secs > 0.0 {
            self.bytes_sent as f64 / elapsed_secs
        } else {
            0.0
        }
    }

    /// Get completion percentage.
    pub fn percent_complete(&self) -> f64 {
        if self.bytes_total == 0 {
            100.0
        } else {
            (self.bytes_sent as f64 / self.bytes_total as f64) * 100.0
        }
    }

    /// Mark transfer as complete.
    pub fn complete(&mut self) {
        self.end_time = Some(Instant::now());
    }
}

/// Zmodem file sender.
///
/// Implements the sender side of the Zmodem protocol for transmitting files
/// from a BBS to a remote client.
///
/// # Type Parameters
///
/// * `S` - Stream type that implements AsyncRead + AsyncWrite + Unpin
///
/// # Examples
///
/// ```no_run
/// use impulse_protocol::zmodem::send::{ZmodemSender, SenderConfig};
/// use tokio::net::TcpStream;
/// use std::path::Path;
///
/// # async fn example() -> impulse_protocol::zmodem::Result<()> {
/// let stream = TcpStream::connect("127.0.0.1:2323").await?;
/// let config = SenderConfig::default();
/// let mut sender = ZmodemSender::new(stream, config);
///
/// sender.init().await?;
/// let stats = sender.send_file(Path::new("file.txt")).await?;
/// sender.finish().await?;
/// # Ok(())
/// # }
/// ```
pub struct ZmodemSender<S> {
    stream: S,
    parser: FrameParser,
    state: ZmodemStateMachine,
    config: SenderConfig,
    negotiated: Option<NegotiatedParams>,
}

impl<S: AsyncRead + AsyncWrite + Unpin> ZmodemSender<S> {
    /// Create a new Zmodem sender.
    ///
    /// # Arguments
    ///
    /// * `stream` - Communication stream (must implement AsyncRead + AsyncWrite + Unpin)
    /// * `config` - Sender configuration
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use impulse_protocol::zmodem::send::{ZmodemSender, SenderConfig};
    /// use tokio::net::TcpStream;
    ///
    /// # async fn example() -> std::io::Result<()> {
    /// let stream = TcpStream::connect("127.0.0.1:2323").await?;
    /// let sender = ZmodemSender::new(stream, SenderConfig::default());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(stream: S, config: SenderConfig) -> Self {
        Self {
            stream,
            parser: FrameParser::new(),
            state: ZmodemStateMachine::new(),
            config,
            negotiated: None,
        }
    }

    /// Initialize Zmodem session.
    ///
    /// Sends ZRQINIT and waits for ZRINIT response to negotiate parameters.
    ///
    /// # Returns
    ///
    /// Negotiated session parameters
    ///
    /// # Errors
    ///
    /// Returns error if initialization fails or times out
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use impulse_protocol::zmodem::send::{ZmodemSender, SenderConfig};
    /// # use tokio::net::TcpStream;
    /// # async fn example() -> impulse_protocol::zmodem::Result<()> {
    /// # let stream = TcpStream::connect("127.0.0.1:2323").await?;
    /// let mut sender = ZmodemSender::new(stream, SenderConfig::default());
    /// let params = sender.init().await?;
    /// println!("Negotiated buffer size: {}", params.buffer_size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn init(&mut self) -> Result<NegotiatedParams> {
        // Create sender init parameters
        let sender_init = ZmodemInit {
            use_crc32: self.config.use_crc32,
            escape_ctrl: self.config.escape_control,
            escape_8bit: self.config.escape_8bit,
            buffer_size: self.config.block_size as u16,
        };

        // Send ZRQINIT
        let zrqinit = ZmodemInit::create_zrqinit();
        self.send_frame(&zrqinit).await?;
        self.state.advance(ZmodemState::InitSent);

        // Wait for ZRINIT response
        let zrinit = self
            .wait_for_frame_type(
                FrameType::ZRINIT,
                Duration::from_millis(self.config.timeout_ms),
            )
            .await?;

        // Parse receiver capabilities from ZRINIT
        let receiver_init = ZmodemInit::from_zrinit(&zrinit)?;

        // Negotiate parameters
        let params = super::negotiate::negotiate(&sender_init, &receiver_init);
        self.negotiated = Some(params.clone());
        self.state.set_negotiated(params.clone());
        self.state.advance(ZmodemState::InitReceived);

        Ok(params)
    }

    /// Send a single file.
    ///
    /// Transmits the specified file to the receiver, handling retransmissions
    /// and error recovery automatically.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to file to send
    ///
    /// # Returns
    ///
    /// Transfer statistics for the completed transfer
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be opened, transmission fails, or is cancelled
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use impulse_protocol::zmodem::send::{ZmodemSender, SenderConfig};
    /// # use tokio::net::TcpStream;
    /// # use std::path::Path;
    /// # async fn example() -> impulse_protocol::zmodem::Result<()> {
    /// # let stream = TcpStream::connect("127.0.0.1:2323").await?;
    /// # let mut sender = ZmodemSender::new(stream, SenderConfig::default());
    /// # sender.init().await?;
    /// let stats = sender.send_file(Path::new("document.pdf")).await?;
    /// println!("Sent {} bytes with {} retries", stats.bytes_sent, stats.retries);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_file(&mut self, path: &Path) -> Result<TransferStats> {
        // Open file
        let mut file = File::open(path).await?;
        let metadata = file.metadata().await?;
        let file_size = metadata.len();

        // Create file info
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| ZmodemError::InvalidFrame("Invalid filename".to_string()))?;

        let file_info = ZmodemFileInfo::new(file_name, file_size);

        // Initialize statistics
        let mut stats = TransferStats::new(file_size);

        // Send file header and wait for position
        let start_pos = self.send_file_header(&file_info).await?;

        // Seek to start position if resuming
        if start_pos > 0 {
            file.seek(SeekFrom::Start(start_pos)).await?;
            stats.bytes_sent = start_pos;
        }

        // Send file data
        self.send_file_data(&mut file, start_pos, &mut stats)
            .await?;

        // Send EOF
        self.send_eof(file_size, &mut stats).await?;

        stats.complete();
        Ok(stats)
    }

    /// Send multiple files in batch.
    ///
    /// Transmits multiple files in a single Zmodem session.
    ///
    /// # Arguments
    ///
    /// * `paths` - Paths to files to send
    ///
    /// # Returns
    ///
    /// Vector of transfer statistics (one per file)
    ///
    /// # Errors
    ///
    /// Returns error if any file fails to transfer
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use impulse_protocol::zmodem::send::{ZmodemSender, SenderConfig};
    /// # use tokio::net::TcpStream;
    /// # use std::path::PathBuf;
    /// # async fn example() -> impulse_protocol::zmodem::Result<()> {
    /// # let stream = TcpStream::connect("127.0.0.1:2323").await?;
    /// # let mut sender = ZmodemSender::new(stream, SenderConfig::default());
    /// # sender.init().await?;
    /// let files = vec![
    ///     PathBuf::from("file1.txt"),
    ///     PathBuf::from("file2.txt"),
    /// ];
    /// let results = sender.send_files(&files).await?;
    /// for stats in results {
    ///     println!("Sent {} bytes", stats.bytes_sent);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_files(&mut self, paths: &[PathBuf]) -> Result<Vec<TransferStats>> {
        let mut results = Vec::with_capacity(paths.len());

        for path in paths {
            let stats = self.send_file(path).await?;
            results.push(stats);
        }

        Ok(results)
    }

    /// Finish the Zmodem session.
    ///
    /// Sends ZFIN frame to signal end of session.
    ///
    /// # Errors
    ///
    /// Returns error if session termination fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use impulse_protocol::zmodem::send::{ZmodemSender, SenderConfig};
    /// # use tokio::net::TcpStream;
    /// # async fn example() -> impulse_protocol::zmodem::Result<()> {
    /// # let stream = TcpStream::connect("127.0.0.1:2323").await?;
    /// # let mut sender = ZmodemSender::new(stream, SenderConfig::default());
    /// # sender.init().await?;
    /// sender.finish().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn finish(&mut self) -> Result<()> {
        let zfin = ZmodemFrame::with_defaults(FrameType::ZFIN, self.frame_encoding());
        self.send_frame(&zfin).await?;
        self.state.advance(ZmodemState::SessionComplete);
        Ok(())
    }

    /// Send a frame to the stream.
    async fn send_frame(&mut self, frame: &ZmodemFrame) -> Result<()> {
        let serialized = frame.serialize();
        self.stream.write_all(&serialized).await?;
        self.stream.flush().await?;
        Ok(())
    }

    /// Wait for a specific frame type.
    async fn wait_for_frame_type(
        &mut self,
        expected_type: FrameType,
        timeout_duration: Duration,
    ) -> Result<ZmodemFrame> {
        let result = timeout(timeout_duration, async {
            loop {
                let frame = self.wait_for_frame(timeout_duration).await?;
                if frame.frame_type == expected_type {
                    return Ok(frame);
                }
                // Handle unexpected frames (ZCAN, ZABORT)
                if matches!(frame.frame_type, FrameType::ZCAN | FrameType::ZABORT) {
                    return Err(ZmodemError::Cancelled);
                }
            }
        })
        .await
        .map_err(|_| ZmodemError::Timeout)??;

        Ok(result)
    }

    /// Wait for any frame.
    async fn wait_for_frame(&mut self, timeout_duration: Duration) -> Result<ZmodemFrame> {
        let result = timeout(timeout_duration, async {
            let mut buf = vec![0u8; 256];
            loop {
                let n = self.stream.read(&mut buf).await?;
                if n == 0 {
                    return Err(ZmodemError::UnexpectedEof);
                }

                let frames = self.parser.feed(&buf[..n]);
                if let Some(frame_result) = frames.into_iter().next() {
                    return frame_result;
                }
            }
        })
        .await
        .map_err(|_| ZmodemError::Timeout)??;

        Ok(result)
    }

    /// Send file header and wait for response.
    ///
    /// Returns the starting position for data transfer (0 for new, >0 for resume).
    async fn send_file_header(&mut self, file_info: &ZmodemFileInfo) -> Result<u64> {
        self.state.set_current_file(file_info.clone());

        let zfile = file_info.to_zfile_frame();
        self.send_frame(&zfile).await?;
        self.state.advance(ZmodemState::FileHeaderSent);

        // Wait for ZRPOS or ZSKIP
        let response = self
            .wait_for_frame(Duration::from_millis(self.config.timeout_ms))
            .await?;

        match response.frame_type {
            FrameType::ZRPOS => {
                // Extract position from flags
                let pos = response.flags_as_u32() as u64;
                self.state.set_position(pos);
                self.state.advance(ZmodemState::DataTransfer);
                Ok(pos)
            }
            FrameType::ZSKIP => {
                // Receiver wants to skip this file
                self.state.clear_current_file();
                self.state.advance(ZmodemState::InitReceived);
                Err(ZmodemError::InvalidFrame(
                    "File skipped by receiver".to_string(),
                ))
            }
            _ => Err(ZmodemError::InvalidFrame(format!(
                "Unexpected response to ZFILE: {:?}",
                response.frame_type
            ))),
        }
    }

    /// Send file data.
    async fn send_file_data(
        &mut self,
        file: &mut File,
        start_pos: u64,
        stats: &mut TransferStats,
    ) -> Result<()> {
        let mut position = start_pos;
        let file_size = stats.bytes_total;
        let mut retries = 0;

        while position < file_size {
            // Read block
            let block_size = self.config.block_size.min((file_size - position) as usize);
            let mut buffer = vec![0u8; block_size];
            let bytes_read = file.read(&mut buffer).await?;
            buffer.truncate(bytes_read);

            // Determine block type
            let block_type = if position + bytes_read as u64 >= file_size {
                ZCRCW // End of file, wait for ACK
            } else {
                ZCRCG // More data coming
            };

            // Send data block
            match self.send_data_block(&buffer, position, block_type).await {
                Ok(()) => {
                    position += bytes_read as u64;
                    stats.bytes_sent = position;
                    retries = 0;
                }
                Err(ZmodemError::Cancelled) => {
                    return Err(ZmodemError::Cancelled);
                }
                Err(_) if retries < self.config.max_retries => {
                    // Handle retransmission request
                    retries += 1;
                    stats.retries += 1;

                    // Wait for ZRPOS
                    let response = self
                        .wait_for_frame_type(
                            FrameType::ZRPOS,
                            Duration::from_millis(self.config.timeout_ms),
                        )
                        .await?;

                    let retry_pos = response.flags_as_u32() as u64;
                    file.seek(SeekFrom::Start(retry_pos)).await?;
                    position = retry_pos;
                    stats.bytes_sent = retry_pos;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    /// Send a data block with CRC.
    async fn send_data_block(&mut self, data: &[u8], position: u64, block_type: u8) -> Result<()> {
        // Send ZDATA frame
        let mut zdata = ZmodemFrame::with_defaults(FrameType::ZDATA, self.frame_encoding());
        zdata.set_flags_from_u32(position as u32);
        self.send_frame(&zdata).await?;

        // Encode data
        let encoded_data = escape::encode(data);

        // Send encoded data
        self.stream.write_all(&encoded_data).await?;

        // Send block type (ZCRCE, ZCRCG, ZCRCQ, ZCRCW)
        self.stream.write_all(&[ZDLE, block_type]).await?;

        // Calculate and send CRC
        let crc = if self.use_crc32() {
            let crc_val = crc32::calculate(data);
            crc_val.to_le_bytes().to_vec()
        } else {
            let crc_val = crc16::calculate(data);
            crc_val.to_be_bytes().to_vec()
        };

        let encoded_crc = escape::encode(&crc);
        self.stream.write_all(&encoded_crc).await?;
        self.stream.flush().await?;

        // Wait for ACK if needed
        if block_type == ZCRCW || block_type == ZCRCQ {
            self.wait_for_frame_type(
                FrameType::ZACK,
                Duration::from_millis(self.config.timeout_ms),
            )
            .await?;
        }

        Ok(())
    }

    /// Send EOF and wait for acknowledgment.
    async fn send_eof(&mut self, file_size: u64, _stats: &mut TransferStats) -> Result<()> {
        let mut zeof = ZmodemFrame::with_defaults(FrameType::ZEOF, self.frame_encoding());
        zeof.set_flags_from_u32(file_size as u32);
        self.send_frame(&zeof).await?;

        loop {
            let response = self
                .wait_for_frame(Duration::from_millis(self.config.timeout_ms))
                .await?;

            match response.frame_type {
                FrameType::ZRINIT => {
                    // Ready for next file
                    self.state.advance(ZmodemState::FileComplete);
                    self.state.advance(ZmodemState::InitReceived);
                    return Ok(());
                }
                FrameType::ZRPOS => {
                    // Need to retransmit
                    // Retransmit from requested position
                    let _pos = response.flags_as_u32() as u64;
                    // Would need to reopen file and retransmit, for now return error
                    return Err(ZmodemError::InvalidFrame(
                        "EOF retransmission not yet implemented".to_string(),
                    ));
                }
                _ => {
                    continue;
                }
            }
        }
    }

    /// Get frame encoding based on negotiated parameters.
    fn frame_encoding(&self) -> FrameEncoding {
        if self.use_crc32() {
            FrameEncoding::Bin32
        } else {
            FrameEncoding::Bin16
        }
    }

    /// Check if using CRC-32.
    fn use_crc32(&self) -> bool {
        self.negotiated
            .as_ref()
            .map(|p| p.crc_type == CrcType::Crc32)
            .unwrap_or(self.config.use_crc32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sender_config_default() {
        let config = SenderConfig::default();
        assert_eq!(config.block_size, 1024);
        assert_eq!(config.timeout_ms, 10000);
        assert_eq!(config.max_retries, 10);
        assert!(config.use_crc32);
        assert!(!config.escape_control);
        assert!(!config.escape_8bit);
    }

    #[test]
    fn test_transfer_stats_new() {
        let stats = TransferStats::new(1024);
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.bytes_total, 1024);
        assert_eq!(stats.retries, 0);
        assert!(stats.end_time.is_none());
    }

    #[test]
    fn test_transfer_stats_percent_complete() {
        let mut stats = TransferStats::new(1000);
        assert_eq!(stats.percent_complete(), 0.0);

        stats.bytes_sent = 500;
        assert_eq!(stats.percent_complete(), 50.0);

        stats.bytes_sent = 1000;
        assert_eq!(stats.percent_complete(), 100.0);
    }

    #[test]
    fn test_transfer_stats_zero_size() {
        let stats = TransferStats::new(0);
        assert_eq!(stats.percent_complete(), 100.0);
    }

    #[test]
    fn test_transfer_stats_complete() {
        let mut stats = TransferStats::new(1024);
        assert!(stats.end_time.is_none());

        stats.complete();
        assert!(stats.end_time.is_some());
    }

    #[test]
    fn test_transfer_stats_bytes_per_second() {
        let mut stats = TransferStats::new(1000);
        stats.bytes_sent = 1000;

        // Immediately after start
        let rate = stats.bytes_per_second();
        assert!(rate >= 0.0);

        // Mark complete
        stats.complete();
        let rate_complete = stats.bytes_per_second();
        assert!(rate_complete >= 0.0);
    }
}
