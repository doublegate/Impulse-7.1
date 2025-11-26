# Sprint 15: User Profile & Statistics

**Phase:** Phase 2 - Core Features
**Duration:** 3 weeks
**Sprint Dates:** 2025-11-25
**Status:** COMPLETE

---

## Sprint Overview

Sprint 15 implements user profile displays, statistics tracking, and user settings configuration. This sprint enhances the user experience by providing visibility into account information and customization options.

**Context:** This is the seventh sprint of Phase 2 (Core Features). User profiles provide transparency and customization for the BBS experience.

**Expected Outcomes:** By the end of this sprint, users will have comprehensive profile pages showing their statistics and the ability to configure their preferences.

---

## Objectives

- [ ] Implement user profile display screens
- [ ] Track and display user statistics
- [ ] Add user preference settings editor
- [ ] Support theme and terminal customization

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| User profile screen | UI | Display comprehensive user information |
| Statistics tracking system | Code | Track calls, uploads, downloads, posts |
| Settings editor | UI | Allow users to modify preferences |
| Preference persistence | Code | Save and load user settings |

---

## Detailed Tasks

### Task Category 1: Profile Display

- [ ] **Task 1.1**: Design profile ANSI screen
  - Implementation notes: User info, stats summary, badges/achievements
  - Files affected: `assets/screens/profile.ans`, `crates/impulse-user/src/screens/profile.rs`
  - Estimated hours: 5

- [ ] **Task 1.2**: Display user information
  - Implementation notes: Name, location, email, member since, last login
  - Files affected: `crates/impulse-user/src/screens/profile.rs`
  - Estimated hours: 3

- [ ] **Task 1.3**: Show upload/download ratios
  - Implementation notes: Calculate ratio, display formatted (e.g., "1:3.5")
  - Files affected: `crates/impulse-user/src/screens/profile.rs`
  - Estimated hours: 2

- [ ] **Task 1.4**: Display user signature
  - Implementation notes: Custom user tagline, ANSI art signature
  - Files affected: `crates/impulse-user/src/screens/profile.rs`
  - Estimated hours: 2

- [ ] **Task 1.5**: Show user badges/achievements
  - Implementation notes: Top poster, upload king, loyal member, etc.
  - Files affected: `crates/impulse-user/src/badges.rs`
  - Estimated hours: 4

### Task Category 2: Statistics Tracking

- [ ] **Task 2.1**: Add statistics fields to User schema
  - Implementation notes: total_calls, total_uploads, total_downloads, total_posts, time_online
  - Files affected: `crates/impulse-storage/migrations/005_user_stats.sql`
  - Estimated hours: 2

- [ ] **Task 2.2**: Implement call tracking
  - Implementation notes: Increment on login, track last login date
  - Files affected: `crates/impulse-user/src/stats/calls.rs`
  - Estimated hours: 2

- [ ] **Task 2.3**: Implement upload/download tracking
  - Implementation notes: Increment counters, track bytes transferred
  - Files affected: `crates/impulse-user/src/stats/transfers.rs`
  - Estimated hours: 3

- [ ] **Task 2.4**: Implement message post tracking
  - Implementation notes: Increment on message post
  - Files affected: `crates/impulse-user/src/stats/messages.rs`
  - Estimated hours: 2

- [ ] **Task 2.5**: Track time online
  - Implementation notes: Update session duration on logout
  - Files affected: `crates/impulse-user/src/stats/time.rs`
  - Estimated hours: 3

- [ ] **Task 2.6**: Display statistics graphs
  - Implementation notes: Simple ASCII bar charts for activity over time
  - Files affected: `crates/impulse-user/src/screens/stats_graphs.rs`
  - Estimated hours: 5

### Task Category 3: User Settings

- [ ] **Task 3.1**: Add preferences fields to User schema
  - Implementation notes: theme, terminal_width, terminal_height, hotkeys_enabled, color_enabled
  - Files affected: `crates/impulse-storage/migrations/005_user_stats.sql`
  - Estimated hours: 2

- [ ] **Task 3.2**: Design settings editor ANSI screen
  - Implementation notes: Menu of settings, current values, prompts
  - Files affected: `assets/screens/settings.ans`, `crates/impulse-user/src/screens/settings.rs`
  - Estimated hours: 5

- [ ] **Task 3.3**: Implement password change
  - Implementation notes: Verify old password, validate new password, update hash
  - Files affected: `crates/impulse-user/src/settings/password.rs`
  - Estimated hours: 4

- [ ] **Task 3.4**: Theme preference setting
  - Implementation notes: List available themes, preview, save selection
  - Files affected: `crates/impulse-user/src/settings/theme.rs`
  - Estimated hours: 3

- [ ] **Task 3.5**: Terminal settings
  - Implementation notes: Set width, height, color support
  - Files affected: `crates/impulse-user/src/settings/terminal.rs`
  - Estimated hours: 3

- [ ] **Task 3.6**: Hotkeys vs full menu toggle
  - Implementation notes: Switch between single-key and command entry modes
  - Files affected: `crates/impulse-user/src/settings/ui_mode.rs`
  - Estimated hours: 2

- [ ] **Task 3.7**: Edit user signature
  - Implementation notes: Multi-line editor for custom signature
  - Files affected: `crates/impulse-user/src/settings/signature.rs`
  - Estimated hours: 4

### Task Category 4: Other Users' Profiles

- [ ] **Task 4.1**: Implement "View User Profile" command
  - Implementation notes: Search for user, display their public profile
  - Files affected: `crates/impulse-user/src/screens/view_profile.rs`
  - Estimated hours: 3

- [ ] **Task 4.2**: Privacy settings
  - Implementation notes: Hide email, hide stats, hide online status
  - Files affected: `crates/impulse-user/src/privacy.rs`
  - Estimated hours: 4

- [ ] **Task 4.3**: User directory
  - Implementation notes: List all users (paginated), search by name
  - Files affected: `crates/impulse-user/src/screens/user_directory.rs`
  - Estimated hours: 4

### Task Category 5: Achievements System (Optional)

- [ ] **Task 5.1**: Define achievement types
  - Implementation notes: First post, 100 uploads, 1000 calls, loyal member (1 year), etc.
  - Files affected: `crates/impulse-user/src/achievements/types.rs`
  - Estimated hours: 3

- [ ] **Task 5.2**: Implement achievement checking
  - Implementation notes: Check conditions on stat updates, award achievements
  - Files affected: `crates/impulse-user/src/achievements/checker.rs`
  - Estimated hours: 4

- [ ] **Task 5.3**: Store achievements in database
  - Implementation notes: user_achievements table with user_id, achievement_id, awarded_date
  - Files affected: `crates/impulse-storage/migrations/006_achievements.sql`
  - Estimated hours: 2

- [ ] **Task 5.4**: Display achievement notifications
  - Implementation notes: "Achievement unlocked!" message on login
  - Files affected: `crates/impulse-user/src/achievements/notify.rs`
  - Estimated hours: 3

### Task Category 6: Testing

- [ ] **Task 6.1**: Test statistics tracking
  - Implementation notes: Verify counters increment correctly
  - Files affected: `tests/stats_tracking_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.2**: Test settings persistence
  - Implementation notes: Change settings, log out/in, verify persistence
  - Files affected: `tests/settings_persistence_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.3**: Test ratio calculations
  - Implementation notes: Various upload/download combinations
  - Files affected: `tests/ratio_calculation_test.rs`
  - Estimated hours: 2

- [ ] **Task 6.4**: Test privacy settings
  - Implementation notes: Verify hidden fields not shown to other users
  - Files affected: `tests/privacy_test.rs`
  - Estimated hours: 3

- [ ] **Task 6.5**: Integration test for profile flow
  - Implementation notes: View profile → edit settings → view changes
  - Files affected: `tests/profile_integration_test.rs`
  - Estimated hours: 4

---

## Technical Details

### Architecture Considerations

- Cache user statistics in session for performance
- Update statistics asynchronously to avoid blocking
- Provide atomic operations for stat increments
- Support statistics export for analytics

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
sqlx = { workspace = true }
chrono = "0.4"
```

### Code Patterns

**Statistics Tracking:**
```rust
pub struct StatsTracker {
    storage: Arc<dyn Storage>,
}

impl StatsTracker {
    pub async fn increment_calls(&self, user_id: u32) -> Result<()> {
        self.storage.execute(
            "UPDATE users SET total_calls = total_calls + 1, last_login = ? WHERE id = ?",
            &[&Utc::now(), &user_id]
        ).await?;
        Ok(())
    }

    pub async fn record_upload(&self, user_id: u32, bytes: u64) -> Result<()> {
        self.storage.execute(
            "UPDATE users SET total_uploads = total_uploads + 1, upload_bytes = upload_bytes + ? WHERE id = ?",
            &[&bytes, &user_id]
        ).await?;
        Ok(())
    }

    pub async fn get_ratio(&self, user_id: u32) -> Result<f64> {
        let user = self.storage.get_user(user_id).await?;
        let ratio = if user.download_bytes > 0 {
            user.upload_bytes as f64 / user.download_bytes as f64
        } else {
            0.0
        };
        Ok(ratio)
    }
}
```

**Settings Management:**
```rust
pub struct UserSettings {
    pub theme: String,
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub hotkeys_enabled: bool,
    pub color_enabled: bool,
    pub page_pause: bool,
}

impl UserSettings {
    pub async fn load(storage: &dyn Storage, user_id: u32) -> Result<Self> {
        let user = storage.get_user(user_id).await?;
        Ok(Self {
            theme: user.theme.unwrap_or_else(|| "default".to_string()),
            terminal_width: user.terminal_width.unwrap_or(80),
            terminal_height: user.terminal_height.unwrap_or(25),
            hotkeys_enabled: user.hotkeys_enabled,
            color_enabled: user.color_enabled,
            page_pause: user.page_pause,
        })
    }

    pub async fn save(&self, storage: &dyn Storage, user_id: u32) -> Result<()> {
        storage.update_user_settings(user_id, self).await?;
        Ok(())
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 09**: User authentication provides user context
- **Sprint 04**: Storage layer for user data persistence

### Blocks Downstream
- **Sprint 20**: Theme system uses theme preferences
- **Sprint 23**: Admin may need user profile access

---

## Acceptance Criteria

- [ ] User profiles display all relevant information
- [ ] Statistics update in real-time
- [ ] Settings changes persist across sessions
- [ ] Password change works correctly
- [ ] Theme selection applies immediately
- [ ] Privacy settings are enforced
- [ ] Upload/download ratios calculate correctly
- [ ] Achievements are awarded appropriately (if implemented)

---

## Testing Requirements

### Unit Tests
- [ ] Statistics increment logic
- [ ] Ratio calculation
- [ ] Settings validation
- [ ] Achievement condition checking

### Integration Tests
- [ ] Profile display
- [ ] Settings editor flow
- [ ] Statistics tracking across actions
- [ ] Privacy enforcement

### User Acceptance Tests
- [ ] User can view own profile
- [ ] User can change password
- [ ] User can customize theme
- [ ] User can view other users' profiles

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Default theme: "Classic BBS"
- Default terminal size: 80x25
- Hotkeys enabled by default
- Ratio calculation: upload_bytes / download_bytes
- Hide email by default for privacy
- Achievement notifications shown once per session

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Statistics may become inaccurate over time
- **Mitigation**: Regular audit jobs, manual correction tools for SysOps
- **Risk**: Privacy settings may be bypassed
- **Mitigation**: Thorough testing, code review, access control enforcement
- **Risk**: Achievement spam may annoy users
- **Mitigation**: Show once per login, allow users to disable notifications

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
