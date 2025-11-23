# Impulse BBS - Technical Implementation Details
## Deep Dive: Pascal to Rust Conversion

---

## Table of Contents

1. [Pascal to Rust Type Mapping](#pascal-to-rust-type-mapping)
2. [Binary File Format Handling](#binary-file-format-handling)
3. [DOS Interrupt Replacement Strategies](#dos-interrupt-replacement-strategies)
4. [ANSI/ASCII Art Rendering Engine](#ansiiascii-art-rendering-engine)
5. [File Transfer Protocol Specifications](#file-transfer-protocol-specifications)
6. [Message Base Format Deep Dive](#message-base-format-deep-dive)
7. [Performance Optimization Techniques](#performance-optimization-techniques)
8. [Testing Methodologies](#testing-methodologies)
9. [Build System Configuration](#build-system-configuration)
10. [Deployment Recipes](#deployment-recipes)

---

## 1. Pascal to Rust Type Mapping

### 1.1 Fundamental Type Conversions

| Borland Pascal 7.0 | Rust Equivalent | Notes |
|-------------------|-----------------|-------|
| `Byte` | `u8` | Unsigned 8-bit integer |
| `ShortInt` | `i8` | Signed 8-bit integer |
| `Word` | `u16` | Unsigned 16-bit integer |
| `Integer` | `i16` | Signed 16-bit integer (16-bit in BP7) |
| `LongInt` | `i32` | Signed 32-bit integer |
| `LongWord` / `Cardinal` | `u32` | Unsigned 32-bit integer |
| `Real` | `f64` | 64-bit floating point |
| `Single` | `f32` | 32-bit floating point |
| `Boolean` | `bool` | Boolean type |
| `Char` | `u8` | Single byte character (not `char`, which is UTF-8 in Rust) |

### 1.2 String Type Conversions

Pascal strings in Impulse use the `String[N]` format (length-prefixed):

```pascal
{ Pascal: String[255] - 1 byte length + 255 bytes data }
type
  UserName = String[30];
  
var
  Name: UserName;
```

**Rust Equivalent**:

```rust
/// Fixed-size Pascal-compatible string
#[derive(Debug, Clone)]
pub struct PascalString<const N: usize> {
    length: u8,
    data: [u8; N],
}

impl<const N: usize> PascalString<N> {
    /// Create from Rust string
    pub fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        if bytes.len() > N {
            return Err(Error::StringTooLong(bytes.len(), N));
        }
        
        let mut data = [0u8; N];
        data[..bytes.len()].copy_from_slice(bytes);
        
        Ok(Self {
            length: bytes.len() as u8,
            data,
        })
    }
    
    /// Convert to Rust String
    pub fn to_string(&self) -> String {
        let len = self.length as usize;
        // Handle CP437 encoding (DOS code page)
        encoding_rs::IBM866
            .decode(&self.data[..len])
            .0
            .to_string()
    }
}

// Type alias for common sizes
pub type UserName = PascalString<30>;
pub type FilePath = PascalString<255>;
```

### 1.3 Record (Struct) Conversions

Pascal records require careful attention to memory layout:

```pascal
{ Pascal: User record }
type
  UserRec = record
    Name: String[30];
    Password: String[20];
    SecurityLevel: Byte;
    LastCall: LongInt;  { Unix timestamp }
    Flags: Word;        { Bitfield }
  end;
```

**Rust Equivalent**:

```rust
/// User record with C-compatible memory layout
#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct UserRecordDisk {
    pub name: PascalString<30>,
    pub password: PascalString<20>,
    pub security_level: u8,
    pub last_call: i32,
    pub flags: u16,
}

// For runtime use, convert to idiomatic Rust types
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub password_hash: String,  // Modern hash, not plain Pascal string
    pub security_level: u8,
    pub last_call: DateTime<Utc>,
    pub flags: UserFlags,
}

impl From<UserRecordDisk> for User {
    fn from(disk: UserRecordDisk) -> Self {
        Self {
            name: disk.name.to_string(),
            password_hash: disk.password.to_string(),  // Will need re-hashing
            security_level: disk.security_level,
            last_call: DateTime::from_timestamp(disk.last_call as i64, 0)
                .unwrap_or_default(),
            flags: UserFlags::from_bits_truncate(disk.flags),
        }
    }
}
```

### 1.4 Pointer and Reference Conversions

Pascal uses raw pointers extensively:

```pascal
{ Pascal pointer example }
type
  PNode = ^NodeRec;
  NodeRec = record
    Data: Integer;
    Next: PNode;
  end;

var
  Head: PNode;
```

**Rust Equivalent**:

```rust
/// Linked list node using Box (owned heap allocation)
pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}

// If shared ownership is needed (rare), use Arc<RefCell<T>>
pub struct SharedNode {
    pub data: i32,
    pub next: Option<Arc<RefCell<SharedNode>>>,
}
```

**Important**: Rust's ownership model prevents entire classes of bugs:
- No null pointer dereferences (use `Option`)
- No dangling pointers (lifetime checking)
- No double-frees (automatic with `Drop`)

---

## 2. Binary File Format Handling

### 2.1 Reading Pascal Binary Files

Impulse stores data in binary .DAT files with C-style struct layout:

```rust
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

/// Read a Pascal record from a binary file
pub fn read_record<T>(file: &mut File, index: usize) -> Result<T>
where
    T: Copy + Default,
{
    let record_size = size_of::<T>();
    let offset = (index * record_size) as u64;
    
    // Seek to record position
    file.seek(SeekFrom::Start(offset))?;
    
    // Read raw bytes
    let mut buffer = vec![0u8; record_size];
    file.read_exact(&mut buffer)?;
    
    // Interpret as struct (UNSAFE: requires #[repr(C, packed)])
    let record: T = unsafe {
        std::ptr::read(buffer.as_ptr() as *const T)
    };
    
    Ok(record)
}

/// Write a Pascal record to a binary file
pub fn write_record<T>(file: &mut File, index: usize, record: &T) -> Result<()>
where
    T: Copy,
{
    let record_size = size_of::<T>();
    let offset = (index * record_size) as u64;
    
    file.seek(SeekFrom::Start(offset))?;
    
    // Convert struct to bytes
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            record as *const T as *const u8,
            record_size,
        )
    };
    
    file.write_all(bytes)?;
    file.sync_all()?;  // Ensure written to disk
    
    Ok(())
}
```

### 2.2 Endianness Considerations

DOS/Pascal uses little-endian by default (x86). Modern Rust should explicitly handle this:

```rust
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// Read a 32-bit integer with explicit endianness
pub fn read_i32_le(file: &mut File) -> Result<i32> {
    Ok(file.read_i32::<LittleEndian>()?)
}

/// Write a 32-bit integer with explicit endianness
pub fn write_i32_le(file: &mut File, value: i32) -> Result<()> {
    Ok(file.write_i32::<LittleEndian>(value)?)
}
```

### 2.3 Safe Binary Serialization with Bincode

For new data formats, avoid `unsafe` and use `bincode`:

```rust
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModernUserRecord {
    pub name: String,
    pub password_hash: String,
    pub security_level: u8,
    pub last_call: i64,
}

impl ModernUserRecord {
    pub fn save(&self, path: &Path) -> Result<()> {
        let encoded = bincode::serialize(self)?;
        std::fs::write(path, encoded)?;
        Ok(())
    }
    
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        let decoded = bincode::deserialize(&data)?;
        Ok(decoded)
    }
}
```

---

## 3. DOS Interrupt Replacement Strategies

### 3.1 UART Direct Access (INT 14h)

Original Pascal code:

```pascal
{ Direct UART access via DOS interrupt }
procedure SendByte(B: Byte);
var
  Regs: Registers;
begin
  Regs.AH := $01;  { Transmit character }
  Regs.AL := B;
  Regs.DX := 0;    { COM1 }
  Intr($14, Regs); { Call INT 14h }
end;
```

**Rust Replacement** using `serialport` crate:

```rust
use serialport::{SerialPort, DataBits, StopBits, Parity};
use std::time::Duration;

/// Open a serial port with classic BBS settings
pub fn open_serial_port(port: &str) -> Result<Box<dyn SerialPort>> {
    let port = serialport::new(port, 38_400)  // 38.4 kbps
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .timeout(Duration::from_millis(100))
        .open()?;
    
    Ok(port)
}

/// Send a byte over serial
pub fn send_byte(port: &mut Box<dyn SerialPort>, byte: u8) -> Result<()> {
    port.write_all(&[byte])?;
    Ok(())
}

/// Receive a byte from serial (non-blocking)
pub fn receive_byte(port: &mut Box<dyn SerialPort>) -> Result<Option<u8>> {
    let mut buffer = [0u8; 1];
    match port.read(&mut buffer) {
        Ok(1) => Ok(Some(buffer[0])),
        Ok(0) => Ok(None),  // No data available
        Ok(_) => unreachable!(),
        Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(None),
        Err(e) => Err(e.into()),
    }
}
```

### 3.2 Keyboard Input (INT 16h)

Pascal keyboard handling:

```pascal
{ Wait for keypress }
function ReadKey: Char;
var
  Regs: Registers;
begin
  Regs.AH := $00;  { Read key }
  Intr($16, Regs);
  ReadKey := Chr(Regs.AL);
end;
```

**Rust Replacement** using `crossterm`:

```rust
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

/// Read a single keypress (blocking)
pub fn read_key() -> Result<KeyEvent> {
    loop {
        match event::read()? {
            Event::Key(key_event) => return Ok(key_event),
            _ => continue,  // Ignore mouse events, etc.
        }
    }
}

/// Check if a key is available (non-blocking)
pub fn key_available() -> Result<bool> {
    Ok(event::poll(Duration::from_millis(0))?)
}

/// Read a character with echo
pub async fn read_char_async(terminal: &mut TerminalDriver) -> Result<char> {
    let key = read_key()?;
    
    match key.code {
        KeyCode::Char(c) => {
            // Echo character
            terminal.write(&[c as u8]).await?;
            Ok(c)
        }
        KeyCode::Enter => Ok('\n'),
        KeyCode::Backspace => Ok('\x08'),
        _ => Err(Error::InvalidInput("Non-character key".into())),
    }
}
```

### 3.3 Video Memory Direct Access

Pascal code directly writes to video memory:

```pascal
{ Direct video memory access (B800:0000 for text mode) }
var
  VideoMem: ^Word absolute $B800:$0000;

procedure WriteChar(X, Y: Byte; Ch: Char; Attr: Byte);
begin
  VideoMem^[(Y * 80) + X] := Ord(Ch) or (Attr shl 8);
end;
```

**Rust Replacement**: Use terminal escape sequences instead:

```rust
/// Write a character at specific screen position
pub async fn write_at(
    terminal: &mut TerminalDriver,
    x: u16,
    y: u16,
    ch: char,
    color: AnsiColor,
) -> Result<()> {
    // Move cursor
    terminal.move_cursor(x, y).await?;
    
    // Set color
    let color_code = format!("\x1b[{}m", color.to_ansi_code());
    terminal.write(color_code.as_bytes()).await?;
    
    // Write character
    terminal.write(&[ch as u8]).await?;
    
    // Reset color
    terminal.write(b"\x1b[0m").await?;
    
    Ok(())
}
```

---

## 4. ANSI/ASCII Art Rendering Engine

### 4.1 ANSI Escape Sequence Parser

Implement a state machine to parse ANSI sequences:

```rust
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, char},
    combinator::{map_res, opt},
    multi::separated_list0,
    sequence::{delimited, preceded},
};

/// ANSI escape sequence types
#[derive(Debug, Clone, PartialEq)]
pub enum AnsiSequence {
    /// Set Graphics Mode (SGR) - colors, bold, etc.
    Sgr(Vec<u8>),
    
    /// Cursor Up
    CursorUp(u16),
    
    /// Cursor Down
    CursorDown(u16),
    
    /// Cursor Forward (Right)
    CursorForward(u16),
    
    /// Cursor Back (Left)
    CursorBack(u16),
    
    /// Cursor Position
    CursorPosition { row: u16, col: u16 },
    
    /// Erase Display
    EraseDisplay(u8),  // 0=cursor to end, 1=start to cursor, 2=entire screen
    
    /// Erase Line
    EraseLine(u8),
    
    /// Save Cursor Position
    SaveCursor,
    
    /// Restore Cursor Position
    RestoreCursor,
}

/// Parse a single ANSI escape sequence
pub fn parse_ansi_sequence(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    // ANSI sequences start with ESC [ (0x1b 0x5b)
    preceded(
        tag(b"\x1b["),
        alt((
            parse_sgr,
            parse_cursor_up,
            parse_cursor_down,
            parse_cursor_forward,
            parse_cursor_back,
            parse_cursor_position,
            parse_erase_display,
            parse_erase_line,
            parse_save_cursor,
            parse_restore_cursor,
        ))
    )(input)
}

/// Parse SGR (Select Graphic Rendition) - ESC[...m
fn parse_sgr(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    let (input, params) = delimited(
        opt(char(';')),
        separated_list0(
            char(';'),
            map_res(digit1, |s: &[u8]| {
                std::str::from_utf8(s)
                    .unwrap()
                    .parse::<u8>()
            })
        ),
        char('m')
    )(input)?;
    
    Ok((input, AnsiSequence::Sgr(params)))
}

/// Parse Cursor Up - ESC[<n>A
fn parse_cursor_up(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    let (input, n) = map_res(
        digit1,
        |s: &[u8]| std::str::from_utf8(s).unwrap().parse::<u16>()
    )(input)?;
    let (input, _) = char('A')(input)?;
    
    Ok((input, AnsiSequence::CursorUp(n)))
}

// ... similar parsers for other sequences ...

/// Parse Cursor Position - ESC[<row>;<col>H
fn parse_cursor_position(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    let (input, row) = map_res(
        digit1,
        |s: &[u8]| std::str::from_utf8(s).unwrap().parse::<u16>()
    )(input)?;
    let (input, _) = char(';')(input)?;
    let (input, col) = map_res(
        digit1,
        |s: &[u8]| std::str::from_utf8(s).unwrap().parse::<u16>()
    )(input)?;
    let (input, _) = char('H')(input)?;
    
    Ok((input, AnsiSequence::CursorPosition { row, col }))
}
```

### 4.2 ANSI Renderer Implementation

```rust
/// ANSI art renderer with state tracking
pub struct AnsiRenderer {
    terminal: Box<dyn TerminalDriver>,
    
    // Current rendering state
    cursor_x: u16,
    cursor_y: u16,
    foreground: AnsiColor,
    background: AnsiColor,
    bold: bool,
    blink: bool,
    
    // Saved cursor position (for save/restore)
    saved_x: u16,
    saved_y: u16,
}

impl AnsiRenderer {
    /// Render ANSI art from bytes
    pub async fn render(&mut self, data: &[u8]) -> Result<()> {
        let mut pos = 0;
        
        while pos < data.len() {
            // Look for ESC character
            if data[pos] == 0x1b && pos + 1 < data.len() && data[pos + 1] == b'[' {
                // Parse ANSI sequence
                match parse_ansi_sequence(&data[pos..]) {
                    Ok((remaining, sequence)) => {
                        let consumed = data.len() - pos - remaining.len();
                        self.apply_sequence(&sequence).await?;
                        pos += consumed;
                    }
                    Err(_) => {
                        // Invalid sequence, just output the character
                        self.write_char(data[pos] as char).await?;
                        pos += 1;
                    }
                }
            } else {
                // Regular character
                self.write_char(data[pos] as char).await?;
                pos += 1;
            }
        }
        
        Ok(())
    }
    
    /// Apply an ANSI sequence to the renderer state
    async fn apply_sequence(&mut self, seq: &AnsiSequence) -> Result<()> {
        match seq {
            AnsiSequence::Sgr(params) => {
                for &param in params {
                    match param {
                        0 => self.reset_style(),           // Reset
                        1 => self.bold = true,             // Bold
                        5 => self.blink = true,            // Blink
                        30..=37 => self.foreground = AnsiColor::from_code(param - 30),
                        40..=47 => self.background = AnsiColor::from_code(param - 40),
                        _ => {},  // Ignore unknown codes
                    }
                }
                self.update_terminal_color().await?;
            }
            
            AnsiSequence::CursorUp(n) => {
                self.cursor_y = self.cursor_y.saturating_sub(*n);
                self.terminal.move_cursor(self.cursor_x, self.cursor_y).await?;
            }
            
            AnsiSequence::CursorDown(n) => {
                self.cursor_y += n;
                self.terminal.move_cursor(self.cursor_x, self.cursor_y).await?;
            }
            
            AnsiSequence::CursorPosition { row, col } => {
                self.cursor_x = col.saturating_sub(1);  // ANSI is 1-indexed
                self.cursor_y = row.saturating_sub(1);
                self.terminal.move_cursor(self.cursor_x, self.cursor_y).await?;
            }
            
            AnsiSequence::EraseDisplay(mode) => {
                match mode {
                    2 => self.terminal.clear_screen().await?,
                    _ => {},  // TODO: Implement other modes
                }
            }
            
            AnsiSequence::SaveCursor => {
                self.saved_x = self.cursor_x;
                self.saved_y = self.cursor_y;
            }
            
            AnsiSequence::RestoreCursor => {
                self.cursor_x = self.saved_x;
                self.cursor_y = self.saved_y;
                self.terminal.move_cursor(self.cursor_x, self.cursor_y).await?;
            }
            
            _ => {},  // TODO: Implement remaining sequences
        }
        
        Ok(())
    }
    
    /// Write a single character and update cursor position
    async fn write_char(&mut self, ch: char) -> Result<()> {
        match ch {
            '\n' => {
                self.cursor_y += 1;
                self.cursor_x = 0;
            }
            '\r' => {
                self.cursor_x = 0;
            }
            _ => {
                self.terminal.write(&[ch as u8]).await?;
                self.cursor_x += 1;
            }
        }
        
        Ok(())
    }
}
```

### 4.3 SAUCE Metadata Parsing

SAUCE (Standard Architecture for Universal Comment Extensions) is metadata embedded in ANSI art files:

```rust
/// SAUCE record structure (128 bytes at end of file)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct SauceRecord {
    pub id: [u8; 5],           // "SAUCE"
    pub version: [u8; 2],      // "00"
    pub title: [u8; 35],
    pub author: [u8; 20],
    pub group: [u8; 20],
    pub date: [u8; 8],         // CCYYMMDD
    pub file_size: u32,
    pub data_type: u8,
    pub file_type: u8,
    pub t_info1: u16,          // Character width
    pub t_info2: u16,          // Number of lines
    pub t_info3: u16,
    pub t_info4: u16,
    pub comments: u8,          // Number of comment lines
    pub t_flags: u8,
    pub t_info_s: [u8; 22],
}

impl SauceRecord {
    /// Extract SAUCE record from file
    pub fn from_file(path: &Path) -> Result<Option<Self>> {
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        
        if file_size < 128 {
            return Ok(None);  // File too small
        }
        
        // SAUCE is in last 128 bytes
        file.seek(SeekFrom::End(-128))?;
        
        let mut buffer = [0u8; 128];
        file.read_exact(&mut buffer)?;
        
        // Check for "SAUCE" signature
        if &buffer[0..5] != b"SAUCE" {
            return Ok(None);
        }
        
        let sauce: SauceRecord = unsafe {
            std::ptr::read(buffer.as_ptr() as *const SauceRecord)
        };
        
        Ok(Some(sauce))
    }
    
    /// Get title as String
    pub fn title(&self) -> String {
        String::from_utf8_lossy(&self.title)
            .trim_end_matches('\0')
            .to_string()
    }
    
    /// Get author as String
    pub fn author(&self) -> String {
        String::from_utf8_lossy(&self.author)
            .trim_end_matches('\0')
            .to_string()
    }
}
```

---

## 5. File Transfer Protocol Specifications

### 5.1 Zmodem Protocol Implementation

Zmodem is a complex protocol. Here's a skeleton implementation:

```rust
/// Zmodem frame types
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ZmodemFrameType {
    ZRQINIT = b'0',  // Request receiver info
    ZRINIT  = b'1',  // Receiver info
    ZSINIT  = b'2',  // Sender info
    ZACK    = b'3',  // ACK to ZRQINIT
    ZFILE   = b'4',  // File header
    ZSKIP   = b'5',  // Skip file
    ZNAK    = b'6',  // Last packet was garbled
    ZABORT  = b'7',  // Abort batch transfer
    ZFIN    = b'8',  // Finish session
    ZRPOS   = b'9',  // Resume from position
    ZDATA   = b'A',  // Data packet(s) follow
    ZEOF    = b'B',  // End of file
    ZFERR   = b'C',  // Error in reading/writing file
    ZCRC    = b'D',  // Request for CRC
    ZCHALLENGE = b'E', // Challenge
    ZCOMPL  = b'F',  // Request is complete
    ZCAN    = b'G',  // Cancel (sent 5 times)
    ZFREECNT = b'H', // Free space
    ZCOMMAND = b'I', // Command
}

/// Zmodem protocol handler
pub struct ZmodemProtocol {
    stream: Box<dyn AsyncRead + AsyncWrite + Unpin + Send>,
    state: ZmodemState,
    config: ZmodemConfig,
    
    // Transfer state
    file_size: u64,
    bytes_transferred: u64,
    crc: u32,
}

#[derive(Debug, Clone)]
pub struct ZmodemConfig {
    pub block_size: usize,      // 1024 or 8192
    pub use_crc32: bool,         // CRC-32 vs CRC-16
    pub escape_control: bool,    // Escape control characters
}

impl ZmodemProtocol {
    /// Send a file using Zmodem
    pub async fn send_file(
        &mut self,
        path: &Path,
        progress_callback: Box<dyn ProgressCallback>,
    ) -> Result<TransferStats> {
        let start_time = Instant::now();
        
        // 1. Send ZRQINIT to request receiver info
        self.send_frame(ZmodemFrameType::ZRQINIT, &[]).await?;
        
        // 2. Expect ZRINIT response
        let response = self.receive_frame().await?;
        if response.frame_type != ZmodemFrameType::ZRINIT {
            return Err(Error::ProtocolError("Expected ZRINIT".into()));
        }
        
        // 3. Open file
        let mut file = File::open(path).await?;
        let metadata = file.metadata().await?;
        self.file_size = metadata.len();
        
        // 4. Send ZFILE with file info
        let file_info = self.build_file_header(path, &metadata)?;
        self.send_frame(ZmodemFrameType::ZFILE, &file_info).await?;
        
        // 5. Expect ZRPOS (receiver ready at position)
        let response = self.receive_frame().await?;
        match response.frame_type {
            ZmodemFrameType::ZSKIP => {
                return Err(Error::TransferCanceled("Receiver skipped file".into()));
            }
            ZmodemFrameType::ZRPOS => {
                let resume_pos = u32::from_le_bytes(response.data[0..4].try_into()?);
                file.seek(SeekFrom::Start(resume_pos as u64)).await?;
                self.bytes_transferred = resume_pos as u64;
            }
            _ => return Err(Error::ProtocolError("Unexpected response to ZFILE".into())),
        }
        
        // 6. Send file data in blocks
        loop {
            let mut buffer = vec![0u8; self.config.block_size];
            let bytes_read = file.read(&mut buffer).await?;
            
            if bytes_read == 0 {
                break;  // EOF
            }
            
            // Send ZDATA frame
            self.send_data_block(&buffer[..bytes_read]).await?;
            
            self.bytes_transferred += bytes_read as u64;
            progress_callback.update(self.bytes_transferred, self.file_size);
            
            // Check for ACK periodically
            if self.bytes_transferred % (self.config.block_size as u64 * 8) == 0 {
                let ack = self.receive_frame().await?;
                if ack.frame_type == ZNAK {
                    // Retransmit from last ACK position
                    // TODO: Implement retransmission
                }
            }
        }
        
        // 7. Send ZEOF
        let eof_pos = self.bytes_transferred as u32;
        self.send_frame(ZmodemFrameType::ZEOF, &eof_pos.to_le_bytes()).await?;
        
        // 8. Expect ZRINIT (receiver ready for next file)
        let response = self.receive_frame().await?;
        if response.frame_type != ZmodemFrameType::ZRINIT {
            return Err(Error::ProtocolError("Expected ZRINIT after EOF".into()));
        }
        
        // 9. Send ZFIN (no more files)
        self.send_frame(ZmodemFrameType::ZFIN, &[]).await?;
        
        let duration = start_time.elapsed();
        
        Ok(TransferStats {
            bytes_transferred: self.bytes_transferred,
            duration,
            bytes_per_second: (self.bytes_transferred as f64 / duration.as_secs_f64()) as u64,
            errors: 0,
            retries: 0,
        })
    }
    
    /// Send a Zmodem frame
    async fn send_frame(&mut self, frame_type: ZmodemFrameType, data: &[u8]) -> Result<()> {
        // Zmodem frame format:
        // *<DLE> <frame_type> <data> <CRC>
        
        let mut frame = Vec::new();
        frame.push(b'*');  // ZPAD
        frame.push(0x18);  // ZDLE (Ctrl-X)
        frame.push(frame_type as u8);
        
        // Add data
        for &byte in data {
            if self.needs_escaping(byte) {
                frame.push(0x18);  // ZDLE
                frame.push(byte ^ 0x40);  // Escape by XOR with 0x40
            } else {
                frame.push(byte);
            }
        }
        
        // Calculate and append CRC
        let crc = if self.config.use_crc32 {
            crc32fast::hash(&frame[2..])
        } else {
            crc16(&frame[2..]) as u32
        };
        
        frame.extend_from_slice(&crc.to_le_bytes());
        
        // Send frame
        self.stream.write_all(&frame).await?;
        self.stream.flush().await?;
        
        Ok(())
    }
    
    /// Receive a Zmodem frame
    async fn receive_frame(&mut self) -> Result<ZmodemFrame> {
        // Read until we find ZPAD ZDLE sequence
        loop {
            let byte = self.read_byte().await?;
            if byte == b'*' {
                let next = self.read_byte().await?;
                if next == 0x18 {  // ZDLE
                    break;
                }
            }
        }
        
        // Read frame type
        let frame_type_byte = self.read_byte().await?;
        let frame_type = ZmodemFrameType::from_u8(frame_type_byte)?;
        
        // Read data until CRC
        let mut data = Vec::new();
        // ... (complex state machine to read escaped data) ...
        
        // Verify CRC
        // ... (CRC verification) ...
        
        Ok(ZmodemFrame {
            frame_type,
            data,
        })
    }
}

/// CRC-16 calculation (CCITT polynomial)
fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    
    crc
}
```

**Note**: Full Zmodem implementation is ~2000+ lines. Consider using an existing crate if available, or implementing incrementally with extensive testing.

---

## 6. Message Base Format Deep Dive

### 6.1 JAM Message Base Structure

JAM consists of four files per message area:

```
area.jhr  - Header file (message metadata)
area.jdt  - Text file (message bodies)
area.jdx  - Index file (for quick lookups)
area.jlr  - Last-read pointers (per-user tracking)
```

**JAM Header Record**:

```rust
/// JAM message header (on disk)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct JamMsgHeader {
    pub signature: [u8; 4],    // "JAM\0"
    pub revision: u16,          // 1
    pub reserved: u16,
    pub subfieldlen: u32,       // Length of subfields
    pub timesread: u32,
    pub msgidcrc: u32,          // CRC of MSGID
    pub replycrc: u32,          // CRC of REPLY
    pub replyto: u32,           // Message number this replies to
    pub reply1st: u32,          // First reply to this message
    pub replynext: u32,         // Next reply in thread
    pub datewritten: u32,       // Unix timestamp
    pub datereceived: u32,
    pub dateprocessed: u32,
    pub messagenumber: u32,
    pub attribute: u32,         // Flags (private, etc.)
    pub attribute2: u32,
    pub textoffset: u32,        // Offset in .JDT file
    pub textlen: u32,           // Length of message text
    pub passwordcrc: u32,
    pub cost: u32,
}

const JAM_HEADER_SIZE: usize = std::mem::size_of::<JamMsgHeader>();

/// JAM subfield (variable-length header extensions)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct JamSubfield {
    pub loid: u16,              // Subfield type (0=from, 1=to, 2=subject, etc.)
    pub hid: u16,
    pub datlen: u32,            // Length of data
    // Followed by `datlen` bytes of data
}
```

**Implementation**:

```rust
use std::collections::HashMap;

pub struct JamMessageBase {
    base_path: PathBuf,
    
    // File handles
    header_file: File,
    text_file: File,
    index_file: File,
    lastread_file: File,
    
    // In-memory indices (built on startup for performance)
    msg_index: BTreeMap<u32, u64>,  // msg_number -> header_offset
    thread_index: HashMap<u32, Vec<u32>>,  // msg_number -> reply_numbers
}

impl JamMessageBase {
    /// Open a JAM message base
    pub async fn open(base_path: &Path) -> Result<Self> {
        let header_path = base_path.with_extension("jhr");
        let text_path = base_path.with_extension("jdt");
        let index_path = base_path.with_extension("jdx");
        let lastread_path = base_path.with_extension("jlr");
        
        // Open files with read/write access
        let header_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&header_path)?;
        
        let text_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&text_path)?;
        
        let index_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&index_path)?;
        
        let lastread_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&lastread_path)?;
        
        let mut base = Self {
            base_path: base_path.to_path_buf(),
            header_file,
            text_file,
            index_file,
            lastread_file,
            msg_index: BTreeMap::new(),
            thread_index: HashMap::new(),
        };
        
        // Build indices
        base.build_indices().await?;
        
        Ok(base)
    }
    
    /// Build in-memory indices by scanning header file
    async fn build_indices(&mut self) -> Result<()> {
        self.header_file.seek(SeekFrom::Start(0))?;
        
        let mut offset = 0u64;
        loop {
            // Read header
            let mut header_buf = [0u8; JAM_HEADER_SIZE];
            match self.header_file.read_exact(&mut header_buf) {
                Ok(_) => {},
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            }
            
            let header: JamMsgHeader = unsafe {
                std::ptr::read(header_buf.as_ptr() as *const JamMsgHeader)
            };
            
            // Verify signature
            if &header.signature != b"JAM\0" {
                return Err(Error::CorruptMessageBase("Invalid JAM signature".into()));
            }
            
            // Add to message index
            self.msg_index.insert(header.messagenumber, offset);
            
            // Build thread index
            if header.replyto != 0 {
                self.thread_index
                    .entry(header.replyto)
                    .or_insert_with(Vec::new)
                    .push(header.messagenumber);
            }
            
            // Skip subfields
            let skip_len = header.subfieldlen as u64;
            self.header_file.seek(SeekFrom::Current(skip_len as i64))?;
            
            offset += JAM_HEADER_SIZE as u64 + skip_len;
        }
        
        info!("Built JAM indices: {} messages", self.msg_index.len());
        Ok(())
    }
    
    /// Read a message by number
    pub async fn read_message(&mut self, msg_num: u32) -> Result<Message> {
        let offset = self.msg_index
            .get(&msg_num)
            .ok_or_else(|| Error::MessageNotFound(msg_num))?;
        
        // Seek to header
        self.header_file.seek(SeekFrom::Start(*offset))?;
        
        // Read header
        let mut header_buf = [0u8; JAM_HEADER_SIZE];
        self.header_file.read_exact(&mut header_buf)?;
        
        let header: JamMsgHeader = unsafe {
            std::ptr::read(header_buf.as_ptr() as *const JamMsgHeader)
        };
        
        // Read subfields to extract from/to/subject
        let mut from = String::new();
        let mut to = String::new();
        let mut subject = String::new();
        
        let mut remaining = header.subfieldlen as usize;
        while remaining > 0 {
            let mut subfield_buf = [0u8; 8];
            self.header_file.read_exact(&mut subfield_buf)?;
            
            let subfield: JamSubfield = unsafe {
                std::ptr::read(subfield_buf.as_ptr() as *const JamSubfield)
            };
            
            let mut data = vec![0u8; subfield.datlen as usize];
            self.header_file.read_exact(&mut data)?;
            
            match subfield.loid {
                0 => from = String::from_utf8_lossy(&data).to_string(),
                1 => to = String::from_utf8_lossy(&data).to_string(),
                2 => subject = String::from_utf8_lossy(&data).to_string(),
                _ => {},  // Ignore other subfields
            }
            
            remaining -= 8 + subfield.datlen as usize;
        }
        
        // Read message text from .JDT file
        self.text_file.seek(SeekFrom::Start(header.textoffset as u64))?;
        let mut text_buf = vec![0u8; header.textlen as usize];
        self.text_file.read_exact(&mut text_buf)?;
        
        let body = String::from_utf8_lossy(&text_buf).to_string();
        
        // Build Message object
        Ok(Message {
            id: header.messagenumber,
            area: "".to_string(),  // TODO: Track area name
            from,
            to,
            subject,
            date_written: DateTime::from_timestamp(header.datewritten as i64, 0)
                .unwrap_or_default(),
            body,
            reply_to: if header.replyto != 0 { Some(header.replyto) } else { None },
            flags: MessageFlags::from_bits_truncate(header.attribute),
        })
    }
}
```

---

## 7. Performance Optimization Techniques

### 7.1 Connection Pool for Database

```rust
use sqlx::sqlite::SqlitePoolOptions;

/// Create optimized connection pool
pub async fn create_db_pool(database_url: &str) -> Result<SqlitePool> {
    SqlitePoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(600))
        .connect(database_url)
        .await
        .map_err(Into::into)
}
```

### 7.2 Caching Strategy

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

/// Cache for frequently accessed data
pub struct DataCache {
    users: Arc<Mutex<LruCache<u32, User>>>,
    files: Arc<Mutex<LruCache<u64, FileRecord>>>,
}

impl DataCache {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(
                LruCache::new(NonZeroUsize::new(1000).unwrap())
            )),
            files: Arc::new(Mutex::new(
                LruCache::new(NonZeroUsize::new(5000).unwrap())
            )),
        }
    }
    
    pub async fn get_user(&self, id: u32, storage: &dyn Storage) -> Result<User> {
        // Check cache first
        if let Some(user) = self.users.lock().await.get(&id) {
            return Ok(user.clone());
        }
        
        // Miss: fetch from storage
        let user = storage.get_user(id).await?
            .ok_or_else(|| Error::UserNotFound(id))?;
        
        // Store in cache
        self.users.lock().await.put(id, user.clone());
        
        Ok(user)
    }
}
```

### 7.3 Async File I/O

```rust
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Async file reading for large files
pub async fn read_file_async(path: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}

/// Async file writing with fsync
pub async fn write_file_async(path: &Path, data: &[u8]) -> Result<()> {
    let mut file = File::create(path).await?;
    file.write_all(data).await?;
    file.sync_all().await?;  // Ensure written to disk
    Ok(())
}
```

---

## 8. Testing Methodologies

### 8.1 Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pascal_string_conversion() {
        let ps = PascalString::<30>::from_str("TestUser").unwrap();
        assert_eq!(ps.length, 8);
        assert_eq!(ps.to_string(), "TestUser");
    }
    
    #[test]
    fn test_pascal_string_too_long() {
        let long_str = "a".repeat(31);
        let result = PascalString::<30>::from_str(&long_str);
        assert!(result.is_err());
    }
}
```

### 8.2 Integration Test Example

```rust
// tests/integration_test.rs

use impulse_bbs::*;
use tokio::net::TcpStream;

#[tokio::test]
async fn test_login_flow() {
    // Start test server in background
    let server = spawn_test_server().await;
    
    // Connect
    let stream = TcpStream::connect("127.0.0.1:2323").await.unwrap();
    let mut client = TestClient::new(stream);
    
    // Wait for welcome
    let welcome = client.read_until_prompt().await.unwrap();
    assert!(welcome.contains("Impulse BBS"));
    
    // Login
    client.send_line("testuser").await.unwrap();
    client.send_line("password123").await.unwrap();
    
    // Verify main menu
    let menu = client.read_until_prompt().await.unwrap();
    assert!(menu.contains("Main Menu"));
    
    // Cleanup
    server.shutdown().await;
}
```

### 8.3 Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_ansi_parser_never_panics(s in "\\PC*") {
        let _ = parse_ansi_sequence(s.as_bytes());
    }
    
    #[test]
    fn test_crc16_deterministic(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let crc1 = crc16(&data);
        let crc2 = crc16(&data);
        assert_eq!(crc1, crc2);
    }
}
```

---

## 9. Build System Configuration

### 9.1 Workspace Cargo.toml

```toml
[workspace]
members = [
    "crates/impulse-core",
    "crates/impulse-session",
    "crates/impulse-terminal",
    "crates/impulse-message",
    "crates/impulse-files",
    "crates/impulse-user",
    "crates/impulse-config",
    "crates/impulse-storage",
    "crates/impulse-protocol",
    "crates/impulse-door",
    "crates/impulse-telnet",
    "crates/impulse-ssh",
    "crates/impulse-api",
    "crates/impulse-legacy",
    "crates/impulse-cli",
    "impulse-server",
]

[workspace.package]
version = "1.0.0"
edition = "2021"
license = "MIT OR Apache-2.0"
authors = ["Impulse Modernization Team"]

[workspace.dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }
tokio-util = "0.7"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"
toml = "0.8"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "sqlite", "postgres"] }

# Networking
rustls = "0.21"
tokio-rustls = "0.24"

# Utilities
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
bitflags = "2.4"
bytes = "1.5"
tracing = "0.1"
tracing-subscriber = "0.3"

# Parsing
nom = "7.1"

# Encoding
encoding_rs = "0.8"

# CRC
crc32fast = "1.3"

# Caching
lru = "0.12"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### 9.2 CI/CD Pipeline (GitHub Actions)

```yaml
# .github/workflows/ci.yml

name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, nightly]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
    
    - name: Build
      run: cargo build --verbose --all-features
    
    - name: Run tests
      run: cargo test --verbose --all-features
    
    - name: Run integration tests
      run: cargo test --test '*' --verbose
  
  coverage:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin
    
    - name: Generate coverage
      run: cargo tarpaulin --out Xml --all-features
    
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        files: ./cobertura.xml
```

---

## 10. Deployment Recipes

### 10.1 Systemd Service Unit

```ini
# /etc/systemd/system/impulse-bbs.service

[Unit]
Description=Impulse BBS Server
After=network.target

[Service]
Type=simple
User=impulse
Group=impulse
WorkingDirectory=/opt/impulse-bbs
ExecStart=/opt/impulse-bbs/impulse-server --config /etc/impulse/impulse.toml
Restart=on-failure
RestartSec=5

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/impulse

[Install]
WantedBy=multi-user.target
```

### 10.2 Docker Compose Setup

```yaml
# docker-compose.yml

version: '3.8'

services:
  impulse-bbs:
    image: impulse-bbs:latest
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "2323:23"    # Telnet
      - "2222:22"    # SSH
      - "8080:8080"  # HTTP API
    volumes:
      - ./data:/var/lib/impulse
      - ./config:/etc/impulse
    environment:
      - RUST_LOG=info
      - DATABASE_URL=sqlite:///var/lib/impulse/impulse.db
    restart: unless-stopped
    
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - impulse-bbs
    restart: unless-stopped
```

### 10.3 Kubernetes Deployment

```yaml
# k8s/deployment.yaml

apiVersion: v1
kind: ConfigMap
metadata:
  name: impulse-config
data:
  impulse.toml: |
    system_name = "Impulse BBS"
    telnet_port = 23
    ssh_port = 22
    max_connections = 100
    database_url = "postgresql://impulse:password@postgres:5432/impulse"

---

apiVersion: apps/v1
kind: Deployment
metadata:
  name: impulse-bbs
  labels:
    app: impulse-bbs
spec:
  replicas: 2
  selector:
    matchLabels:
      app: impulse-bbs
  template:
    metadata:
      labels:
        app: impulse-bbs
    spec:
      containers:
      - name: impulse
        image: impulse-bbs:1.0.0
        ports:
        - containerPort: 23
          name: telnet
        - containerPort: 22
          name: ssh
        - containerPort: 8080
          name: http
        volumeMounts:
        - name: config
          mountPath: /etc/impulse
        - name: data
          mountPath: /var/lib/impulse
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 30
      volumes:
      - name: config
        configMap:
          name: impulse-config
      - name: data
        persistentVolumeClaim:
          claimName: impulse-data

---

apiVersion: v1
kind: Service
metadata:
  name: impulse-bbs
spec:
  type: LoadBalancer
  selector:
    app: impulse-bbs
  ports:
  - name: telnet
    port: 23
    targetPort: 23
  - name: ssh
    port: 22
    targetPort: 22
  - name: http
    port: 80
    targetPort: 8080
```

---

## Conclusion

This technical specification provides the low-level implementation details necessary to convert Impulse BBS from Borland Pascal 7.0 to modern Rust. The document covers:

- Precise type mappings between Pascal and Rust
- Binary file format handling with proper memory layout
- Replacement strategies for DOS interrupts using modern crates
- Complete ANSI rendering engine with parser
- File transfer protocol specifications (Zmodem focus)
- Message base format deep-dive (JAM)
- Performance optimization techniques
- Comprehensive testing methodologies
- Build system configuration
- Production deployment recipes

These specifications serve as a reference implementation guide for developers working on the conversion project, ensuring consistency, safety, and modern best practices.

---

**Document Version**: 1.0  
**Last Updated**: 2025-01-21  
**Author**: Impulse Modernization Team - Technical Specifications  
**Status**: Reference Implementation
