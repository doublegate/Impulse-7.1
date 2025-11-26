//! Virus scanning status display

/// Scanning status
#[derive(Debug, Clone, Copy)]
pub enum ScanStatus {
    /// Scanning in progress
    Scanning,

    /// Scan complete - file is clean
    Clean,

    /// Virus detected
    Infected,

    /// Scanner unavailable
    Unavailable,
}

/// Scanning status screen
pub struct ScanningScreen {
    /// Current scan status
    pub status: ScanStatus,

    /// Filename being scanned
    pub filename: String,

    /// Threat name (if infected)
    pub threat_name: Option<String>,
}

impl ScanningScreen {
    /// Create a new scanning screen
    pub fn new(filename: String) -> Self {
        Self {
            status: ScanStatus::Scanning,
            filename,
            threat_name: None,
        }
    }

    /// Mark as clean
    pub fn mark_clean(&mut self) {
        self.status = ScanStatus::Clean;
    }

    /// Mark as infected
    pub fn mark_infected(&mut self, threat_name: String) {
        self.status = ScanStatus::Infected;
        self.threat_name = Some(threat_name);
    }

    /// Mark as unavailable
    pub fn mark_unavailable(&mut self) {
        self.status = ScanStatus::Unavailable;
    }

    /// Render scanning display
    pub fn render(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("Scanning: {}\n\n", self.filename));

        match self.status {
            ScanStatus::Scanning => {
                output.push_str("Virus scan in progress...\n");
                output.push_str("Please wait.\n");
            }
            ScanStatus::Clean => {
                output.push_str("Scan complete: File is clean.\n");
            }
            ScanStatus::Infected => {
                output.push_str("VIRUS DETECTED!\n\n");
                if let Some(ref threat) = self.threat_name {
                    output.push_str(&format!("Threat: {}\n", threat));
                }
                output.push_str("\nFile has been quarantined and will not be added.\n");
                output.push_str("The system operator has been notified.\n");
            }
            ScanStatus::Unavailable => {
                output.push_str("Virus scanner unavailable.\n");
                output.push_str("File will be added without scanning.\n");
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanning_screen_new() {
        let screen = ScanningScreen::new("test.zip".to_string());
        assert_eq!(screen.filename, "test.zip");
        assert!(matches!(screen.status, ScanStatus::Scanning));
    }

    #[test]
    fn test_scanning_screen_mark_clean() {
        let mut screen = ScanningScreen::new("test.zip".to_string());
        screen.mark_clean();

        assert!(matches!(screen.status, ScanStatus::Clean));
    }

    #[test]
    fn test_scanning_screen_mark_infected() {
        let mut screen = ScanningScreen::new("test.zip".to_string());
        screen.mark_infected("Trojan.Generic".to_string());

        assert!(matches!(screen.status, ScanStatus::Infected));
        assert_eq!(screen.threat_name, Some("Trojan.Generic".to_string()));
    }

    #[test]
    fn test_scanning_screen_render() {
        let mut screen = ScanningScreen::new("test.zip".to_string());
        screen.mark_infected("Trojan.Generic".to_string());

        let output = screen.render();

        assert!(output.contains("VIRUS DETECTED"));
        assert!(output.contains("Trojan.Generic"));
        assert!(output.contains("quarantined"));
    }
}
