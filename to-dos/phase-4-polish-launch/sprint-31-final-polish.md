# Sprint 31: Final Polish & Packaging

**Phase:** Phase 4 - Polish & Launch
**Duration:** 3 weeks
**Sprint Dates:** TBD
**Status:** Not Started

---

## Sprint Overview

Sprint 31 performs final code cleanup, creates release binaries for all platforms, packages installers, and prepares all release materials including release notes and press kit.

**Context:** Sprint 7 of Phase 4. Final preparation for 1.0 release.

**Expected Outcomes:** Production-ready binaries and complete release materials.

---

## Objectives

- [ ] Final code cleanup and polish
- [ ] Build release binaries for Linux, Windows, macOS
- [ ] Create platform installers (DEB, RPM, MSI)
- [ ] Prepare release notes and announcement materials

---

## Deliverables

| Deliverable | Type | Acceptance Criteria |
|-------------|------|---------------------|
| Release binaries | Binaries | Linux, Windows, macOS tested |
| Platform installers | Packages | DEB, RPM, MSI, DMG |
| Docker images | Images | Official tagged images on Docker Hub |
| Release materials | Docs | Notes, blog post, screenshots |

---

## Detailed Tasks

### Task Category 1: Code Cleanup

- [ ] **Task 1.1**: Remove dead code and debug logs
  - Estimated hours: 6

- [ ] **Task 1.2**: Final clippy and fmt pass
  - Estimated hours: 3

- [ ] **Task 1.3**: Update all dependencies
  - Estimated hours: 4

- [ ] **Task 1.4**: Security audit final review
  - Estimated hours: 4

### Task Category 2: Binary Packaging

- [ ] **Task 2.1**: Build Linux binaries (x86_64, ARM64)
  - Files affected: Build scripts
  - Estimated hours: 4

- [ ] **Task 2.2**: Build Windows binaries
  - Files affected: Build scripts
  - Estimated hours: 4

- [ ] **Task 2.3**: Build macOS binaries (Intel, Apple Silicon)
  - Files affected: Build scripts
  - Estimated hours: 4

- [ ] **Task 2.4**: Create DEB packages
  - Files affected: `packaging/debian/`
  - Estimated hours: 5

- [ ] **Task 2.5**: Create RPM packages
  - Files affected: `packaging/rpm/`
  - Estimated hours: 5

- [ ] **Task 2.6**: Create MSI installer
  - Files affected: `packaging/windows/`
  - Estimated hours: 6

- [ ] **Task 2.7**: Sign binaries
  - Estimated hours: 3

### Task Category 3: Docker Images

- [ ] **Task 3.1**: Build multi-arch Docker images
  - Files affected: `Dockerfile`
  - Estimated hours: 4

- [ ] **Task 3.2**: Tag and push to Docker Hub
  - Estimated hours: 2

- [ ] **Task 3.3**: Docker Compose examples
  - Files affected: `docker-compose.yml`
  - Estimated hours: 3

### Task Category 4: Release Materials

- [ ] **Task 4.1**: Write release notes
  - Files affected: `RELEASE-NOTES-1.0.md`
  - Estimated hours: 4

- [ ] **Task 4.2**: Create announcement blog post
  - Files affected: Blog post draft
  - Estimated hours: 4

- [ ] **Task 4.3**: Prepare screenshots and demos
  - Files affected: Media files
  - Estimated hours: 4

- [ ] **Task 4.4**: Update website
  - Files affected: Project website
  - Estimated hours: 5

---

## Acceptance Criteria

- [ ] All platforms have tested binaries
- [ ] Installers work correctly
- [ ] Docker images functional
- [ ] Release notes comprehensive
- [ ] Materials ready for launch

---

## Technical Details

### Architecture Considerations

- Implement automated code quality checks (dead code, unused deps)
- Configure cross-platform Rust builds with cargo-dist
- Generate native installers for each platform (DEB, RPM, MSI, DMG)
- Create optimized Docker images with multi-stage builds
- Use GitHub Actions for automated release builds
- Sign binaries with platform-specific signing tools
- Generate SBOM (Software Bill of Materials) for security
- Implement versioning automation with cargo-release
- Create comprehensive release notes from git commits
- Optimize binary size with LTO and stripping symbols

### Dependencies

**Crate-Level Dependencies:**
```toml
[dependencies]
# (All production dependencies already defined in previous sprints)

[build-dependencies]
built = "0.7"  # Build-time information
shadow-rs = "0.26"  # Build metadata embedding

[dev-dependencies]
# (All test dependencies already defined)

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"

[profile.release-with-debug]
inherits = "release"
strip = false
debug = true
```

**Build Tools:**
- `cargo-dist` - Cross-platform distribution
- `cargo-release` - Release automation
- `cargo-audit` - Security vulnerability scanning
- `cargo-outdated` - Dependency update checking
- `cargo-udeps` - Unused dependency detection
- `cargo-bloat` - Binary size analysis
- Platform-specific: `dpkg-deb`, `rpmbuild`, `WiX Toolset`, `create-dmg`

**External Dependencies:**
- Docker buildx (multi-architecture builds)
- GitHub CLI (release automation)
- Platform signing tools (GPG, osslsigncode, codesign)

### Code Examples

**Automated Code Quality Checks:**
```rust
// build.rs - Build-time quality checks and metadata generation
use std::process::{Command, Output};
use std::env;

fn main() {
    // Generate build-time metadata
    generate_build_metadata();

    // Run quality checks in development
    if env::var("PROFILE").unwrap_or_default() == "debug" {
        run_quality_checks();
    }

    // Platform-specific configuration
    configure_platform_features();
}

fn generate_build_metadata() {
    // Use built crate for build info
    built::write_built_file()
        .expect("Failed to acquire build-time information");

    // Use shadow-rs for git info
    shadow_rs::new().expect("Failed to generate shadow-rs build info");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
}

fn run_quality_checks() {
    println!("cargo:warning=Running quality checks...");

    // Check for dead code
    if let Err(e) = check_dead_code() {
        println!("cargo:warning=Dead code check failed: {}", e);
    }

    // Check for unused dependencies
    if let Err(e) = check_unused_deps() {
        println!("cargo:warning=Unused dependency check failed: {}", e);
    }

    // Check for outdated dependencies
    if let Err(e) = check_outdated_deps() {
        println!("cargo:warning=Outdated dependency check: {}", e);
    }
}

fn check_dead_code() -> Result<(), String> {
    let output = Command::new("cargo")
        .args(&["rustc", "--", "-W", "dead_code"])
        .output()
        .map_err(|e| format!("Failed to run dead code check: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}

fn check_unused_deps() -> Result<(), String> {
    // Check if cargo-udeps is installed
    let check = Command::new("cargo")
        .args(&["udeps", "--version"])
        .output();

    if check.is_err() {
        return Ok(()); // Skip if not installed
    }

    let output = Command::new("cargo")
        .args(&["udeps", "--all-targets"])
        .output()
        .map_err(|e| format!("Failed to run unused deps check: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("unused dependencies") {
            return Err(stderr.to_string());
        }
    }

    Ok(())
}

fn check_outdated_deps() -> Result<(), String> {
    let check = Command::new("cargo")
        .args(&["outdated", "--version"])
        .output();

    if check.is_err() {
        return Ok(()); // Skip if not installed
    }

    let output = Command::new("cargo")
        .args(&["outdated", "--exit-code", "1"])
        .output()
        .map_err(|e| format!("Failed to run outdated check: {}", e))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("Outdated dependencies found:\n{}", stdout));
    }

    Ok(())
}

fn configure_platform_features() {
    let target = env::var("TARGET").unwrap_or_default();

    // Platform-specific features
    if target.contains("windows") {
        println!("cargo:rustc-cfg=platform_windows");
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=bcrypt");
    } else if target.contains("darwin") {
        println!("cargo:rustc-cfg=platform_macos");
        println!("cargo:rustc-link-lib=framework=Security");
    } else if target.contains("linux") {
        println!("cargo:rustc-cfg=platform_linux");
    }

    // Architecture-specific features
    if target.contains("x86_64") {
        println!("cargo:rustc-cfg=arch_x86_64");
    } else if target.contains("aarch64") {
        println!("cargo:rustc-cfg=arch_aarch64");
    }
}

// src/build_info.rs - Build information module
pub mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
    include!(concat!(env!("OUT_DIR"), "/shadow.rs"));

    use std::fmt;

    pub struct BuildInfo;

    impl BuildInfo {
        pub fn version() -> &'static str {
            PKG_VERSION
        }

        pub fn git_commit() -> &'static str {
            COMMIT_HASH
        }

        pub fn git_branch() -> &'static str {
            BRANCH
        }

        pub fn build_timestamp() -> &'static str {
            BUILD_TIME
        }

        pub fn rust_version() -> &'static str {
            RUST_VERSION
        }

        pub fn target_triple() -> &'static str {
            TARGET
        }

        pub fn profile() -> &'static str {
            PROFILE
        }

        pub fn full_version() -> String {
            format!(
                "{} ({}@{} built {} with rustc {})",
                Self::version(),
                Self::git_branch(),
                &Self::git_commit()[..7],
                Self::build_timestamp(),
                Self::rust_version()
            )
        }
    }

    impl fmt::Display for BuildInfo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", Self::full_version())
        }
    }
}

// CLI version command integration
pub fn print_version_info() {
    use crate::build_info::BuildInfo;

    println!("Impulse 7.1 BBS {}", BuildInfo::full_version());
    println!();
    println!("Build Information:");
    println!("  Target:       {}", BuildInfo::target_triple());
    println!("  Profile:      {}", BuildInfo::profile());
    println!("  Git Branch:   {}", BuildInfo::git_branch());
    println!("  Git Commit:   {}", BuildInfo::git_commit());
    println!("  Build Time:   {}", BuildInfo::build_timestamp());
    println!("  Rust Version: {}", BuildInfo::rust_version());
}
```

**Cross-Platform Build Configuration:**
```rust
// .cargo/config.toml
[build]
# Target-specific configurations
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-unknown-linux-musl]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld", "-C", "target-feature=+crt-static"]

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-mmacosx-version-min=10.15"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-mmacosx-version-min=11.0"]

// scripts/build-release.sh
#!/bin/bash
set -euo pipefail

# Build release binaries for all supported platforms

VERSION="${1:-$(cargo pkgid | cut -d# -f2 | cut -d: -f2)}"
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
    "x86_64-pc-windows-msvc"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

BUILD_DIR="target/release-artifacts"
mkdir -p "$BUILD_DIR"

echo "Building Impulse 7.1 BBS v${VERSION}"
echo "======================================="

for target in "${TARGETS[@]}"; do
    echo ""
    echo "Building for $target..."

    # Install target if not already installed
    rustup target add "$target" 2>/dev/null || true

    # Build
    cargo build --release --target "$target" \
        --features "production" \
        --no-default-features

    # Copy binary
    BINARY_NAME="impulse-bbs"
    if [[ "$target" == *"windows"* ]]; then
        BINARY_NAME="impulse-bbs.exe"
    fi

    ARTIFACT_NAME="impulse-bbs-${VERSION}-${target}"
    cp "target/${target}/release/${BINARY_NAME}" \
       "${BUILD_DIR}/${ARTIFACT_NAME}"

    # Strip symbols for size optimization (except Windows)
    if [[ "$target" != *"windows"* ]]; then
        strip "${BUILD_DIR}/${ARTIFACT_NAME}" 2>/dev/null || true
    fi

    # Create tarball
    if [[ "$target" == *"windows"* ]]; then
        (cd "$BUILD_DIR" && zip -q "${ARTIFACT_NAME}.zip" "${ARTIFACT_NAME}")
        rm "${BUILD_DIR}/${ARTIFACT_NAME}"
    else
        (cd "$BUILD_DIR" && tar -czf "${ARTIFACT_NAME}.tar.gz" "${ARTIFACT_NAME}")
        rm "${BUILD_DIR}/${ARTIFACT_NAME}"
    fi

    echo "✓ Built ${ARTIFACT_NAME}"
done

echo ""
echo "Build complete! Artifacts in ${BUILD_DIR}"
ls -lh "$BUILD_DIR"

// Cargo.toml feature configuration
[features]
default = ["full"]
full = ["terminal-ui", "web-admin", "door-games", "file-transfer"]

# Core features
terminal-ui = ["crossterm", "ratatui"]
web-admin = ["axum", "tower-http", "utoipa"]
door-games = ["dosbox-integration"]
file-transfer = ["xmodem", "ymodem", "zmodem"]

# Optional features
dosbox-integration = []
production = ["optimizations"]
optimizations = []

# Platform-specific features
unix = []
windows = []
macos = []
```

**Package Generation Scripts:**
```bash
#!/bin/bash
# scripts/package-deb.sh - Generate Debian package

set -euo pipefail

VERSION="${1:-1.0.0}"
ARCH="${2:-amd64}"
PACKAGE_NAME="impulse-bbs"
BUILD_DIR="packaging/deb/${PACKAGE_NAME}_${VERSION}_${ARCH}"

echo "Creating Debian package for ${PACKAGE_NAME} v${VERSION} (${ARCH})"

# Clean and create directory structure
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/DEBIAN"
mkdir -p "$BUILD_DIR/usr/bin"
mkdir -p "$BUILD_DIR/usr/share/impulse-bbs"
mkdir -p "$BUILD_DIR/usr/share/doc/impulse-bbs"
mkdir -p "$BUILD_DIR/etc/impulse-bbs"
mkdir -p "$BUILD_DIR/lib/systemd/system"

# Copy binary
cp "target/release/impulse-bbs" "$BUILD_DIR/usr/bin/"
chmod 755 "$BUILD_DIR/usr/bin/impulse-bbs"

# Copy configuration files
cp config/default.toml "$BUILD_DIR/etc/impulse-bbs/config.toml"
chmod 644 "$BUILD_DIR/etc/impulse-bbs/config.toml"

# Copy documentation
cp README.md "$BUILD_DIR/usr/share/doc/impulse-bbs/"
cp LICENSE "$BUILD_DIR/usr/share/doc/impulse-bbs/"
cp CHANGELOG.md "$BUILD_DIR/usr/share/doc/impulse-bbs/"
gzip -9 -n "$BUILD_DIR/usr/share/doc/impulse-bbs/CHANGELOG.md"

# Create systemd service file
cat > "$BUILD_DIR/lib/systemd/system/impulse-bbs.service" <<'EOF'
[Unit]
Description=Impulse 7.1 BBS Server
After=network.target postgresql.service

[Service]
Type=simple
User=impulse
Group=impulse
WorkingDirectory=/var/lib/impulse-bbs
ExecStart=/usr/bin/impulse-bbs --config /etc/impulse-bbs/config.toml
Restart=on-failure
RestartSec=5s

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/impulse-bbs
ReadWritePaths=/var/log/impulse-bbs

[Install]
WantedBy=multi-user.target
EOF

# Create control file
cat > "$BUILD_DIR/DEBIAN/control" <<EOF
Package: ${PACKAGE_NAME}
Version: ${VERSION}
Section: net
Priority: optional
Architecture: ${ARCH}
Depends: libc6 (>= 2.31), libssl3 (>= 3.0.0), postgresql-client (>= 12)
Maintainer: Impulse BBS Team <impulse@example.com>
Description: Impulse 7.1 BBS - Modern Rust-based Bulletin Board System
 Impulse 7.1 BBS is a modernized version of the classic Impulse BBS software,
 rewritten in Rust for performance, security, and reliability.
 .
 Features include:
  - Multi-node support with PostgreSQL backend
  - ANSI/ASCII terminal emulation
  - Door game integration via DOSBox
  - File transfer protocols (XMODEM, YMODEM, ZMODEM)
  - Web-based administration interface
  - Modern authentication and security
Homepage: https://github.com/yourusername/impulse-7.1
EOF

# Create postinst script
cat > "$BUILD_DIR/DEBIAN/postinst" <<'EOF'
#!/bin/bash
set -e

# Create impulse user and group
if ! getent group impulse >/dev/null; then
    addgroup --system impulse
fi

if ! getent passwd impulse >/dev/null; then
    adduser --system --ingroup impulse --home /var/lib/impulse-bbs impulse
fi

# Create directories
mkdir -p /var/lib/impulse-bbs
mkdir -p /var/log/impulse-bbs
chown impulse:impulse /var/lib/impulse-bbs
chown impulse:impulse /var/log/impulse-bbs
chmod 755 /var/lib/impulse-bbs
chmod 755 /var/log/impulse-bbs

# Reload systemd
systemctl daemon-reload

echo ""
echo "Impulse 7.1 BBS has been installed!"
echo ""
echo "Next steps:"
echo "  1. Edit configuration: /etc/impulse-bbs/config.toml"
echo "  2. Initialize database: impulse-bbs init-db"
echo "  3. Start service: systemctl start impulse-bbs"
echo "  4. Enable on boot: systemctl enable impulse-bbs"
echo ""
EOF

chmod 755 "$BUILD_DIR/DEBIAN/postinst"

# Create postrm script
cat > "$BUILD_DIR/DEBIAN/postrm" <<'EOF'
#!/bin/bash
set -e

if [ "$1" = "purge" ]; then
    # Remove user and group
    userdel impulse 2>/dev/null || true
    groupdel impulse 2>/dev/null || true

    # Remove data directories
    rm -rf /var/lib/impulse-bbs
    rm -rf /var/log/impulse-bbs
fi
EOF

chmod 755 "$BUILD_DIR/DEBIAN/postrm"

# Build package
dpkg-deb --build --root-owner-group "$BUILD_DIR"

echo "✓ Debian package created: ${BUILD_DIR}.deb"

# Verify package
echo ""
echo "Package information:"
dpkg-deb --info "${BUILD_DIR}.deb"

# scripts/package-rpm.sh - Generate RPM package
#!/bin/bash
set -euo pipefail

VERSION="${1:-1.0.0}"
ARCH="${2:-x86_64}"
PACKAGE_NAME="impulse-bbs"
RELEASE="1"

echo "Creating RPM package for ${PACKAGE_NAME} v${VERSION} (${ARCH})"

# Setup RPM build environment
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Create spec file
cat > ~/rpmbuild/SPECS/impulse-bbs.spec <<EOF
Name:           ${PACKAGE_NAME}
Version:        ${VERSION}
Release:        ${RELEASE}%{?dist}
Summary:        Impulse 7.1 BBS - Modern Rust-based Bulletin Board System

License:        MIT
URL:            https://github.com/yourusername/impulse-7.1
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  systemd-rpm-macros
Requires:       glibc >= 2.31, openssl >= 3.0.0, postgresql >= 12

%description
Impulse 7.1 BBS is a modernized version of the classic Impulse BBS software,
rewritten in Rust for performance, security, and reliability.

%prep
%setup -q

%install
rm -rf \$RPM_BUILD_ROOT
mkdir -p \$RPM_BUILD_ROOT/usr/bin
mkdir -p \$RPM_BUILD_ROOT/etc/impulse-bbs
mkdir -p \$RPM_BUILD_ROOT/usr/lib/systemd/system
mkdir -p \$RPM_BUILD_ROOT/var/lib/impulse-bbs
mkdir -p \$RPM_BUILD_ROOT/var/log/impulse-bbs

install -m 755 target/release/impulse-bbs \$RPM_BUILD_ROOT/usr/bin/
install -m 644 config/default.toml \$RPM_BUILD_ROOT/etc/impulse-bbs/config.toml
install -m 644 packaging/impulse-bbs.service \$RPM_BUILD_ROOT/usr/lib/systemd/system/

%files
%defattr(-,root,root,-)
/usr/bin/impulse-bbs
%config(noreplace) /etc/impulse-bbs/config.toml
/usr/lib/systemd/system/impulse-bbs.service
%dir %attr(755,impulse,impulse) /var/lib/impulse-bbs
%dir %attr(755,impulse,impulse) /var/log/impulse-bbs

%pre
getent group impulse >/dev/null || groupadd -r impulse
getent passwd impulse >/dev/null || useradd -r -g impulse -d /var/lib/impulse-bbs -s /sbin/nologin impulse

%post
%systemd_post impulse-bbs.service

%preun
%systemd_preun impulse-bbs.service

%postun
%systemd_postun_with_restart impulse-bbs.service

%changelog
* $(date '+%a %b %d %Y') Impulse BBS Team <impulse@example.com> - ${VERSION}-${RELEASE}
- Release ${VERSION}
EOF

# Build RPM
rpmbuild -bb ~/rpmbuild/SPECS/impulse-bbs.spec

echo "✓ RPM package created in ~/rpmbuild/RPMS/${ARCH}/"
```

**Docker Multi-Architecture Build:**
```dockerfile
# Dockerfile - Multi-stage optimized build

# Stage 1: Build stage
FROM rust:1.75-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build dependencies (cached layer)
RUN cargo build --release --locked

# Copy source code
COPY src/ ./src/
COPY config/ ./config/

# Build application
RUN cargo build --release --locked --bin impulse-bbs

# Strip binary for size optimization
RUN strip target/release/impulse-bbs

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r impulse && useradd -r -g impulse impulse

# Create directories
RUN mkdir -p /var/lib/impulse-bbs /var/log/impulse-bbs /etc/impulse-bbs \
    && chown -R impulse:impulse /var/lib/impulse-bbs /var/log/impulse-bbs

# Copy binary from builder
COPY --from=builder /build/target/release/impulse-bbs /usr/local/bin/

# Copy default configuration
COPY --from=builder /build/config/default.toml /etc/impulse-bbs/config.toml

# Set user
USER impulse

# Expose ports
EXPOSE 23 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD /usr/local/bin/impulse-bbs healthcheck || exit 1

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/impulse-bbs"]
CMD ["--config", "/etc/impulse-bbs/config.toml"]

# Metadata labels
LABEL org.opencontainers.image.title="Impulse 7.1 BBS"
LABEL org.opencontainers.image.description="Modern Rust-based Bulletin Board System"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.vendor="Impulse BBS Team"
LABEL org.opencontainers.image.licenses="MIT"
```

```bash
#!/bin/bash
# scripts/build-docker.sh - Multi-architecture Docker build

set -euo pipefail

VERSION="${1:-latest}"
REGISTRY="${2:-ghcr.io/yourusername}"
IMAGE_NAME="${REGISTRY}/impulse-bbs"

echo "Building Docker images for Impulse 7.1 BBS v${VERSION}"
echo "========================================================="

# Enable Docker buildx
docker buildx create --name impulse-builder --use 2>/dev/null || docker buildx use impulse-builder

# Build multi-architecture images
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    --tag "${IMAGE_NAME}:${VERSION}" \
    --tag "${IMAGE_NAME}:latest" \
    --build-arg VERSION="${VERSION}" \
    --build-arg BUILD_DATE="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
    --build-arg VCS_REF="$(git rev-parse --short HEAD)" \
    --push \
    .

echo "✓ Docker images pushed to ${IMAGE_NAME}"
echo ""
echo "Available tags:"
echo "  - ${IMAGE_NAME}:${VERSION}"
echo "  - ${IMAGE_NAME}:latest"
echo ""
echo "Pull with: docker pull ${IMAGE_NAME}:${VERSION}"
```

```yaml
# docker-compose.yml - Production deployment example
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: impulse_bbs
      POSTGRES_USER: impulse
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U impulse"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  bbs:
    image: ghcr.io/yourusername/impulse-bbs:latest
    depends_on:
      postgres:
        condition: service_healthy
    ports:
      - "23:23"     # Telnet
      - "8080:8080" # Web admin
    environment:
      DATABASE_URL: postgres://impulse:${DB_PASSWORD}@postgres/impulse_bbs
      RUST_LOG: info
      BBS_CONFIG_PATH: /etc/impulse-bbs/config.toml
    volumes:
      - bbs_data:/var/lib/impulse-bbs
      - bbs_logs:/var/log/impulse-bbs
      - ./config.toml:/etc/impulse-bbs/config.toml:ro
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "/usr/local/bin/impulse-bbs", "healthcheck"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

volumes:
  postgres_data:
  bbs_data:
  bbs_logs:
```

---

## Dependencies

### Upstream Dependencies
- **Sprint 30**: Beta testing validates final release quality
- **All previous sprints**: Complete BBS implementation

### Blocks Downstream
- **Sprint 32**: Launch requires all packaging complete

---

## Testing Requirements

### Build Tests
- [ ] Linux x86_64 binary builds successfully
- [ ] Linux ARM64 binary builds successfully
- [ ] Windows x86_64 binary builds successfully
- [ ] macOS Intel binary builds successfully
- [ ] macOS Apple Silicon binary builds successfully
- [ ] All binaries pass smoke tests

### Package Tests
- [ ] DEB package installs on Ubuntu 22.04
- [ ] DEB package installs on Debian 12
- [ ] RPM package installs on Fedora 39
- [ ] RPM package installs on RHEL 9
- [ ] MSI installer works on Windows 11
- [ ] DMG installer works on macOS 13+
- [ ] Systemd service starts correctly
- [ ] Configuration files in correct locations
- [ ] Uninstallation removes all files

### Docker Tests
- [ ] Docker image builds for amd64
- [ ] Docker image builds for arm64
- [ ] Docker container starts successfully
- [ ] Docker healthcheck passes
- [ ] Docker Compose deployment works
- [ ] PostgreSQL connection works
- [ ] Persistent volumes work correctly

### Quality Tests
- [ ] No clippy warnings with -D warnings
- [ ] cargo fmt --check passes
- [ ] cargo audit shows no vulnerabilities
- [ ] Binary size < 50MB
- [ ] Startup time < 5 seconds
- [ ] Memory usage < 100MB idle

---

## Notes and Decisions

### Design Decisions
*To be populated during sprint*

**Proposed Decisions:**
- Use cargo-dist for cross-platform binary distribution
- Sign binaries with GPG (Linux), osslsigncode (Windows), codesign (macOS)
- Generate SBOM (Software Bill of Materials) with cargo-sbom
- Use multi-stage Docker builds to minimize image size
- Publish Docker images to GitHub Container Registry
- Create both archive (.tar.gz/.zip) and installer packages
- Include systemd service files for Linux packages
- Optimize release binaries with LTO and symbol stripping
- Generate release notes from git conventional commits

### Lessons Learned
*To be populated during sprint*

### Risks and Mitigations
- **Risk**: Cross-compilation failures on different platforms
- **Mitigation**: Use GitHub Actions matrix builds, test on real hardware
- **Risk**: Binary signing certificate management
- **Mitigation**: Use GitHub Secrets, rotate certificates annually
- **Risk**: Package incompatibilities with older distributions
- **Mitigation**: Test on oldest supported versions, document requirements
- **Risk**: Docker image size bloat
- **Mitigation**: Multi-stage builds, minimal base images, strip binaries
- **Risk**: Release automation breaks
- **Mitigation**: Test release process in staging, manual fallback procedures

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
