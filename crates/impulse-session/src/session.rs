//! Session data structures

use std::fmt;
use std::time::{Duration, Instant};

/// Unique session identifier (UUID v4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(uuid::Uuid);

impl SessionId {
    /// Generate a new random session ID
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Create from existing UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SessionId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(s)?))
    }
}

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Session created, waiting for authentication
    Connected,
    /// User is authenticating
    Authenticating,
    /// User authenticated successfully
    Authenticated,
    /// User is active in the system
    Active,
    /// Session idle (no activity)
    Idle,
    /// Session terminated
    Terminated,
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connected => write!(f, "Connected"),
            Self::Authenticating => write!(f, "Authenticating"),
            Self::Authenticated => write!(f, "Authenticated"),
            Self::Active => write!(f, "Active"),
            Self::Idle => write!(f, "Idle"),
            Self::Terminated => write!(f, "Terminated"),
        }
    }
}

/// A user session
#[derive(Debug, Clone)]
pub struct Session {
    /// Unique session identifier
    id: SessionId,
    /// Username (None if not authenticated)
    username: Option<String>,
    /// User ID (None if not authenticated)
    user_id: Option<u32>,
    /// Remote address
    remote_addr: String,
    /// Session state
    state: SessionState,
    /// When the session was created
    created_at: Instant,
    /// Last activity time
    last_activity: Instant,
    /// Terminal width
    terminal_width: u16,
    /// Terminal height
    terminal_height: u16,
    /// Terminal type (e.g., "ANSI", "VT100")
    terminal_type: String,
    /// Whether idle timeout warning has been sent
    idle_warning_sent: bool,
    /// Whether absolute timeout warning has been sent
    absolute_warning_sent: bool,
}

impl Session {
    /// Create a new session
    pub fn new(remote_addr: String) -> Self {
        let now = Instant::now();
        Self {
            id: SessionId::new(),
            username: None,
            user_id: None,
            remote_addr,
            state: SessionState::Connected,
            created_at: now,
            last_activity: now,
            terminal_width: 80,
            terminal_height: 24,
            terminal_type: "ANSI".to_string(),
            idle_warning_sent: false,
            absolute_warning_sent: false,
        }
    }

    /// Get session ID
    pub fn id(&self) -> SessionId {
        self.id
    }

    /// Get username (if authenticated)
    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    /// Get user ID (if authenticated)
    pub fn user_id(&self) -> Option<u32> {
        self.user_id
    }

    /// Get remote address
    pub fn remote_addr(&self) -> &str {
        &self.remote_addr
    }

    /// Get current state
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Get time since session was created
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Get time since last activity
    pub fn idle_time(&self) -> Duration {
        self.last_activity.elapsed()
    }

    /// Get terminal dimensions
    pub fn terminal_size(&self) -> (u16, u16) {
        (self.terminal_width, self.terminal_height)
    }

    /// Get terminal type
    pub fn terminal_type(&self) -> &str {
        &self.terminal_type
    }

    /// Check if session is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.username.is_some()
            && matches!(
                self.state,
                SessionState::Authenticated | SessionState::Active | SessionState::Idle
            )
    }

    /// Check if session is idle
    pub fn is_idle(&self, idle_timeout: Duration) -> bool {
        self.idle_time() >= idle_timeout
    }

    /// Set session state
    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
        self.update_activity();
    }

    /// Authenticate session with user information
    pub fn authenticate(&mut self, username: String, user_id: u32) {
        self.username = Some(username);
        self.user_id = Some(user_id);
        self.state = SessionState::Authenticated;
        self.update_activity();
    }

    /// Update last activity time
    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
        // Reset warning flags on activity
        self.idle_warning_sent = false;
    }

    /// Set terminal dimensions
    pub fn set_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_width = width;
        self.terminal_height = height;
    }

    /// Set terminal type
    pub fn set_terminal_type(&mut self, terminal_type: String) {
        self.terminal_type = terminal_type;
    }

    /// Check if idle warning should be sent
    pub fn should_send_idle_warning(
        &self,
        idle_timeout: Duration,
        warning_before: Duration,
    ) -> bool {
        let idle = self.idle_time();
        let warning_threshold = idle_timeout.saturating_sub(warning_before);
        !self.idle_warning_sent && idle >= warning_threshold && idle < idle_timeout
    }

    /// Check if absolute timeout warning should be sent
    pub fn should_send_absolute_warning(
        &self,
        absolute_timeout: Duration,
        warning_before: Duration,
    ) -> bool {
        let age = self.age();
        let warning_threshold = absolute_timeout.saturating_sub(warning_before);
        !self.absolute_warning_sent && age >= warning_threshold && age < absolute_timeout
    }

    /// Check if session has exceeded absolute timeout
    pub fn is_absolute_timeout(&self, absolute_timeout: Duration) -> bool {
        self.age() >= absolute_timeout
    }

    /// Mark idle warning as sent
    pub fn mark_idle_warning_sent(&mut self) {
        self.idle_warning_sent = true;
    }

    /// Mark absolute timeout warning as sent
    pub fn mark_absolute_warning_sent(&mut self) {
        self.absolute_warning_sent = true;
    }

    /// Check if any warning has been sent
    pub fn has_warning_sent(&self) -> bool {
        self.idle_warning_sent || self.absolute_warning_sent
    }

    /// Check if idle warning has been sent
    pub fn is_idle_warning_sent(&self) -> bool {
        self.idle_warning_sent
    }

    /// Check if absolute warning has been sent
    pub fn is_absolute_warning_sent(&self) -> bool {
        self.absolute_warning_sent
    }

    /// Terminate the session
    pub fn terminate(&mut self) {
        self.state = SessionState::Terminated;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new("192.168.1.1:1234".to_string());
        assert_eq!(session.state(), SessionState::Connected);
        assert_eq!(session.remote_addr(), "192.168.1.1:1234");
        assert!(!session.is_authenticated());
    }

    #[test]
    fn test_session_authentication() {
        let mut session = Session::new("192.168.1.1:1234".to_string());
        session.authenticate("testuser".to_string(), 123);
        assert!(session.is_authenticated());
        assert_eq!(session.username(), Some("testuser"));
        assert_eq!(session.user_id(), Some(123));
    }

    #[test]
    fn test_session_id_display() {
        let id = SessionId::new();
        let id_str = id.to_string();
        assert!(uuid::Uuid::parse_str(&id_str).is_ok());
    }

    #[test]
    fn test_terminal_size() {
        let mut session = Session::new("192.168.1.1:1234".to_string());
        assert_eq!(session.terminal_size(), (80, 24));
        session.set_terminal_size(120, 40);
        assert_eq!(session.terminal_size(), (120, 40));
    }

    #[test]
    fn test_idle_detection() {
        let session = Session::new("192.168.1.1:1234".to_string());
        assert!(!session.is_idle(Duration::from_secs(1)));
    }

    #[test]
    fn test_warning_flags_initialization() {
        let session = Session::new("192.168.1.1:1234".to_string());
        assert!(!session.has_warning_sent());
        assert!(!session.is_idle_warning_sent());
        assert!(!session.is_absolute_warning_sent());
    }

    #[test]
    fn test_mark_warnings() {
        let mut session = Session::new("192.168.1.1:1234".to_string());

        session.mark_idle_warning_sent();
        assert!(session.is_idle_warning_sent());
        assert!(session.has_warning_sent());

        session.mark_absolute_warning_sent();
        assert!(session.is_absolute_warning_sent());
        assert!(session.has_warning_sent());
    }

    #[test]
    fn test_activity_resets_idle_warning() {
        let mut session = Session::new("192.168.1.1:1234".to_string());
        session.mark_idle_warning_sent();
        assert!(session.is_idle_warning_sent());

        session.update_activity();
        assert!(!session.is_idle_warning_sent());
    }

    #[test]
    fn test_idle_warning_threshold() {
        let session = Session::new("192.168.1.1:1234".to_string());
        let idle_timeout = Duration::from_secs(900);
        let warning_before = Duration::from_secs(60);

        // Session is brand new, no warning should be sent
        assert!(!session.should_send_idle_warning(idle_timeout, warning_before));
    }

    #[test]
    fn test_absolute_timeout_check() {
        let session = Session::new("192.168.1.1:1234".to_string());

        // Brand new session should not be timed out
        assert!(!session.is_absolute_timeout(Duration::from_secs(3600)));
    }
}
