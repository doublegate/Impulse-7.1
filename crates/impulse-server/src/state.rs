//! Server state management
//!
//! Holds all the managers, services, and shared state for the BBS server.

use anyhow::Result;
use impulse_admin::{AdminAccessControl, AuditLogger};
use impulse_auth::AuthService;
use impulse_door::DoorManager;
use impulse_file::InMemoryFileAreaManager;
use impulse_message::formats::JamMessageBase;
use impulse_session::{SessionConfig, SessionManager};
use impulse_terminal::theme::ThemeManager;
use impulse_user::{InMemoryUserManager, UserManager};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Server state containing all managers and services
///
/// Note: Some fields are initialized but not yet fully utilized in the menu system.
/// They will be connected as menu handlers are expanded.
#[derive(Clone)]
#[allow(dead_code)]
pub struct ServerState {
    /// Authentication service
    pub auth_service: Arc<AuthService>,

    /// User manager
    pub user_manager: Arc<RwLock<InMemoryUserManager>>,

    /// Message base manager (simplified for now - single base)
    pub message_base: Arc<RwLock<JamMessageBase>>,

    /// File area manager
    pub file_manager: Arc<RwLock<InMemoryFileAreaManager>>,

    /// Admin access control
    pub admin_access: Arc<AdminAccessControl>,

    /// Audit logger
    pub audit_logger: Arc<AuditLogger>,

    /// Door manager
    pub door_manager: Arc<DoorManager>,

    /// Theme manager
    pub theme_manager: Arc<RwLock<ThemeManager>>,

    /// Session manager
    pub session_manager: Arc<SessionManager>,

    /// Base paths
    pub paths: ServerPaths,
}

/// Server paths configuration
#[derive(Clone, Debug)]
pub struct ServerPaths {
    /// Data directory
    pub data_dir: PathBuf,

    /// Message base directory
    pub message_dir: PathBuf,

    /// File areas directory
    pub files_dir: PathBuf,

    /// Door games directory
    pub doors_dir: PathBuf,

    /// Node directories
    pub nodes_dir: PathBuf,

    /// Theme directory
    pub theme_dir: PathBuf,
}

impl Default for ServerPaths {
    fn default() -> Self {
        let data_dir = PathBuf::from("/tmp/impulse-next-bbs/data");

        // Use project themes directory for built-in themes
        // In production, this would be configurable
        let project_themes = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("themes"))
            .unwrap_or_else(|| data_dir.join("themes"));

        Self {
            data_dir: data_dir.clone(),
            message_dir: data_dir.join("messages"),
            files_dir: data_dir.join("files"),
            doors_dir: data_dir.join("doors"),
            nodes_dir: data_dir.join("nodes"),
            theme_dir: project_themes,
        }
    }
}

impl ServerState {
    /// Create a new server state with default configuration
    pub async fn new() -> Result<Self> {
        let paths = ServerPaths::default();

        // Create directories if they don't exist
        std::fs::create_dir_all(&paths.data_dir)?;
        std::fs::create_dir_all(&paths.message_dir)?;
        std::fs::create_dir_all(&paths.files_dir)?;
        std::fs::create_dir_all(&paths.doors_dir)?;
        std::fs::create_dir_all(&paths.nodes_dir)?;
        std::fs::create_dir_all(&paths.theme_dir)?;

        // Initialize auth service
        let auth_service = Arc::new(AuthService::new(Duration::from_secs(1800))); // 30 min sessions

        // Initialize user manager with a default sysop user
        let mut user_manager = InMemoryUserManager::new();

        // Create default sysop user
        let mut sysop = impulse_types::user::User::new("sysop")
            .map_err(|e| anyhow::anyhow!("Failed to create sysop user: {}", e))?;
        sysop.set_security_level(impulse_types::security::SecurityLevel::new(255)); // Max security
        user_manager.create_user(sysop).await?;

        // Create a test user for demonstration
        let mut testuser = impulse_types::user::User::new("testuser")
            .map_err(|e| anyhow::anyhow!("Failed to create test user: {}", e))?;
        testuser.set_security_level(impulse_types::security::SecurityLevel::new(10)); // Normal user
        user_manager.create_user(testuser).await?;

        let user_manager = Arc::new(RwLock::new(user_manager));

        // Initialize message base
        let message_base_path = paths.message_dir.join("general");
        let message_base = Arc::new(RwLock::new(JamMessageBase::new(message_base_path)));

        // Initialize file area manager
        let file_manager = Arc::new(RwLock::new(InMemoryFileAreaManager::new()));

        // Initialize admin components
        let admin_access = Arc::new(AdminAccessControl::new(200, 200)); // SysOp level: 200
        let audit_logger = Arc::new(AuditLogger::new());

        // Initialize door manager
        let door_manager =
            Arc::new(DoorManager::new(paths.doors_dir.clone(), paths.nodes_dir.clone()).await?);

        // Initialize theme manager
        let theme_manager = Arc::new(RwLock::new(
            ThemeManager::new(paths.theme_dir.clone()).await?,
        ));

        // Initialize session manager
        let session_config = SessionConfig::default()
            .with_idle_timeout(Duration::from_secs(900)) // 15 min idle timeout
            .with_max_sessions_per_user(3)
            .with_max_total_sessions(100);
        let session_manager = Arc::new(SessionManager::new(session_config));

        // Log loaded themes
        let theme_list = theme_manager.read().await.list_themes().await;
        let theme_names: Vec<String> = theme_list.iter().map(|t| t.name.clone()).collect();

        tracing::info!("Server state initialized successfully");
        tracing::info!("  Data directory: {:?}", paths.data_dir);
        tracing::info!("  Theme directory: {:?}", paths.theme_dir);
        tracing::info!("  Themes loaded: {:?}", theme_names);
        tracing::info!("  Default users created: sysop (255), testuser (10)");

        Ok(Self {
            auth_service,
            user_manager,
            message_base,
            file_manager,
            admin_access,
            audit_logger,
            door_manager,
            theme_manager,
            session_manager,
            paths,
        })
    }

    /// Create server state with custom paths
    #[allow(dead_code)]
    pub async fn with_paths(paths: ServerPaths) -> Result<Self> {
        let mut state = Self::new().await?;
        state.paths = paths;
        Ok(state)
    }
}
