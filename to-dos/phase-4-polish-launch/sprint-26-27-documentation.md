# Sprint 26-27: Comprehensive Documentation

**Phase:** Phase 4 - Polish & Launch
**Duration:** 6 weeks (Double Sprint)
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprints 26-27 produce comprehensive documentation including user manual, SysOp installation and configuration guide, API documentation, and optional video tutorials. This documentation is essential for adoption and support.

**Context:** Sprints 2-3 of Phase 4. Documentation for users, SysOps, and developers.

**Expected Outcomes:** Complete documentation suite enabling users to install, configure, and use the BBS.

---

## Objectives

- [ ] Write comprehensive user manual (50+ pages)
- [ ] Create SysOp installation and configuration guide (30+ pages)
- [ ] Generate complete API documentation (rustdoc)
- [ ] Produce video tutorials (optional)

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| User manual | Docs | 50+ pages covering all features |
| SysOp guide | Docs | 30+ pages installation & config |
| API documentation | Docs | Complete rustdoc coverage |
| Video tutorials | Video | Optional 5-10 tutorials |

---

## Detailed Tasks

### Task Category 1: User Manual

- [ ] **Task 1.1**: Getting started guide
  - Files affected: `docs/user-manual/01-getting-started.md`
  - Estimated hours: 8

- [ ] **Task 1.2**: Navigating the BBS
  - Files affected: `docs/user-manual/02-navigation.md`
  - Estimated hours: 6

- [ ] **Task 1.3**: Messaging guide
  - Files affected: `docs/user-manual/03-messaging.md`
  - Estimated hours: 8

- [ ] **Task 1.4**: File transfer guide
  - Files affected: `docs/user-manual/04-files.md`
  - Estimated hours: 8

- [ ] **Task 1.5**: FAQ section
  - Files affected: `docs/user-manual/99-faq.md`
  - Estimated hours: 6

### Task Category 2: SysOp Guide

- [ ] **Task 2.1**: Installation instructions
  - Files affected: `docs/sysop-guide/01-installation.md`
  - Estimated hours: 8

- [ ] **Task 2.2**: Configuration reference
  - Files affected: `docs/sysop-guide/02-configuration.md`
  - Estimated hours: 10

- [ ] **Task 2.3**: Customization (themes, menus)
  - Files affected: `docs/sysop-guide/03-customization.md`
  - Estimated hours: 8

- [ ] **Task 2.4**: Maintenance and backups
  - Files affected: `docs/sysop-guide/04-maintenance.md`
  - Estimated hours: 6

- [ ] **Task 2.5**: Troubleshooting
  - Files affected: `docs/sysop-guide/05-troubleshooting.md`
  - Estimated hours: 8

### Task Category 3: Developer Documentation

- [ ] **Task 3.1**: Architecture overview
  - Files affected: `docs/developer/architecture.md`
  - Estimated hours: 6

- [ ] **Task 3.2**: API documentation (rustdoc)
  - Files affected: Rustdoc comments throughout
  - Estimated hours: 16

- [ ] **Task 3.3**: Contributing guidelines
  - Files affected: `CONTRIBUTING.md`
  - Estimated hours: 4

- [ ] **Task 3.4**: Extension development
  - Files affected: `docs/developer/extensions.md`
  - Estimated hours: 6

### Task Category 4: Video Tutorials (Optional)

- [ ] **Task 4.1**: Installation tutorial
  - Estimated hours: 8

- [ ] **Task 4.2**: Configuration tutorial
  - Estimated hours: 6

- [ ] **Task 4.3**: Theme customization tutorial
  - Estimated hours: 5

---

## Acceptance Criteria

- [ ] Documentation comprehensive and clear
- [ ] All public APIs have rustdoc
- [ ] Tutorials tested and accurate
- [ ] Installation guide verified on all platforms

---


## Technical Details

### Architecture Considerations

- Use mdBook for user and SysOp guides (Markdown-based, easy to maintain)
- Generate API documentation with rustdoc (built into Cargo)
- Host docs on GitHub Pages or dedicated server
- Integrate doc generation into CI/CD pipeline
- Version documentation alongside code releases
- Support offline documentation bundles (PDF, HTML)
- Include searchable API reference
- Cross-reference between user guide, SysOp guide, and API docs
- Provide code examples in documentation
- Support multiple documentation versions (latest, stable, archives)

### Dependencies

**Crate-Level Dependencies:**
```toml
[dev-dependencies]
mdbook = "0.4"
mdbook-linkcheck = "0.7"
mdbook-toc = "0.14"
cargo-doc = { workspace = true }
insta = "1.34"  # Snapshot testing for docs

[build-dependencies]
pulldown-cmark = "0.9"  # Markdown parsing for doc validation
```

**External Dependencies:**
- mdBook (documentation generator)
- rustdoc (API documentation)
- GitHub Pages or static site hosting
- Optional: OBS Studio for video tutorials
- Optional: Asciidoctor for advanced formatting

**Documentation Files Being Created:**
- User manual (Markdown chapters in `docs/user-manual/`)
- SysOp guide (Markdown chapters in `docs/sysop-guide/`)
- Developer docs (Markdown in `docs/developer/`)
- Rustdoc comments (throughout all Rust source files)
- CONTRIBUTING.md (GitHub contribution guidelines)

### Code Examples

**Documentation Build System:**
```rust
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, Result};

/// Comprehensive documentation builder for Impulse 7.1 BBS
///
/// Handles building user manual, SysOp guide, API docs, and deploying to hosting.
pub struct DocumentationBuilder {
    project_root: PathBuf,
    output_dir: PathBuf,
    version: String,
}

impl DocumentationBuilder {
    pub fn new(project_root: PathBuf, version: String) -> Self {
        let output_dir = project_root.join("target").join("docs");
        Self {
            project_root,
            output_dir,
            version,
        }
    }

    /// Build all documentation types
    pub async fn build_all(&self) -> Result<()> {
        println!("Building comprehensive documentation for v{}", self.version);

        // Build user manual with mdBook
        self.build_user_manual().await
            .context("Failed to build user manual")?;

        // Build SysOp guide with mdBook
        self.build_sysop_guide().await
            .context("Failed to build SysOp guide")?;

        // Build developer docs
        self.build_developer_docs().await
            .context("Failed to build developer docs")?;

        // Build API documentation with rustdoc
        self.build_api_docs().await
            .context("Failed to build API documentation")?;

        // Generate combined index page
        self.generate_index_page().await
            .context("Failed to generate index page")?;

        // Validate all links
        self.validate_links().await
            .context("Link validation failed")?;

        println!("Documentation built successfully in {}", self.output_dir.display());
        Ok(())
    }

    async fn build_user_manual(&self) -> Result<()> {
        let book_dir = self.project_root.join("docs").join("user-manual");
        let output = self.output_dir.join("user-manual");

        let status = Command::new("mdbook")
            .arg("build")
            .arg(&book_dir)
            .arg("--dest-dir")
            .arg(&output)
            .status()
            .context("Failed to run mdbook for user manual")?;

        if !status.success() {
            anyhow::bail!("mdbook build failed for user manual");
        }

        Ok(())
    }

    async fn build_sysop_guide(&self) -> Result<()> {
        let book_dir = self.project_root.join("docs").join("sysop-guide");
        let output = self.output_dir.join("sysop-guide");

        let status = Command::new("mdbook")
            .arg("build")
            .arg(&book_dir)
            .arg("--dest-dir")
            .arg(&output)
            .status()
            .context("Failed to run mdbook for SysOp guide")?;

        if !status.success() {
            anyhow::bail!("mdbook build failed for SysOp guide");
        }

        Ok(())
    }

    async fn build_developer_docs(&self) -> Result<()> {
        let book_dir = self.project_root.join("docs").join("developer");
        let output = self.output_dir.join("developer");

        let status = Command::new("mdbook")
            .arg("build")
            .arg(&book_dir)
            .arg("--dest-dir")
            .arg(&output)
            .status()
            .context("Failed to run mdbook for developer docs")?;

        if !status.success() {
            anyhow::bail!("mdbook build failed for developer docs");
        }

        Ok(())
    }

    async fn build_api_docs(&self) -> Result<()> {
        let output = self.output_dir.join("api");

        let status = Command::new("cargo")
            .arg("doc")
            .arg("--no-deps")
            .arg("--all-features")
            .arg("--target-dir")
            .arg(&output)
            .current_dir(&self.project_root)
            .status()
            .context("Failed to run cargo doc")?;

        if !status.success() {
            anyhow::bail!("cargo doc failed");
        }

        Ok(())
    }

    async fn generate_index_page(&self) -> Result<()> {
        let index_html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Impulse 7.1 BBS Documentation</title>
    <style>
        body {{ font-family: sans-serif; max-width: 800px; margin: 50px auto; padding: 20px; }}
        h1 {{ border-bottom: 2px solid #333; }}
        .doc-section {{ margin: 30px 0; padding: 20px; background: #f5f5f5; border-radius: 5px; }}
        .doc-section h2 {{ margin-top: 0; }}
        a {{ color: #0066cc; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <h1>Impulse 7.1 BBS Documentation (v{})</h1>

    <div class="doc-section">
        <h2>User Manual</h2>
        <p>Complete guide for BBS users covering login, messaging, file transfers, and door games.</p>
        <a href="user-manual/index.html">Read User Manual →</a>
    </div>

    <div class="doc-section">
        <h2>SysOp Guide</h2>
        <p>Installation, configuration, customization, and maintenance guide for system operators.</p>
        <a href="sysop-guide/index.html">Read SysOp Guide →</a>
    </div>

    <div class="doc-section">
        <h2>Developer Documentation</h2>
        <p>Architecture overview, contributing guidelines, and extension development.</p>
        <a href="developer/index.html">Read Developer Docs →</a>
    </div>

    <div class="doc-section">
        <h2>API Reference</h2>
        <p>Complete Rust API documentation generated from source code.</p>
        <a href="api/impulse_bbs/index.html">Browse API Docs →</a>
    </div>
</body>
</html>
"#,
            self.version
        );

        tokio::fs::write(self.output_dir.join("index.html"), index_html).await
            .context("Failed to write index.html")?;

        Ok(())
    }

    async fn validate_links(&self) -> Result<()> {
        // Run mdbook-linkcheck on all books
        let books = vec!["user-manual", "sysop-guide", "developer"];

        for book in books {
            let book_dir = self.project_root.join("docs").join(book);
            let status = Command::new("mdbook-linkcheck")
                .arg(&book_dir)
                .status()
                .context(format!("Failed to validate links in {}", book))?;

            if !status.success() {
                anyhow::bail!("Link validation failed for {}", book);
            }
        }

        println!("All links validated successfully");
        Ok(())
    }

    /// Deploy documentation to GitHub Pages
    pub async fn deploy_to_github_pages(&self, branch: &str) -> Result<()> {
        println!("Deploying documentation to GitHub Pages (branch: {})", branch);

        // Build all docs first
        self.build_all().await?;

        // Copy to gh-pages branch (assuming worktree setup)
        let status = Command::new("git")
            .arg("worktree")
            .arg("add")
            .arg("--force")
            .arg("/tmp/gh-pages")
            .arg(branch)
            .current_dir(&self.project_root)
            .status()
            .context("Failed to create worktree for gh-pages")?;

        if !status.success() {
            anyhow::bail!("Failed to create gh-pages worktree");
        }

        // Copy built docs
        let status = Command::new("rsync")
            .arg("-av")
            .arg("--delete")
            .arg(format!("{}/", self.output_dir.display()))
            .arg("/tmp/gh-pages/")
            .status()
            .context("Failed to copy docs to gh-pages")?;

        if !status.success() {
            anyhow::bail!("Failed to copy docs");
        }

        // Commit and push
        Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir("/tmp/gh-pages")
            .status()?;

        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(format!("Update documentation for v{}", self.version))
            .current_dir("/tmp/gh-pages")
            .status()?;

        Command::new("git")
            .arg("push")
            .arg("origin")
            .arg(branch)
            .current_dir("/tmp/gh-pages")
            .status()?;

        println!("Documentation deployed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_documentation_builds() {
        let builder = DocumentationBuilder::new(
            PathBuf::from("../.."),
            "7.1.0".to_string(),
        );

        builder.build_all().await.expect("Documentation should build");
    }
}
```

**Comprehensive Rustdoc Examples:**
```rust
//! # Impulse 7.1 BBS - Message System
//!
//! This module provides a complete message board system with support for
//! public/private messages, attachments, threading, and QWK offline reader packets.
//!
//! ## Quick Start
//!
//! ```rust
//! use impulse_bbs::messaging::{MessageSystem, MessageArea, Message};
//! use sqlx::PgPool;
//!
//! # async fn example(db: PgPool) -> anyhow::Result<()> {
//! let msg_system = MessageSystem::new(db.clone());
//!
//! // Create a message area
//! let area = msg_system.create_area(
//!     "General Discussion",
//!     "Public discussion area",
//!     1, // min security level
//! ).await?;
//!
//! // Post a message
//! let msg = msg_system.post_message(
//!     area.id,
//!     user_id,
//!     "Hello, BBS!",
//!     "This is my first message.",
//!     None, // no reply_to
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **Public Message Areas**: Threaded discussions visible to all users
//! - **Private Messages**: Direct user-to-user communication
//! - **Attachments**: File attachments up to 10MB per message
//! - **QWK Support**: Offline reading with QWK packet generation
//! - **Search**: Full-text search across message content
//! - **Moderation**: Message editing, deletion, and user banning
//!
//! ## Architecture
//!
//! The message system is built on PostgreSQL with full-text search indices.
//! Messages are stored with efficient threading via `reply_to_id` relationships.
//!
//! ## Examples
//!
//! ### Listing Messages in an Area
//!
//! ```rust
//! # use impulse_bbs::messaging::MessageSystem;
//! # use sqlx::PgPool;
//! # async fn example(db: PgPool) -> anyhow::Result<()> {
//! let msg_system = MessageSystem::new(db);
//!
//! let messages = msg_system.list_messages(
//!     area_id,
//!     0,   // offset
//!     25,  // limit
//! ).await?;
//!
//! for msg in messages {
//!     println!("{}: {}", msg.subject, msg.from_user_name);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Sending Private Message
//!
//! ```rust
//! # use impulse_bbs::messaging::MessageSystem;
//! # use sqlx::PgPool;
//! # async fn example(db: PgPool) -> anyhow::Result<()> {
//! let msg_system = MessageSystem::new(db);
//!
//! msg_system.send_private_message(
//!     from_user_id,
//!     to_user_id,
//!     "Meeting Tomorrow",
//!     "Don't forget about the sysop meeting at 8pm!",
//! ).await?;
//! # Ok(())
//! # }
//! ```

use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Main message system interface
///
/// Provides all message-related operations including posting, reading,
/// searching, and QWK packet generation.
///
/// # Example
///
/// ```rust
/// use impulse_bbs::messaging::MessageSystem;
/// use sqlx::PgPool;
///
/// # async fn example(db: PgPool) -> anyhow::Result<()> {
/// let msg_system = MessageSystem::new(db);
/// let areas = msg_system.list_areas().await?;
/// # Ok(())
/// # }
/// ```
pub struct MessageSystem {
    db: PgPool,
}

impl MessageSystem {
    /// Create a new MessageSystem instance
    ///
    /// # Arguments
    ///
    /// * `db` - PostgreSQL connection pool
    ///
    /// # Example
    ///
    /// ```rust
    /// # use impulse_bbs::messaging::MessageSystem;
    /// # use sqlx::PgPool;
    /// # async fn example(db: PgPool) {
    /// let msg_system = MessageSystem::new(db);
    /// # }
    /// ```
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Create a new message area
    ///
    /// Message areas are discussion forums visible to users with sufficient
    /// security level.
    ///
    /// # Arguments
    ///
    /// * `name` - Display name for the area
    /// * `description` - Brief description of the area's purpose
    /// * `min_security_level` - Minimum security level required to access (0-255)
    ///
    /// # Returns
    ///
    /// The newly created `MessageArea`
    ///
    /// # Errors
    ///
    /// Returns error if database operation fails or area name is duplicate
    ///
    /// # Example
    ///
    /// ```rust
    /// # use impulse_bbs::messaging::MessageSystem;
    /// # use sqlx::PgPool;
    /// # async fn example(db: PgPool) -> anyhow::Result<()> {
    /// let msg_system = MessageSystem::new(db);
    ///
    /// let area = msg_system.create_area(
    ///     "Tech Support",
    ///     "Get help with technical issues",
    ///     10, // security level 10+
    /// ).await?;
    ///
    /// println!("Created area #{}: {}", area.id, area.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_area(
        &self,
        name: &str,
        description: &str,
        min_security_level: u8,
    ) -> anyhow::Result<MessageArea> {
        let area = sqlx::query_as!(
            MessageArea,
            "INSERT INTO message_areas (name, description, min_security_level)
             VALUES ($1, $2, $3)
             RETURNING id, name, description, min_security_level, created_at, updated_at",
            name,
            description,
            min_security_level as i16
        )
        .fetch_one(&self.db)
        .await?;

        Ok(area)
    }

    /// Post a message to an area
    ///
    /// Creates a new message in the specified message area. Messages can be
    /// threaded by providing a `reply_to_id`.
    ///
    /// # Arguments
    ///
    /// * `area_id` - Target message area ID
    /// * `user_id` - ID of user posting the message
    /// * `subject` - Message subject (max 72 characters)
    /// * `body` - Message body (max 32KB)
    /// * `reply_to_id` - Optional ID of message being replied to
    ///
    /// # Returns
    ///
    /// The newly created `Message`
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - User lacks access to area
    /// - Subject/body exceeds length limits
    /// - Database operation fails
    ///
    /// # Example
    ///
    /// ```rust
    /// # use impulse_bbs::messaging::MessageSystem;
    /// # use sqlx::PgPool;
    /// # async fn example(db: PgPool, area_id: i32, user_id: i32) -> anyhow::Result<()> {
    /// let msg_system = MessageSystem::new(db);
    ///
    /// let msg = msg_system.post_message(
    ///     area_id,
    ///     user_id,
    ///     "Feature Request",
    ///     "It would be great to have support for ANSI music!",
    ///     None,
    /// ).await?;
    ///
    /// println!("Message posted with ID #{}", msg.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_message(
        &self,
        area_id: i32,
        user_id: i32,
        subject: &str,
        body: &str,
        reply_to_id: Option<i32>,
    ) -> anyhow::Result<Message> {
        // Validate subject and body lengths
        if subject.len() > 72 {
            anyhow::bail!("Subject must be 72 characters or less");
        }
        if body.len() > 32768 {
            anyhow::bail!("Message body must be 32KB or less");
        }

        let msg = sqlx::query_as!(
            Message,
            "INSERT INTO messages (area_id, user_id, subject, body, reply_to_id)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, area_id, user_id, subject, body, reply_to_id, posted_at, edited_at, is_deleted",
            area_id,
            user_id,
            subject,
            body,
            reply_to_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(msg)
    }

    /// List messages in an area with pagination
    ///
    /// Returns messages ordered by post date (newest first).
    ///
    /// # Arguments
    ///
    /// * `area_id` - Message area to list from
    /// * `offset` - Number of messages to skip
    /// * `limit` - Maximum messages to return (max 100)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use impulse_bbs::messaging::MessageSystem;
    /// # use sqlx::PgPool;
    /// # async fn example(db: PgPool, area_id: i32) -> anyhow::Result<()> {
    /// let msg_system = MessageSystem::new(db);
    ///
    /// // Get first page (messages 0-24)
    /// let page1 = msg_system.list_messages(area_id, 0, 25).await?;
    ///
    /// // Get second page (messages 25-49)
    /// let page2 = msg_system.list_messages(area_id, 25, 25).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_messages(
        &self,
        area_id: i32,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Vec<Message>> {
        let limit = limit.min(100); // Cap at 100 messages per query

        let messages = sqlx::query_as!(
            Message,
            "SELECT id, area_id, user_id, subject, body, reply_to_id, posted_at, edited_at, is_deleted
             FROM messages
             WHERE area_id = $1 AND is_deleted = false
             ORDER BY posted_at DESC
             LIMIT $2 OFFSET $3",
            area_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.db)
        .await?;

        Ok(messages)
    }
}

/// Message area (discussion forum)
///
/// # Example
///
/// ```rust
/// # use impulse_bbs::messaging::MessageArea;
/// # use chrono::Utc;
/// let area = MessageArea {
///     id: 1,
///     name: "General".to_string(),
///     description: "General discussion".to_string(),
///     min_security_level: 1,
///     created_at: Utc::now(),
///     updated_at: Utc::now(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArea {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub min_security_level: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Individual message
///
/// # Example
///
/// ```rust
/// # use impulse_bbs::messaging::Message;
/// # use chrono::Utc;
/// let msg = Message {
///     id: 42,
///     area_id: 1,
///     user_id: 5,
///     subject: "Welcome!".to_string(),
///     body: "Thanks for joining the BBS!".to_string(),
///     reply_to_id: None,
///     posted_at: Utc::now(),
///     edited_at: None,
///     is_deleted: false,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub area_id: i32,
    pub user_id: i32,
    pub subject: String,
    pub body: String,
    pub reply_to_id: Option<i32>,
    pub posted_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}
```

**mdBook Configuration:**
```rust
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

/// Configuration for mdBook-based documentation
///
/// Sets up the structure for user manual, SysOp guide, and developer docs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdBookConfig {
    pub book: BookConfig,
    pub build: BuildConfig,
    pub preprocessor: PreprocessorConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookConfig {
    pub title: String,
    pub authors: Vec<String>,
    pub description: String,
    pub language: String,
    pub multilingual: bool,
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub build_dir: String,
    pub create_missing: bool,
    pub use_default_preprocessors: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessorConfig {
    pub links: LinkCheckConfig,
    pub toc: TocConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCheckConfig {
    pub follow_web_links: bool,
    pub warning_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocConfig {
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub html: HtmlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlConfig {
    pub additional_css: Vec<String>,
    pub additional_js: Vec<String>,
    pub no_section_label: bool,
    pub git_repository_url: String,
    pub git_repository_icon: String,
}

impl MdBookConfig {
    /// Create config for user manual
    pub fn user_manual() -> Self {
        Self {
            book: BookConfig {
                title: "Impulse 7.1 BBS User Manual".to_string(),
                authors: vec!["Impulse BBS Project".to_string()],
                description: "Complete user guide for Impulse 7.1 BBS".to_string(),
                language: "en".to_string(),
                multilingual: false,
                src: "src".to_string(),
            },
            build: BuildConfig {
                build_dir: "book".to_string(),
                create_missing: true,
                use_default_preprocessors: true,
            },
            preprocessor: PreprocessorConfig {
                links: LinkCheckConfig {
                    follow_web_links: true,
                    warning_policy: "warn".to_string(),
                },
                toc: TocConfig {
                    marker: "<!-- toc -->".to_string(),
                },
            },
            output: OutputConfig {
                html: HtmlConfig {
                    additional_css: vec!["theme/custom.css".to_string()],
                    additional_js: vec![],
                    no_section_label: false,
                    git_repository_url: "https://github.com/impulse-bbs/impulse-7.1".to_string(),
                    git_repository_icon: "fa-github".to_string(),
                },
            },
        }
    }

    /// Create config for SysOp guide
    pub fn sysop_guide() -> Self {
        Self {
            book: BookConfig {
                title: "Impulse 7.1 BBS SysOp Guide".to_string(),
                authors: vec!["Impulse BBS Project".to_string()],
                description: "Installation, configuration, and maintenance guide for system operators".to_string(),
                language: "en".to_string(),
                multilingual: false,
                src: "src".to_string(),
            },
            build: BuildConfig {
                build_dir: "book".to_string(),
                create_missing: true,
                use_default_preprocessors: true,
            },
            preprocessor: PreprocessorConfig {
                links: LinkCheckConfig {
                    follow_web_links: true,
                    warning_policy: "error".to_string(),  // Stricter for SysOp guide
                },
                toc: TocConfig {
                    marker: "<!-- toc -->".to_string(),
                },
            },
            output: OutputConfig {
                html: HtmlConfig {
                    additional_css: vec!["theme/sysop.css".to_string()],
                    additional_js: vec!["theme/config-validator.js".to_string()],
                    no_section_label: false,
                    git_repository_url: "https://github.com/impulse-bbs/impulse-7.1".to_string(),
                    git_repository_icon: "fa-github".to_string(),
                },
            },
        }
    }

    /// Write config to TOML file
    pub fn write_to_file(&self, path: &PathBuf) -> Result<()> {
        let toml = toml::to_string_pretty(self)
            .context("Failed to serialize mdBook config")?;

        fs::write(path, toml)
            .context(format!("Failed to write config to {}", path.display()))?;

        Ok(())
    }
}

/// Initialize documentation structure
pub struct DocumentationStructure {
    project_root: PathBuf,
}

impl DocumentationStructure {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    /// Initialize all documentation directories and configs
    pub async fn initialize(&self) -> Result<()> {
        let docs_dir = self.project_root.join("docs");

        // Create user manual structure
        self.init_user_manual(&docs_dir).await?;

        // Create SysOp guide structure
        self.init_sysop_guide(&docs_dir).await?;

        // Create developer docs structure
        self.init_developer_docs(&docs_dir).await?;

        println!("Documentation structure initialized in {}", docs_dir.display());
        Ok(())
    }

    async fn init_user_manual(&self, docs_dir: &PathBuf) -> Result<()> {
        let manual_dir = docs_dir.join("user-manual");
        let src_dir = manual_dir.join("src");

        tokio::fs::create_dir_all(&src_dir).await?;

        // Write book.toml
        let config = MdBookConfig::user_manual();
        config.write_to_file(&manual_dir.join("book.toml"))?;

        // Create SUMMARY.md
        let summary = r#"# Summary

[Introduction](./introduction.md)

# Getting Started

- [First Login](./01-getting-started.md)
- [Navigating the BBS](./02-navigation.md)
- [User Profile](./03-profile.md)

# Features

- [Messaging](./04-messaging.md)
  - [Reading Messages](./04-messaging.md#reading)
  - [Posting Messages](./04-messaging.md#posting)
  - [Private Messages](./04-messaging.md#private)
- [File Transfers](./05-files.md)
  - [Downloading Files](./05-files.md#downloading)
  - [Uploading Files](./05-files.md#uploading)
  - [File Protocols](./05-files.md#protocols)
- [Door Games](./06-doors.md)
- [Chat & Messaging](./07-chat.md)

# Reference

- [Keyboard Commands](./08-commands.md)
- [ANSI Graphics](./09-ansi.md)
- [FAQ](./99-faq.md)
"#;
        tokio::fs::write(src_dir.join("SUMMARY.md"), summary).await?;

        Ok(())
    }

    async fn init_sysop_guide(&self, docs_dir: &PathBuf) -> Result<()> {
        let guide_dir = docs_dir.join("sysop-guide");
        let src_dir = guide_dir.join("src");

        tokio::fs::create_dir_all(&src_dir).await?;

        // Write book.toml
        let config = MdBookConfig::sysop_guide();
        config.write_to_file(&guide_dir.join("book.toml"))?;

        // Create SUMMARY.md
        let summary = r#"# Summary

[Introduction](./introduction.md)

# Installation

- [System Requirements](./01-installation.md#requirements)
- [Installation Steps](./01-installation.md#steps)
- [First-Time Setup](./01-installation.md#setup)

# Configuration

- [Main Config File](./02-configuration.md)
- [Database Setup](./02-configuration.md#database)
- [Network Settings](./02-configuration.md#network)
- [Security Settings](./02-configuration.md#security)

# Customization

- [Menu Customization](./03-customization.md#menus)
- [ANSI Themes](./03-customization.md#themes)
- [Text Files](./03-customization.md#text)
- [Door Games](./03-customization.md#doors)

# Administration

- [User Management](./04-admin.md#users)
- [File Area Management](./04-admin.md#files)
- [Message Area Management](./04-admin.md#messages)
- [Moderation](./04-admin.md#moderation)

# Maintenance

- [Backups](./05-maintenance.md#backups)
- [Log Files](./05-maintenance.md#logs)
- [Database Maintenance](./05-maintenance.md#database)
- [Performance Tuning](./05-maintenance.md#performance)

# Troubleshooting

- [Common Issues](./06-troubleshooting.md#common)
- [Connection Problems](./06-troubleshooting.md#connection)
- [Database Issues](./06-troubleshooting.md#database)
- [Performance Issues](./06-troubleshooting.md#performance)
"#;
        tokio::fs::write(src_dir.join("SUMMARY.md"), summary).await?;

        Ok(())
    }

    async fn init_developer_docs(&self, docs_dir: &PathBuf) -> Result<()> {
        let dev_dir = docs_dir.join("developer");
        let src_dir = dev_dir.join("src");

        tokio::fs::create_dir_all(&src_dir).await?;

        // Write book.toml
        let config = MdBookConfig {
            book: BookConfig {
                title: "Impulse 7.1 BBS Developer Documentation".to_string(),
                authors: vec!["Impulse BBS Project".to_string()],
                description: "Architecture and development guide".to_string(),
                language: "en".to_string(),
                multilingual: false,
                src: "src".to_string(),
            },
            ..MdBookConfig::sysop_guide()
        };
        config.write_to_file(&dev_dir.join("book.toml"))?;

        // Create SUMMARY.md
        let summary = r#"# Summary

[Introduction](./introduction.md)

# Architecture

- [System Overview](./architecture.md)
- [Database Schema](./database.md)
- [Module Organization](./modules.md)

# Development

- [Contributing Guidelines](./contributing.md)
- [Code Style](./code-style.md)
- [Testing](./testing.md)

# Extension Development

- [Plugin System](./extensions.md#plugins)
- [Door Game Integration](./extensions.md#doors)
- [Custom Protocols](./extensions.md#protocols)

# API Reference

- [Core APIs](./api-core.md)
- [Extension APIs](./api-extensions.md)
"#;
        tokio::fs::write(src_dir.join("SUMMARY.md"), summary).await?;

        Ok(())
    }
}
```

**Documentation Testing and Validation:**
```rust
use std::path::{Path, PathBuf};
use pulldown_cmark::{Parser, Event};
use anyhow::{Context, Result};

/// Documentation validator
///
/// Ensures documentation quality through:
/// - Link checking (internal and external)
/// - Code snippet validation (Rust examples compile)
/// - Spelling and grammar checks
/// - Consistency validation
pub struct DocumentationValidator {
    docs_dir: PathBuf,
    errors: Vec<ValidationError>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub file: PathBuf,
    pub line: Option<usize>,
    pub kind: ValidationErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationErrorKind {
    BrokenLink,
    InvalidCodeSnippet,
    SpellingError,
    InconsistentTerminology,
}

impl DocumentationValidator {
    pub fn new(docs_dir: PathBuf) -> Self {
        Self {
            docs_dir,
            errors: Vec::new(),
        }
    }

    /// Validate all documentation
    pub async fn validate_all(&mut self) -> Result<ValidationReport> {
        println!("Validating documentation in {}", self.docs_dir.display());

        // Validate all Markdown files
        self.validate_markdown_files().await?;

        // Validate code snippets compile
        self.validate_code_snippets().await?;

        // Check for broken links
        self.validate_links().await?;

        // Check terminology consistency
        self.validate_terminology().await?;

        Ok(ValidationReport {
            total_files: self.count_files().await?,
            total_errors: self.errors.len(),
            errors: self.errors.clone(),
        })
    }

    async fn validate_markdown_files(&mut self) -> Result<()> {
        let markdown_files = self.find_markdown_files().await?;

        for file in markdown_files {
            if let Err(e) = self.validate_single_markdown(&file).await {
                self.errors.push(ValidationError {
                    file: file.clone(),
                    line: None,
                    kind: ValidationErrorKind::BrokenLink,
                    message: format!("Failed to validate: {}", e),
                });
            }
        }

        Ok(())
    }

    async fn validate_single_markdown(&self, file: &PathBuf) -> Result<()> {
        let content = tokio::fs::read_to_string(file).await
            .context(format!("Failed to read {}", file.display()))?;

        // Parse markdown
        let parser = Parser::new(&content);

        for event in parser {
            match event {
                Event::Start(tag) => {
                    // Validate tags
                }
                Event::Code(code) => {
                    // Validate inline code
                }
                Event::Html(html) => {
                    // Validate HTML blocks
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn validate_code_snippets(&mut self) -> Result<()> {
        let markdown_files = self.find_markdown_files().await?;

        for file in markdown_files {
            let content = tokio::fs::read_to_string(&file).await?;
            let snippets = self.extract_rust_snippets(&content);

            for (line_num, snippet) in snippets {
                if let Err(e) = self.validate_rust_snippet(&snippet).await {
                    self.errors.push(ValidationError {
                        file: file.clone(),
                        line: Some(line_num),
                        kind: ValidationErrorKind::InvalidCodeSnippet,
                        message: format!("Code snippet compilation failed: {}", e),
                    });
                }
            }
        }

        Ok(())
    }

    fn extract_rust_snippets(&self, content: &str) -> Vec<(usize, String)> {
        let mut snippets = Vec::new();
        let mut in_rust_block = false;
        let mut current_snippet = String::new();
        let mut snippet_start_line = 0;

        for (line_num, line) in content.lines().enumerate() {
            if line.trim().starts_with("```rust") {
                in_rust_block = true;
                snippet_start_line = line_num;
                current_snippet.clear();
            } else if line.trim() == "```" && in_rust_block {
                in_rust_block = false;
                if !current_snippet.trim().is_empty() {
                    snippets.push((snippet_start_line, current_snippet.clone()));
                }
            } else if in_rust_block {
                current_snippet.push_str(line);
                current_snippet.push('\n');
            }
        }

        snippets
    }

    async fn validate_rust_snippet(&self, snippet: &str) -> Result<()> {
        // Skip snippets marked as no_run or ignore
        if snippet.contains("# no_run") || snippet.contains("# ignore") {
            return Ok(());
        }

        // Create temporary file
        let temp_dir = tempfile::tempdir()?;
        let temp_file = temp_dir.path().join("snippet.rs");

        // Wrap snippet in a main function if not already present
        let wrapped = if snippet.contains("fn main") {
            snippet.to_string()
        } else {
            format!(
                "use impulse_bbs::*;\n\nfn main() {{\n{}\n}}",
                snippet
            )
        };

        tokio::fs::write(&temp_file, wrapped).await?;

        // Try to compile with rustc
        let output = tokio::process::Command::new("rustc")
            .arg("--crate-type")
            .arg("bin")
            .arg("--edition")
            .arg("2021")
            .arg(&temp_file)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Compilation failed:\n{}", stderr);
        }

        Ok(())
    }

    async fn validate_links(&mut self) -> Result<()> {
        // Use mdbook-linkcheck or similar
        let output = tokio::process::Command::new("mdbook-linkcheck")
            .arg(&self.docs_dir)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            self.errors.push(ValidationError {
                file: self.docs_dir.clone(),
                line: None,
                kind: ValidationErrorKind::BrokenLink,
                message: format!("Link validation failed:\n{}", stderr),
            });
        }

        Ok(())
    }

    async fn validate_terminology(&mut self) -> Result<()> {
        // Check for consistent use of terms
        let terminology = vec![
            ("BBS", vec!["bbs", "bulletin board"]),
            ("SysOp", vec!["sysop", "system operator", "admin"]),
            ("ANSI", vec!["ansi", "Ansi"]),
        ];

        let markdown_files = self.find_markdown_files().await?;

        for file in markdown_files {
            let content = tokio::fs::read_to_string(&file).await?;

            for (preferred, alternatives) in &terminology {
                for alt in alternatives {
                    if content.contains(alt) && alt != preferred {
                        self.errors.push(ValidationError {
                            file: file.clone(),
                            line: None,
                            kind: ValidationErrorKind::InconsistentTerminology,
                            message: format!(
                                "Use '{}' instead of '{}'",
                                preferred, alt
                            ),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    async fn find_markdown_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        let mut entries = tokio::fs::read_dir(&self.docs_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_file() && path.extension() == Some("md".as_ref()) {
                files.push(path);
            } else if path.is_dir() {
                // Recursively search subdirectories
                let subfiles = self.find_markdown_files_in(&path).await?;
                files.extend(subfiles);
            }
        }

        Ok(files)
    }

    async fn find_markdown_files_in(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        let mut entries = tokio::fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_file() && path.extension() == Some("md".as_ref()) {
                files.push(path);
            } else if path.is_dir() {
                let subfiles = self.find_markdown_files_in(&path).await?;
                files.extend(subfiles);
            }
        }

        Ok(files)
    }

    async fn count_files(&self) -> Result<usize> {
        Ok(self.find_markdown_files().await?.len())
    }
}

#[derive(Debug)]
pub struct ValidationReport {
    pub total_files: usize,
    pub total_errors: usize,
    pub errors: Vec<ValidationError>,
}

impl ValidationReport {
    pub fn print_summary(&self) {
        println!("\n=== Documentation Validation Report ===");
        println!("Total files validated: {}", self.total_files);
        println!("Total errors found: {}", self.total_errors);

        if self.total_errors > 0 {
            println!("\nErrors:");
            for error in &self.errors {
                println!(
                    "  [{:?}] {} (line {}): {}",
                    error.kind,
                    error.file.display(),
                    error.line.map(|l| l.to_string()).unwrap_or_else(|| "?".to_string()),
                    error.message
                );
            }
        } else {
            println!("\nAll documentation is valid!");
        }
    }

    pub fn is_valid(&self) -> bool {
        self.total_errors == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_documentation_validation() {
        let mut validator = DocumentationValidator::new(
            PathBuf::from("../../docs")
        );

        let report = validator.validate_all().await.expect("Validation should complete");
        report.print_summary();

        assert!(report.is_valid(), "Documentation should be valid");
    }

    #[test]
    fn test_rust_snippet_extraction() {
        let content = r#"
# Example

```rust
fn main() {
    println!("Hello");
}
```

Some text

```rust
let x = 5;
```
"#;

        let validator = DocumentationValidator::new(PathBuf::from("."));
        let snippets = validator.extract_rust_snippets(content);

        assert_eq!(snippets.len(), 2);
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 25**: Performance optimization ensures docs build quickly
- **Sprint 05**: Terminal I/O for ANSI examples
- **Sprint 06**: User system for user guide examples
- **All previous sprints**: Complete feature set to document

### Blocks Downstream
- **Sprint 28**: Migration tools reference documentation
- **Sprint 29**: Web admin interface references SysOp guide
- **Sprint 30**: Beta testing uses documentation for user onboarding
- **Sprint 32**: Launch requires complete documentation

---

## Testing Requirements

### Documentation Build Tests
- [ ] User manual builds successfully with mdBook
- [ ] SysOp guide builds successfully with mdBook
- [ ] Developer docs build successfully with mdBook
- [ ] API documentation generates with rustdoc
- [ ] All internal links valid
- [ ] All external links valid (or marked as optional)
- [ ] Combined documentation index page renders correctly

### Content Quality Tests
- [ ] All Rust code examples compile
- [ ] All code examples include proper error handling
- [ ] Step-by-step tutorials can be followed successfully
- [ ] Screenshots accurate and up-to-date
- [ ] Configuration examples valid (TOML syntax)
- [ ] All public APIs have comprehensive rustdoc comments

### Documentation Validation Tests
- [ ] Terminology consistent across all docs
- [ ] No spelling errors (automated check)
- [ ] Proper grammar (automated check)
- [ ] Code snippets follow project style guidelines
- [ ] Cross-references between docs work correctly

### Platform Tests
- [ ] Documentation builds on Linux
- [ ] Documentation builds on macOS
- [ ] Documentation builds on Windows
- [ ] Mobile-responsive HTML output
- [ ] PDF generation works (optional)

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use mdBook for all prose documentation (user, SysOp, developer guides)
- Generate API docs with rustdoc (standard Rust documentation)
- Host on GitHub Pages for easy access
- Maintain versioned documentation (one version per release)
- Include searchable index across all documentation
- Provide offline documentation bundles (HTML + PDF)
- Video tutorials optional (depend on community feedback)
- Document configuration with TOML examples and validation

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Documentation quickly becomes outdated
- **Mitigation**: Integrate doc builds into CI/CD; fail builds if rustdoc incomplete; automated link checking
- **Risk**: Code examples may not compile
- **Mitigation**: Use doctest to validate all Rust examples; include examples in integration tests
- **Risk**: Screenshots become outdated with UI changes
- **Mitigation**: Use text descriptions where possible; automate screenshot generation; version screenshots
- **Risk**: Documentation too technical for end users
- **Mitigation**: Separate user manual (beginner-friendly) from SysOp/developer docs; user testing during beta
- **Risk**: Documentation not discoverable
- **Mitigation**: Clear index page; search functionality; cross-linking between related topics

---

## Progress Log

### Week 1
- *Date*: Progress notes will be added here as sprint progresses

### Week 2
- *Date*: Progress notes will be added here as sprint progresses

### Week 3
- *Date*: Progress notes will be added here as sprint progresses

### Week 4
- *Date*: Progress notes will be added here as sprint progresses

### Week 5
- *Date*: Progress notes will be added here as sprint progresses

### Week 6
- *Date*: Progress notes will be added here as sprint progresses

### Sprint Completion
- **Completed**: TBD
- **Velocity**: TBD
- **Burndown**: TBD
