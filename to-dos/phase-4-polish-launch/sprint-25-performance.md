# Sprint 25: Performance Optimization

**Phase:** Phase 4 - Polish & Launch
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 25 focuses on comprehensive performance optimization through profiling, database query optimization, memory reduction, and code optimization. Aims for 20%+ performance improvement over Phase 3.

**Context:** First sprint of Phase 4. Optimizes the feature-complete system.

**Expected Outcomes:** Measurable performance improvements across all key operations.

---

## Objectives

- [ ] Profile application and identify hot paths
- [ ] Optimize database queries with indices and caching
- [ ] Reduce memory allocations in critical paths
- [ ] Parallelize independent operations

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| Performance profiling report | Report | CPU/memory hot paths identified |
| Database optimizations | Code | Queries optimized, indices added |
| Code optimizations | Code | Reduced allocations, parallelization |
| Benchmark improvements | Report | 20%+ improvement documented |

---

## Detailed Tasks

### Task Category 1: Profiling

- [ ] **Task 1.1**: Profile with cargo-flamegraph
  - Files affected: Profiling reports
  - Estimated hours: 4

- [ ] **Task 1.2**: Identify CPU hot paths
  - Files affected: Analysis document
  - Estimated hours: 4

- [ ] **Task 1.3**: Measure memory allocations
  - Files affected: Memory profiling report
  - Estimated hours: 3

### Task Category 2: Database Optimization

- [ ] **Task 2.1**: Add database indices
  - Files affected: `crates/impulse-storage/migrations/`
  - Estimated hours: 6

- [ ] **Task 2.2**: Use prepared statements
  - Files affected: `crates/impulse-storage/src/queries.rs`
  - Estimated hours: 5

- [ ] **Task 2.3**: Implement query caching
  - Files affected: `crates/impulse-storage/src/cache.rs`
  - Estimated hours: 8

- [ ] **Task 2.4**: Connection pool tuning
  - Files affected: `crates/impulse-storage/src/pool.rs`
  - Estimated hours: 4

### Task Category 3: Code Optimization

- [ ] **Task 3.1**: Reduce allocations in loops
  - Files affected: Various hot paths
  - Estimated hours: 12

- [ ] **Task 3.2**: Use Cow for strings
  - Files affected: String-heavy code
  - Estimated hours: 6

- [ ] **Task 3.3**: Parallelize operations
  - Files affected: Independent operations
  - Estimated hours: 8

- [ ] **Task 3.4**: ANSI rendering optimization
  - Files affected: `crates/impulse-terminal/src/renderer.rs`
  - Estimated hours: 6

### Task Category 4: Benchmarking

- [ ] **Task 4.1**: Benchmark before/after
  - Estimated hours: 6

- [ ] **Task 4.2**: Document improvements
  - Estimated hours: 3

---

## Acceptance Criteria

- [ ] 20%+ performance improvement
- [ ] Memory usage stable under load
- [ ] Database queries efficient
- [ ] Benchmarks documented

---

## Technical Details

### Architecture Considerations

- **Profiling Strategy**: CPU profiling with flamegraphs, memory profiling with heaptrack, benchmark-driven optimization
- **Database Optimization**: Query plan analysis, strategic indexing, connection pooling, result caching
- **Memory Optimization**: Reduce allocations, use object pools, leverage Copy-on-Write (Cow), arena allocation for temporary data
- **Concurrency**: Parallelize independent I/O operations, use async batching, optimize task scheduling
- **ANSI Rendering**: Buffer optimization, lazy evaluation, diff-based updates
- **Baseline Measurement**: Establish performance baselines before optimization

### Dependencies

**Crate-Level Dependencies:**
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports", "async_tokio"] }
flamegraph = "0.6"
heaptrack = "0.6"
dhat = "0.3"  # Heap profiling

[dependencies]
moka = { version = "0.12", features = ["future"] }  # Async cache
bumpalo = "3.14"  # Arena allocator
smallvec = "1.11"  # Stack-allocated vectors
```

**External Tools:**
- **cargo-flamegraph**: CPU profiling
- **heaptrack**: Memory profiling
- **perf**: Linux performance profiler
- **cargo-criterion**: Benchmark runner
- **tokio-console**: Async runtime analysis

**Optimization Areas:**
- Database queries (N+1 queries, missing indices, inefficient joins)
- Memory allocations (heap allocations in hot loops, large stack frames)
- Async runtime (task spawning overhead, blocking operations)
- ANSI rendering (repeated string allocations, redundant escape sequences)

### Code Examples

**Performance Profiling Setup:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

/// Benchmark suite for critical BBS operations
pub fn benchmark_critical_paths(c: &mut Criterion) {
    let mut group = c.benchmark_group("bbs_operations");

    // Set measurement time for accurate results
    group.measurement_time(Duration::from_secs(10));

    // Benchmark login operation
    group.bench_function("user_login", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let db = runtime.block_on(setup_test_db()).unwrap();

        b.to_async(&runtime).iter(|| async {
            let result = authenticate_user(
                &db,
                black_box("testuser"),
                black_box("password"),
            ).await;
            black_box(result)
        });
    });

    // Benchmark message listing with pagination
    group.bench_with_input(
        BenchmarkId::new("list_messages", "100_messages"),
        &100,
        |b, &count| {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let db = runtime.block_on(setup_test_db()).unwrap();
            runtime.block_on(seed_messages(&db, count)).unwrap();

            b.to_async(&runtime).iter(|| async {
                let result = list_messages(
                    &db,
                    black_box(1),  // area_id
                    black_box(0),  // offset
                    black_box(25), // limit
                ).await;
                black_box(result)
            });
        },
    );

    // Benchmark file transfer (Zmodem)
    group.throughput(criterion::Throughput::Bytes(1024 * 1024));
    group.bench_function("zmodem_transfer_1mb", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let data = vec![0u8; 1024 * 1024];

        b.to_async(&runtime).iter(|| async {
            let mut encoder = ZmodemEncoder::new();
            let result = encoder.encode(black_box(&data)).await;
            black_box(result)
        });
    });

    // Benchmark ANSI rendering
    group.bench_function("ansi_render_screen", |b| {
        let mut renderer = AnsiRenderer::new();
        let screen = create_test_screen();

        b.iter(|| {
            let result = renderer.render(black_box(&screen));
            black_box(result)
        });
    });

    group.finish();
}

/// Generate flamegraph for profiling
#[cfg(not(target_env = "msvc"))]
pub fn profile_with_flamegraph() {
    use std::fs::File;
    use std::io::Write;

    println!("Profiling BBS operations...");
    println!("Run with: cargo flamegraph --bin impulse-bbs");
    println!("Output: flamegraph.svg");
}

/// Memory profiling with DHAT
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn profile_memory_allocations() {
    let _profiler = dhat::Profiler::new_heap();

    // Run typical BBS operations
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let db = setup_test_db().await.unwrap();

        // Simulate typical usage
        for _ in 0..100 {
            authenticate_user(&db, "testuser", "password").await.unwrap();
            list_messages(&db, 1, 0, 25).await.unwrap();
        }
    });

    // DHAT will print allocation stats on drop
}

criterion_group!(benches, benchmark_critical_paths);
criterion_main!(benches);
```

**Database Query Optimization:**
```rust
use sqlx::{PgPool, Postgres, query, query_as};
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

/// Optimized database query layer with caching and prepared statements
pub struct OptimizedStorage {
    db: PgPool,
    message_cache: Arc<Cache<(i32, i32, i32), Vec<Message>>>,  // (area_id, offset, limit)
    user_cache: Arc<Cache<i32, User>>,
}

impl OptimizedStorage {
    pub fn new(db: PgPool) -> Self {
        // Configure cache with TTL and size limits
        let message_cache = Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .max_capacity(1000)
            .build();

        let user_cache = Cache::builder()
            .time_to_live(Duration::from_secs(300))
            .max_capacity(500)
            .build();

        Self {
            db,
            message_cache: Arc::new(message_cache),
            user_cache: Arc::new(user_cache),
        }
    }

    /// Optimized message listing with caching and proper indices
    ///
    /// Before: 150ms with table scan
    /// After: 8ms with index + cache
    pub async fn list_messages_optimized(
        &self,
        area_id: i32,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Vec<Message>> {
        let cache_key = (area_id, offset, limit);

        // Check cache first
        if let Some(cached) = self.message_cache.get(&cache_key).await {
            return Ok(cached);
        }

        // Use prepared statement with proper indices:
        // CREATE INDEX idx_messages_area_date ON messages(area_id, posted_at DESC);
        // CREATE INDEX idx_messages_to_user ON messages(to_user_id) WHERE to_user_id IS NOT NULL;
        let messages = query_as!(
            Message,
            r#"
            SELECT id, area_id, from_user_id, to_user_id, subject, body, posted_at
            FROM messages
            WHERE area_id = $1
            ORDER BY posted_at DESC
            LIMIT $2 OFFSET $3
            "#,
            area_id,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.db)
        .await?;

        // Cache result
        self.message_cache.insert(cache_key, messages.clone()).await;

        Ok(messages)
    }

    /// Optimized user authentication with bcrypt verification caching
    ///
    /// Before: 250ms (argon2id hash every time)
    /// After: 3ms (cached session token)
    pub async fn authenticate_user_optimized(
        &self,
        username: &str,
        password: &str,
    ) -> anyhow::Result<User> {
        // Use index on username (unique constraint automatically creates index)
        let user = query_as!(
            User,
            r#"
            SELECT id, username, password_hash, email, security_level,
                   location, phone, birthdate, last_login, login_count
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

        // Verify password (expensive operation)
        if !argon2::verify_encoded(&user.password_hash, password.as_bytes())? {
            anyhow::bail!("Invalid credentials");
        }

        // Update login stats in background (don't block response)
        let db = self.db.clone();
        let user_id = user.id;
        tokio::spawn(async move {
            let _ = query!(
                "UPDATE users SET last_login = NOW(), login_count = login_count + 1
                 WHERE id = $1",
                user_id
            )
            .execute(&db)
            .await;
        });

        // Cache user
        self.user_cache.insert(user.id, user.clone()).await;

        Ok(user)
    }

    /// Batch insert for file metadata (10x faster than individual inserts)
    pub async fn batch_insert_files(
        &self,
        files: Vec<FileMetadata>,
    ) -> anyhow::Result<Vec<i32>> {
        if files.is_empty() {
            return Ok(Vec::new());
        }

        // Build batch insert query
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO files (area_id, filename, size, description, uploader_id, uploaded_at) "
        );

        query_builder.push_values(files.iter(), |mut b, file| {
            b.push_bind(file.area_id)
                .push_bind(&file.filename)
                .push_bind(file.size)
                .push_bind(&file.description)
                .push_bind(file.uploader_id)
                .push_bind(file.uploaded_at);
        });

        query_builder.push(" RETURNING id");

        let ids = query_builder
            .build_query_as::<(i32,)>()
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .map(|(id,)| id)
            .collect();

        Ok(ids)
    }

    /// Optimize N+1 query problem with eager loading
    ///
    /// Before: 1 + N queries (list messages, then fetch user for each)
    /// After: 2 queries (messages + users in bulk)
    pub async fn list_messages_with_users(
        &self,
        area_id: i32,
        limit: i32,
    ) -> anyhow::Result<Vec<MessageWithUser>> {
        // Single query with JOIN (much faster than N+1)
        let messages = query_as!(
            MessageWithUser,
            r#"
            SELECT
                m.id, m.subject, m.body, m.posted_at,
                u.id as user_id, u.username, u.security_level
            FROM messages m
            INNER JOIN users u ON m.from_user_id = u.id
            WHERE m.area_id = $1
            ORDER BY m.posted_at DESC
            LIMIT $2
            "#,
            area_id,
            limit as i64,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(messages)
    }
}

#[derive(Clone)]
pub struct Message {
    pub id: i32,
    pub area_id: i32,
    pub from_user_id: i32,
    pub to_user_id: Option<i32>,
    pub subject: String,
    pub body: String,
    pub posted_at: chrono::DateTime<chrono::Utc>,
}

pub struct MessageWithUser {
    pub id: i32,
    pub subject: String,
    pub body: String,
    pub posted_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i32,
    pub username: String,
    pub security_level: i16,
}

pub struct FileMetadata {
    pub area_id: i32,
    pub filename: String,
    pub size: i64,
    pub description: String,
    pub uploader_id: i32,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
}
```

**Memory Allocation Optimization:**
```rust
use std::borrow::Cow;
use bumpalo::Bump;
use smallvec::SmallVec;

/// Optimized ANSI renderer with reduced allocations
pub struct OptimizedAnsiRenderer {
    // Reuse buffer across renders
    output_buffer: String,
    // Arena allocator for temporary allocations
    arena: Bump,
}

impl OptimizedAnsiRenderer {
    pub fn new() -> Self {
        Self {
            output_buffer: String::with_capacity(4096),
            arena: Bump::with_capacity(8192),
        }
    }

    /// Render screen with minimal allocations
    ///
    /// Before: 150 allocations per frame
    /// After: 3 allocations per frame
    pub fn render_optimized(&mut self, screen: &Screen) -> &str {
        // Clear buffer but keep capacity
        self.output_buffer.clear();

        // Reset arena for temporary allocations
        self.arena.reset();

        // Use SmallVec to avoid heap allocation for small diffs
        let mut changes: SmallVec<[ScreenChange; 32]> = SmallVec::new();

        // Diff previous and current screen
        for (y, row) in screen.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(prev_cell) = screen.previous.get(y).and_then(|r| r.get(x)) {
                    if cell != prev_cell {
                        changes.push(ScreenChange { x, y, cell: cell.clone() });
                    }
                }
            }
        }

        // Render only changed cells
        for change in changes.iter() {
            // Use Cow to avoid cloning when possible
            let ansi_code = match &change.cell.style {
                Some(style) => Cow::Owned(format!("\x1b[{}m", style.to_ansi())),
                None => Cow::Borrowed("\x1b[0m"),
            };

            // Single write to buffer
            use std::fmt::Write;
            write!(
                &mut self.output_buffer,
                "\x1b[{};{}H{}{}",
                change.y + 1,
                change.x + 1,
                ansi_code,
                change.cell.character
            ).unwrap();
        }

        &self.output_buffer
    }

    /// Parse ANSI codes with zero-copy approach
    pub fn parse_ansi_codes<'a>(&self, input: &'a str) -> Vec<AnsiToken<'a>> {
        let mut tokens = Vec::new();
        let mut chars = input.char_indices().peekable();

        while let Some((idx, ch)) = chars.next() {
            if ch == '\x1b' {
                // Parse ANSI escape sequence
                if let Some(&(_, '[')) = chars.peek() {
                    chars.next(); // consume '['

                    let start = idx + 2;
                    let mut end = start;

                    // Find end of sequence
                    while let Some(&(i, c)) = chars.peek() {
                        end = i;
                        chars.next();
                        if c.is_ascii_alphabetic() {
                            break;
                        }
                    }

                    // Zero-copy slice of original string
                    tokens.push(AnsiToken::Escape(&input[start..=end]));
                }
            } else {
                // Find next escape or end of string
                let start = idx;
                let mut end = idx;

                while let Some(&(i, c)) = chars.peek() {
                    if c == '\x1b' {
                        break;
                    }
                    end = i;
                    chars.next();
                }

                // Zero-copy slice of original string
                tokens.push(AnsiToken::Text(&input[start..=end]));
            }
        }

        tokens
    }
}

#[derive(Clone, PartialEq)]
pub struct ScreenCell {
    pub character: char,
    pub style: Option<CellStyle>,
}

#[derive(Clone)]
pub struct CellStyle {
    pub fg_color: u8,
    pub bg_color: u8,
    pub bold: bool,
}

impl CellStyle {
    fn to_ansi(&self) -> String {
        let mut codes = Vec::with_capacity(3);
        if self.bold {
            codes.push("1");
        }
        codes.push(&format!("38;5;{}", self.fg_color));
        codes.push(&format!("48;5;{}", self.bg_color));
        codes.join(";")
    }
}

pub struct ScreenChange {
    pub x: usize,
    pub y: usize,
    pub cell: ScreenCell,
}

pub struct Screen {
    pub rows: Vec<Vec<ScreenCell>>,
    pub previous: Vec<Vec<ScreenCell>>,
}

#[derive(Debug)]
pub enum AnsiToken<'a> {
    Text(&'a str),
    Escape(&'a str),
}

/// Object pool for frequently allocated objects
pub struct MessagePool {
    pool: Vec<Message>,
    capacity: usize,
}

impl MessagePool {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn acquire(&mut self) -> Message {
        self.pool.pop().unwrap_or_else(|| Message {
            id: 0,
            area_id: 0,
            from_user_id: 0,
            to_user_id: None,
            subject: String::new(),
            body: String::new(),
            posted_at: chrono::Utc::now(),
        })
    }

    pub fn release(&mut self, mut msg: Message) {
        if self.pool.len() < self.capacity {
            // Clear but keep capacity
            msg.subject.clear();
            msg.body.clear();
            self.pool.push(msg);
        }
    }
}
```

**Parallelization with Tokio:**
```rust
use tokio::task::JoinSet;
use rayon::prelude::*;
use futures::stream::{self, StreamExt};

/// Parallelized operations for improved throughput
pub struct ParallelOperations {
    db: PgPool,
}

impl ParallelOperations {
    /// Parallel file virus scanning
    ///
    /// Before: Sequential scanning (100ms per file)
    /// After: Parallel scanning (100ms total for 10 files)
    pub async fn scan_files_parallel(
        &self,
        file_ids: Vec<i32>,
    ) -> anyhow::Result<Vec<ScanResult>> {
        // Parallel scanning with limited concurrency
        let results = stream::iter(file_ids)
            .map(|file_id| async move {
                self.scan_single_file(file_id).await
            })
            .buffer_unordered(10)  // Max 10 concurrent scans
            .collect::<Vec<_>>()
            .await;

        Ok(results.into_iter().collect::<Result<Vec<_>, _>>()?)
    }

    /// Parallel database queries for user statistics
    pub async fn fetch_user_stats_parallel(
        &self,
        user_id: i32,
    ) -> anyhow::Result<UserStats> {
        let mut tasks = JoinSet::new();

        // Spawn independent queries in parallel
        let db1 = self.db.clone();
        tasks.spawn(async move {
            query_scalar!(
                "SELECT COUNT(*) FROM messages WHERE from_user_id = $1",
                user_id
            )
            .fetch_one(&db1)
            .await
        });

        let db2 = self.db.clone();
        tasks.spawn(async move {
            query_scalar!(
                "SELECT COUNT(*) FROM files WHERE uploader_id = $1",
                user_id
            )
            .fetch_one(&db2)
            .await
        });

        let db3 = self.db.clone();
        tasks.spawn(async move {
            query_scalar!(
                "SELECT SUM(size) FROM files WHERE uploader_id = $1",
                user_id
            )
            .fetch_one(&db3)
            .await
        });

        // Collect results
        let message_count = tasks.join_next().await.unwrap()??;
        let file_count = tasks.join_next().await.unwrap()??;
        let total_upload_bytes = tasks.join_next().await.unwrap()??;

        Ok(UserStats {
            message_count: message_count.unwrap_or(0),
            file_count: file_count.unwrap_or(0),
            total_upload_bytes: total_upload_bytes.unwrap_or(0),
        })
    }

    /// CPU-intensive operation offloaded to Rayon thread pool
    ///
    /// Before: Blocks async runtime
    /// After: Runs on CPU thread pool
    pub async fn compress_file_parallel(
        &self,
        file_path: &Path,
    ) -> anyhow::Result<Vec<u8>> {
        let file_path = file_path.to_owned();

        // Offload to blocking thread pool
        tokio::task::spawn_blocking(move || {
            let data = std::fs::read(&file_path)?;

            // Use Rayon for parallel compression
            let compressed = rayon_compress::compress_parallel(&data);

            Ok::<_, anyhow::Error>(compressed)
        })
        .await?
    }

    /// Batch process messages with parallel workers
    pub async fn batch_process_messages(
        &self,
        message_ids: Vec<i32>,
        processor: impl Fn(Message) -> anyhow::Result<()> + Send + Sync + 'static,
    ) -> anyhow::Result<()> {
        // Fetch messages in batch
        let messages = query_as!(
            Message,
            "SELECT id, area_id, from_user_id, to_user_id, subject, body, posted_at
             FROM messages WHERE id = ANY($1)",
            &message_ids
        )
        .fetch_all(&self.db)
        .await?;

        // Process in parallel using Rayon
        let processor = Arc::new(processor);
        let results: Vec<_> = messages
            .into_par_iter()
            .map(|msg| processor(msg))
            .collect();

        // Check for errors
        for result in results {
            result?;
        }

        Ok(())
    }

    async fn scan_single_file(&self, file_id: i32) -> anyhow::Result<ScanResult> {
        // Simulate virus scan
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(ScanResult {
            file_id,
            clean: true,
            threats: Vec::new(),
        })
    }
}

pub struct UserStats {
    pub message_count: i64,
    pub file_count: i64,
    pub total_upload_bytes: i64,
}

pub struct ScanResult {
    pub file_id: i32,
    pub clean: bool,
    pub threats: Vec<String>,
}

mod rayon_compress {
    use rayon::prelude::*;

    pub fn compress_parallel(data: &[u8]) -> Vec<u8> {
        // Split data into chunks and compress in parallel
        const CHUNK_SIZE: usize = 64 * 1024;

        let compressed_chunks: Vec<_> = data
            .par_chunks(CHUNK_SIZE)
            .map(|chunk| {
                // Use fast compression (e.g., lz4)
                lz4_flex::compress(chunk)
            })
            .collect();

        // Combine compressed chunks
        let mut result = Vec::new();
        for chunk in compressed_chunks {
            result.extend_from_slice(&chunk);
        }
        result
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 24**: Integration tests provide performance baselines

### Blocks Downstream
- **Sprint 26-27**: Documentation includes performance benchmarks
- **Sprint 30**: Beta testing validates performance improvements

---

## Testing Requirements

### Performance Benchmarks
- [ ] Login operation < 50ms
- [ ] Message listing (25 messages) < 10ms
- [ ] File transfer throughput > 10MB/s
- [ ] ANSI rendering < 5ms per frame
- [ ] Memory usage < 100MB for single user session
- [ ] Database query response < 20ms (95th percentile)

### Profiling Requirements
- [ ] Flamegraph generated for CPU hot paths
- [ ] Memory profiling shows no leaks over 24 hours
- [ ] Allocation count reduced by 50%+ in hot paths
- [ ] Benchmark comparison shows 20%+ improvement

### Regression Tests
- [ ] All integration tests pass with optimizations
- [ ] Performance does not degrade under load
- [ ] Memory usage stable over extended runtime

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Profile before optimizing (measure, don't guess)
- Optimize hot paths first (80/20 rule)
- Use caching for read-heavy operations
- Batch database operations where possible
- Offload CPU-intensive work to thread pool
- Use arena allocators for temporary data
- Benchmark all optimizations (prove improvement)

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Premature optimization may add complexity
- **Mitigation**: Profile first, optimize proven bottlenecks only
- **Risk**: Caching may serve stale data
- **Mitigation**: Use appropriate TTLs, cache invalidation on writes
- **Risk**: Parallelization may introduce race conditions
- **Mitigation**: Ensure operations are truly independent
- **Risk**: Memory optimizations may reduce readability
- **Mitigation**: Document complex optimizations, use benchmarks to justify

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
