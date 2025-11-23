# Sprint 29: Web-Based Administration

**Phase:** Phase 4 - Polish & Launch
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 29 creates a REST API and web-based administration interface, providing SysOps with a modern browser-based management tool alongside the terminal interface.

**Context:** Sprint 5 of Phase 4. Modern administration alternative.

**Expected Outcomes:** SysOps can manage the BBS via a web browser.

---

## Objectives

- [ ] Create REST API for administration
- [ ] Build basic web UI for admin tasks
- [ ] Implement API authentication (JWT)
- [ ] Document API with OpenAPI specification

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| `impulse-api` crate | Code | REST API with Axum/Actix-web |
| Web admin interface | Web | HTML/CSS/JS admin dashboard |
| API authentication | Code | JWT token-based auth |
| OpenAPI specification | Docs | Complete API documentation |

---

## Detailed Tasks

### Task Category 1: REST API

- [ ] **Task 1.1**: API framework setup (Axum)
  - Files affected: `crates/impulse-api/src/main.rs`
  - Estimated hours: 4

- [ ] **Task 1.2**: User management endpoints
  - Files affected: `crates/impulse-api/src/routes/users.rs`
  - Estimated hours: 8

- [ ] **Task 1.3**: File management endpoints
  - Files affected: `crates/impulse-api/src/routes/files.rs`
  - Estimated hours: 6

- [ ] **Task 1.4**: System stats endpoints
  - Files affected: `crates/impulse-api/src/routes/system.rs`
  - Estimated hours: 5

- [ ] **Task 1.5**: OpenAPI specification
  - Files affected: `docs/api/openapi.yaml`
  - Estimated hours: 6

### Task Category 2: Web UI

- [ ] **Task 2.1**: User management interface
  - Files affected: `web-admin/src/users.html`
  - Estimated hours: 10

- [ ] **Task 2.2**: System dashboard
  - Files affected: `web-admin/src/dashboard.html`
  - Estimated hours: 8

- [ ] **Task 2.3**: Configuration editor
  - Files affected: `web-admin/src/config.html`
  - Estimated hours: 8

### Task Category 3: Authentication

- [ ] **Task 3.1**: JWT implementation
  - Files affected: `crates/impulse-api/src/auth/jwt.rs`
  - Estimated hours: 6

- [ ] **Task 3.2**: RBAC for endpoints
  - Files affected: `crates/impulse-api/src/auth/rbac.rs`
  - Estimated hours: 5

---

## Acceptance Criteria

- [ ] API functional and documented
- [ ] Web UI allows basic admin tasks
- [ ] API secured with authentication
- [ ] OpenAPI spec complete

---

## Technical Details

### Architecture Considerations

- Use Axum web framework for REST API (Tokio-native, high performance)
- JWT-based authentication with refresh tokens
- RBAC middleware for endpoint authorization
- OpenAPI/Swagger specification with utoipa auto-generation
- CORS support for web UI integration
- Rate limiting for API abuse prevention
- WebSocket support for real-time updates (dashboard statistics)
- Serve static web UI from embedded assets
- Separate API server from BBS telnet server (different ports)

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
tokio = { workspace = true }
axum = { version = "0.7", features = ["ws", "macros"] }
tower = { version = "0.4", features = ["limit", "buffer", "timeout"] }
tower-http = { version = "0.5", features = ["fs", "cors", "trace"] }
serde = { workspace = true }
serde_json = { workspace = true }
sqlx = { workspace = true, features = ["postgres", "runtime-tokio-native-tls"] }
jsonwebtoken = "9.2"
bcrypt = "0.15"
utoipa = { version = "4.2", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "6.0", features = ["axum"] }
chrono = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }

[build-dependencies]
which = "5.0"
```

**Pascal Units Providing Data:**
- USERS.PAS (User management)
- CONFIG.PAS (System configuration)
- STATS.PAS (System statistics)
- FILES.PAS (File area management)

**Web UI Technologies:**
- HTML5 + vanilla JavaScript (no framework dependencies)
- CSS3 with responsive design
- Fetch API for REST communication
- Chart.js for dashboard visualizations

### Code Examples

**Axum REST API with OpenAPI:**
```rust
use axum::{
    Router,
    extract::{State, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

/// API state shared across all routes
#[derive(Clone)]
pub struct ApiState {
    pub db: PgPool,
    pub jwt_secret: String,
}

/// Main API router
pub fn create_router(state: ApiState) -> Router {
    Router::new()
        // User management routes
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user).put(update_user).delete(delete_user))

        // System statistics routes
        .route("/api/system/stats", get(get_system_stats))
        .route("/api/system/nodes", get(get_active_nodes))

        // File management routes
        .route("/api/files/areas", get(list_file_areas))
        .route("/api/files/:area_id", get(list_files_in_area))

        // Authentication routes
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh_token))

        // Serve OpenAPI spec with Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))

        // CORS configuration
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )

        // Shared state
        .with_state(Arc::new(state))
}

/// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        list_users,
        get_user,
        create_user,
        update_user,
        delete_user,
        get_system_stats,
        login,
    ),
    components(
        schemas(User, UserCreateRequest, UserUpdateRequest, SystemStats, LoginRequest, LoginResponse)
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "system", description = "System administration endpoints"),
        (name = "auth", description = "Authentication endpoints")
    ),
    info(
        title = "Impulse 7.1 BBS Admin API",
        version = "1.0.0",
        description = "RESTful API for administering Impulse 7.1 BBS"
    )
)]
struct ApiDoc;

/// User record
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub security_level: i16,
    pub location: String,
    pub phone: String,
    pub login_count: i32,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}

/// Request to create a new user
#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCreateRequest {
    pub name: String,
    pub password: String,
    pub security_level: i16,
    pub location: String,
    pub phone: String,
}

/// Request to update user details
#[derive(Debug, Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    pub security_level: Option<i16>,
    pub location: Option<String>,
    pub phone: Option<String>,
}

/// System statistics
#[derive(Debug, Serialize, ToSchema)]
pub struct SystemStats {
    pub total_users: i64,
    pub active_users_today: i64,
    pub total_messages: i64,
    pub messages_today: i64,
    pub total_files: i64,
    pub total_downloads: i64,
    pub active_nodes: i32,
    pub uptime_seconds: u64,
}

/// List all users with optional pagination
#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    params(
        ("offset" = Option<i64>, Query, description = "Pagination offset"),
        ("limit" = Option<i64>, Query, description = "Number of results (max 100)")
    ),
    responses(
        (status = 200, description = "List of users", body = Vec<User>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn list_users(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(50).min(100);

    let users = sqlx::query_as!(
        User,
        "SELECT id, name, security_level, location, phone, login_count, last_login, created_at
         FROM users
         ORDER BY id
         LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

#[derive(Debug, Deserialize)]
struct PaginationParams {
    offset: Option<i64>,
    limit: Option<i64>,
}

/// Get a single user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn get_user(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, security_level, location, phone, login_count, last_login, created_at
         FROM users
         WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/api/users",
    tag = "users",
    request_body = UserCreateRequest,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 400, description = "Invalid request"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn create_user(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<UserCreateRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    // Hash password with bcrypt
    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert user
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, password_hash, security_level, location, phone, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
         RETURNING id, name, security_level, location, phone, login_count, last_login, created_at",
        req.name,
        password_hash,
        req.security_level,
        req.location,
        req.phone
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") {
            StatusCode::CONFLICT
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// Update user details
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = UserUpdateRequest,
    responses(
        (status = 200, description = "User updated", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn update_user(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<i32>,
    Json(req): Json<UserUpdateRequest>,
) -> Result<Json<User>, StatusCode> {
    // Build dynamic UPDATE query based on provided fields
    let user = sqlx::query_as!(
        User,
        "UPDATE users
         SET security_level = COALESCE($2, security_level),
             location = COALESCE($3, location),
             phone = COALESCE($4, phone),
             updated_at = NOW()
         WHERE id = $1
         RETURNING id, name, security_level, location, phone, login_count, last_login, created_at",
        id,
        req.security_level,
        req.location,
        req.phone
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

/// Delete a user
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn delete_user(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

/// Get system statistics
#[utoipa::path(
    get,
    path = "/api/system/stats",
    tag = "system",
    responses(
        (status = 200, description = "System statistics", body = SystemStats),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn get_system_stats(
    State(state): State<Arc<ApiState>>,
) -> Result<Json<SystemStats>, StatusCode> {
    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let active_users_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM login_history WHERE DATE(login_time) = CURRENT_DATE"
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    let total_messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);

    let stats = SystemStats {
        total_users,
        active_users_today,
        total_messages,
        messages_today: 0, // TODO: Implement
        total_files: 0, // TODO: Implement
        total_downloads: 0, // TODO: Implement
        active_nodes: 0, // TODO: Implement from active sessions
        uptime_seconds: 0, // TODO: Track server start time
    };

    Ok(Json(stats))
}

async fn get_active_nodes() -> impl IntoResponse {
    // TODO: Implement node monitoring
    Json(vec![])
}

async fn list_file_areas() -> impl IntoResponse {
    // TODO: Implement file area listing
    Json(vec![])
}

async fn list_files_in_area() -> impl IntoResponse {
    // TODO: Implement file listing
    Json(vec![])
}
```

**JWT Authentication Middleware:**
```rust
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub role: String, // User role (admin, sysop, user)
    pub exp: usize,   // Expiration time (Unix timestamp)
    pub iat: usize,   // Issued at (Unix timestamp)
}

impl Claims {
    /// Create new claims for a user
    pub fn new(user_id: i32, role: &str, duration_hours: i64) -> Self {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::hours(duration_hours);

        Self {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        }
    }

    /// Check if token has expired
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as usize;
        self.exp < now
    }
}

/// Generate JWT token
pub fn generate_token(claims: &Claims, secret: &str) -> anyhow::Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Validate and decode JWT token
pub fn validate_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

/// Authentication middleware
pub async fn auth_middleware(
    State(state): State<Arc<ApiState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extract Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate token
    let claims = validate_token(token, &state.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if expired
    if claims.is_expired() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Add claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// RBAC middleware - require specific role
pub fn require_role(required_role: &'static str) -> impl Fn(Request, Next) -> Result<Response, StatusCode> {
    move |request: Request, next: Next| async move {
        // Extract claims from request extensions
        let claims = request
            .extensions()
            .get::<Claims>()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Check if user has required role
        if claims.role != required_role && claims.role != "admin" {
            return Err(StatusCode::FORBIDDEN);
        }

        Ok(next.run(request).await)
    }
}

/// Login request
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// Login endpoint
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Query user by username
    let user: Option<(i32, String, i16)> = sqlx::query_as(
        "SELECT id, password_hash, security_level FROM users WHERE name = $1"
    )
    .bind(&req.username)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (user_id, password_hash, security_level) = user.ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let valid = bcrypt::verify(&req.password, &password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Determine role based on security level
    let role = if security_level >= 255 {
        "admin"
    } else if security_level >= 100 {
        "sysop"
    } else {
        "user"
    };

    // Generate access token (1 hour)
    let claims = Claims::new(user_id, role, 1);
    let token = generate_token(&claims, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Generate refresh token (7 days)
    let refresh_claims = Claims::new(user_id, role, 24 * 7);
    let refresh_token = generate_token(&refresh_claims, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        refresh_token,
        expires_in: 3600,
    }))
}

async fn refresh_token() -> impl IntoResponse {
    // TODO: Implement refresh token logic
    StatusCode::NOT_IMPLEMENTED
}
```

**Web Admin Dashboard (HTML/CSS/JS):**
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Impulse 7.1 BBS - Admin Dashboard</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: #1a1a2e;
            color: #eee;
        }

        .container {
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }

        header {
            background: #16213e;
            padding: 20px;
            border-radius: 10px;
            margin-bottom: 30px;
        }

        header h1 {
            color: #00d9ff;
            font-size: 28px;
        }

        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }

        .stat-card {
            background: #16213e;
            padding: 25px;
            border-radius: 10px;
            border-left: 4px solid #00d9ff;
        }

        .stat-card h3 {
            color: #aaa;
            font-size: 14px;
            font-weight: normal;
            margin-bottom: 10px;
        }

        .stat-card .value {
            font-size: 36px;
            font-weight: bold;
            color: #00d9ff;
        }

        .section {
            background: #16213e;
            padding: 25px;
            border-radius: 10px;
            margin-bottom: 20px;
        }

        .section h2 {
            color: #00d9ff;
            margin-bottom: 20px;
            font-size: 22px;
        }

        table {
            width: 100%;
            border-collapse: collapse;
        }

        table thead {
            background: #0f3460;
        }

        table th {
            padding: 15px;
            text-align: left;
            color: #00d9ff;
            font-weight: 600;
        }

        table td {
            padding: 15px;
            border-top: 1px solid #2a2a4a;
        }

        table tbody tr:hover {
            background: #1a1a3a;
        }

        button {
            background: #00d9ff;
            color: #1a1a2e;
            border: none;
            padding: 10px 20px;
            border-radius: 5px;
            cursor: pointer;
            font-weight: 600;
        }

        button:hover {
            background: #00b8d9;
        }

        .btn-danger {
            background: #ff4757;
        }

        .btn-danger:hover {
            background: #ee5a6f;
        }

        .loading {
            text-align: center;
            padding: 40px;
            color: #aaa;
        }

        .error {
            background: #ff4757;
            color: white;
            padding: 15px;
            border-radius: 5px;
            margin-bottom: 20px;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Impulse 7.1 BBS - Admin Dashboard</h1>
        </header>

        <div id="error-container"></div>

        <div class="stats-grid">
            <div class="stat-card">
                <h3>Total Users</h3>
                <div class="value" id="stat-total-users">-</div>
            </div>
            <div class="stat-card">
                <h3>Active Today</h3>
                <div class="value" id="stat-active-today">-</div>
            </div>
            <div class="stat-card">
                <h3>Total Messages</h3>
                <div class="value" id="stat-total-messages">-</div>
            </div>
            <div class="stat-card">
                <h3>Active Nodes</h3>
                <div class="value" id="stat-active-nodes">-</div>
            </div>
        </div>

        <div class="section">
            <h2>Recent Users</h2>
            <div id="users-container">
                <div class="loading">Loading users...</div>
            </div>
        </div>
    </div>

    <script>
        const API_BASE = 'http://localhost:3000';
        const TOKEN_KEY = 'impulse_admin_token';

        // Get auth token from localStorage
        function getToken() {
            return localStorage.getItem(TOKEN_KEY);
        }

        // Set auth token
        function setToken(token) {
            localStorage.setItem(TOKEN_KEY, token);
        }

        // API request helper
        async function apiRequest(endpoint, options = {}) {
            const token = getToken();

            const headers = {
                'Content-Type': 'application/json',
                ...options.headers,
            };

            if (token) {
                headers['Authorization'] = `Bearer ${token}`;
            }

            const response = await fetch(`${API_BASE}${endpoint}`, {
                ...options,
                headers,
            });

            if (response.status === 401) {
                // Token expired - redirect to login
                window.location.href = '/login.html';
                return;
            }

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            return response.json();
        }

        // Show error message
        function showError(message) {
            const container = document.getElementById('error-container');
            container.innerHTML = `<div class="error">${message}</div>`;
            setTimeout(() => {
                container.innerHTML = '';
            }, 5000);
        }

        // Load system statistics
        async function loadStats() {
            try {
                const stats = await apiRequest('/api/system/stats');

                document.getElementById('stat-total-users').textContent = stats.total_users.toLocaleString();
                document.getElementById('stat-active-today').textContent = stats.active_users_today.toLocaleString();
                document.getElementById('stat-total-messages').textContent = stats.total_messages.toLocaleString();
                document.getElementById('stat-active-nodes').textContent = stats.active_nodes.toLocaleString();
            } catch (error) {
                console.error('Failed to load stats:', error);
                showError('Failed to load system statistics');
            }
        }

        // Load recent users
        async function loadUsers() {
            try {
                const users = await apiRequest('/api/users?limit=10');

                const container = document.getElementById('users-container');

                if (users.length === 0) {
                    container.innerHTML = '<p>No users found</p>';
                    return;
                }

                let html = '<table>';
                html += '<thead><tr>';
                html += '<th>ID</th><th>Name</th><th>Security</th><th>Location</th><th>Logins</th><th>Last Login</th><th>Actions</th>';
                html += '</tr></thead><tbody>';

                users.forEach(user => {
                    const lastLogin = user.last_login
                        ? new Date(user.last_login).toLocaleString()
                        : 'Never';

                    html += '<tr>';
                    html += `<td>${user.id}</td>`;
                    html += `<td>${user.name}</td>`;
                    html += `<td>${user.security_level}</td>`;
                    html += `<td>${user.location}</td>`;
                    html += `<td>${user.login_count}</td>`;
                    html += `<td>${lastLogin}</td>`;
                    html += `<td>
                        <button onclick="editUser(${user.id})">Edit</button>
                        <button class="btn-danger" onclick="deleteUser(${user.id}, '${user.name}')">Delete</button>
                    </td>`;
                    html += '</tr>';
                });

                html += '</tbody></table>';
                container.innerHTML = html;
            } catch (error) {
                console.error('Failed to load users:', error);
                showError('Failed to load users');
            }
        }

        // Edit user
        function editUser(userId) {
            // TODO: Implement user edit modal
            alert(`Edit user ${userId}`);
        }

        // Delete user
        async function deleteUser(userId, userName) {
            if (!confirm(`Delete user "${userName}"?`)) {
                return;
            }

            try {
                await apiRequest(`/api/users/${userId}`, {
                    method: 'DELETE',
                });

                // Reload users table
                loadUsers();
                loadStats();
            } catch (error) {
                console.error('Failed to delete user:', error);
                showError('Failed to delete user');
            }
        }

        // Initialize dashboard
        async function init() {
            // Check if logged in
            if (!getToken()) {
                window.location.href = '/login.html';
                return;
            }

            // Load data
            await Promise.all([
                loadStats(),
                loadUsers(),
            ]);

            // Refresh stats every 30 seconds
            setInterval(loadStats, 30000);
        }

        // Start when DOM is ready
        document.addEventListener('DOMContentLoaded', init);
    </script>
</body>
</html>
```

**WebSocket Real-Time Updates:**
```rust
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade, Message},
        State,
    },
    response::IntoResponse,
};
use futures::{StreamExt, SinkExt};
use std::sync::Arc;
use tokio::sync::broadcast;

/// WebSocket handler for real-time dashboard updates
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ApiState>) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to broadcast channel for system events
    let mut rx = state.event_broadcast.subscribe();

    // Spawn task to send events to client
    let send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let json = serde_json::to_string(&event).unwrap();

            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from client
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Handle client requests
                println!("Received: {}", text);
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

/// System event types
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum SystemEvent {
    UserLogin { user_id: i32, username: String },
    UserLogout { user_id: i32 },
    MessagePosted { area_id: i32, message_id: i32 },
    FileUploaded { area_id: i32, filename: String },
    NodeStatusChange { node: i32, status: String },
}

/// Extended API state with event broadcasting
pub struct ApiStateWithEvents {
    pub db: PgPool,
    pub jwt_secret: String,
    pub event_broadcast: broadcast::Sender<SystemEvent>,
}

impl ApiStateWithEvents {
    /// Emit system event to all WebSocket clients
    pub fn emit_event(&self, event: SystemEvent) {
        let _ = self.event_broadcast.send(event);
    }
}
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 06**: User system provides data for user management API
- **Sprint 07**: Database layer provides data access for all endpoints
- **Sprint 18**: RBAC system integrates with API authentication

### Blocks Downstream
- **Sprint 30**: Beta testing includes API testing and web UI validation

---

## Testing Requirements

### Unit Tests
- [ ] JWT generation and validation
- [ ] Claims expiration checking
- [ ] Password hashing and verification
- [ ] API route handlers (users, system, files)

### Integration Tests
- [ ] Full API authentication flow (login → request with token)
- [ ] RBAC enforcement (admin vs user access)
- [ ] CORS configuration
- [ ] OpenAPI spec generation
- [ ] WebSocket connection and event broadcasting

### API Tests
- [ ] GET /api/users (pagination, filtering)
- [ ] POST /api/users (create with validation)
- [ ] PUT /api/users/:id (partial updates)
- [ ] DELETE /api/users/:id
- [ ] GET /api/system/stats
- [ ] POST /api/auth/login (valid/invalid credentials)

### Security Tests
- [ ] JWT token expiration enforcement
- [ ] Invalid token rejection
- [ ] Missing token rejection (401 Unauthorized)
- [ ] Role-based access control (403 Forbidden)
- [ ] SQL injection prevention
- [ ] XSS protection in web UI

### Performance Tests
- [ ] API response times < 100ms
- [ ] Concurrent requests (100+ users)
- [ ] WebSocket connection stability
- [ ] Rate limiting effectiveness

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use Axum web framework (Tokio-native, high performance)
- JWT-based authentication (stateless, scalable)
- OpenAPI/Swagger with utoipa (auto-generated from code)
- Vanilla JavaScript for web UI (no framework dependencies)
- WebSocket for real-time dashboard updates
- Separate API server port from BBS telnet port
- CORS enabled for cross-origin requests
- Rate limiting to prevent API abuse
- Serve static web UI from embedded assets (include_dir!)

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: API security vulnerabilities (injection, XSS)
- **Mitigation**: Use sqlx query macros; validate all inputs; enable CORS selectively
- **Risk**: JWT token theft
- **Mitigation**: Short expiration times (1 hour); refresh tokens; HTTPS only in production
- **Risk**: Web UI complexity
- **Mitigation**: Start with vanilla JS; add framework later if needed
- **Risk**: API breaking changes
- **Mitigation**: Version API endpoints (/api/v1/users); maintain OpenAPI spec
- **Risk**: WebSocket scalability
- **Mitigation**: Use broadcast channels; limit message frequency; add reconnection logic

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
