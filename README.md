# Impulse 7.1 BBS - Rust Modernization

Modern BBS server implementation converting Impulse 7.1 from Borland Pascal to Rust.

## What is Impulse 7.1?

Impulse 7.1 is a classic BBS software that powered dial-up bulletin board systems during the height of the BBS era. It provided features like message boards, file areas, multi-node support, user management, and door game support.

More details: <https://web.archive.org/web/20011204010133/http://www.demonic.net/impulse/>

## Project Status

* **Rust Modernization (Active):** Phase 1, Sprint 1-2 complete. Full workspace infrastructure established with 16 crates.
* **Pascal Source (Legacy):** Stable. Builds with the provided `build.sh` script. Preserved for reference.

---

## Rust Workspace

### Quick Start

```bash
# Build all crates
cargo build --all

# Run tests
cargo test --all

# Run the BBS server
cargo run --bin impulse-server

# Build with optimizations
cargo build --all --release
```

### Development

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all -- -D warnings

# Generate documentation
cargo doc --workspace --no-deps --open
```

See `docs/04-development-guide.md` for detailed development instructions.

### Architecture

The project is organized as a Cargo workspace with 16 crates:

**Core Crates:**
- `impulse-core` - Core BBS logic and state management
- `impulse-types` - Shared data types and constants
- `impulse-config` - Configuration management

**Protocol Crates:**
- `impulse-protocol` - Protocol trait definitions
- `impulse-telnet` - Telnet protocol implementation
- `impulse-ssh` - SSH protocol implementation

**Feature Crates:**
- `impulse-session` - Session management
- `impulse-terminal` - Terminal I/O (crossterm integration)
- `impulse-auth` - Authentication (Argon2id)
- `impulse-message` - Message bases (JAM/Hudson)
- `impulse-file` - File areas and transfers
- `impulse-user` - User management
- `impulse-door` - Door game support

**Application Crates:**
- `impulse-web` - Web admin panel (Axum)
- `impulse-cli` - CLI tools
- `impulse-server` - Main server binary

See `docs/02-architecture.md` for complete architecture documentation.

---

## Legacy Pascal Build

## Repository Structure

```
├── source/          # Pascal source files
├── output/          # Compiled TPU/EXE files destination
├── include/         # Dependencies (only checkpat.tpu)
├── imp71rel/        # Complete official Impulse 7.1 release files
├── BP/              # Borland Pascal 7.0 compiler and tools
├── build/           # Build artifacts
├── newins/          # [WIP] New installer (haven't really started this)
├── build.sh         # Automated Linux build script (requires DOSBox)
├── .gitlab-ci.yml   # GitLab CI/CD pipeline
└── clean.sh         # Cleanup script
```

## Docker

not tested yet:  docker build -t impulse-bbs . && docker run --rm -v $(pwd)/build:/impulse-build/build impulse-bbs

## Building from Source

### Requirements

* **Linux**: DOSBox (for running DOS-based Borland Pascal compiler)
* **CI/CD**: Docker + DOSBox (handled automatically by GitLab CI)

### Quick Start

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   cd impulse-7.1
   ```

2. **Automated build (Linux + dosbox):**

   ```bash
   ./build.sh
   ```

3. Copy IMP.EXE and IMP.OVR from build dir, combine them into the full imp71rel release.  run IMP -D in dos on new install to initialize files/dirs

### Manual Compilation

To build in a DOS environment, you can use the Borland Pascal IDE (\BP\BIN\BP) or run from DOS commandline with \BP\BIN\BPC.  see BPC help on the -U and -E flags, i used them but to keep stuff in seprate dirs but they're not required

1. **Build step** (creates TPU units):

   ```
   bpc -$G+ -B -Uf:\ -Ee:\ imp.pas
   ```

   This generates TPU (Turbo Pascal Unit) files.

2. **Compile step** (creates executables):

   ```
   bpc -Uf:\ -Ee:\ imp.pas
   ```

   This creates `IMP.EXE` and `IMP.OVR`.

## CI/CD Pipeline

For fun, we setup GitLab CI/CD automation:

* **`.gitlab-ci.yml`** - Configures the GitLab CI/CD pipeline
* Uses Docker with DOSBox to provide a consistent build environment
* Automatically builds on commits and provides build artifacts

## Using the Compiled Software

After building, you'll find the compiled files in the `output/` directory:

* `IMP.EXE` - Main executable
* `IMP.OVR` - Overlay file

To run a complete BBS:

1. Copy the compiled files to the `imp71rel/` directory
2. The `imp71rel/` folder contains the complete original Impulse 7.1 (y2k bugs and all)
3. Configure the BBS according to the docs in `imp71rel/` (note IMP.EXE -D when run for first time)

docs:  check IMP.DOC and README

## Development Notes

* **Source Origin**: Started from `http://software.bbsdocumentary.com/IBM/DOS/IMPULSE/`
* **Compiler**: Uses Borland Pascal 7.0 (included in `BP/` directory)
* **Architecture**: Large codebase with 96+ Pascal units
* **Build System**: Modern shell scripts wrapping classic DOS tools via DOSBox

## Recent Changes

* Organized source code into Git repository structure
* Added automated build system for Linux
* Created CI/CD pipeline for consistent builds
* Fixed Y2K handling in date functions (see recent commits)

## Contributing

This is primarily a preservation project, but improvements are welcome:

* Bug fixes to the original code
* Build system improvements
* Documentation enhancements
* Modern development tooling

## License

This software is considered **abandonware** and is essentially **public domain**.

Credit goes to:

* Brandon Sneed (Nivenh): Original developer through Version 6
* Phillip Foose (Horrid): Further bugfixes and Version 7

---

*"We're figuring it out!"*
