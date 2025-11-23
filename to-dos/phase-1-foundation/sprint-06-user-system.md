# Sprint 06: Async Runtime & Session Skeleton

**Phase:** Phase 1 - Foundation
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 06 establishes the asynchronous runtime foundation using Tokio and creates the session management framework. This sprint implements the core infrastructure for handling multiple concurrent connections, managing session lifecycles, and gracefully shutting down the system. This is critical infrastructure that all future networking features will build upon.

**Context:** This is the sixth sprint of Phase 1 (Foundation). The async runtime and session management form the backbone of all concurrent operations in the BBS.

**Expected Outcomes:** By the end of this sprint, the project will have a fully functional session management system capable of accepting and managing multiple simultaneous telnet connections with proper lifecycle management.

---

## Objectives

- [ ] Establish Tokio async runtime as project foundation
- [ ] Create SessionManager for tracking active connections
- [ ] Implement connection lifecycle (accept → authenticate → disconnect)
- [ ] Build basic telnet server with graceful shutdown

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-session` crate | Code | SessionManager implemented with concurrent session tracking |
| Session struct with event loop | Code | Session handles connection lifecycle from accept to cleanup |
| Basic telnet server | Code | TCP listener accepts connections and spawns session tasks |
| Graceful shutdown handling | Code | Server closes all sessions cleanly on SIGTERM/SIGINT |

---

## Detailed Tasks

### Task Category 1: SessionManager Implementation

- [ ] **Task 1.1**: Create SessionManager struct
  - Implementation notes: Track active sessions in HashMap<SessionId, SessionHandle>
  - Files affected: `crates/impulse-session/src/manager.rs`
  - Estimated hours: 3

- [ ] **Task 1.2**: Implement session spawning
  - Implementation notes: Create new Session, spawn Tokio task, store handle
  - Files affected: `crates/impulse-session/src/manager.rs`
  - Estimated hours: 4

- [ ] **Task 1.3**: Add session tracking and cleanup
  - Implementation notes: Remove sessions on disconnect, track session count
  - Files affected: `crates/impulse-session/src/manager.rs`
  - Estimated hours: 3

- [ ] **Task 1.4**: Implement graceful shutdown
  - Implementation notes: Send shutdown signal to all sessions, wait for completion
  - Files affected: `crates/impulse-session/src/manager.rs`
  - Estimated hours: 4

- [ ] **Task 1.5**: Add session limits and throttling
  - Implementation notes: Max concurrent sessions, connection rate limiting
  - Files affected: `crates/impulse-session/src/manager.rs`
  - Estimated hours: 3

### Task Category 2: Session Lifecycle Implementation

- [ ] **Task 2.1**: Define Session struct
  - Implementation notes: Session ID, user info (optional), connection stream, state
  - Files affected: `crates/impulse-session/src/session.rs`
  - Estimated hours: 2

- [ ] **Task 2.2**: Implement connection acceptance
  - Implementation notes: Accept TCP stream, create Session, send welcome message
  - Files affected: `crates/impulse-session/src/session.rs`
  - Estimated hours: 3

- [ ] **Task 2.3**: Add authentication placeholder
  - Implementation notes: Basic auth flow structure (to be implemented in Sprint 09)
  - Files affected: `crates/impulse-session/src/session.rs`
  - Estimated hours: 2

- [ ] **Task 2.4**: Implement idle timeout detection
  - Implementation notes: Track last activity, disconnect after configured idle time
  - Files affected: `crates/impulse-session/src/session.rs`
  - Estimated hours: 4

- [ ] **Task 2.5**: Add disconnection cleanup
  - Implementation notes: Close streams, update session tracking, log disconnect
  - Files affected: `crates/impulse-session/src/session.rs`
  - Estimated hours: 2

### Task Category 3: Basic Telnet Server

- [ ] **Task 3.1**: Create TCP listener
  - Implementation notes: Bind to configured port, accept incoming connections
  - Files affected: `crates/impulse-server/src/telnet.rs`
  - Estimated hours: 3

- [ ] **Task 3.2**: Pass streams to SessionManager
  - Implementation notes: For each accepted connection, spawn via SessionManager
  - Files affected: `crates/impulse-server/src/telnet.rs`
  - Estimated hours: 2

- [ ] **Task 3.3**: Implement signal handling
  - Implementation notes: Catch SIGTERM/SIGINT, trigger graceful shutdown
  - Files affected: `crates/impulse-server/src/main.rs`
  - Estimated hours: 3

- [ ] **Task 3.4**: Add startup/shutdown logging
  - Implementation notes: Log server start, listen address, shutdown progress
  - Files affected: `crates/impulse-server/src/main.rs`
  - Estimated hours: 2

### Task Category 4: Testing with Telnet Clients

- [ ] **Task 4.1**: Test with standard telnet client
  - Implementation notes: Verify connection, welcome message, disconnect
  - Files affected: Manual testing, document results
  - Estimated hours: 2

- [ ] **Task 4.2**: Test multiple simultaneous connections
  - Implementation notes: Connect 10+ clients, verify all work correctly
  - Files affected: Integration tests
  - Estimated hours: 3

- [ ] **Task 4.3**: Test connection timeout
  - Implementation notes: Verify idle connections are closed after timeout
  - Files affected: `tests/timeout_test.rs`
  - Estimated hours: 3

- [ ] **Task 4.4**: Test graceful shutdown
  - Implementation notes: Connect clients, send SIGTERM, verify clean shutdown
  - Files affected: `tests/shutdown_test.rs`
  - Estimated hours: 4

- [ ] **Task 4.5**: Test session limits
  - Implementation notes: Attempt to exceed max connections, verify rejection
  - Files affected: `tests/limits_test.rs`
  - Estimated hours: 3

### Task Category 5: Documentation and Code Quality

- [ ] **Task 5.1**: Add comprehensive rustdoc comments
  - Implementation notes: Document all public APIs with examples
  - Files affected: All source files
  - Estimated hours: 4

- [ ] **Task 5.2**: Create session management guide
  - Implementation notes: Explain session lifecycle, state transitions
  - Files affected: `docs/session-management.md`
  - Estimated hours: 3

- [ ] **Task 5.3**: Document graceful shutdown procedure
  - Implementation notes: How shutdown works, timeouts, force kill
  - Files affected: `docs/shutdown-procedure.md`
  - Estimated hours: 2

---

## Technical Details

### Architecture Considerations

- Use Tokio for async runtime (industry standard, excellent ecosystem)
- One Tokio task per session for isolation
- Use channels for inter-session communication
- Implement backpressure to prevent memory exhaustion

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
tokio-util = "0.7"
anyhow = { workspace = true }
thiserror = { workspace = true }
tracing = "0.1"
tracing-subscriber = "0.3"
signal-hook = "0.3"
signal-hook-tokio = "0.3"

[dev-dependencies]
tokio-test = "0.4"
```

### Code Patterns

**SessionManager Pattern:**
```rust
use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<SessionId, JoinHandle<()>>>>,
    max_sessions: usize,
    shutdown_tx: broadcast::Sender<()>,
}

impl SessionManager {
    pub async fn spawn_session(&self, stream: TcpStream) -> Result<SessionId> {
        let sessions = self.sessions.read().await;
        if sessions.len() >= self.max_sessions {
            return Err(anyhow!("Maximum sessions reached"));
        }
        drop(sessions);

        let session_id = SessionId::new();
        let shutdown_rx = self.shutdown_tx.subscribe();

        let session = Session::new(session_id, stream, shutdown_rx);
        let handle = tokio::spawn(async move {
            session.run().await;
        });

        self.sessions.write().await.insert(session_id, handle);
        Ok(session_id)
    }

    pub async fn shutdown_all(&self) {
        let _ = self.shutdown_tx.send(());

        let handles: Vec<_> = self.sessions
            .write()
            .await
            .drain()
            .map(|(_, handle)| handle)
            .collect();

        for handle in handles {
            let _ = handle.await;
        }
    }
}
```

**Session Event Loop:**
```rust
pub struct Session {
    id: SessionId,
    stream: TcpStream,
    shutdown_rx: broadcast::Receiver<()>,
    last_activity: Instant,
}

impl Session {
    pub async fn run(mut self) {
        let mut idle_timer = interval(Duration::from_secs(60));

        loop {
            select! {
                _ = idle_timer.tick() => {
                    if self.is_idle() {
                        break;
                    }
                }
                _ = self.shutdown_rx.recv() => {
                    break;
                }
                // Future: handle user input, commands, etc.
            }
        }

        self.cleanup().await;
    }

    fn is_idle(&self) -> bool {
        self.last_activity.elapsed() > Duration::from_secs(600)
    }

    async fn cleanup(&mut self) {
        // Close streams, save session data, etc.
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 05**: Configuration system provides session limits and timeouts

### Blocks Downstream
- **Sprint 07**: Terminal I/O requires active sessions
- **Sprint 09**: Authentication requires session framework
- **All connection-based features**: Session management is foundation

---

## Acceptance Criteria

- [ ] Server accepts multiple telnet connections
- [ ] Sessions are tracked and cleaned up properly
- [ ] Graceful shutdown closes all sessions cleanly
- [ ] Idle connections timeout correctly
- [ ] Max session limit is enforced
- [ ] No resource leaks (connections, memory, tasks)
- [ ] All public APIs documented

---

## Testing Requirements

### Unit Tests
- [ ] SessionManager session tracking
- [ ] Session state transitions
- [ ] Idle timeout calculation
- [ ] Shutdown signal propagation

### Integration Tests
- [ ] Full connection lifecycle (accept → use → disconnect)
- [ ] Multiple simultaneous connections
- [ ] Graceful shutdown with active sessions
- [ ] Connection limit enforcement
- [ ] Idle timeout triggers

### Load Tests
- [ ] 50+ simultaneous connections
- [ ] Rapid connect/disconnect cycles
- [ ] Resource usage under load

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use Tokio for async runtime (mature, well-supported)
- One task per session (simple, isolated failure domains)
- Broadcast channel for shutdown signal (efficient for many receivers)
- Default idle timeout: 10 minutes

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Task spawning overhead for many connections
- **Mitigation**: Benchmark, consider connection pooling if needed
- **Risk**: Shutdown may hang if sessions don't respond
- **Mitigation**: Implement shutdown timeout, force kill after deadline
- **Risk**: Resource leaks if cleanup fails
- **Mitigation**: Comprehensive testing, use RAII patterns

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
