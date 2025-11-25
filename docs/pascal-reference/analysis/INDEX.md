# Pascal Source Analysis

Deep analysis of the original Pascal codebase structure, dependencies, and characteristics.

**Last Updated:** 2025-11-24

---

## Overview

This directory contains detailed analysis of the 96 Pascal units that comprise Impulse 7.1, including dependency graphs, global state analysis, overlay system documentation, and DOS-specific functionality.

---

## Documentation Files

### [pascal-inventory.md](pascal-inventory.md)

**Complete inventory of 96 Pascal units**

Comprehensive listing of all Pascal source files with:
- File names and sizes
- Primary functionality
- Category classification
- Dependencies count
- Complexity assessment

**Unit Categories:**
- Core System (12 units)
- Communications (8 units)
- Message System (15 units)
- File Management (18 units)
- User Interface (22 units)
- Door Support (6 units)
- Utilities (15 units)

### [pascal-unit-analysis.md](pascal-unit-analysis.md)

**Per-unit detailed analysis**

In-depth analysis of each Pascal unit including:
- Purpose and functionality
- Key procedures and functions
- Data structures used
- Dependencies (uses clause)
- Global variables accessed
- DOS-specific features
- Conversion notes and recommendations

**Usage:** Reference when converting specific units to understand context and dependencies.

### [pascal-dependencies.md](pascal-dependencies.md)

**Dependency graph documentation**

Complete dependency analysis of the Pascal codebase:
- 1,070 unit dependencies identified
- Topological sort for conversion order
- Circular dependency detection
- Dependency depth analysis
- Critical path identification

**Includes:**
- Textual dependency documentation
- See pascal-dependencies.dot for GraphViz source
- See pascal-dependencies.svg for visual diagram
- See pascal-dependency-matrix.csv for matrix format

### [pascal-globals.md](pascal-globals.md)

**Global variables and state management**

Analysis of global state in the Pascal codebase:
- Global variable declarations (COMMON.PAS)
- State mutation patterns
- Data flow across units
- Refactoring opportunities for Rust
- Ownership model mapping

**Conversion Strategy:**
- Global → thread-local or struct fields
- Mutable state → Interior mutability (RefCell, RwLock)
- Shared data → Arc/Mutex patterns

### [pascal-overlays.md](pascal-overlays.md)

**Overlay system analysis (VROOMM)**

Documentation of the VROOMM overlay management system:
- How overlays work in DOS
- Memory constraints (640KB limit)
- Overlay swapping mechanism
- Performance implications
- Modern equivalent (none needed)

**Conversion Impact:**
- Overlays eliminated in Rust version
- All code loaded into memory
- Modern systems have abundant RAM
- Performance improvement expected

### [pascal-interrupts.md](pascal-interrupts.md)

**DOS interrupt usage analysis**

Catalog of DOS and BIOS interrupts used:
- **INT 14h:** Serial I/O (COMMS.PAS)
- **INT 21h:** DOS services (file I/O, etc.)
- **INT 10h:** Video services (ANSI rendering)
- **INT 33h:** Mouse services
- **INT 16h:** Keyboard services

**Rust Replacements:**
- Serial I/O → serialport crate
- File I/O → std::fs
- Video → crossterm crate
- Input → crossterm or termion

### [pascal-dos-specific.md](pascal-dos-specific.md)

**DOS-specific functionality**

Analysis of features tied to DOS architecture:
- File system specifics (8.3 filenames, drive letters)
- Memory model (far/near pointers)
- Executable format (COM/EXE)
- TSR (Terminate and Stay Resident) usage
- Device drivers (FOSSIL, etc.)

**Conversion Strategy:**
- Replace with OS-agnostic abstractions
- Use std::path for cross-platform paths
- Modern process model (no TSR needed)
- Standard OS APIs instead of device drivers

---

## Data Files

### dependencies.json

Machine-readable dependency data in JSON format.

**Structure:**
```json
{
  "units": [
    {
      "name": "COMMON",
      "dependencies": ["DOS", "CRT"],
      "dependents": ["INIT", "USER", ...],
      "depth": 0
    },
    ...
  ]
}
```

**Usage:** Automated tooling, conversion order calculation.

### pascal-dependencies.dot

GraphViz DOT format dependency graph.

**Usage:**
```bash
# Generate PNG
dot -Tpng pascal-dependencies.dot -o dependencies.png

# Generate SVG (already generated)
dot -Tsvg pascal-dependencies.dot -o pascal-dependencies.svg
```

### pascal-dependencies.svg

Visual dependency diagram (SVG format).

**Features:**
- All 96 units visualized
- Dependency arrows
- Color-coded by category
- Interactive (SVG supports links)

**Usage:** View in browser or SVG viewer for interactive exploration.

### pascal-dependency-matrix.csv

Dependency matrix in CSV format.

**Structure:**
- Rows: Source units
- Columns: Target units
- Values: 1 (depends on) or 0 (no dependency)

**Usage:** Spreadsheet analysis, automated processing.

### risk-data.json

Machine-readable risk assessment data.

**Structure:**
```json
{
  "units": [
    {
      "name": "COMMS",
      "risk_level": "high",
      "factors": ["DOS interrupts", "UART access"],
      "mitigation": "Use serialport crate"
    },
    ...
  ]
}
```

**Usage:** Automated risk reporting, resource allocation.

---

## Analysis Methodology

**Sprint 3 Process:**
1. **Inventory:** Cataloged all 96 Pascal source files
2. **Dependency Mapping:** Extracted uses clauses, built graph
3. **Complexity Analysis:** Assessed DOS-specific features
4. **Global State:** Identified shared mutable state
5. **Risk Assessment:** Evaluated conversion challenges
6. **Documentation:** Created comprehensive guides

**Tools Used:**
- Manual source code review
- Dependency graph generation (custom script)
- GraphViz for visualization
- JSON/CSV for machine-readable data

---

## Related Documentation

- **[Conversion Guides](../conversion/)** - How to convert Pascal to Rust
- **[Risk Assessment](../risk-assessment/)** - Conversion risks and mitigations
- **[Planning](../../planning/)** - Conversion strategy and schedule

---

[← Back to Pascal Reference](../INDEX.md) | [← Back to Documentation Index](../../INDEX.md)
