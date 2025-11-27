//! Menu handlers for BBS features

pub mod admin;
pub mod doors;
pub mod files;
pub mod messages;
pub mod stats;
pub mod theme;
pub mod user_profile;
pub mod whos_online;

pub use admin::handle_admin;
pub use doors::handle_doors;
pub use files::handle_files;
pub use messages::handle_messages;
pub use stats::handle_system_stats;
pub use theme::handle_theme_selection;
pub use user_profile::handle_user_profile;
pub use whos_online::handle_whos_online;
