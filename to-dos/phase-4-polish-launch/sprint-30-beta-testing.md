# Sprint 30: Beta Testing & Bug Fixes

**Phase:** Phase 4 - Polish & Launch
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 30 conducts public beta testing with community participation, collecting feedback, triaging issues, and fixing reported bugs. Aims for 20+ beta testers providing real-world validation.

**Context:** Sprint 6 of Phase 4. Final testing before release.

**Expected Outcomes:** BBS validated by real users, critical bugs fixed, positive feedback.

---

## Objectives

- [ ] Conduct public beta test with 20+ participants
- [ ] Collect and triage user feedback
- [ ] Fix all reported critical and high-priority bugs
- [ ] Release iterative beta builds weekly

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| Public beta release | Binaries | Linux, Windows, macOS builds |
| Bug tracker | System | GitHub Issues with templates |
| Bug fix patches | Code | Weekly beta builds |
| Feedback analysis | Report | Summarized user feedback |

---

## Detailed Tasks

### Task Category 1: Beta Release

- [ ] **Task 1.1**: Prepare beta builds
  - Files affected: Build scripts
  - Estimated hours: 4

- [ ] **Task 1.2**: Publish Docker images
  - Files affected: Docker Hub
  - Estimated hours: 3

- [ ] **Task 1.3**: Announce beta to community
  - Files affected: Announcements
  - Estimated hours: 2

### Task Category 2: Feedback Collection

- [ ] **Task 2.1**: Set up GitHub Issues
  - Files affected: `.github/ISSUE_TEMPLATE/`
  - Estimated hours: 2

- [ ] **Task 2.2**: Create Discord/IRC channels
  - Estimated hours: 2

- [ ] **Task 2.3**: Triage incoming reports
  - Estimated hours: 12

### Task Category 3: Bug Fixing

- [ ] **Task 3.1**: Fix critical bugs
  - Estimated hours: 20

- [ ] **Task 3.2**: Fix high-priority bugs
  - Estimated hours: 16

- [ ] **Task 3.3**: Respond to user questions
  - Estimated hours: 8

### Task Category 4: Iterative Releases

- [ ] **Task 4.1**: Weekly beta builds
  - Estimated hours: 6

- [ ] **Task 4.2**: Communicate fixes to testers
  - Estimated hours: 4

---

## Acceptance Criteria

- [ ] 20+ beta testers participated
- [ ] All critical bugs fixed
- [ ] User feedback positive
- [ ] Weekly builds released

---

## Technical Details

### Architecture Considerations

- Implement beta user invitation system with registration codes
- Build structured feedback collection API with categorization
- Integrate GitHub Issues API for automatic bug report creation
- Deploy telemetry system for anonymous usage data collection
- Create beta tester dashboard for progress tracking
- Set up automated crash reporting with stack trace capture
- Implement feature flag system for staged rollouts
- Use rate limiting to prevent feedback spam
- Store feedback in database with full-text search
- Generate weekly analytics reports for beta progress

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
axum = "0.7"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }
serde = { workspace = true }
serde_json = "1.0"
octocrab = "0.32"  # GitHub API client
reqwest = { version = "0.11", features = ["json"] }
chrono = { workspace = true }
uuid = { version = "1.6", features = ["v4", "serde"] }
bcrypt = "0.15"
rand = "0.8"
anyhow = { workspace = true }
thiserror = "1.0"
tracing = { workspace = true }
backtrace = "0.3"
sysinfo = "0.30"

[dev-dependencies]
mockall = "0.12"
wiremock = "0.5"
```

**Pascal Units Being Tested:**
- All modules (comprehensive integration testing)
- USER.PAS (beta tester accounts)
- MSGBASE.PAS (feedback messages)
- FILEBASE.PAS (crash log uploads)
- DOORS.PAS (door game stability)
- MENU.PAS (UI navigation)

**External Dependencies:**
- GitHub API (issue creation, labeling)
- PostgreSQL (feedback storage)
- Optional: Discord Webhooks for notifications

### Code Examples

**Beta User Invitation System:**
```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
    routing::{get, post},
};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BetaInvitation {
    pub id: Uuid,
    pub code: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub used_by_user_id: Option<i32>,
    pub inviter_user_id: Option<i32>,
    pub max_uses: i32,
    pub current_uses: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BetaTester {
    pub id: i32,
    pub user_id: i32,
    pub invitation_code: String,
    pub joined_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub feedback_count: i32,
    pub bug_reports_count: i32,
    pub is_active: bool,
    pub platform: String,  // linux, windows, macos
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvitationRequest {
    pub email: String,
    pub max_uses: Option<i32>,
    pub expires_in_days: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedeemInvitationRequest {
    pub code: String,
    pub username: String,
    pub email: String,
    pub platform: String,
}

pub struct BetaInvitationService {
    db: PgPool,
}

impl BetaInvitationService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Generate a unique invitation code (8 characters, alphanumeric)
    fn generate_code() -> String {
        let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
        let mut rng = rand::thread_rng();
        (0..8)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    pub async fn create_invitation(
        &self,
        req: CreateInvitationRequest,
        inviter_user_id: Option<i32>,
    ) -> anyhow::Result<BetaInvitation> {
        let code = Self::generate_code();
        let expires_at = Utc::now() + Duration::days(req.expires_in_days.unwrap_or(30) as i64);
        let max_uses = req.max_uses.unwrap_or(1);

        let invitation = sqlx::query_as!(
            BetaInvitation,
            r#"
            INSERT INTO beta_invitations (id, code, email, created_at, expires_at, inviter_user_id, max_uses, current_uses)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 0)
            RETURNING id, code, email, created_at, expires_at, used_at, used_by_user_id, inviter_user_id, max_uses, current_uses
            "#,
            Uuid::new_v4(),
            code,
            req.email,
            Utc::now(),
            expires_at,
            inviter_user_id,
            max_uses,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(invitation)
    }

    pub async fn redeem_invitation(
        &self,
        req: RedeemInvitationRequest,
    ) -> anyhow::Result<BetaTester> {
        // Start transaction
        let mut tx = self.db.begin().await?;

        // Fetch and validate invitation
        let invitation = sqlx::query_as!(
            BetaInvitation,
            r#"
            SELECT id, code, email, created_at, expires_at, used_at, used_by_user_id, inviter_user_id, max_uses, current_uses
            FROM beta_invitations
            WHERE code = $1
            FOR UPDATE
            "#,
            req.code
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid invitation code"))?;

        // Validate not expired
        if invitation.expires_at < Utc::now() {
            return Err(anyhow::anyhow!("Invitation code has expired"));
        }

        // Validate max uses
        if invitation.current_uses >= invitation.max_uses {
            return Err(anyhow::anyhow!("Invitation code has been fully used"));
        }

        // Create user account (simplified - integrate with existing user system)
        let user_id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (name, email, security_level, created_at)
            VALUES ($1, $2, 10, $3)
            RETURNING id
            "#,
            req.username,
            req.email,
            Utc::now(),
        )
        .fetch_one(&mut *tx)
        .await?;

        // Create beta tester record
        let tester = sqlx::query_as!(
            BetaTester,
            r#"
            INSERT INTO beta_testers (user_id, invitation_code, joined_at, last_active, feedback_count, bug_reports_count, is_active, platform, version)
            VALUES ($1, $2, $3, $3, 0, 0, true, $4, $5)
            RETURNING id, user_id, invitation_code, joined_at, last_active, feedback_count, bug_reports_count, is_active, platform, version
            "#,
            user_id,
            req.code,
            Utc::now(),
            req.platform,
            env!("CARGO_PKG_VERSION"),
        )
        .fetch_one(&mut *tx)
        .await?;

        // Update invitation usage
        sqlx::query!(
            r#"
            UPDATE beta_invitations
            SET current_uses = current_uses + 1,
                used_at = CASE WHEN used_at IS NULL THEN $1 ELSE used_at END,
                used_by_user_id = CASE WHEN used_by_user_id IS NULL THEN $2 ELSE used_by_user_id END
            WHERE code = $3
            "#,
            Utc::now(),
            user_id,
            req.code,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(tester)
    }

    pub async fn get_active_testers(&self) -> anyhow::Result<Vec<BetaTester>> {
        let testers = sqlx::query_as!(
            BetaTester,
            r#"
            SELECT id, user_id, invitation_code, joined_at, last_active, feedback_count, bug_reports_count, is_active, platform, version
            FROM beta_testers
            WHERE is_active = true
            ORDER BY joined_at DESC
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(testers)
    }

    pub async fn update_tester_activity(&self, user_id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE beta_testers
            SET last_active = $1
            WHERE user_id = $2
            "#,
            Utc::now(),
            user_id,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

// REST API endpoints
pub fn beta_routes(db: PgPool) -> Router {
    let service = Arc::new(BetaInvitationService::new(db));

    Router::new()
        .route("/beta/invitations", post(create_invitation))
        .route("/beta/invitations/redeem", post(redeem_invitation))
        .route("/beta/testers", get(list_testers))
        .with_state(service)
}

async fn create_invitation(
    State(service): State<Arc<BetaInvitationService>>,
    Json(req): Json<CreateInvitationRequest>,
) -> Result<Json<BetaInvitation>, StatusCode> {
    let invitation = service
        .create_invitation(req, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(invitation))
}

async fn redeem_invitation(
    State(service): State<Arc<BetaInvitationService>>,
    Json(req): Json<RedeemInvitationRequest>,
) -> Result<Json<BetaTester>, StatusCode> {
    let tester = service
        .redeem_invitation(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(tester))
}

async fn list_testers(
    State(service): State<Arc<BetaInvitationService>>,
) -> Result<Json<Vec<BetaTester>>, StatusCode> {
    let testers = service
        .get_active_testers()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(testers))
}
```

**Feedback Collection API:**
```rust
use axum::{extract::State, http::StatusCode, Json, Router, routing::post};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "feedback_category", rename_all = "lowercase")]
pub enum FeedbackCategory {
    Bug,
    Feature,
    Usability,
    Performance,
    Documentation,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "feedback_severity", rename_all = "lowercase")]
pub enum FeedbackSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Feedback {
    pub id: Uuid,
    pub user_id: i32,
    pub category: FeedbackCategory,
    pub severity: Option<FeedbackSeverity>,
    pub title: String,
    pub description: String,
    pub steps_to_reproduce: Option<String>,
    pub expected_behavior: Option<String>,
    pub actual_behavior: Option<String>,
    pub system_info: serde_json::Value,
    pub attachments: Vec<String>,
    pub rating: Option<i16>,  // 1-5 stars
    pub created_at: DateTime<Utc>,
    pub github_issue_number: Option<i32>,
    pub status: String,  // open, triaged, fixed, closed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitFeedbackRequest {
    pub category: FeedbackCategory,
    pub severity: Option<FeedbackSeverity>,
    pub title: String,
    pub description: String,
    pub steps_to_reproduce: Option<String>,
    pub expected_behavior: Option<String>,
    pub actual_behavior: Option<String>,
    pub rating: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub version: String,
    pub rust_version: String,
    pub uptime_seconds: u64,
    pub memory_mb: u64,
    pub cpu_cores: usize,
}

pub struct FeedbackService {
    db: PgPool,
    github_service: Option<Arc<GitHubIssueService>>,
}

impl FeedbackService {
    pub fn new(db: PgPool, github_service: Option<Arc<GitHubIssueService>>) -> Self {
        Self { db, github_service }
    }

    pub async fn submit_feedback(
        &self,
        user_id: i32,
        req: SubmitFeedbackRequest,
        system_info: SystemInfo,
    ) -> anyhow::Result<Feedback> {
        // Validate rating if provided
        if let Some(rating) = req.rating {
            if !(1..=5).contains(&rating) {
                return Err(anyhow::anyhow!("Rating must be between 1 and 5"));
            }
        }

        // Insert feedback
        let feedback = sqlx::query_as!(
            Feedback,
            r#"
            INSERT INTO feedback (id, user_id, category, severity, title, description, steps_to_reproduce, expected_behavior, actual_behavior, system_info, attachments, rating, created_at, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, 'open')
            RETURNING id, user_id, category as "category: FeedbackCategory", severity as "severity: FeedbackSeverity", title, description, steps_to_reproduce, expected_behavior, actual_behavior, system_info, attachments, rating, created_at, github_issue_number, status
            "#,
            Uuid::new_v4(),
            user_id,
            req.category as FeedbackCategory,
            req.severity as Option<FeedbackSeverity>,
            req.title,
            req.description,
            req.steps_to_reproduce,
            req.expected_behavior,
            req.actual_behavior,
            serde_json::to_value(&system_info)?,
            &[] as &[String],
            req.rating,
            Utc::now(),
        )
        .fetch_one(&self.db)
        .await?;

        // Update beta tester feedback count
        sqlx::query!(
            r#"
            UPDATE beta_testers
            SET feedback_count = feedback_count + 1,
                bug_reports_count = CASE WHEN $1 = 'bug' THEN bug_reports_count + 1 ELSE bug_reports_count END
            WHERE user_id = $2
            "#,
            req.category as FeedbackCategory,
            user_id,
        )
        .execute(&self.db)
        .await?;

        // Auto-create GitHub issue for bugs with high/critical severity
        if matches!(req.category, FeedbackCategory::Bug) {
            if let Some(severity) = req.severity {
                if matches!(severity, FeedbackSeverity::Critical | FeedbackSeverity::High) {
                    if let Some(github) = &self.github_service {
                        match github.create_issue_from_feedback(&feedback).await {
                            Ok(issue_number) => {
                                // Update feedback with GitHub issue number
                                sqlx::query!(
                                    r#"
                                    UPDATE feedback
                                    SET github_issue_number = $1
                                    WHERE id = $2
                                    "#,
                                    issue_number,
                                    feedback.id,
                                )
                                .execute(&self.db)
                                .await?;
                            }
                            Err(e) => {
                                tracing::error!("Failed to create GitHub issue: {}", e);
                            }
                        }
                    }
                }
            }
        }

        Ok(feedback)
    }

    pub async fn get_feedback_stats(&self) -> anyhow::Result<FeedbackStats> {
        let stats = sqlx::query_as!(
            FeedbackStats,
            r#"
            SELECT
                COUNT(*) as total_feedback,
                COUNT(*) FILTER (WHERE category = 'bug') as bug_reports,
                COUNT(*) FILTER (WHERE category = 'feature') as feature_requests,
                COUNT(*) FILTER (WHERE severity = 'critical') as critical_issues,
                COUNT(*) FILTER (WHERE severity = 'high') as high_priority_issues,
                COUNT(*) FILTER (WHERE status = 'open') as open_items,
                COUNT(*) FILTER (WHERE status = 'fixed') as fixed_items,
                AVG(rating) as average_rating
            FROM feedback
            "#
        )
        .fetch_one(&self.db)
        .await?;

        Ok(stats)
    }

    pub async fn search_feedback(
        &self,
        query: &str,
        category: Option<FeedbackCategory>,
        severity: Option<FeedbackSeverity>,
        status: Option<&str>,
    ) -> anyhow::Result<Vec<Feedback>> {
        let mut sql = String::from(
            r#"
            SELECT id, user_id, category, severity, title, description, steps_to_reproduce, expected_behavior, actual_behavior, system_info, attachments, rating, created_at, github_issue_number, status
            FROM feedback
            WHERE 1=1
            "#
        );

        if !query.is_empty() {
            sql.push_str(" AND (title ILIKE $1 OR description ILIKE $1)");
        }

        if category.is_some() {
            sql.push_str(" AND category = $2");
        }

        if severity.is_some() {
            sql.push_str(" AND severity = $3");
        }

        if status.is_some() {
            sql.push_str(" AND status = $4");
        }

        sql.push_str(" ORDER BY created_at DESC LIMIT 100");

        // Use dynamic query building (simplified for example)
        let feedback = sqlx::query_as::<_, Feedback>(&sql)
            .fetch_all(&self.db)
            .await?;

        Ok(feedback)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FeedbackStats {
    pub total_feedback: Option<i64>,
    pub bug_reports: Option<i64>,
    pub feature_requests: Option<i64>,
    pub critical_issues: Option<i64>,
    pub high_priority_issues: Option<i64>,
    pub open_items: Option<i64>,
    pub fixed_items: Option<i64>,
    pub average_rating: Option<f64>,
}

// REST API endpoints
pub fn feedback_routes(db: PgPool, github_service: Option<Arc<GitHubIssueService>>) -> Router {
    let service = Arc::new(FeedbackService::new(db, github_service));

    Router::new()
        .route("/feedback", post(submit_feedback))
        .route("/feedback/stats", axum::routing::get(get_feedback_stats))
        .with_state(service)
}

async fn submit_feedback(
    State(service): State<Arc<FeedbackService>>,
    Json(req): Json<SubmitFeedbackRequest>,
) -> Result<Json<Feedback>, StatusCode> {
    // Extract system info (simplified - would come from client)
    let system_info = SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: "1.75.0".to_string(),
        uptime_seconds: 0,
        memory_mb: 0,
        cpu_cores: 0,
    };

    let feedback = service
        .submit_feedback(1, req, system_info)  // TODO: Get user_id from auth
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(feedback))
}

async fn get_feedback_stats(
    State(service): State<Arc<FeedbackService>>,
) -> Result<Json<FeedbackStats>, StatusCode> {
    let stats = service
        .get_feedback_stats()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(stats))
}
```

**GitHub Issues Integration:**
```rust
use octocrab::{Octocrab, OctocrabBuilder};
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub struct GitHubIssueService {
    client: Octocrab,
    owner: String,
    repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueLabels {
    pub category: String,
    pub severity: String,
    pub status: String,
}

impl GitHubIssueService {
    pub fn new(token: String, owner: String, repo: String) -> Result<Self> {
        let client = OctocrabBuilder::new()
            .personal_token(token)
            .build()?;

        Ok(Self { client, owner, repo })
    }

    pub async fn create_issue_from_feedback(&self, feedback: &Feedback) -> Result<i32> {
        let labels = self.get_labels_for_feedback(feedback);

        let body = self.format_issue_body(feedback);

        let issue = self.client
            .issues(&self.owner, &self.repo)
            .create(&feedback.title)
            .body(&body)
            .labels(vec![
                labels.category,
                labels.severity,
                "beta-testing".to_string(),
            ])
            .send()
            .await?;

        Ok(issue.number as i32)
    }

    fn get_labels_for_feedback(&self, feedback: &Feedback) -> IssueLabels {
        let category = match feedback.category {
            FeedbackCategory::Bug => "bug",
            FeedbackCategory::Feature => "enhancement",
            FeedbackCategory::Usability => "ux",
            FeedbackCategory::Performance => "performance",
            FeedbackCategory::Documentation => "documentation",
            FeedbackCategory::Other => "question",
        };

        let severity = if let Some(sev) = feedback.severity {
            match sev {
                FeedbackSeverity::Critical => "priority:critical",
                FeedbackSeverity::High => "priority:high",
                FeedbackSeverity::Medium => "priority:medium",
                FeedbackSeverity::Low => "priority:low",
            }
        } else {
            "priority:medium"
        };

        IssueLabels {
            category: category.to_string(),
            severity: severity.to_string(),
            status: "status:new".to_string(),
        }
    }

    fn format_issue_body(&self, feedback: &Feedback) -> String {
        let mut body = format!(
            "## Description\n\n{}\n\n",
            feedback.description
        );

        if let Some(steps) = &feedback.steps_to_reproduce {
            body.push_str(&format!(
                "## Steps to Reproduce\n\n{}\n\n",
                steps
            ));
        }

        if let Some(expected) = &feedback.expected_behavior {
            body.push_str(&format!(
                "## Expected Behavior\n\n{}\n\n",
                expected
            ));
        }

        if let Some(actual) = &feedback.actual_behavior {
            body.push_str(&format!(
                "## Actual Behavior\n\n{}\n\n",
                actual
            ));
        }

        // Add system info
        if let Ok(system_info) = serde_json::from_value::<SystemInfo>(feedback.system_info.clone()) {
            body.push_str(&format!(
                "## System Information\n\n- **OS**: {}\n- **Architecture**: {}\n- **Version**: {}\n- **Rust**: {}\n\n",
                system_info.os,
                system_info.arch,
                system_info.version,
                system_info.rust_version,
            ));
        }

        body.push_str(&format!(
            "---\n*Automatically created from beta feedback (ID: {})*\n",
            feedback.id
        ));

        body
    }

    pub async fn update_issue_status(
        &self,
        issue_number: i32,
        status: &str,
    ) -> Result<()> {
        // Remove old status labels
        let issue = self.client
            .issues(&self.owner, &self.repo)
            .get(issue_number as u64)
            .await?;

        for label in issue.labels {
            if label.name.starts_with("status:") {
                self.client
                    .issues(&self.owner, &self.repo)
                    .remove_label(issue_number as u64, &label.name)
                    .await?;
            }
        }

        // Add new status label
        let new_label = format!("status:{}", status);
        self.client
            .issues(&self.owner, &self.repo)
            .add_labels(issue_number as u64, &[new_label])
            .await?;

        Ok(())
    }

    pub async fn close_issue(&self, issue_number: i32) -> Result<()> {
        self.client
            .issues(&self.owner, &self.repo)
            .update(issue_number as u64)
            .state(octocrab::params::State::Closed)
            .send()
            .await?;

        Ok(())
    }

    pub async fn add_comment(&self, issue_number: i32, comment: &str) -> Result<()> {
        self.client
            .issues(&self.owner, &self.repo)
            .create_comment(issue_number as u64, comment)
            .await?;

        Ok(())
    }
}
```

**Telemetry & Crash Reporting:**
```rust
use backtrace::Backtrace;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::panic;
use std::sync::Arc;
use sysinfo::{System, SystemExt};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub id: Uuid,
    pub user_id: Option<i32>,
    pub session_id: Uuid,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    pub id: Uuid,
    pub user_id: Option<i32>,
    pub session_id: Uuid,
    pub panic_message: String,
    pub backtrace: String,
    pub system_info: SystemInfo,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageMetrics {
    pub session_id: Uuid,
    pub user_id: Option<i32>,
    pub login_time: DateTime<Utc>,
    pub logout_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub messages_read: i32,
    pub messages_posted: i32,
    pub files_downloaded: i32,
    pub files_uploaded: i32,
    pub doors_played: Vec<String>,
    pub menus_visited: Vec<String>,
}

pub struct TelemetryService {
    db: PgPool,
    session_id: Uuid,
    system_info: SystemInfo,
}

impl TelemetryService {
    pub fn new(db: PgPool) -> Self {
        let session_id = Uuid::new_v4();
        let system_info = Self::collect_system_info();

        Self {
            db,
            session_id,
            system_info,
        }
    }

    fn collect_system_info() -> SystemInfo {
        let mut sys = System::new_all();
        sys.refresh_all();

        SystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            rust_version: rustc_version_runtime::version().to_string(),
            uptime_seconds: sys.uptime(),
            memory_mb: sys.total_memory() / 1024 / 1024,
            cpu_cores: sys.cpus().len(),
        }
    }

    pub async fn track_event(
        &self,
        user_id: Option<i32>,
        event_type: String,
        event_data: serde_json::Value,
    ) -> anyhow::Result<()> {
        let event = TelemetryEvent {
            id: Uuid::new_v4(),
            user_id,
            session_id: self.session_id,
            event_type,
            event_data,
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            platform: std::env::consts::OS.to_string(),
        };

        sqlx::query!(
            r#"
            INSERT INTO telemetry_events (id, user_id, session_id, event_type, event_data, timestamp, version, platform)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            event.id,
            event.user_id,
            event.session_id,
            event.event_type,
            event.event_data,
            event.timestamp,
            event.version,
            event.platform,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub fn install_panic_handler(db: PgPool) {
        let original_hook = panic::take_hook();

        panic::set_hook(Box::new(move |panic_info| {
            let backtrace = Backtrace::new();
            let panic_message = panic_info.to_string();

            let crash_report = CrashReport {
                id: Uuid::new_v4(),
                user_id: None,  // TODO: Get from current session
                session_id: Uuid::new_v4(),
                panic_message: panic_message.clone(),
                backtrace: format!("{:?}", backtrace),
                system_info: TelemetryService::collect_system_info(),
                timestamp: Utc::now(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            };

            // Attempt to save crash report to database
            let db_clone = db.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::save_crash_report(&db_clone, &crash_report).await {
                    eprintln!("Failed to save crash report: {}", e);
                }
            });

            // Call original panic hook
            original_hook(panic_info);
        }));
    }

    async fn save_crash_report(db: &PgPool, report: &CrashReport) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO crash_reports (id, user_id, session_id, panic_message, backtrace, system_info, timestamp, version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            report.id,
            report.user_id,
            report.session_id,
            report.panic_message,
            report.backtrace,
            serde_json::to_value(&report.system_info)?,
            report.timestamp,
            report.version,
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn get_crash_report_stats(&self) -> anyhow::Result<CrashStats> {
        let stats = sqlx::query_as!(
            CrashStats,
            r#"
            SELECT
                COUNT(*) as total_crashes,
                COUNT(DISTINCT panic_message) as unique_crash_types,
                COUNT(*) FILTER (WHERE timestamp > NOW() - INTERVAL '24 hours') as crashes_last_24h,
                COUNT(*) FILTER (WHERE timestamp > NOW() - INTERVAL '7 days') as crashes_last_week
            FROM crash_reports
            "#
        )
        .fetch_one(&self.db)
        .await?;

        Ok(stats)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CrashStats {
    pub total_crashes: Option<i64>,
    pub unique_crash_types: Option<i64>,
    pub crashes_last_24h: Option<i64>,
    pub crashes_last_week: Option<i64>,
}

// Usage example
pub async fn initialize_telemetry(db: PgPool) -> Arc<TelemetryService> {
    let service = Arc::new(TelemetryService::new(db.clone()));

    TelemetryService::install_panic_handler(db);

    service
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 29**: Web admin provides UI for viewing feedback and stats
- **Sprint 06**: User system for beta tester accounts
- **Sprint 11**: Message base for feedback messages

### Blocks Downstream
- **Sprint 31**: Final polish based on beta feedback
- **Sprint 32**: Launch readiness depends on successful beta test

---

## Testing Requirements

### Unit Tests
- [ ] Invitation code generation is unique
- [ ] Invitation redemption validates expiration
- [ ] Invitation max uses enforcement
- [ ] Feedback category and severity validation
- [ ] Rating validation (1-5 range)
- [ ] Telemetry event serialization

### Integration Tests
- [ ] Beta tester registration flow (invitation → account)
- [ ] Feedback submission and storage
- [ ] GitHub issue creation from critical bugs
- [ ] Crash report capture and storage
- [ ] Telemetry event aggregation

### API Tests
- [ ] POST /beta/invitations creates valid codes
- [ ] POST /beta/invitations/redeem creates user and tester record
- [ ] GET /beta/testers returns active testers only
- [ ] POST /feedback creates feedback record and updates stats
- [ ] GET /feedback/stats returns accurate counts
- [ ] Unauthorized requests return 401

### GitHub Integration Tests
- [ ] Issue creation with correct labels
- [ ] Issue body formatting matches template
- [ ] Status label updates work correctly
- [ ] Issue closure works
- [ ] Comment posting works

### Performance Tests
- [ ] Handle 100+ concurrent feedback submissions
- [ ] Invitation redemption handles race conditions
- [ ] Telemetry events don't slow down BBS operations
- [ ] Crash report saving completes within 5 seconds
- [ ] Feedback search returns results in < 1 second

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use invitation codes instead of open registration (controlled beta)
- Auto-create GitHub issues for critical/high severity bugs
- Collect anonymous telemetry for usage patterns
- Implement panic handler for automatic crash reporting
- Require structured feedback (category, severity, description)
- Weekly beta builds with cumulative bug fixes
- Beta tester dashboard showing contribution stats
- Rate limit feedback submission to prevent spam (5 submissions/hour)

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Not enough beta testers sign up
- **Mitigation**: Announce on forums, social media, offer incentives (beta badge)
- **Risk**: GitHub API rate limits
- **Mitigation**: Cache responses, batch issue creation, use higher tier token
- **Risk**: Spam/abuse feedback submissions
- **Mitigation**: Rate limiting, require invitation code, moderation queue
- **Risk**: Crash reports contain sensitive data
- **Mitigation**: Sanitize user data, encrypt reports at rest, purge old reports
- **Risk**: Beta testers abandon testing mid-sprint
- **Mitigation**: Regular communication, weekly builds with visible fixes, engagement incentives

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
