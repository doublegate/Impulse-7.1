# Gemini Code Assistant Context: Impulse 7.1 BBS

This document provides context for Gemini or other AI assistants working on the Impulse 7.1 BBS project.

## Project Overview

Impulse 7.1 is a legacy DOS-based Bulletin Board System (BBS) written in **Borland Pascal 7.0**. The project aims to preserve this software, enable modern compilation (via DOSBox), and explore potential modernization paths (e.g., porting to Rust).

## Repository State

*   **Branch: `main`**: Contains the clean, original Pascal source code and build scripts. This is the stable base.
*   **Branch: `gemini_vOLD`**: Contains a **comprehensive but incomplete attempt to port the codebase to Rust**. This branch includes:
    *   A crate `impulse-rs` with mapped modules for almost all Pascal units.
    *   Significant progress on `common`, `file`, `init`, and `output` systems.
    *   **Known Issue:** The build in `gemini_vOLD` is currently broken due to persistent syntax errors in `common4.rs`, `file1.rs`, `file11.rs`, and `impdos.rs`. These files resisted automated fixes during the conversion session (Nov 2025). Future attempts should study this branch for logic but might consider a fresh start or manual intervention for the problematic files.

## Directory Structure

*   `source/`: Original Pascal source (`.PAS`).
*   `BP/`: Borland Pascal compiler tools.
*   `imp71rel/`: Runtime files for the BBS.
*   `build.sh`: Script to compile the Pascal code using DOSBox.

## Operational Guidelines

### 1. Working with Pascal
*   The codebase is 1990s Pascal. Respect the naming conventions and style.
*   Use `build.sh` to verify changes.
*   Edits to `.PAS` files require understanding Borland Pascal specific features (Overlays, specific units).

### 2. Rust Porting (Future)
*   If restarting or continuing the port, refer to `gemini_vOLD` for data structure mappings (`records.rs`) and logic translation.
*   **Lesson Learned:** Automated mass-conversion can lead to subtle, persistent syntax errors that are hard to fix via tool calls if the environment desyncs. Incremental, compile-tested conversion of single units is recommended over batch conversion.

## Key Files
*   `source/IMP.PAS`: The main entry point.
*   `source/COMMON.PAS`: Global variables and core types.
*   `source/RECORDS.PAS`: Data structure definitions.
