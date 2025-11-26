//! Dropfile generator for creating various dropfile formats.
//!
//! This module provides a unified interface for generating different types of
//! dropfiles used by BBS door games.

use crate::error::{DoorError, Result};
use crate::session::DoorSession;
use std::path::{Path, PathBuf};

use super::dorinfo::DorinfoDropfile;
use super::doorsys::DoorSysDropfile;

/// Supported dropfile types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropfileType {
    /// DOOR.SYS format (52 lines)
    DoorSys,
    /// DORINFO1.DEF format (13 lines)
    Dorinfo1Def,
    /// DORINFOx.DEF format (node-specific, 13 lines)
    DorinfoDef,
    /// CHAIN.TXT format (used by some WWIV doors)
    ChainTxt,
    /// CALLINFO.BBS format (used by WildCat! doors)
    CallInfo,
}

impl DropfileType {
    /// Get the filename for this dropfile type.
    pub fn filename(&self, node_id: u16) -> String {
        match self {
            DropfileType::DoorSys => "DOOR.SYS".to_string(),
            DropfileType::Dorinfo1Def => "DORINFO1.DEF".to_string(),
            DropfileType::DorinfoDef => format!("DORINFO{}.DEF", node_id),
            DropfileType::ChainTxt => "CHAIN.TXT".to_string(),
            DropfileType::CallInfo => "CALLINFO.BBS".to_string(),
        }
    }

    /// Get a description of this dropfile type.
    pub fn description(&self) -> &'static str {
        match self {
            DropfileType::DoorSys => "DOOR.SYS (52-line standard format)",
            DropfileType::Dorinfo1Def => "DORINFO1.DEF (13-line format)",
            DropfileType::DorinfoDef => "DORINFOx.DEF (node-specific 13-line format)",
            DropfileType::ChainTxt => "CHAIN.TXT (WWIV format)",
            DropfileType::CallInfo => "CALLINFO.BBS (WildCat! format)",
        }
    }
}

/// Dropfile generator for creating various dropfile formats.
pub struct DropfileGenerator;

impl DropfileGenerator {
    /// Generate a dropfile of the specified type.
    ///
    /// # Arguments
    ///
    /// * `dropfile_type` - The type of dropfile to generate
    /// * `session` - The door session containing user information
    /// * `output_dir` - The directory where the dropfile should be created
    ///
    /// # Returns
    ///
    /// The path to the created dropfile, or an error if generation failed.
    pub fn generate(
        dropfile_type: DropfileType,
        session: &DoorSession,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        // Ensure output directory exists
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }

        let filename = dropfile_type.filename(session.node_id);
        let file_path = output_dir.join(&filename);

        match dropfile_type {
            DropfileType::DoorSys => {
                let dropfile = DoorSysDropfile::from_session(session);
                dropfile.write_to_file(&file_path)?;
            }
            DropfileType::Dorinfo1Def | DropfileType::DorinfoDef => {
                let dropfile = DorinfoDropfile::from_session(session);
                dropfile.write_to_file(&file_path)?;
            }
            DropfileType::ChainTxt => {
                return Err(DoorError::DropfileCreation(
                    "CHAIN.TXT format not yet implemented".to_string(),
                ));
            }
            DropfileType::CallInfo => {
                return Err(DoorError::DropfileCreation(
                    "CALLINFO.BBS format not yet implemented".to_string(),
                ));
            }
        }

        Ok(file_path)
    }

    /// Generate all commonly-used dropfiles for maximum compatibility.
    ///
    /// This creates both DOOR.SYS and DORINFO1.DEF in the specified directory,
    /// ensuring compatibility with most door games.
    pub fn generate_all(session: &DoorSession, output_dir: &Path) -> Result<Vec<PathBuf>> {
        let paths = vec![
            Self::generate(DropfileType::DoorSys, session, output_dir)?,
            Self::generate(DropfileType::Dorinfo1Def, session, output_dir)?,
        ];

        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_session() -> DoorSession {
        DoorSession {
            node_id: 3,
            user_name: "Test User".to_string(),
            user_alias: Some("TestUser".to_string()),
            location: "Test City, TS".to_string(),
            security_level: 50,
            time_remaining_seconds: 1800,
            ansi_enabled: true,
            login_time: Utc::now(),
            total_calls: 10,
            last_call_date: "11/26/25".to_string(),
            upload_kb: 100,
            download_kb: 200,
        }
    }

    #[test]
    fn test_dropfile_type_filename_doorsys() {
        let filename = DropfileType::DoorSys.filename(1);
        assert_eq!(filename, "DOOR.SYS");
    }

    #[test]
    fn test_dropfile_type_filename_dorinfo1() {
        let filename = DropfileType::Dorinfo1Def.filename(1);
        assert_eq!(filename, "DORINFO1.DEF");
    }

    #[test]
    fn test_dropfile_type_filename_dorinfo_node() {
        let filename = DropfileType::DorinfoDef.filename(5);
        assert_eq!(filename, "DORINFO5.DEF");
    }

    #[test]
    fn test_dropfile_type_filename_chain() {
        let filename = DropfileType::ChainTxt.filename(1);
        assert_eq!(filename, "CHAIN.TXT");
    }

    #[test]
    fn test_dropfile_type_filename_callinfo() {
        let filename = DropfileType::CallInfo.filename(1);
        assert_eq!(filename, "CALLINFO.BBS");
    }

    #[test]
    fn test_dropfile_type_description() {
        assert_eq!(
            DropfileType::DoorSys.description(),
            "DOOR.SYS (52-line standard format)"
        );
        assert_eq!(
            DropfileType::Dorinfo1Def.description(),
            "DORINFO1.DEF (13-line format)"
        );
    }

    #[test]
    fn test_generate_doorsys() {
        let session = create_test_session();
        let temp_dir = tempfile::tempdir().unwrap();

        let result = DropfileGenerator::generate(
            DropfileType::DoorSys,
            &session,
            temp_dir.path(),
        );

        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
        assert_eq!(path.file_name().unwrap(), "DOOR.SYS");

        // Verify file has 52 lines
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content.lines().count(), 52);
    }

    #[test]
    fn test_generate_dorinfo1() {
        let session = create_test_session();
        let temp_dir = tempfile::tempdir().unwrap();

        let result = DropfileGenerator::generate(
            DropfileType::Dorinfo1Def,
            &session,
            temp_dir.path(),
        );

        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
        assert_eq!(path.file_name().unwrap(), "DORINFO1.DEF");

        // Verify file has 13 lines
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content.lines().count(), 13);
    }

    #[test]
    fn test_generate_dorinfo_node_specific() {
        let session = create_test_session();
        let temp_dir = tempfile::tempdir().unwrap();

        let result = DropfileGenerator::generate(
            DropfileType::DorinfoDef,
            &session,
            temp_dir.path(),
        );

        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
        assert_eq!(path.file_name().unwrap(), "DORINFO3.DEF");
    }

    #[test]
    fn test_generate_chain_txt_not_implemented() {
        let session = create_test_session();
        let temp_dir = tempfile::tempdir().unwrap();

        let result = DropfileGenerator::generate(
            DropfileType::ChainTxt,
            &session,
            temp_dir.path(),
        );

        assert!(result.is_err());
        assert!(matches!(result, Err(DoorError::DropfileCreation(_))));
    }

    #[test]
    fn test_generate_callinfo_not_implemented() {
        let session = create_test_session();
        let temp_dir = tempfile::tempdir().unwrap();

        let result = DropfileGenerator::generate(
            DropfileType::CallInfo,
            &session,
            temp_dir.path(),
        );

        assert!(result.is_err());
        assert!(matches!(result, Err(DoorError::DropfileCreation(_))));
    }

    #[test]
    fn test_generate_all() {
        let session = create_test_session();
        let temp_dir = tempfile::tempdir().unwrap();

        let result = DropfileGenerator::generate_all(&session, temp_dir.path());
        assert!(result.is_ok());

        let paths = result.unwrap();
        assert_eq!(paths.len(), 2);

        // Check DOOR.SYS exists
        assert!(temp_dir.path().join("DOOR.SYS").exists());

        // Check DORINFO1.DEF exists
        assert!(temp_dir.path().join("DORINFO1.DEF").exists());
    }

    #[test]
    fn test_generate_creates_directory() {
        let session = create_test_session();
        let temp_dir = tempfile::tempdir().unwrap();
        let output_dir = temp_dir.path().join("nodes").join("node3");

        assert!(!output_dir.exists());

        let result = DropfileGenerator::generate(
            DropfileType::DoorSys,
            &session,
            &output_dir,
        );

        assert!(result.is_ok());
        assert!(output_dir.exists());
        assert!(output_dir.join("DOOR.SYS").exists());
    }

    #[test]
    fn test_dropfile_type_equality() {
        assert_eq!(DropfileType::DoorSys, DropfileType::DoorSys);
        assert_ne!(DropfileType::DoorSys, DropfileType::Dorinfo1Def);
    }

    #[test]
    fn test_dropfile_type_clone() {
        let original = DropfileType::DoorSys;
        let cloned = original;
        assert_eq!(original, cloned);
    }
}
