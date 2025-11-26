//! Zmodem protocol state machine.
//!
//! This module implements the state machine that tracks the progress
//! of a Zmodem session through its various phases.

use super::error::{Result, ZmodemError};
use super::file::ZmodemFileInfo;
use super::frame::{FrameType, ZmodemFrame};
use super::negotiate::NegotiatedParams;

/// Zmodem protocol state.
///
/// Tracks the current phase of the protocol session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZmodemState {
    /// Idle state, no transfer in progress
    Idle,

    /// Sent ZRQINIT, waiting for ZRINIT response
    InitSent,

    /// Received ZRINIT, ready to send files
    InitReceived,

    /// Sent ZFILE, waiting for ZRPOS or ZSKIP response
    FileHeaderSent,

    /// Transferring file data
    DataTransfer,

    /// Sent ZEOF, waiting for ZRINIT for next file
    FileComplete,

    /// All files transferred, session complete
    SessionComplete,

    /// Error state
    Error,
}

impl ZmodemState {
    /// Check if state represents an active transfer.
    pub fn is_active(self) -> bool {
        !matches!(
            self,
            ZmodemState::Idle | ZmodemState::SessionComplete | ZmodemState::Error
        )
    }

    /// Check if state allows sending data.
    pub fn can_send_data(self) -> bool {
        matches!(self, ZmodemState::DataTransfer)
    }

    /// Check if state is terminal (no further transitions).
    pub fn is_terminal(self) -> bool {
        matches!(self, ZmodemState::SessionComplete | ZmodemState::Error)
    }
}

/// Zmodem protocol state machine.
///
/// Manages state transitions and tracks session progress.
///
/// # Examples
///
/// ```
/// use impulse_protocol::zmodem::state::{ZmodemStateMachine, ZmodemState};
/// use impulse_protocol::zmodem::init::ZmodemInit;
///
/// let mut sm = ZmodemStateMachine::new();
/// assert_eq!(sm.state(), ZmodemState::Idle);
///
/// // Send init
/// let frame = ZmodemInit::create_zrqinit();
/// sm.advance(ZmodemState::InitSent);
/// assert_eq!(sm.state(), ZmodemState::InitSent);
/// ```
pub struct ZmodemStateMachine {
    state: ZmodemState,
    negotiated: Option<NegotiatedParams>,
    current_file: Option<ZmodemFileInfo>,
    position: u64,
}

impl ZmodemStateMachine {
    /// Create a new state machine in idle state.
    pub fn new() -> Self {
        Self {
            state: ZmodemState::Idle,
            negotiated: None,
            current_file: None,
            position: 0,
        }
    }

    /// Get the current state.
    pub fn state(&self) -> ZmodemState {
        self.state
    }

    /// Get negotiated parameters (if available).
    pub fn negotiated(&self) -> Option<&NegotiatedParams> {
        self.negotiated.as_ref()
    }

    /// Get current file info (if transferring).
    pub fn current_file(&self) -> Option<&ZmodemFileInfo> {
        self.current_file.as_ref()
    }

    /// Get current file position.
    pub fn position(&self) -> u64 {
        self.position
    }

    /// Set negotiated parameters.
    pub fn set_negotiated(&mut self, params: NegotiatedParams) {
        self.negotiated = Some(params);
    }

    /// Set current file.
    pub fn set_current_file(&mut self, file: ZmodemFileInfo) {
        self.current_file = Some(file);
        self.position = 0;
    }

    /// Clear current file.
    pub fn clear_current_file(&mut self) {
        self.current_file = None;
        self.position = 0;
    }

    /// Set file position.
    pub fn set_position(&mut self, pos: u64) {
        self.position = pos;
    }

    /// Advance to a new state.
    ///
    /// # Arguments
    ///
    /// * `new_state` - State to transition to
    ///
    /// # Panics
    ///
    /// Panics in debug builds if transition is invalid
    pub fn advance(&mut self, new_state: ZmodemState) {
        debug_assert!(
            self.is_valid_transition(new_state),
            "Invalid state transition: {:?} -> {:?}",
            self.state,
            new_state
        );

        self.state = new_state;
    }

    /// Handle an incoming frame and return response frame if needed.
    ///
    /// Processes the frame according to current state and protocol rules.
    ///
    /// # Arguments
    ///
    /// * `frame` - Incoming frame to process
    ///
    /// # Returns
    ///
    /// Optional response frame to send, or error if frame is invalid for current state
    ///
    /// # Errors
    ///
    /// Returns error if frame is unexpected for current state
    pub fn handle_frame(&mut self, frame: &ZmodemFrame) -> Result<Option<ZmodemFrame>> {
        match (self.state, frame.frame_type) {
            // Idle state: Expect ZRQINIT to start session
            (ZmodemState::Idle, FrameType::ZRQINIT) => {
                self.state = ZmodemState::InitReceived;
                // Caller should send ZRINIT
                Ok(None)
            }

            // InitSent state: Expect ZRINIT response
            (ZmodemState::InitSent, FrameType::ZRINIT) => {
                self.state = ZmodemState::InitReceived;
                // Negotiation complete, ready to send files
                Ok(None)
            }

            // FileHeaderSent: Expect ZRPOS or ZSKIP
            (ZmodemState::FileHeaderSent, FrameType::ZRPOS) => {
                // Extract position from flags
                let pos = frame.flags_as_u32() as u64;
                self.position = pos;
                self.state = ZmodemState::DataTransfer;
                Ok(None)
            }

            (ZmodemState::FileHeaderSent, FrameType::ZSKIP) => {
                // Skip this file
                self.clear_current_file();
                self.state = ZmodemState::InitReceived;
                Ok(None)
            }

            // DataTransfer: Expect ZACK
            (ZmodemState::DataTransfer, FrameType::ZACK) => {
                // Acknowledgment received, continue transfer
                Ok(None)
            }

            // FileComplete: Expect ZRINIT for next file
            (ZmodemState::FileComplete, FrameType::ZRINIT) => {
                self.clear_current_file();
                self.state = ZmodemState::InitReceived;
                Ok(None)
            }

            // ZFIN can be received anytime to finish session
            (_, FrameType::ZFIN) => {
                self.state = ZmodemState::SessionComplete;
                Ok(None)
            }

            // ZCAN cancels transfer
            (_, FrameType::ZCAN) | (_, FrameType::ZABORT) => {
                self.state = ZmodemState::Error;
                Err(ZmodemError::Cancelled)
            }

            // Unexpected frame for current state
            _ => Err(ZmodemError::InvalidFrame(format!(
                "Unexpected frame {:?} in state {:?}",
                frame.frame_type, self.state
            ))),
        }
    }

    /// Check if a state transition is valid.
    fn is_valid_transition(&self, new_state: ZmodemState) -> bool {
        use ZmodemState::*;

        match (self.state, new_state) {
            // From Idle
            (Idle, InitSent) => true,
            (Idle, InitReceived) => true,

            // From InitSent
            (InitSent, InitReceived) => true,
            (InitSent, Error) => true,

            // From InitReceived
            (InitReceived, FileHeaderSent) => true,
            (InitReceived, SessionComplete) => true,
            (InitReceived, Error) => true,

            // From FileHeaderSent
            (FileHeaderSent, DataTransfer) => true,
            (FileHeaderSent, InitReceived) => true, // ZSKIP
            (FileHeaderSent, Error) => true,

            // From DataTransfer
            (DataTransfer, FileComplete) => true,
            (DataTransfer, Error) => true,

            // From FileComplete
            (FileComplete, InitReceived) => true,
            (FileComplete, SessionComplete) => true,
            (FileComplete, Error) => true,

            // Terminal states
            (SessionComplete, _) => false,
            (Error, _) => false,

            // Invalid transitions
            _ => false,
        }
    }

    /// Reset state machine to idle.
    pub fn reset(&mut self) {
        self.state = ZmodemState::Idle;
        self.negotiated = None;
        self.current_file = None;
        self.position = 0;
    }
}

impl Default for ZmodemStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zmodem::frame::FrameEncoding;

    #[test]
    fn test_state_is_active() {
        assert!(!ZmodemState::Idle.is_active());
        assert!(ZmodemState::InitSent.is_active());
        assert!(ZmodemState::InitReceived.is_active());
        assert!(ZmodemState::FileHeaderSent.is_active());
        assert!(ZmodemState::DataTransfer.is_active());
        assert!(ZmodemState::FileComplete.is_active());
        assert!(!ZmodemState::SessionComplete.is_active());
        assert!(!ZmodemState::Error.is_active());
    }

    #[test]
    fn test_state_can_send_data() {
        assert!(!ZmodemState::Idle.can_send_data());
        assert!(!ZmodemState::InitSent.can_send_data());
        assert!(!ZmodemState::InitReceived.can_send_data());
        assert!(!ZmodemState::FileHeaderSent.can_send_data());
        assert!(ZmodemState::DataTransfer.can_send_data());
        assert!(!ZmodemState::FileComplete.can_send_data());
        assert!(!ZmodemState::SessionComplete.can_send_data());
        assert!(!ZmodemState::Error.can_send_data());
    }

    #[test]
    fn test_state_is_terminal() {
        assert!(!ZmodemState::Idle.is_terminal());
        assert!(!ZmodemState::InitSent.is_terminal());
        assert!(ZmodemState::SessionComplete.is_terminal());
        assert!(ZmodemState::Error.is_terminal());
    }

    #[test]
    fn test_new() {
        let sm = ZmodemStateMachine::new();
        assert_eq!(sm.state(), ZmodemState::Idle);
        assert!(sm.negotiated().is_none());
        assert!(sm.current_file().is_none());
        assert_eq!(sm.position(), 0);
    }

    #[test]
    fn test_advance() {
        let mut sm = ZmodemStateMachine::new();

        sm.advance(ZmodemState::InitSent);
        assert_eq!(sm.state(), ZmodemState::InitSent);

        sm.advance(ZmodemState::InitReceived);
        assert_eq!(sm.state(), ZmodemState::InitReceived);
    }

    #[test]
    fn test_set_current_file() {
        let mut sm = ZmodemStateMachine::new();
        let file = ZmodemFileInfo::new("test.txt", 1024);

        sm.set_current_file(file.clone());

        assert_eq!(sm.current_file().unwrap().name, "test.txt");
        assert_eq!(sm.position(), 0);
    }

    #[test]
    fn test_clear_current_file() {
        let mut sm = ZmodemStateMachine::new();
        let file = ZmodemFileInfo::new("test.txt", 1024);

        sm.set_current_file(file);
        sm.set_position(512);

        sm.clear_current_file();

        assert!(sm.current_file().is_none());
        assert_eq!(sm.position(), 0);
    }

    #[test]
    fn test_set_position() {
        let mut sm = ZmodemStateMachine::new();

        sm.set_position(1024);
        assert_eq!(sm.position(), 1024);

        sm.set_position(2048);
        assert_eq!(sm.position(), 2048);
    }

    #[test]
    fn test_handle_zrqinit() {
        let mut sm = ZmodemStateMachine::new();
        let frame = ZmodemFrame::with_defaults(FrameType::ZRQINIT, FrameEncoding::Hex);

        let result = sm.handle_frame(&frame);
        assert!(result.is_ok());
        assert_eq!(sm.state(), ZmodemState::InitReceived);
    }

    #[test]
    fn test_handle_zrinit() {
        let mut sm = ZmodemStateMachine::new();
        sm.advance(ZmodemState::InitSent);

        let frame = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);

        let result = sm.handle_frame(&frame);
        assert!(result.is_ok());
        assert_eq!(sm.state(), ZmodemState::InitReceived);
    }

    #[test]
    fn test_handle_zrpos() {
        let mut sm = ZmodemStateMachine::new();
        sm.advance(ZmodemState::InitSent);
        sm.advance(ZmodemState::InitReceived);
        sm.advance(ZmodemState::FileHeaderSent);

        let mut frame = ZmodemFrame::with_defaults(FrameType::ZRPOS, FrameEncoding::Bin16);
        frame.set_flags_from_u32(512); // Position = 512

        let result = sm.handle_frame(&frame);
        assert!(result.is_ok());
        assert_eq!(sm.state(), ZmodemState::DataTransfer);
        assert_eq!(sm.position(), 512);
    }

    #[test]
    fn test_handle_zskip() {
        let mut sm = ZmodemStateMachine::new();
        sm.advance(ZmodemState::InitSent);
        sm.advance(ZmodemState::InitReceived);
        sm.advance(ZmodemState::FileHeaderSent);

        let file = ZmodemFileInfo::new("skip.txt", 1024);
        sm.set_current_file(file);

        let frame = ZmodemFrame::with_defaults(FrameType::ZSKIP, FrameEncoding::Hex);

        let result = sm.handle_frame(&frame);
        assert!(result.is_ok());
        assert_eq!(sm.state(), ZmodemState::InitReceived);
        assert!(sm.current_file().is_none());
    }

    #[test]
    fn test_handle_zack() {
        let mut sm = ZmodemStateMachine::new();
        sm.advance(ZmodemState::InitSent);
        sm.advance(ZmodemState::InitReceived);
        sm.advance(ZmodemState::FileHeaderSent);
        sm.advance(ZmodemState::DataTransfer);

        let frame = ZmodemFrame::with_defaults(FrameType::ZACK, FrameEncoding::Bin16);

        let result = sm.handle_frame(&frame);
        assert!(result.is_ok());
        assert_eq!(sm.state(), ZmodemState::DataTransfer);
    }

    #[test]
    fn test_handle_zfin() {
        let mut sm = ZmodemStateMachine::new();
        sm.advance(ZmodemState::InitSent);
        sm.advance(ZmodemState::InitReceived);

        let frame = ZmodemFrame::with_defaults(FrameType::ZFIN, FrameEncoding::Hex);

        let result = sm.handle_frame(&frame);
        assert!(result.is_ok());
        assert_eq!(sm.state(), ZmodemState::SessionComplete);
    }

    #[test]
    fn test_handle_zcan() {
        let mut sm = ZmodemStateMachine::new();
        sm.advance(ZmodemState::InitSent);
        sm.advance(ZmodemState::InitReceived);

        let frame = ZmodemFrame::with_defaults(FrameType::ZCAN, FrameEncoding::Hex);

        let result = sm.handle_frame(&frame);
        assert!(result.is_err());
        assert_eq!(sm.state(), ZmodemState::Error);
    }

    #[test]
    fn test_handle_unexpected_frame() {
        let mut sm = ZmodemStateMachine::new();
        sm.advance(ZmodemState::InitSent);
        sm.advance(ZmodemState::InitReceived);
        sm.advance(ZmodemState::FileHeaderSent);
        sm.advance(ZmodemState::DataTransfer);

        let frame = ZmodemFrame::with_defaults(FrameType::ZRQINIT, FrameEncoding::Hex);

        let result = sm.handle_frame(&frame);
        assert!(result.is_err());
    }

    #[test]
    fn test_reset() {
        let mut sm = ZmodemStateMachine::new();

        sm.advance(ZmodemState::InitSent);
        sm.advance(ZmodemState::InitReceived);
        sm.advance(ZmodemState::FileHeaderSent);
        sm.advance(ZmodemState::DataTransfer);
        sm.set_position(1024);

        let file = ZmodemFileInfo::new("test.txt", 2048);
        sm.set_current_file(file);

        sm.reset();

        assert_eq!(sm.state(), ZmodemState::Idle);
        assert!(sm.current_file().is_none());
        assert_eq!(sm.position(), 0);
    }

    #[test]
    fn test_valid_transitions() {
        let mut sm = ZmodemStateMachine::new();

        // Valid: Idle -> InitSent
        assert!(sm.is_valid_transition(ZmodemState::InitSent));
        sm.advance(ZmodemState::InitSent);

        // Valid: InitSent -> InitReceived
        assert!(sm.is_valid_transition(ZmodemState::InitReceived));
        sm.advance(ZmodemState::InitReceived);

        // Valid: InitReceived -> FileHeaderSent
        assert!(sm.is_valid_transition(ZmodemState::FileHeaderSent));
        sm.advance(ZmodemState::FileHeaderSent);

        // Valid: FileHeaderSent -> DataTransfer
        assert!(sm.is_valid_transition(ZmodemState::DataTransfer));
        sm.advance(ZmodemState::DataTransfer);

        // Valid: DataTransfer -> FileComplete
        assert!(sm.is_valid_transition(ZmodemState::FileComplete));
        sm.advance(ZmodemState::FileComplete);

        // Valid: FileComplete -> SessionComplete
        assert!(sm.is_valid_transition(ZmodemState::SessionComplete));
    }

    #[test]
    fn test_invalid_transitions() {
        let sm = ZmodemStateMachine::new();

        // Invalid: Idle -> DataTransfer
        assert!(!sm.is_valid_transition(ZmodemState::DataTransfer));

        // Invalid: from terminal states
        let mut sm = ZmodemStateMachine::new();
        sm.advance(ZmodemState::InitSent);
        sm.advance(ZmodemState::InitReceived);
        sm.advance(ZmodemState::SessionComplete);

        assert!(!sm.is_valid_transition(ZmodemState::Idle));
        assert!(!sm.is_valid_transition(ZmodemState::InitSent));
    }

    #[test]
    fn test_full_session_flow() {
        let mut sm = ZmodemStateMachine::new();

        // 1. Send ZRQINIT
        assert_eq!(sm.state(), ZmodemState::Idle);
        sm.advance(ZmodemState::InitSent);

        // 2. Receive ZRINIT
        let zrinit = ZmodemFrame::with_defaults(FrameType::ZRINIT, FrameEncoding::Hex);
        sm.handle_frame(&zrinit).unwrap();
        assert_eq!(sm.state(), ZmodemState::InitReceived);

        // 3. Send ZFILE
        sm.advance(ZmodemState::FileHeaderSent);

        // 4. Receive ZRPOS
        let mut zrpos = ZmodemFrame::with_defaults(FrameType::ZRPOS, FrameEncoding::Bin16);
        zrpos.set_flags_from_u32(0);
        sm.handle_frame(&zrpos).unwrap();
        assert_eq!(sm.state(), ZmodemState::DataTransfer);

        // 5. Send data, receive ZACKs
        let zack = ZmodemFrame::with_defaults(FrameType::ZACK, FrameEncoding::Bin16);
        sm.handle_frame(&zack).unwrap();

        // 6. Send ZEOF
        sm.advance(ZmodemState::FileComplete);

        // 7. Receive ZFIN
        let zfin = ZmodemFrame::with_defaults(FrameType::ZFIN, FrameEncoding::Hex);
        sm.handle_frame(&zfin).unwrap();
        assert_eq!(sm.state(), ZmodemState::SessionComplete);
    }
}
