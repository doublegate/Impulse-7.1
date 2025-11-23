# Sprint 21: Door Game Interface

**Phase:** Phase 3 - Feature Completion
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 21 implements the door game interface system, allowing execution of external DOS door games via DOSBox. Includes dropfile generation, process management, and I/O capture for classic BBS door games.

**Context:** Sprint 5 of Phase 3. Enables running classic BBS games.

**Expected Outcomes:** Users can play classic DOS door games like LORD and TradeWars from the BBS.

---

## Objectives

- [ ] Implement DOOR.SYS and DORINFO1.DEF dropfile generation
- [ ] Integrate DOSBox for running DOS doors
- [ ] Test with 2-3 classic door games
- [ ] Handle door I/O and session state

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-door` crate | Code | Dropfile generation and DOSBox integration |
| Dropfile formats | Code | DOOR.SYS, DORINFO1.DEF support |
| DOSBox integration | Code | Execute DOS programs, capture I/O |
| Tested door games | Config | LORD, TradeWars verified working |

---

## Detailed Tasks

### Task Category 1: Dropfile Generation

- [ ] **Task 1.1**: Implement DOOR.SYS format
  - Files affected: `crates/impulse-door/src/dropfiles/doorsys.rs`
  - Estimated hours: 5

- [ ] **Task 1.2**: Implement DORINFO1.DEF format
  - Files affected: `crates/impulse-door/src/dropfiles/dorinfo.rs`
  - Estimated hours: 4

- [ ] **Task 1.3**: Generate from session state
  - Files affected: `crates/impulse-door/src/dropfiles/generator.rs`
  - Estimated hours: 4

### Task Category 2: DOSBox Integration

- [ ] **Task 2.1**: Spawn DOSBox process
  - Files affected: `crates/impulse-door/src/dosbox/spawn.rs`
  - Estimated hours: 6

- [ ] **Task 2.2**: Mount virtual drive
  - Files affected: `crates/impulse-door/src/dosbox/mount.rs`
  - Estimated hours: 4

- [ ] **Task 2.3**: Capture I/O via pipe
  - Files affected: `crates/impulse-door/src/dosbox/io.rs`
  - Estimated hours: 8

- [ ] **Task 2.4**: Process cleanup on exit
  - Files affected: `crates/impulse-door/src/dosbox/cleanup.rs`
  - Estimated hours: 3

### Task Category 3: Door Execution Flow

- [ ] **Task 3.1**: Save session state
  - Files affected: `crates/impulse-door/src/session/save.rs`
  - Estimated hours: 3

- [ ] **Task 3.2**: Execute door
  - Files affected: `crates/impulse-door/src/execute.rs`
  - Estimated hours: 5

- [ ] **Task 3.3**: Restore session on exit
  - Files affected: `crates/impulse-door/src/session/restore.rs`
  - Estimated hours: 3

- [ ] **Task 3.4**: Update user stats from door
  - Files affected: `crates/impulse-door/src/stats.rs`
  - Estimated hours: 4

### Task Category 4: Testing

- [ ] **Task 4.1**: Test with Legend of the Red Dragon
  - Estimated hours: 6

- [ ] **Task 4.2**: Test with TradeWars 2002
  - Estimated hours: 5

- [ ] **Task 4.3**: Verify stat updates
  - Estimated hours: 3

---

## Acceptance Criteria

- [ ] Dropfiles generated correctly
- [ ] DOS doors run successfully
- [ ] User stats update after playing
- [ ] Session resumes cleanly after door

## Technical Details

### Architecture Considerations

- Use DOSBox emulation for DOS door games compatibility
- Generate dropfiles dynamically from current session state
- Pipe I/O between BBS and DOSBox for seamless integration
- Handle process lifecycle (spawn, monitor, cleanup)
- Support multiple concurrent door sessions

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
tokio-process = "0.2"
tempfile = "3.8"
serde = { workspace = true }

[build-dependencies]
which = "5.0"
```

**Pascal Units Being Converted:**
- DOORS.PAS (Door execution system)
- DROPFILE.PAS (Dropfile generation - DOOR.SYS, DORINFO1.DEF)
- FOSSIL.PAS (FOSSIL emulation for door I/O)

**External Dependencies:**
- DOSBox (must be installed on system)

### Code Examples

**Dropfile Generation (DOOR.SYS):**
```rust
use std::fmt::Write as FmtWrite;
use std::io::Write;
use std::path::PathBuf;

pub struct DoorSysDropfile {
    pub com_port: String,
    pub baud_rate: u32,
    pub parity: u8,
    pub node_number: u16,
    pub com_port_baud: u32,
    pub screen_display: char,
    pub printer_toggle: char,
    pub page_bell: char,
    pub caller_alarm: char,
    pub user_name: String,
    pub location: String,
    pub phone_number: String,
    pub password: String,
    pub security_level: u8,
    pub num_times_on: u32,
    pub last_date_on: String,
    pub seconds_remaining: u32,
    pub minutes_remaining: u32,
    pub graphics_mode: char,
    pub page_length: u8,
    pub expert_mode: char,
    pub conferences: String,
    pub conference_number: u16,
    pub upload_kb: u32,
    pub download_kb: u32,
    pub daily_download_limit_kb: u32,
    pub birthdate: String,
    pub path_to_user_file: PathBuf,
    pub path_to_gen_dir: PathBuf,
    pub sysop_name: String,
    pub alias: String,
    pub event_time: String,
    pub error_free_connect: char,
    pub ansi_supported: char,
    pub use_record_locking: char,
    pub bbs_color: u8,
    pub time_credit_minutes: u32,
    pub last_new_files_scan_date: String,
    pub time_of_this_call: String,
    pub time_of_last_call: String,
    pub daily_file_limit: u32,
    pub files_downloaded_today: u32,
    pub total_kb_downloaded_today: u32,
    pub total_kb_uploaded_today: u32,
    pub delete_flag: char,
    pub last_message_read: u32,
    pub sec_for_conf: Vec<u8>,
    pub total_uploads: u32,
    pub total_downloads: u32,
    pub daily_download_kb_total: u32,
    pub daily_download_limit: u32,
}

impl DoorSysDropfile {
    pub fn from_session(session: &SessionState) -> Self {
        Self {
            com_port: "COM1".to_string(),
            baud_rate: 57600,
            parity: 8,
            node_number: session.node,
            com_port_baud: 0,
            screen_display: 'Y',
            printer_toggle: 'N',
            page_bell: 'Y',
            caller_alarm: 'N',
            user_name: session.user.name.clone(),
            location: session.user.location.clone(),
            phone_number: session.user.phone.clone(),
            password: "XXXX".to_string(),  // Never expose real password
            security_level: session.user.security_level,
            num_times_on: session.user.login_count,
            last_date_on: session.user.last_login.format("%m/%d/%y").to_string(),
            seconds_remaining: session.time_remaining(),
            minutes_remaining: session.time_remaining() / 60,
            graphics_mode: if session.ansi_enabled { 'Y' } else { 'N' },
            page_length: 24,
            expert_mode: if session.expert_mode { 'Y' } else { 'N' },
            conferences: "0".to_string(),
            conference_number: 0,
            upload_kb: session.user.upload_kb,
            download_kb: session.user.download_kb,
            daily_download_limit_kb: 10240,  // 10MB default
            birthdate: session.user.birthdate.format("%m/%d/%y").to_string(),
            path_to_user_file: PathBuf::from("/bbs/users/"),
            path_to_gen_dir: PathBuf::from("/bbs/gen/"),
            sysop_name: "SysOp".to_string(),
            alias: session.user.alias.clone().unwrap_or_default(),
            event_time: "00:00".to_string(),
            error_free_connect: 'Y',
            ansi_supported: if session.ansi_enabled { 'Y' } else { 'N' },
            use_record_locking: 'Y',
            bbs_color: 7,
            time_credit_minutes: 0,
            last_new_files_scan_date: session.user.last_file_scan.format("%m/%d/%y").to_string(),
            time_of_this_call: session.login_time.format("%H:%M").to_string(),
            time_of_last_call: session.user.last_login.format("%H:%M").to_string(),
            daily_file_limit: 100,
            files_downloaded_today: session.user.files_downloaded_today,
            total_kb_downloaded_today: session.user.kb_downloaded_today,
            total_kb_uploaded_today: session.user.kb_uploaded_today,
            delete_flag: 'N',
            last_message_read: session.user.last_message_read,
            sec_for_conf: vec![session.user.security_level; 10],
            total_uploads: session.user.total_uploads,
            total_downloads: session.user.total_downloads,
            daily_download_kb_total: session.user.kb_downloaded_today,
            daily_download_limit: 10240,
        }
    }

    pub fn write_to_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        let mut file = std::fs::File::create(path)?;
        
        writeln!(file, "{}", self.com_port)?;
        writeln!(file, "{}", self.baud_rate)?;
        writeln!(file, "{}", self.parity)?;
        writeln!(file, "{}", self.node_number)?;
        writeln!(file, "{}L", self.com_port_baud)?;  // L = locked baud
        writeln!(file, "{}", self.screen_display)?;
        writeln!(file, "{}", self.printer_toggle)?;
        writeln!(file, "{}", self.page_bell)?;
        writeln!(file, "{}", self.caller_alarm)?;
        writeln!(file, "{}", self.user_name)?;
        writeln!(file, "{}", self.location)?;
        writeln!(file, "{}", self.phone_number)?;
        writeln!(file, "{}", self.password)?;
        writeln!(file, "{}", self.security_level)?;
        writeln!(file, "{}", self.num_times_on)?;
        writeln!(file, "{}", self.last_date_on)?;
        writeln!(file, "{}", self.seconds_remaining)?;
        writeln!(file, "{}", self.minutes_remaining)?;
        writeln!(file, "{}", self.graphics_mode)?;
        writeln!(file, "{}", self.page_length)?;
        writeln!(file, "{}", self.expert_mode)?;
        writeln!(file, "{}", self.conferences)?;
        writeln!(file, "{}", self.conference_number)?;
        // ... (remaining 30+ lines)
        
        Ok(())
    }
}
```

**DOSBox Execution:**
```rust
use tokio::process::{Child, Command};
use tokio::io::{AsyncBufReadExt, BufReader};

pub struct DoorExecutor {
    dosbox_path: PathBuf,
    door_dir: PathBuf,
    node_dir: PathBuf,
}

impl DoorExecutor {
    pub async fn execute_door(
        &self,
        door_name: &str,
        session: &SessionState,
    ) -> anyhow::Result<DoorResult> {
        // Create node directory
        let node_path = self.node_dir.join(format!("node{}", session.node));
        tokio::fs::create_dir_all(&node_path).await?;

        // Generate dropfile
        let dropfile_path = node_path.join("DOOR.SYS");
        let dropfile = DoorSysDropfile::from_session(session);
        dropfile.write_to_file(&dropfile_path)?;

        // Create DOSBox config
        let config_path = node_path.join("dosbox.conf");
        self.create_dosbox_config(&config_path, door_name).await?;

        // Spawn DOSBox process
        let mut child = Command::new(&self.dosbox_path)
            .arg("-conf")
            .arg(&config_path)
            .arg("-noconsole")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        // Pipe I/O between BBS session and DOSBox
        let result = self.handle_door_io(&mut child, session).await?;

        // Cleanup
        child.wait().await?;
        tokio::fs::remove_dir_all(&node_path).await?;

        Ok(result)
    }

    async fn handle_door_io(
        &self,
        child: &mut Child,
        session: &SessionState,
    ) -> anyhow::Result<DoorResult> {
        let stdout = child.stdout.take().unwrap();
        let mut reader = BufReader::new(stdout);

        let stdin = child.stdin.take().unwrap();
        let mut writer = stdin;

        // Bidirectional I/O loop
        let mut line = String::new();
        loop {
            tokio::select! {
                // Read from door, send to user
                result = reader.read_line(&mut line) => {
                    match result {
                        Ok(0) => break,  // EOF
                        Ok(_) => {
                            session.terminal.write(&line).await?;
                            line.clear();
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                // Read from user, send to door
                key = session.terminal.read_key() => {
                    use tokio::io::AsyncWriteExt;
                    writer.write_all(&[key]).await?;
                    writer.flush().await?;
                }
            }
        }

        Ok(DoorResult {
            played: true,
            updated_stats: true,
        })
    }

    async fn create_dosbox_config(
        &self,
        path: &PathBuf,
        door_name: &str,
    ) -> anyhow::Result<()> {
        let config = format!(
            "[autoexec]\n\
             mount c {}\n\
             c:\n\
             cd \\{}\n\
             {}.bat\n\
             exit\n",
            self.door_dir.display(),
            door_name,
            door_name
        );

        tokio::fs::write(path, config).await?;
        Ok(())
    }
}

pub struct DoorResult {
    pub played: bool,
    pub updated_stats: bool,
}
```

**Door Configuration:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorConfig {
    pub name: String,
    pub description: String,
    pub executable: String,
    pub directory: PathBuf,
    pub dropfile_type: DropfileType,
    pub min_security_level: u8,
    pub max_time_minutes: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DropfileType {
    DoorSys,
    DorinfoeDef,
    DorinfoDef,
    ChainTxt,
}

impl DoorConfig {
    pub fn load_from_dir(doors_dir: &Path) -> anyhow::Result<Vec<Self>> {
        let mut doors = Vec::new();

        for entry in std::fs::read_dir(doors_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension() == Some("toml".as_ref()) {
                let content = std::fs::read_to_string(&path)?;
                let door: DoorConfig = toml::from_str(&content)?;
                doors.push(door);
            }
        }

        Ok(doors)
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 06**: User system provides session state for dropfiles
- **Sprint 05**: Terminal I/O for piping between BBS and door

### Blocks Downstream
- **Sprint 30**: Beta testing includes door game verification

---

## Testing Requirements

### Unit Tests
- [ ] DOOR.SYS generation matches specification
- [ ] DORINFO1.DEF generation matches specification
- [ ] Dropfile parsing by doors
- [ ] DOSBox config generation

### Integration Tests
- [ ] Full door execution cycle
- [ ] I/O piping between BBS and door
- [ ] Session state restoration after door exit
- [ ] User stats update from door play

### Door Compatibility Tests
- [ ] Legend of the Red Dragon (LORD) runs successfully
- [ ] TradeWars 2002 runs successfully
- [ ] Barren Realms Elite (BRE) runs successfully
- [ ] User stats persist across door plays

### Performance Tests
- [ ] Door startup < 3 seconds
- [ ] I/O latency < 50ms
- [ ] Memory cleanup after door exit
- [ ] Multiple concurrent door sessions supported

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use DOSBox for DOS door compatibility (cross-platform)
- Generate DOOR.SYS by default (most widely supported)
- Support DORINFO1.DEF for doors that require it
- Limit door session time to 30 minutes (configurable)
- Require SysOp configuration of each door game
- Clean up node directory after each session

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: DOSBox may not be installed on all systems
- **Mitigation**: Check for DOSBox during BBS startup; provide clear installation instructions
- **Risk**: Some doors may not work correctly
- **Mitigation**: Test with popular doors; provide troubleshooting guide
- **Risk**: Door I/O piping may have latency
- **Mitigation**: Use async I/O; optimize buffer sizes
- **Risk**: Doors may crash or hang
- **Mitigation**: Timeout mechanism; cleanup orphaned processes

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
