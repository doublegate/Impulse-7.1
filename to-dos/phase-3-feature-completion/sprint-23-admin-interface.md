# Sprint 23: Administration Interface

**Phase:** Phase 3 - Feature Completion
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 23 implements a comprehensive SysOp administration interface for managing users, file areas, message bases, and system maintenance. Provides essential tools for BBS operators.

**Context:** Sprint 7 of Phase 3. Enables SysOps to administer the BBS.

**Expected Outcomes:** SysOps have full control over users, files, and system configuration.

---

## Objectives

- [ ] Create SysOp menu system with access control
- [ ] Implement user management functions
- [ ] Add file area management
- [ ] Provide system maintenance tools

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| SysOp menu | UI | Admin-only menu tree |
| User management | Code | Edit, delete, ban users |
| File area management | Code | Create, edit, delete areas |
| System maintenance | Code | View sessions, kick users, broadcast |

---

## Detailed Tasks

### Task Category 1: SysOp Menu

- [ ] **Task 1.1**: Design SysOp menu structure
  - Files affected: `assets/menus/sysop/`
  - Estimated hours: 4

- [ ] **Task 1.2**: Access control checks
  - Files affected: `crates/impulse-admin/src/access.rs`
  - Estimated hours: 3

### Task Category 2: User Management

- [ ] **Task 2.1**: List all users (paginated)
  - Files affected: `crates/impulse-admin/src/users/list.rs`
  - Estimated hours: 4

- [ ] **Task 2.2**: Edit user profiles
  - Files affected: `crates/impulse-admin/src/users/edit.rs`
  - Estimated hours: 6

- [ ] **Task 2.3**: Delete or ban users
  - Files affected: `crates/impulse-admin/src/users/remove.rs`
  - Estimated hours: 4

- [ ] **Task 2.4**: View login history
  - Files affected: `crates/impulse-admin/src/users/history.rs`
  - Estimated hours: 4

### Task Category 3: File Area Management

- [ ] **Task 3.1**: Create new file areas
  - Files affected: `crates/impulse-admin/src/files/create.rs`
  - Estimated hours: 5

- [ ] **Task 3.2**: Edit area settings
  - Files affected: `crates/impulse-admin/src/files/edit.rs`
  - Estimated hours: 4

- [ ] **Task 3.3**: Set security levels
  - Files affected: `crates/impulse-admin/src/files/security.rs`
  - Estimated hours: 3

### Task Category 4: System Maintenance

- [ ] **Task 4.1**: View active sessions
  - Files affected: `crates/impulse-admin/src/system/sessions.rs`
  - Estimated hours: 4

- [ ] **Task 4.2**: Kick idle users
  - Files affected: `crates/impulse-admin/src/system/kick.rs`
  - Estimated hours: 3

- [ ] **Task 4.3**: Broadcast system messages
  - Files affected: `crates/impulse-admin/src/system/broadcast.rs`
  - Estimated hours: 4

### Task Category 5: Testing

- [ ] **Task 5.1**: Test user management
  - Estimated hours: 5

- [ ] **Task 5.2**: Test file area management
  - Estimated hours: 4

- [ ] **Task 5.3**: Verify access control
  - Estimated hours: 3

---

## Acceptance Criteria

- [ ] SysOps can manage users effectively
- [ ] File areas can be configured
- [ ] System maintenance tools work
- [ ] Access control prevents non-SysOp access

---

## Technical Details

### Architecture Considerations

- Role-based access control (RBAC) for SysOp permissions
- Audit logging for all administrative actions
- Real-time session monitoring via shared state
- Database transactions for user/file area modifications
- Broadcast messaging using pub/sub pattern
- Pagination for large datasets (users, files, sessions)
- TUI-based admin interface for SSH connections
- Command-line admin tools for maintenance scripts

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
sqlx = { workspace = true }
chrono = { workspace = true }
uuid = "1.7"
tracing = "0.1"

[dev-dependencies]
tempfile = "3.8"
```

**Pascal Units Being Converted:**
- SYSOP.PAS (SysOp menu and access control)
- USEREDIT.PAS (User management interface)
- FILEADM.PAS (File area administration)
- SYSMAINT.PAS (System maintenance tools)

### Code Examples

**SysOp Access Control:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdminPermission {
    ViewUsers,
    EditUsers,
    DeleteUsers,
    BanUsers,
    ManageFileAreas,
    ViewSessions,
    KickUsers,
    BroadcastMessages,
    SystemMaintenance,
    ViewLogs,
}

#[derive(Debug, Clone)]
pub struct AdminAccessControl {
    user_security_level: u8,
    sysop_level: u8,
}

impl AdminAccessControl {
    pub fn new(user_security_level: u8, sysop_level: u8) -> Self {
        Self {
            user_security_level,
            sysop_level,
        }
    }

    pub fn has_permission(&self, permission: AdminPermission) -> bool {
        if self.user_security_level < self.sysop_level {
            return false;
        }

        match permission {
            AdminPermission::ViewUsers => self.user_security_level >= self.sysop_level,
            AdminPermission::EditUsers => self.user_security_level >= self.sysop_level,
            AdminPermission::DeleteUsers => self.user_security_level >= 250,
            AdminPermission::BanUsers => self.user_security_level >= self.sysop_level,
            AdminPermission::ManageFileAreas => self.user_security_level >= self.sysop_level,
            AdminPermission::ViewSessions => self.user_security_level >= self.sysop_level,
            AdminPermission::KickUsers => self.user_security_level >= self.sysop_level,
            AdminPermission::BroadcastMessages => self.user_security_level >= self.sysop_level,
            AdminPermission::SystemMaintenance => self.user_security_level >= 255,
            AdminPermission::ViewLogs => self.user_security_level >= self.sysop_level,
        }
    }

    pub fn require_permission(&self, permission: AdminPermission) -> anyhow::Result<()> {
        if !self.has_permission(permission) {
            return Err(anyhow::anyhow!(
                "Access denied: insufficient permissions for {:?}",
                permission
            ));
        }
        Ok(())
    }
}

pub struct AuditLogger {
    db: sqlx::PgPool,
}

impl AuditLogger {
    pub async fn log_admin_action(
        &self,
        admin_user_id: i32,
        action: &str,
        target: Option<&str>,
        details: Option<&str>,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO admin_audit_log (admin_user_id, action, target, details, timestamp)
             VALUES ($1, $2, $3, $4, NOW())",
            admin_user_id,
            action,
            target,
            details
        )
        .execute(&self.db)
        .await?;

        tracing::info!(
            admin_user_id = admin_user_id,
            action = action,
            target = target,
            "Admin action logged"
        );

        Ok(())
    }
}
```

**User Management:**
```rust
use sqlx::PgPool;
use chrono::{DateTime, Utc};

pub struct UserManager {
    db: PgPool,
    audit: AuditLogger,
    access_control: AdminAccessControl,
}

impl UserManager {
    pub async fn list_users(
        &self,
        admin_user_id: i32,
        page: i64,
        page_size: i64,
    ) -> anyhow::Result<Vec<UserSummary>> {
        self.access_control.require_permission(AdminPermission::ViewUsers)?;

        let offset = page * page_size;

        let users = sqlx::query_as!(
            UserSummary,
            r#"
            SELECT
                id,
                username,
                email,
                security_level,
                last_login,
                login_count,
                is_banned
            FROM users
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#,
            page_size,
            offset
        )
        .fetch_all(&self.db)
        .await?;

        self.audit.log_admin_action(
            admin_user_id,
            "list_users",
            None,
            Some(&format!("page={}, count={}", page, users.len())),
        ).await?;

        Ok(users)
    }

    pub async fn edit_user(
        &self,
        admin_user_id: i32,
        user_id: i32,
        changes: UserEditRequest,
    ) -> anyhly::Result<()> {
        self.access_control.require_permission(AdminPermission::EditUsers)?;

        let mut tx = self.db.begin().await?;

        // Build dynamic update query based on provided fields
        if let Some(email) = &changes.email {
            sqlx::query!(
                "UPDATE users SET email = $1 WHERE id = $2",
                email,
                user_id
            )
            .execute(&mut *tx)
            .await?;
        }

        if let Some(security_level) = changes.security_level {
            sqlx::query!(
                "UPDATE users SET security_level = $1 WHERE id = $2",
                security_level,
                user_id
            )
            .execute(&mut *tx)
            .await?;
        }

        if let Some(time_limit) = changes.time_limit_minutes {
            sqlx::query!(
                "UPDATE users SET time_limit_minutes = $1 WHERE id = $2",
                time_limit,
                user_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        self.audit.log_admin_action(
            admin_user_id,
            "edit_user",
            Some(&user_id.to_string()),
            Some(&format!("{:?}", changes)),
        ).await?;

        Ok(())
    }

    pub async fn ban_user(
        &self,
        admin_user_id: i32,
        user_id: i32,
        reason: String,
    ) -> anyhow::Result<()> {
        self.access_control.require_permission(AdminPermission::BanUsers)?;

        sqlx::query!(
            "UPDATE users SET is_banned = true, ban_reason = $1, banned_at = NOW()
             WHERE id = $2",
            reason,
            user_id
        )
        .execute(&self.db)
        .await?;

        self.audit.log_admin_action(
            admin_user_id,
            "ban_user",
            Some(&user_id.to_string()),
            Some(&reason),
        ).await?;

        Ok(())
    }

    pub async fn delete_user(
        &self,
        admin_user_id: i32,
        user_id: i32,
    ) -> anyhow::Result<()> {
        self.access_control.require_permission(AdminPermission::DeleteUsers)?;

        // Soft delete - mark as deleted but preserve data
        sqlx::query!(
            "UPDATE users SET is_deleted = true, deleted_at = NOW() WHERE id = $1",
            user_id
        )
        .execute(&self.db)
        .await?;

        self.audit.log_admin_action(
            admin_user_id,
            "delete_user",
            Some(&user_id.to_string()),
            None,
        ).await?;

        Ok(())
    }

    pub async fn view_login_history(
        &self,
        admin_user_id: i32,
        user_id: i32,
        limit: i64,
    ) -> anyhow::Result<Vec<LoginHistoryEntry>> {
        self.access_control.require_permission(AdminPermission::ViewUsers)?;

        let history = sqlx::query_as!(
            LoginHistoryEntry,
            r#"
            SELECT
                login_time,
                logout_time,
                ip_address,
                session_duration_minutes
            FROM login_history
            WHERE user_id = $1
            ORDER BY login_time DESC
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(&self.db)
        .await?;

        Ok(history)
    }
}

#[derive(Debug)]
pub struct UserSummary {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub security_level: i16,
    pub last_login: Option<DateTime<Utc>>,
    pub login_count: i32,
    pub is_banned: bool,
}

#[derive(Debug)]
pub struct UserEditRequest {
    pub email: Option<String>,
    pub security_level: Option<i16>,
    pub time_limit_minutes: Option<i32>,
}

#[derive(Debug)]
pub struct LoginHistoryEntry {
    pub login_time: DateTime<Utc>,
    pub logout_time: Option<DateTime<Utc>>,
    pub ip_address: String,
    pub session_duration_minutes: Option<i32>,
}
```

**File Area Management:**
```rust
pub struct FileAreaManager {
    db: PgPool,
    audit: AuditLogger,
    access_control: AdminAccessControl,
}

impl FileAreaManager {
    pub async fn create_area(
        &self,
        admin_user_id: i32,
        area: NewFileArea,
    ) -> anyhow::Result<i32> {
        self.access_control.require_permission(AdminPermission::ManageFileAreas)?;

        let area_id = sqlx::query_scalar!(
            r#"
            INSERT INTO file_areas
                (name, description, path, min_security_upload, min_security_download, max_file_size_mb)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            area.name,
            area.description,
            area.path,
            area.min_security_upload,
            area.min_security_download,
            area.max_file_size_mb
        )
        .fetch_one(&self.db)
        .await?;

        // Create directory if it doesn't exist
        tokio::fs::create_dir_all(&area.path).await?;

        self.audit.log_admin_action(
            admin_user_id,
            "create_file_area",
            Some(&area_id.to_string()),
            Some(&area.name),
        ).await?;

        Ok(area_id)
    }

    pub async fn edit_area(
        &self,
        admin_user_id: i32,
        area_id: i32,
        changes: FileAreaEditRequest,
    ) -> anyhow::Result<()> {
        self.access_control.require_permission(AdminPermission::ManageFileAreas)?;

        let mut tx = self.db.begin().await?;

        if let Some(name) = &changes.name {
            sqlx::query!(
                "UPDATE file_areas SET name = $1 WHERE id = $2",
                name,
                area_id
            )
            .execute(&mut *tx)
            .await?;
        }

        if let Some(description) = &changes.description {
            sqlx::query!(
                "UPDATE file_areas SET description = $1 WHERE id = $2",
                description,
                area_id
            )
            .execute(&mut *tx)
            .await?;
        }

        if let Some(min_security_upload) = changes.min_security_upload {
            sqlx::query!(
                "UPDATE file_areas SET min_security_upload = $1 WHERE id = $2",
                min_security_upload,
                area_id
            )
            .execute(&mut *tx)
            .await?;
        }

        if let Some(min_security_download) = changes.min_security_download {
            sqlx::query!(
                "UPDATE file_areas SET min_security_download = $1 WHERE id = $2",
                min_security_download,
                area_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        self.audit.log_admin_action(
            admin_user_id,
            "edit_file_area",
            Some(&area_id.to_string()),
            Some(&format!("{:?}", changes)),
        ).await?;

        Ok(())
    }

    pub async fn delete_area(
        &self,
        admin_user_id: i32,
        area_id: i32,
        delete_files: bool,
    ) -> anyhow::Result<()> {
        self.access_control.require_permission(AdminPermission::ManageFileAreas)?;

        // Get area path before deleting
        let area = sqlx::query!(
            "SELECT path FROM file_areas WHERE id = $1",
            area_id
        )
        .fetch_one(&self.db)
        .await?;

        // Delete from database
        sqlx::query!(
            "DELETE FROM file_areas WHERE id = $1",
            area_id
        )
        .execute(&self.db)
        .await?;

        // Optionally delete files from disk
        if delete_files {
            tokio::fs::remove_dir_all(&area.path).await?;
        }

        self.audit.log_admin_action(
            admin_user_id,
            "delete_file_area",
            Some(&area_id.to_string()),
            Some(&format!("delete_files={}", delete_files)),
        ).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct NewFileArea {
    pub name: String,
    pub description: String,
    pub path: String,
    pub min_security_upload: i16,
    pub min_security_download: i16,
    pub max_file_size_mb: i32,
}

#[derive(Debug)]
pub struct FileAreaEditRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub min_security_upload: Option<i16>,
    pub min_security_download: Option<i16>,
}
```

**System Maintenance:**
```rust
use tokio::sync::broadcast;
use std::sync::Arc;
use std::collections::HashMap;

pub struct SystemMaintenance {
    sessions: Arc<SessionManager>,
    broadcast_tx: broadcast::Sender<SystemMessage>,
    access_control: AdminAccessControl,
    audit: AuditLogger,
}

impl SystemMaintenance {
    pub async fn view_active_sessions(
        &self,
        admin_user_id: i32,
    ) -> anyhow::Result<Vec<ActiveSession>> {
        self.access_control.require_permission(AdminPermission::ViewSessions)?;

        let sessions = self.sessions.get_all_active().await;

        Ok(sessions)
    }

    pub async fn kick_user(
        &self,
        admin_user_id: i32,
        session_id: uuid::Uuid,
        reason: String,
    ) -> anyhow::Result<()> {
        self.access_control.require_permission(AdminPermission::KickUsers)?;

        // Send disconnect message to session
        self.sessions.disconnect_session(session_id, &reason).await?;

        self.audit.log_admin_action(
            admin_user_id,
            "kick_user",
            Some(&session_id.to_string()),
            Some(&reason),
        ).await?;

        Ok(())
    }

    pub async fn broadcast_message(
        &self,
        admin_user_id: i32,
        message: String,
    ) -> anyhow::Result<usize> {
        self.access_control.require_permission(AdminPermission::BroadcastMessages)?;

        let sys_msg = SystemMessage::Broadcast {
            from_admin: admin_user_id,
            message: message.clone(),
            timestamp: Utc::now(),
        };

        // Send to all active sessions via broadcast channel
        let receiver_count = self.broadcast_tx.send(sys_msg)?;

        self.audit.log_admin_action(
            admin_user_id,
            "broadcast_message",
            None,
            Some(&format!("receivers={}, message={}", receiver_count, message)),
        ).await?;

        Ok(receiver_count)
    }

    pub async fn kick_idle_users(
        &self,
        admin_user_id: i32,
        idle_minutes: i64,
    ) -> anyhow::Result<Vec<uuid::Uuid>> {
        self.access_control.require_permission(AdminPermission::KickUsers)?;

        let idle_threshold = Utc::now() - chrono::Duration::minutes(idle_minutes);
        let mut kicked = Vec::new();

        for session in self.sessions.get_all_active().await {
            if session.last_activity < idle_threshold {
                self.sessions.disconnect_session(
                    session.id,
                    &format!("Idle for {} minutes", idle_minutes)
                ).await?;
                kicked.push(session.id);
            }
        }

        self.audit.log_admin_action(
            admin_user_id,
            "kick_idle_users",
            None,
            Some(&format!("kicked={}, idle_minutes={}", kicked.len(), idle_minutes)),
        ).await?;

        Ok(kicked)
    }
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub id: uuid::Uuid,
    pub user_id: i32,
    pub username: String,
    pub node: u16,
    pub ip_address: String,
    pub login_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub current_menu: String,
}

#[derive(Debug, Clone)]
pub enum SystemMessage {
    Broadcast {
        from_admin: i32,
        message: String,
        timestamp: DateTime<Utc>,
    },
    Disconnect {
        reason: String,
    },
}

pub struct SessionManager {
    active_sessions: tokio::sync::RwLock<HashMap<uuid::Uuid, ActiveSession>>,
}

impl SessionManager {
    pub async fn get_all_active(&self) -> Vec<ActiveSession> {
        let sessions = self.active_sessions.read().await;
        sessions.values().cloned().collect()
    }

    pub async fn disconnect_session(
        &self,
        session_id: uuid::Uuid,
        reason: &str,
    ) -> anyhow::Result<()> {
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.remove(&session_id) {
            tracing::info!(
                session_id = %session_id,
                user_id = session.user_id,
                reason = reason,
                "Session disconnected by admin"
            );
        }
        Ok(())
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 06**: User system for user management
- **Sprint 13**: File areas for file area management
- **Sprint 05**: Terminal/Session for active session monitoring

### Blocks Downstream
- **Sprint 24**: Integration testing includes admin functionality
- **Sprint 29**: Web admin interface (complements TUI admin)

---

## Testing Requirements

### Unit Tests
- [ ] Access control permissions validate correctly
- [ ] Audit logging records all actions
- [ ] User edit transactions rollback on error
- [ ] File area creation validates input
- [ ] Session kick removes user correctly

### Integration Tests
- [ ] Complete user management workflow (list, edit, ban, delete)
- [ ] File area creation and deletion with filesystem operations
- [ ] Broadcast message reaches all active sessions
- [ ] Kick idle users identifies and removes correct sessions
- [ ] Audit log captures all admin actions

### Security Tests
- [ ] Non-SysOp users cannot access admin functions
- [ ] Access control properly enforces permission levels
- [ ] Audit log cannot be tampered with
- [ ] User deletion preserves data integrity
- [ ] File area deletion handles missing directories gracefully

### Performance Tests
- [ ] List 10,000 users with pagination < 100ms per page
- [ ] Broadcast message to 100 sessions < 200ms
- [ ] View active sessions < 50ms
- [ ] User edit with transaction < 100ms

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- TUI-based admin interface (works over SSH)
- Audit log for all administrative actions (compliance)
- Soft delete for users (preserve historical data)
- Security level 255 for highest admin permissions
- Broadcast uses Tokio broadcast channel for efficiency
- File area deletion optionally preserves files on disk
- Pagination for all list operations (scalability)
- Transaction-based user edits (atomicity)

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Unauthorized access to admin functions
- **Mitigation**: Strict access control checks; audit all actions; require high security levels
- **Risk**: Admin actions may corrupt data
- **Mitigation**: Use database transactions; validate all input; provide undo for critical operations
- **Risk**: Broadcast messages may spam users
- **Mitigation**: Rate limiting; confirmation before broadcast; log all messages
- **Risk**: Deleting file areas may lose data
- **Mitigation**: Confirmation dialog; optional file preservation; backup recommendation

---

## Progress Log

### Week 1
- *Date*: Progress notes will be added here as sprint progresses

### Week 2
- *Date*: Progress notes will be added here as sprint progresses

### Week 3
- *Date*: Progress notes will be added here as sprint progresses

### Sprint Completion
- **Completed**: TBD
- **Velocity**: TBD
- **Burndown**: TBD
