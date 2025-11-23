//! Configuration types

use serde::{Deserialize, Serialize};

/// BBS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbsConfig {
    /// BBS name
    pub name: String,
    /// SysOp name
    pub sysop: String,
}
