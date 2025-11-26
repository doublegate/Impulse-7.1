//! Protocol preferences for file transfers.
//!
//! This module provides settings for user-preferred file transfer protocols,
//! allowing users to choose their preferred protocol for uploads and downloads.

use impulse_protocol::FileProtocol;
use serde::{Deserialize, Serialize};

/// Protocol settings for a user.
///
/// Stores the user's preferred file transfer protocols and related options.
///
/// # Examples
///
/// ```
/// use impulse_user::settings::protocol::ProtocolSettings;
/// use impulse_protocol::FileProtocol;
///
/// let mut settings = ProtocolSettings::default();
/// settings.download_protocol = FileProtocol::Ymodem;
/// settings.upload_protocol = FileProtocol::Zmodem;
/// assert_eq!(settings.download_protocol, FileProtocol::Ymodem);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolSettings {
    /// Preferred protocol for downloads.
    ///
    /// This protocol will be used by default when the user initiates a download.
    pub download_protocol: FileProtocol,

    /// Preferred protocol for uploads.
    ///
    /// This protocol will be used by default when the user initiates an upload.
    pub upload_protocol: FileProtocol,

    /// Remember the last used protocol and use it next time.
    ///
    /// When true, the system will automatically update `last_used` whenever
    /// a transfer completes, and offer to use that protocol next time.
    pub remember_last: bool,

    /// The last protocol used in a file transfer.
    ///
    /// Only tracked when `remember_last` is true. This allows the BBS to
    /// offer "use same protocol as last time" as a quick option.
    pub last_used: Option<FileProtocol>,
}

impl Default for ProtocolSettings {
    /// Create default protocol settings.
    ///
    /// Defaults to Zmodem for both upload and download, as it's the most
    /// capable protocol with crash recovery and streaming support.
    fn default() -> Self {
        Self {
            download_protocol: FileProtocol::Zmodem,
            upload_protocol: FileProtocol::Zmodem,
            remember_last: true,
            last_used: None,
        }
    }
}

impl ProtocolSettings {
    /// Create new protocol settings with specified defaults.
    ///
    /// # Arguments
    ///
    /// * `download` - Preferred protocol for downloads
    /// * `upload` - Preferred protocol for uploads
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::settings::protocol::ProtocolSettings;
    /// use impulse_protocol::FileProtocol;
    ///
    /// let settings = ProtocolSettings::new(
    ///     FileProtocol::Ymodem,
    ///     FileProtocol::Xmodem1K
    /// );
    /// assert_eq!(settings.download_protocol, FileProtocol::Ymodem);
    /// assert_eq!(settings.upload_protocol, FileProtocol::Xmodem1K);
    /// ```
    pub fn new(download: FileProtocol, upload: FileProtocol) -> Self {
        Self {
            download_protocol: download,
            upload_protocol: upload,
            remember_last: true,
            last_used: None,
        }
    }

    /// Update the last used protocol.
    ///
    /// Only takes effect if `remember_last` is true.
    ///
    /// # Arguments
    ///
    /// * `protocol` - The protocol that was just used
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::settings::protocol::ProtocolSettings;
    /// use impulse_protocol::FileProtocol;
    ///
    /// let mut settings = ProtocolSettings::default();
    /// settings.update_last_used(FileProtocol::Ymodem);
    /// assert_eq!(settings.last_used, Some(FileProtocol::Ymodem));
    /// ```
    pub fn update_last_used(&mut self, protocol: FileProtocol) {
        if self.remember_last {
            self.last_used = Some(protocol);
        }
    }

    /// Get the protocol to use for a download.
    ///
    /// Returns `last_used` if `remember_last` is true and a protocol was
    /// previously used, otherwise returns `download_protocol`.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::settings::protocol::ProtocolSettings;
    /// use impulse_protocol::FileProtocol;
    ///
    /// let mut settings = ProtocolSettings::default();
    /// assert_eq!(settings.get_download_protocol(), FileProtocol::Zmodem);
    ///
    /// settings.update_last_used(FileProtocol::Ymodem);
    /// assert_eq!(settings.get_download_protocol(), FileProtocol::Ymodem);
    /// ```
    pub fn get_download_protocol(&self) -> FileProtocol {
        if self.remember_last {
            self.last_used.unwrap_or(self.download_protocol)
        } else {
            self.download_protocol
        }
    }

    /// Get the protocol to use for an upload.
    ///
    /// Returns `last_used` if `remember_last` is true and a protocol was
    /// previously used, otherwise returns `upload_protocol`.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::settings::protocol::ProtocolSettings;
    /// use impulse_protocol::FileProtocol;
    ///
    /// let mut settings = ProtocolSettings::default();
    /// assert_eq!(settings.get_upload_protocol(), FileProtocol::Zmodem);
    ///
    /// settings.update_last_used(FileProtocol::YmodemG);
    /// assert_eq!(settings.get_upload_protocol(), FileProtocol::YmodemG);
    /// ```
    pub fn get_upload_protocol(&self) -> FileProtocol {
        if self.remember_last {
            self.last_used.unwrap_or(self.upload_protocol)
        } else {
            self.upload_protocol
        }
    }

    /// Set whether to remember the last used protocol.
    ///
    /// If set to false, clears the `last_used` field.
    ///
    /// # Examples
    ///
    /// ```
    /// use impulse_user::settings::protocol::ProtocolSettings;
    /// use impulse_protocol::FileProtocol;
    ///
    /// let mut settings = ProtocolSettings::default();
    /// settings.update_last_used(FileProtocol::Ymodem);
    /// assert!(settings.last_used.is_some());
    ///
    /// settings.set_remember_last(false);
    /// assert!(settings.last_used.is_none());
    /// ```
    pub fn set_remember_last(&mut self, remember: bool) {
        self.remember_last = remember;
        if !remember {
            self.last_used = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let settings = ProtocolSettings::default();
        assert_eq!(settings.download_protocol, FileProtocol::Zmodem);
        assert_eq!(settings.upload_protocol, FileProtocol::Zmodem);
        assert!(settings.remember_last);
        assert_eq!(settings.last_used, None);
    }

    #[test]
    fn test_new() {
        let settings = ProtocolSettings::new(FileProtocol::Ymodem, FileProtocol::Xmodem1K);
        assert_eq!(settings.download_protocol, FileProtocol::Ymodem);
        assert_eq!(settings.upload_protocol, FileProtocol::Xmodem1K);
        assert!(settings.remember_last);
        assert_eq!(settings.last_used, None);
    }

    #[test]
    fn test_update_last_used() {
        let mut settings = ProtocolSettings::default();
        settings.update_last_used(FileProtocol::Ymodem);
        assert_eq!(settings.last_used, Some(FileProtocol::Ymodem));
    }

    #[test]
    fn test_update_last_used_when_remember_false() {
        let mut settings = ProtocolSettings {
            remember_last: false,
            ..Default::default()
        };
        settings.update_last_used(FileProtocol::Ymodem);
        assert_eq!(settings.last_used, None);
    }

    #[test]
    fn test_get_download_protocol_default() {
        let settings = ProtocolSettings::default();
        assert_eq!(settings.get_download_protocol(), FileProtocol::Zmodem);
    }

    #[test]
    fn test_get_download_protocol_with_last_used() {
        let mut settings = ProtocolSettings::default();
        settings.update_last_used(FileProtocol::Ymodem);
        assert_eq!(settings.get_download_protocol(), FileProtocol::Ymodem);
    }

    #[test]
    fn test_get_download_protocol_remember_false() {
        let settings = ProtocolSettings {
            remember_last: false,
            last_used: Some(FileProtocol::Ymodem),
            ..Default::default()
        };
        assert_eq!(settings.get_download_protocol(), FileProtocol::Zmodem);
    }

    #[test]
    fn test_get_upload_protocol_default() {
        let settings = ProtocolSettings::default();
        assert_eq!(settings.get_upload_protocol(), FileProtocol::Zmodem);
    }

    #[test]
    fn test_get_upload_protocol_with_last_used() {
        let mut settings = ProtocolSettings::default();
        settings.update_last_used(FileProtocol::YmodemG);
        assert_eq!(settings.get_upload_protocol(), FileProtocol::YmodemG);
    }

    #[test]
    fn test_get_upload_protocol_remember_false() {
        let settings = ProtocolSettings {
            remember_last: false,
            last_used: Some(FileProtocol::Ymodem),
            ..Default::default()
        };
        assert_eq!(settings.get_upload_protocol(), FileProtocol::Zmodem);
    }

    #[test]
    fn test_set_remember_last_true() {
        let mut settings = ProtocolSettings {
            remember_last: false,
            ..Default::default()
        };
        settings.set_remember_last(true);
        assert!(settings.remember_last);
    }

    #[test]
    fn test_set_remember_last_false_clears_last_used() {
        let mut settings = ProtocolSettings::default();
        settings.update_last_used(FileProtocol::Ymodem);
        assert!(settings.last_used.is_some());

        settings.set_remember_last(false);
        assert!(!settings.remember_last);
        assert_eq!(settings.last_used, None);
    }

    #[test]
    fn test_serialization() {
        let settings = ProtocolSettings::new(FileProtocol::Ymodem, FileProtocol::Xmodem1K);
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: ProtocolSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_clone() {
        let settings = ProtocolSettings::default();
        let cloned = settings.clone();
        assert_eq!(settings, cloned);
    }

    #[test]
    fn test_eq() {
        let settings1 = ProtocolSettings::default();
        let settings2 = ProtocolSettings::default();
        assert_eq!(settings1, settings2);

        let settings3 = ProtocolSettings {
            download_protocol: FileProtocol::Ymodem,
            ..Default::default()
        };
        assert_ne!(settings1, settings3);
    }
}
