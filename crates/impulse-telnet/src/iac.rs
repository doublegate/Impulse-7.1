//! Telnet IAC (Interpret As Command) protocol constants and handling
//!
//! Implements RFC 854 (Telnet Protocol), RFC 857 (Echo), RFC 858 (Suppress Go Ahead),
//! and RFC 1073 (Window Size).

/// IAC (Interpret As Command) byte - signals start of telnet command
pub const IAC: u8 = 255;

/// Telnet command bytes (RFC 854)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IacCommand {
    /// End of subnegotiation parameters
    SE = 240,
    /// No operation
    NOP = 241,
    /// Data mark (for Sync)
    DM = 242,
    /// Break
    BRK = 243,
    /// Interrupt Process
    IP = 244,
    /// Abort Output
    AO = 245,
    /// Are You There
    AYT = 246,
    /// Erase Character
    EC = 247,
    /// Erase Line
    EL = 248,
    /// Go Ahead
    GA = 249,
    /// Begin subnegotiation
    SB = 250,
    /// WILL (option negotiation)
    WILL = 251,
    /// WONT (option negotiation)
    WONT = 252,
    /// DO (option negotiation)
    DO = 253,
    /// DONT (option negotiation)
    DONT = 254,
}

impl IacCommand {
    /// Convert a byte to an IAC command
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            240 => Some(Self::SE),
            241 => Some(Self::NOP),
            242 => Some(Self::DM),
            243 => Some(Self::BRK),
            244 => Some(Self::IP),
            245 => Some(Self::AO),
            246 => Some(Self::AYT),
            247 => Some(Self::EC),
            248 => Some(Self::EL),
            249 => Some(Self::GA),
            250 => Some(Self::SB),
            251 => Some(Self::WILL),
            252 => Some(Self::WONT),
            253 => Some(Self::DO),
            254 => Some(Self::DONT),
            _ => None,
        }
    }

    /// Convert command to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }
}

/// Telnet option codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TelnetOption {
    /// Echo (RFC 857)
    Echo = 1,
    /// Suppress Go Ahead (RFC 858)
    SuppressGoAhead = 3,
    /// Terminal Type (RFC 1091)
    TerminalType = 24,
    /// Window Size (RFC 1073)
    WindowSize = 31,
    /// Terminal Speed (RFC 1079)
    TerminalSpeed = 32,
    /// Remote Flow Control (RFC 1372)
    RemoteFlowControl = 33,
    /// Linemode (RFC 1184)
    Linemode = 34,
    /// Environment Variables (RFC 1408)
    EnvironmentVariables = 36,
}

impl TelnetOption {
    /// Convert a byte to a telnet option
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::Echo),
            3 => Some(Self::SuppressGoAhead),
            24 => Some(Self::TerminalType),
            31 => Some(Self::WindowSize),
            32 => Some(Self::TerminalSpeed),
            33 => Some(Self::RemoteFlowControl),
            34 => Some(Self::Linemode),
            36 => Some(Self::EnvironmentVariables),
            _ => None,
        }
    }

    /// Convert option to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Get option name
    pub fn name(self) -> &'static str {
        match self {
            Self::Echo => "ECHO",
            Self::SuppressGoAhead => "SUPPRESS_GO_AHEAD",
            Self::TerminalType => "TERMINAL_TYPE",
            Self::WindowSize => "WINDOW_SIZE",
            Self::TerminalSpeed => "TERMINAL_SPEED",
            Self::RemoteFlowControl => "REMOTE_FLOW_CONTROL",
            Self::Linemode => "LINEMODE",
            Self::EnvironmentVariables => "ENVIRONMENT_VARIABLES",
        }
    }
}

/// Build an IAC command sequence
pub fn build_iac_command(cmd: IacCommand, option: Option<TelnetOption>) -> Vec<u8> {
    let mut bytes = vec![IAC, cmd.to_byte()];
    if let Some(opt) = option {
        bytes.push(opt.to_byte());
    }
    bytes
}

/// Build WILL command
pub fn will(option: TelnetOption) -> Vec<u8> {
    build_iac_command(IacCommand::WILL, Some(option))
}

/// Build WONT command
pub fn wont(option: TelnetOption) -> Vec<u8> {
    build_iac_command(IacCommand::WONT, Some(option))
}

/// Build DO command
pub fn r#do(option: TelnetOption) -> Vec<u8> {
    build_iac_command(IacCommand::DO, Some(option))
}

/// Build DONT command
pub fn dont(option: TelnetOption) -> Vec<u8> {
    build_iac_command(IacCommand::DONT, Some(option))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iac_command_conversion() {
        assert_eq!(IacCommand::from_byte(251), Some(IacCommand::WILL));
        assert_eq!(IacCommand::from_byte(253), Some(IacCommand::DO));
        assert_eq!(IacCommand::from_byte(255), None);
    }

    #[test]
    fn test_telnet_option_conversion() {
        assert_eq!(TelnetOption::from_byte(1), Some(TelnetOption::Echo));
        assert_eq!(TelnetOption::from_byte(31), Some(TelnetOption::WindowSize));
        assert_eq!(TelnetOption::from_byte(99), None);
    }

    #[test]
    fn test_build_iac_will() {
        let cmd = will(TelnetOption::Echo);
        assert_eq!(cmd, vec![255, 251, 1]);
    }

    #[test]
    fn test_build_iac_do() {
        let cmd = r#do(TelnetOption::SuppressGoAhead);
        assert_eq!(cmd, vec![255, 253, 3]);
    }

    #[test]
    fn test_option_names() {
        assert_eq!(TelnetOption::Echo.name(), "ECHO");
        assert_eq!(TelnetOption::WindowSize.name(), "WINDOW_SIZE");
    }
}
