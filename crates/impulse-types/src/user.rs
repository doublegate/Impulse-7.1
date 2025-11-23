//! User data types

use serde::{Deserialize, Serialize};

/// User record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: u32,
    /// Username
    pub name: String,
}
