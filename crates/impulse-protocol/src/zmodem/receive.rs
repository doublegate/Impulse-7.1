//! Zmodem file receiver implementation.
//!
//! This module implements the receiver (upload) side of the Zmodem protocol,
//! allowing files to be received from a remote client to the BBS.
//!
//! # Protocol Flow
//!
//! 1. Wait for ZRQINIT and respond with ZRINIT
//! 2. Receive file header (ZFILE) with metadata
//! 3. Send ZRPOS with starting position (0 for new, >0 for resume)
//! 4. Receive file data blocks (ZDATA frames)
//! 5. Verify CRC and request retransmission if needed
//! 6. Receive EOF (ZEOF) and acknowledge
//! 7. Repeat for additional files or finish session
//!
//! # Examples
//!
//! ```no_run
//! use impulse_protocol::zmodem::receive::{ZmodemReceiver, ReceiverConfig};
//! use std::path::Path;
//!
//! # async fn example() -> impulse_protocol::zmodem::Result<()> {
//! // Create receiver with TCP stream (or any AsyncRead + AsyncWrite)
//! let stream = tokio::net::TcpStream::connect("127.0.0.1:2323").await?;
//! let config = ReceiverConfig::default();
//! let mut receiver = ZmodemReceiver::new(stream, config);
//!
//! // Initialize session
//! receiver.init().await?;
//!
//! // Receive files to directory
//! let dir = Path::new("/downloads");
//! let results = receiver.receive_files(dir).await?;
//!
//! for file in &results {
//!     println!("Received {} bytes", file.stats.bytes_received);
//! }
//!
//! // Finish session
//! receiver.finish().await?;
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
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeekExt, AsyncWrite, AsyncWriteExt};
use tokio::time::timeout;

/// Configuration for Zmodem receiver.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::receive::ReceiverConfig;
///
/// let config = ReceiverConfig {
///     buffer_size: 4096,
///     timeout_ms: 10000,
///     max_retries: 5,
///     use_crc32: true,
///     allow_resume: true,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ReceiverConfig {
    /// Buffer size for data reception (default: 1024 bytes)
    pub buffer_size: usize,

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

    /// Allow resume of partial transfers (default: true)
    pub allow_resume: bool,

    /// Overwrite existing files (default: false)
    pub overwrite_existing: bool,
}

impl Default for ReceiverConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1024,
            timeout_ms: 10000,
            max_retries: 10,
            use_crc32: true,
            escape_control: false,
            escape_8bit: false,
            allow_resume: true,
            overwrite_existing: false,
        }
    }
}

/// Transfer statistics for a single received file.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::receive::ReceiveStats;
///
/// let stats = ReceiveStats::new(1048576);
/// println!("Transfer rate: {} bytes/sec", stats.bytes_per_second());
/// ```
#[derive(Debug, Clone)]
pub struct ReceiveStats {
    /// Bytes successfully received
    pub bytes_received: u64,

    /// Total bytes expected
    pub bytes_total: u64,

    /// Transfer start time
    pub start_time: Instant,

    /// Transfer end time (if complete)
    pub end_time: Option<Instant>,

    /// Number of retransmission requests
    pub retries: u32,

    /// Whether transfer was resumed
    pub was_resumed: bool,

    /// Starting position (if resumed)
    pub resume_position: u64,

    /// File name
    pub file_name: String,

    /// Final file path
    pub file_path: PathBuf,
}

impl ReceiveStats {
    /// Create new receive statistics.
    pub fn new(bytes_total: u64) -> Self {
        Self {
            bytes_received: 0,
            bytes_total,
            start_time: Instant::now(),
            end_time: None,
            retries: 0,
            was_resumed: false,
            resume_position: 0,
            file_name: String::new(),
            file_path: PathBuf::new(),
        }
    }

    /// Create statistics for resumed transfer.
    pub fn resumed(
        bytes_total: u64,
        resume_pos: u64,
        file_name: String,
        file_path: PathBuf,
    ) -> Self {
        Self {
            bytes_received: resume_pos,
            bytes_total,
            start_time: Instant::now(),
            end_time: None,
            retries: 0,
            was_resumed: true,
            resume_position: resume_pos,
            file_name,
            file_path,
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
            let net_bytes = self.bytes_received - self.resume_position;
            net_bytes as f64 / elapsed_secs
        } else {
            0.0
        }
    }

    /// Get completion percentage.
    pub fn percent_complete(&self) -> f64 {
        if self.bytes_total == 0 {
            100.0
        } else {
            (self.bytes_received as f64 / self.bytes_total as f64) * 100.0
        }
    }

    /// Mark transfer as complete.
    pub fn complete(&mut self) {
        self.end_time = Some(Instant::now());
    }
}

/// Received file result.
///
/// Contains information about a successfully received file.
#[derive(Debug, Clone)]
pub struct ReceivedFile {
    /// File information from sender
    pub file_info: ZmodemFileInfo,

    /// Path where file was saved
    pub saved_path: PathBuf,

    /// Transfer statistics
    pub stats: ReceiveStats,
}

/// Zmodem file receiver.
///
/// Implements the receiver side of the Zmodem protocol for receiving files
/// from a remote client to the BBS.
///
/// # Type Parameters
///
/// * `S` - Stream type that implements AsyncRead + AsyncWrite + Unpin
///
/// # Examples
///
/// ```no_run
/// use impulse_protocol::zmodem::receive::{ZmodemReceiver, ReceiverConfig};
/// use tokio::net::TcpStream;
/// use std::path::Path;
///
/// # async fn example() -> impulse_protocol::zmodem::Result<()> {
/// let stream = TcpStream::connect("127.0.0.1:2323").await?;
/// let config = ReceiverConfig::default();
/// let mut receiver = ZmodemReceiver::new(stream, config);
///
/// receiver.init().await?;
/// let results = receiver.receive_files(Path::new("/uploads")).await?;
/// receiver.finish().await?;
/// # Ok(())
/// # }
/// ```
pub struct ZmodemReceiver<S> {
    stream: S,
    parser: FrameParser,
    state: ZmodemStateMachine,
    config: ReceiverConfig,
    negotiated: Option<NegotiatedParams>,
    data_buffer: Vec<u8>,
}

impl<S: AsyncRead + AsyncWrite + Unpin> ZmodemReceiver<S> {
    /// Create a new Zmodem receiver.
    ///
    /// # Arguments
    ///
    /// * `stream` - Communication stream (must implement AsyncRead + AsyncWrite + Unpin)
    /// * `config` - Receiver configuration
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use impulse_protocol::zmodem::receive::{ZmodemReceiver, ReceiverConfig};
    /// use tokio::net::TcpStream;
    ///
    /// # async fn example() -> std::io::Result<()> {
    /// let stream = TcpStream::connect("127.0.0.1:2323").await?;
    /// let receiver = ZmodemReceiver::new(stream, ReceiverConfig::default());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(stream: S, config: ReceiverConfig) -> Self {
        Self {
            stream,
            parser: FrameParser::new(),
            state: ZmodemStateMachine::new(),
            config,
            negotiated: None,
            data_buffer: Vec::with_capacity(8192),
        }
    }

    /// Initialize Zmodem session.
    ///
    /// Waits for ZRQINIT and responds with ZRINIT to negotiate parameters.
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
    /// # use impulse_protocol::zmodem::receive::{ZmodemReceiver, ReceiverConfig};
    /// # use tokio::net::TcpStream;
    /// # async fn example() -> impulse_protocol::zmodem::Result<()> {
    /// # let stream = TcpStream::connect("127.0.0.1:2323").await?;
    /// let mut receiver = ZmodemReceiver::new(stream, ReceiverConfig::default());
    /// let params = receiver.init().await?;
    /// println!("Negotiated CRC type: {:?}", params.crc_type);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn init(&mut self) -> Result<NegotiatedParams> {
        // Wait for ZRQINIT
        let _zrqinit = self
            .wait_for_frame_type(
                FrameType::ZRQINIT,
                Duration::from_millis(self.config.timeout_ms),
            )
            .await?;

        self.state.advance(ZmodemState::InitReceived);

        // Create receiver init parameters
        let receiver_init = ZmodemInit {
            use_crc32: self.config.use_crc32,
            escape_ctrl: self.config.escape_control,
            escape_8bit: self.config.escape_8bit,
            buffer_size: self.config.buffer_size as u16,
        };

        // Send ZRINIT
        let zrinit = receiver_init.to_zrinit();
        self.send_frame(&zrinit).await?;
        self.state.advance(ZmodemState::InitSent);

        // Create negotiated parameters (we accept our own parameters as baseline)
        let params = NegotiatedParams {
            crc_type: if self.config.use_crc32 {
                CrcType::Crc32
            } else {
                CrcType::Crc16
            },
            escape_mode: super::negotiate::EscapeMode::Minimal,
            buffer_size: self.config.buffer_size as u16,
            can_resume: self.config.allow_resume,
        };

        self.negotiated = Some(params.clone());
        self.state.set_negotiated(params.clone());

        Ok(params)
    }

    /// Receive files to a directory.
    ///
    /// Receives all files sent by the sender and saves them to the specified directory.
    ///
    /// # Arguments
    ///
    /// * `output_dir` - Directory where files will be saved
    ///
    /// # Returns
    ///
    /// Vector of received file information
    ///
    /// # Errors
    ///
    /// Returns error if file reception fails or is cancelled
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use impulse_protocol::zmodem::receive::{ZmodemReceiver, ReceiverConfig};
    /// # use tokio::net::TcpStream;
    /// # use std::path::Path;
    /// # async fn example() -> impulse_protocol::zmodem::Result<()> {
    /// # let stream = TcpStream::connect("127.0.0.1:2323").await?;
    /// # let mut receiver = ZmodemReceiver::new(stream, ReceiverConfig::default());
    /// # receiver.init().await?;
    /// let results = receiver.receive_files(Path::new("/uploads")).await?;
    /// for file in results {
    ///     println!("Received: {} ({} bytes)", file.file_info.name, file.stats.bytes_received);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn receive_files(&mut self, output_dir: &Path) -> Result<Vec<ReceivedFile>> {
        let mut received_files = Vec::new();

        loop {
            // Wait for ZFILE or ZFIN
            let frame = self
                .wait_for_frame(Duration::from_millis(self.config.timeout_ms))
                .await?;

            match frame.frame_type {
                FrameType::ZFILE => {
                    // Parse file info and receive file
                    let file_info = self.parse_file_info(&frame).await?;
                    let result = self.receive_file(&file_info, output_dir).await?;
                    received_files.push(result);

                    // Send ZRINIT to indicate ready for next file
                    let receiver_init = ZmodemInit {
                        use_crc32: self.config.use_crc32,
                        escape_ctrl: self.config.escape_control,
                        escape_8bit: self.config.escape_8bit,
                        buffer_size: self.config.buffer_size as u16,
                    };
                    let zrinit = receiver_init.to_zrinit();
                    self.send_frame(&zrinit).await?;
                }
                FrameType::ZFIN => {
                    // Session complete
                    self.state.advance(ZmodemState::SessionComplete);
                    break;
                }
                FrameType::ZCAN | FrameType::ZABORT => {
                    return Err(ZmodemError::Cancelled);
                }
                _ => {
                    // Ignore unexpected frames
                    continue;
                }
            }
        }

        Ok(received_files)
    }

    /// Receive a single file.
    ///
    /// # Arguments
    ///
    /// * `output_path` - Full path where file will be saved
    ///
    /// # Returns
    ///
    /// Receive statistics for the completed transfer
    pub async fn receive_single_file(&mut self, output_path: &Path) -> Result<ReceivedFile> {
        // Wait for ZFILE
        let frame = self
            .wait_for_frame_type(
                FrameType::ZFILE,
                Duration::from_millis(self.config.timeout_ms),
            )
            .await?;

        let file_info = self.parse_file_info(&frame).await?;

        // Use parent directory of output_path
        let output_dir = output_path.parent().unwrap_or(Path::new("."));
        self.receive_file(&file_info, output_dir).await
    }

    /// Finish the Zmodem session.
    ///
    /// Sends ZFIN acknowledgment.
    ///
    /// # Errors
    ///
    /// Returns error if session termination fails
    pub async fn finish(&mut self) -> Result<()> {
        // Send ZFIN acknowledgment
        let zfin = ZmodemFrame::with_defaults(FrameType::ZFIN, self.frame_encoding());
        self.send_frame(&zfin).await?;
        self.state.advance(ZmodemState::SessionComplete);
        Ok(())
    }

    /// Parse file information from ZFILE frame.
    async fn parse_file_info(&mut self, _frame: &ZmodemFrame) -> Result<ZmodemFileInfo> {
        // ZFILE frame data contains filename and metadata
        // Read the subpacket data following the frame header
        let file_data = self.read_subpacket_data().await?;

        // Parse file info from the data
        ZmodemFileInfo::from_zfile_data(&file_data)
    }

    /// Read subpacket data (following a frame header).
    async fn read_subpacket_data(&mut self) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        let mut raw_buffer = vec![0u8; 1024];
        let mut in_escape = false;

        let deadline = Instant::now() + Duration::from_millis(self.config.timeout_ms);

        loop {
            if Instant::now() >= deadline {
                return Err(ZmodemError::Timeout);
            }

            let remaining = deadline - Instant::now();
            let n = match timeout(remaining, self.stream.read(&mut raw_buffer)).await {
                Ok(Ok(n)) if n > 0 => n,
                Ok(Ok(_)) => return Err(ZmodemError::UnexpectedEof),
                Ok(Err(e)) => return Err(e.into()),
                Err(_) => return Err(ZmodemError::Timeout),
            };

            for &byte in &raw_buffer[..n] {
                if in_escape {
                    in_escape = false;
                    match byte {
                        // End of subpacket markers
                        ZCRCG | ZCRCQ | ZCRCW | 0x68 => {
                            // ZCRCE = 0x68
                            // Read and verify CRC
                            // For now, return data without CRC verification in subpacket
                            return Ok(data);
                        }
                        _ => {
                            // XOR with 0x40 to decode escaped character
                            data.push(byte ^ 0x40);
                        }
                    }
                } else if byte == ZDLE {
                    in_escape = true;
                } else {
                    data.push(byte);
                }
            }

            // Safety limit on subpacket size
            if data.len() > 8192 {
                return Err(ZmodemError::InvalidFrame("Subpacket too large".to_string()));
            }
        }
    }

    /// Receive a single file.
    async fn receive_file(
        &mut self,
        file_info: &ZmodemFileInfo,
        output_dir: &Path,
    ) -> Result<ReceivedFile> {
        self.state.set_current_file(file_info.clone());

        // Determine output path
        let output_path = output_dir.join(&file_info.name);

        // Check for existing file and determine starting position
        let (mut file, start_pos) = self.open_output_file(&output_path, file_info).await?;

        // Initialize statistics
        let mut stats = if start_pos > 0 {
            ReceiveStats::resumed(
                file_info.size,
                start_pos,
                file_info.name.clone(),
                output_path.clone(),
            )
        } else {
            let mut s = ReceiveStats::new(file_info.size);
            s.file_name = file_info.name.clone();
            s.file_path = output_path.clone();
            s
        };

        // Send ZRPOS with starting position
        self.send_position(start_pos).await?;
        self.state.set_position(start_pos);
        self.state.advance(ZmodemState::DataTransfer);

        // Receive file data
        self.receive_file_data(&mut file, &mut stats).await?;

        // Wait for ZEOF
        self.wait_for_eof(&mut stats).await?;

        stats.complete();

        Ok(ReceivedFile {
            file_info: file_info.clone(),
            saved_path: output_path,
            stats,
        })
    }

    /// Open output file, handling resume and overwrite logic.
    async fn open_output_file(
        &self,
        output_path: &Path,
        file_info: &ZmodemFileInfo,
    ) -> Result<(File, u64)> {
        // Check if file exists
        if output_path.exists() {
            if self.config.allow_resume {
                // Try to resume
                let metadata = tokio::fs::metadata(output_path).await?;
                let existing_size = metadata.len();

                if existing_size < file_info.size {
                    // Resume from existing position
                    let file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(output_path)
                        .await?;
                    return Ok((file, existing_size));
                }
            }

            if !self.config.overwrite_existing {
                return Err(ZmodemError::InvalidFrame(format!(
                    "File already exists: {}",
                    output_path.display()
                )));
            }
        }

        // Create new file
        let file = File::create(output_path).await?;
        Ok((file, 0))
    }

    /// Send ZRPOS frame with position.
    async fn send_position(&mut self, position: u64) -> Result<()> {
        let mut zrpos = ZmodemFrame::with_defaults(FrameType::ZRPOS, self.frame_encoding());
        zrpos.set_flags_from_u32(position as u32);
        self.send_frame(&zrpos).await
    }

    /// Send ZACK frame with position.
    async fn send_ack(&mut self, position: u64) -> Result<()> {
        let mut zack = ZmodemFrame::with_defaults(FrameType::ZACK, self.frame_encoding());
        zack.set_flags_from_u32(position as u32);
        self.send_frame(&zack).await
    }

    /// Receive file data blocks.
    async fn receive_file_data(&mut self, file: &mut File, stats: &mut ReceiveStats) -> Result<()> {
        let mut position = stats.bytes_received;
        let file_size = stats.bytes_total;
        let mut retries = 0;

        while position < file_size {
            // Wait for ZDATA frame
            let frame = match self
                .wait_for_frame(Duration::from_millis(self.config.timeout_ms))
                .await
            {
                Ok(f) => f,
                Err(ZmodemError::Timeout) if retries < self.config.max_retries => {
                    retries += 1;
                    stats.retries += 1;
                    // Request retransmission
                    self.send_position(position).await?;
                    continue;
                }
                Err(e) => return Err(e),
            };

            match frame.frame_type {
                FrameType::ZDATA => {
                    // Verify position
                    let data_pos = frame.flags_as_u32() as u64;
                    if data_pos != position {
                        // Out of sync, request correct position
                        retries += 1;
                        stats.retries += 1;
                        if retries >= self.config.max_retries {
                            return Err(ZmodemError::MaxRetriesExceeded);
                        }
                        file.seek(SeekFrom::Start(position)).await?;
                        self.send_position(position).await?;
                        continue;
                    }

                    // Receive data block
                    match self.receive_data_block().await {
                        Ok((data, block_type)) => {
                            // Write data to file
                            file.write_all(&data).await?;
                            position += data.len() as u64;
                            stats.bytes_received = position;
                            retries = 0;

                            // Send ACK if requested
                            if block_type == ZCRCQ || block_type == ZCRCW {
                                self.send_ack(position).await?;
                            }
                        }
                        Err(ZmodemError::CrcMismatch { .. })
                            if retries < self.config.max_retries =>
                        {
                            retries += 1;
                            stats.retries += 1;
                            // Request retransmission from current position
                            self.send_position(position).await?;
                        }
                        Err(e) => return Err(e),
                    }
                }
                FrameType::ZEOF => {
                    // Premature EOF, but might be valid
                    let eof_pos = frame.flags_as_u32() as u64;
                    if eof_pos == position {
                        // Valid EOF at current position
                        stats.bytes_total = position;
                        break;
                    } else {
                        // Wrong position, request correct position
                        self.send_position(position).await?;
                    }
                }
                FrameType::ZCAN | FrameType::ZABORT => {
                    return Err(ZmodemError::Cancelled);
                }
                _ => {
                    // Ignore unexpected frames
                    continue;
                }
            }
        }

        Ok(())
    }

    /// Receive a data block with CRC verification.
    async fn receive_data_block(&mut self) -> Result<(Vec<u8>, u8)> {
        let mut data = Vec::new();
        let mut raw_buffer = vec![0u8; 256];
        let mut in_escape = false;
        let mut block_type: Option<u8> = None;

        let deadline = Instant::now() + Duration::from_millis(self.config.timeout_ms);

        loop {
            if Instant::now() >= deadline {
                return Err(ZmodemError::Timeout);
            }

            let remaining = deadline - Instant::now();
            let n = match timeout(remaining, self.stream.read(&mut raw_buffer)).await {
                Ok(Ok(n)) if n > 0 => n,
                Ok(Ok(_)) => return Err(ZmodemError::UnexpectedEof),
                Ok(Err(e)) => return Err(e.into()),
                Err(_) => return Err(ZmodemError::Timeout),
            };

            for &byte in &raw_buffer[..n] {
                if let Some(block_end) = block_type {
                    // Reading CRC after block end
                    self.data_buffer.push(byte);

                    // Check if we have enough CRC bytes
                    let crc_len = if self.use_crc32() { 4 } else { 2 };
                    let expected_crc_bytes = crc_len * 2; // Escaped CRC can be up to 2x size

                    if self.data_buffer.len() >= expected_crc_bytes || !in_escape {
                        // Try to decode and verify CRC
                        let decoded_crc = escape::decode(&self.data_buffer)
                            .unwrap_or_else(|_| self.data_buffer.clone());

                        if decoded_crc.len() >= crc_len {
                            // Verify CRC
                            if self.use_crc32() && decoded_crc.len() >= 4 {
                                let received_crc = u32::from_le_bytes([
                                    decoded_crc[0],
                                    decoded_crc[1],
                                    decoded_crc[2],
                                    decoded_crc[3],
                                ]);
                                let calculated_crc = crc32::calculate(&data);
                                if received_crc != calculated_crc {
                                    return Err(ZmodemError::CrcMismatch {
                                        expected: calculated_crc,
                                        actual: received_crc,
                                    });
                                }
                            } else if !self.use_crc32() && decoded_crc.len() >= 2 {
                                let received_crc =
                                    u16::from_be_bytes([decoded_crc[0], decoded_crc[1]]);
                                let calculated_crc = crc16::calculate(&data);
                                if received_crc != calculated_crc {
                                    return Err(ZmodemError::CrcMismatch {
                                        expected: calculated_crc as u32,
                                        actual: received_crc as u32,
                                    });
                                }
                            }

                            self.data_buffer.clear();
                            return Ok((data, block_end));
                        }
                    }
                    continue;
                }

                if in_escape {
                    in_escape = false;
                    match byte {
                        // End of block markers
                        ZCRCG | ZCRCQ | ZCRCW | 0x68 => {
                            // ZCRCE = 0x68
                            block_type = Some(byte);
                            self.data_buffer.clear();
                        }
                        _ => {
                            // XOR with 0x40 to decode escaped character
                            data.push(byte ^ 0x40);
                        }
                    }
                } else if byte == ZDLE {
                    in_escape = true;
                } else {
                    data.push(byte);
                }
            }

            // Safety limit on block size
            if data.len() > 8192 {
                return Err(ZmodemError::InvalidFrame(
                    "Data block too large".to_string(),
                ));
            }
        }
    }

    /// Wait for ZEOF frame.
    async fn wait_for_eof(&mut self, stats: &mut ReceiveStats) -> Result<()> {
        loop {
            let frame = self
                .wait_for_frame(Duration::from_millis(self.config.timeout_ms))
                .await?;

            match frame.frame_type {
                FrameType::ZEOF => {
                    let eof_pos = frame.flags_as_u32() as u64;
                    if eof_pos == stats.bytes_received {
                        self.state.advance(ZmodemState::FileComplete);
                        return Ok(());
                    } else {
                        // Wrong EOF position, request correct position
                        self.send_position(stats.bytes_received).await?;
                    }
                }
                FrameType::ZDATA => {
                    // More data, process it
                    // In practice, this shouldn't happen if we're waiting for EOF
                    continue;
                }
                FrameType::ZCAN | FrameType::ZABORT => {
                    return Err(ZmodemError::Cancelled);
                }
                _ => {
                    continue;
                }
            }
        }
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

    /// Skip current file.
    ///
    /// Sends ZSKIP to tell sender to skip to the next file.
    ///
    /// # Errors
    ///
    /// Returns error if communication fails
    pub async fn skip_file(&mut self) -> Result<()> {
        let zskip = ZmodemFrame::with_defaults(FrameType::ZSKIP, self.frame_encoding());
        self.send_frame(&zskip).await
    }

    /// Abort transfer.
    ///
    /// Sends ZCAN to abort the entire transfer session.
    ///
    /// # Errors
    ///
    /// Returns error if communication fails
    pub async fn abort(&mut self) -> Result<()> {
        // Send cancel sequence (5x CAN + 5x BS)
        let cancel_seq = [0x18, 0x18, 0x18, 0x18, 0x18, 0x08, 0x08, 0x08, 0x08, 0x08];
        self.stream.write_all(&cancel_seq).await?;
        self.stream.flush().await?;
        self.state.advance(ZmodemState::SessionComplete);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receiver_config_default() {
        let config = ReceiverConfig::default();
        assert_eq!(config.buffer_size, 1024);
        assert_eq!(config.timeout_ms, 10000);
        assert_eq!(config.max_retries, 10);
        assert!(config.use_crc32);
        assert!(!config.escape_control);
        assert!(!config.escape_8bit);
        assert!(config.allow_resume);
        assert!(!config.overwrite_existing);
    }

    #[test]
    fn test_receive_stats_new() {
        let stats = ReceiveStats::new(1024);
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.bytes_total, 1024);
        assert_eq!(stats.retries, 0);
        assert!(stats.end_time.is_none());
        assert!(!stats.was_resumed);
        assert_eq!(stats.resume_position, 0);
    }

    #[test]
    fn test_receive_stats_resumed() {
        let stats = ReceiveStats::resumed(
            1024,
            512,
            "test.txt".to_string(),
            PathBuf::from("/test/test.txt"),
        );
        assert_eq!(stats.bytes_received, 512);
        assert_eq!(stats.bytes_total, 1024);
        assert!(stats.was_resumed);
        assert_eq!(stats.resume_position, 512);
        assert_eq!(stats.file_name, "test.txt");
    }

    #[test]
    fn test_receive_stats_percent_complete() {
        let mut stats = ReceiveStats::new(1000);
        assert_eq!(stats.percent_complete(), 0.0);

        stats.bytes_received = 500;
        assert_eq!(stats.percent_complete(), 50.0);

        stats.bytes_received = 1000;
        assert_eq!(stats.percent_complete(), 100.0);
    }

    #[test]
    fn test_receive_stats_zero_size() {
        let stats = ReceiveStats::new(0);
        assert_eq!(stats.percent_complete(), 100.0);
    }

    #[test]
    fn test_receive_stats_complete() {
        let mut stats = ReceiveStats::new(1024);
        assert!(stats.end_time.is_none());

        stats.complete();
        assert!(stats.end_time.is_some());
    }

    #[test]
    fn test_receive_stats_bytes_per_second() {
        let mut stats = ReceiveStats::new(1000);
        stats.bytes_received = 1000;

        // Immediately after start
        let rate = stats.bytes_per_second();
        assert!(rate >= 0.0);

        // Mark complete
        stats.complete();
        let rate_complete = stats.bytes_per_second();
        assert!(rate_complete >= 0.0);
    }

    #[test]
    fn test_receive_stats_bytes_per_second_resumed() {
        let mut stats = ReceiveStats::resumed(
            1000,
            500,
            "test.txt".to_string(),
            PathBuf::from("/test/test.txt"),
        );
        stats.bytes_received = 1000;

        // Rate should be calculated from resume position
        let rate = stats.bytes_per_second();
        assert!(rate >= 0.0);
    }

    #[test]
    fn test_received_file_creation() {
        let file_info = ZmodemFileInfo::new("test.txt", 1024);
        let stats = ReceiveStats::new(1024);
        let saved_path = PathBuf::from("/downloads/test.txt");

        let received = ReceivedFile {
            file_info: file_info.clone(),
            saved_path: saved_path.clone(),
            stats,
        };

        assert_eq!(received.file_info.name, "test.txt");
        assert_eq!(received.saved_path, saved_path);
    }

    // Mock stream for testing
    struct MockStream {
        read_data: Vec<u8>,
        read_pos: usize,
        write_data: Vec<u8>,
    }

    impl MockStream {
        fn new(data: Vec<u8>) -> Self {
            Self {
                read_data: data,
                read_pos: 0,
                write_data: Vec::new(),
            }
        }
    }

    impl AsyncRead for MockStream {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            let remaining = &self.read_data[self.read_pos..];
            let to_copy = remaining.len().min(buf.remaining());
            buf.put_slice(&remaining[..to_copy]);
            self.read_pos += to_copy;
            std::task::Poll::Ready(Ok(()))
        }
    }

    impl AsyncWrite for MockStream {
        fn poll_write(
            mut self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
            self.write_data.extend_from_slice(buf);
            std::task::Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::task::Poll::Ready(Ok(()))
        }

        fn poll_shutdown(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::task::Poll::Ready(Ok(()))
        }
    }

    impl Unpin for MockStream {}

    #[test]
    fn test_receiver_creation() {
        let stream = MockStream::new(vec![]);
        let config = ReceiverConfig::default();
        let receiver = ZmodemReceiver::new(stream, config);

        assert!(receiver.negotiated.is_none());
    }

    #[test]
    fn test_receiver_custom_config() {
        let config = ReceiverConfig {
            buffer_size: 4096,
            timeout_ms: 5000,
            max_retries: 3,
            use_crc32: false,
            escape_control: true,
            escape_8bit: true,
            allow_resume: false,
            overwrite_existing: true,
        };

        assert_eq!(config.buffer_size, 4096);
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.max_retries, 3);
        assert!(!config.use_crc32);
        assert!(config.escape_control);
        assert!(config.escape_8bit);
        assert!(!config.allow_resume);
        assert!(config.overwrite_existing);
    }

    #[test]
    fn test_frame_encoding_selection() {
        let stream = MockStream::new(vec![]);

        // With CRC-32
        let config = ReceiverConfig {
            use_crc32: true,
            ..Default::default()
        };
        let receiver = ZmodemReceiver::new(stream, config);
        assert_eq!(receiver.frame_encoding(), FrameEncoding::Bin32);

        // With CRC-16
        let stream = MockStream::new(vec![]);
        let config = ReceiverConfig {
            use_crc32: false,
            ..Default::default()
        };
        let receiver = ZmodemReceiver::new(stream, config);
        assert_eq!(receiver.frame_encoding(), FrameEncoding::Bin16);
    }
}
