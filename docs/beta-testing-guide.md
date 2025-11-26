# Beta Testing Guide - Impulse-Next_BBS

Welcome to the Impulse-Next_BBS beta testing program! This guide will help you install, configure, and test the BBS software.

**Version:** 0.1.0 (Phase 2 Complete)
**Last Updated:** 2025-11-25

---

## Table of Contents

1. [Introduction](#introduction)
2. [System Requirements](#system-requirements)
3. [Installation](#installation)
4. [Configuration](#configuration)
5. [Features to Test](#features-to-test)
6. [Testing Scenarios](#testing-scenarios)
7. [Bug Reporting](#bug-reporting)
8. [Feedback Channels](#feedback-channels)
9. [Known Limitations](#known-limitations)

---

## Introduction

Thank you for participating in the Impulse-Next_BBS beta test! This is a critical phase where we validate that all Phase 2 features work correctly in real-world scenarios.

**Beta Test Goals:**
- Validate core BBS functionality (authentication, messaging, file management)
- Identify bugs and edge cases
- Gather feedback on usability and performance
- Test multi-user scenarios and concurrent access

**What's Included in This Beta:**
- User authentication and session management
- Menu system and navigation
- Message reading and writing (JAM and Hudson formats)
- File area browsing and searching
- User profiles and statistics
- Terminal emulation (ANSI, Avatar, RIP)

---

## System Requirements

### Minimum Requirements

**Operating System:**
- Linux (kernel 4.4+)
- macOS 10.15+
- Windows 10/11

**Hardware:**
- CPU: 1 GHz dual-core processor
- RAM: 512 MB available
- Disk: 100 MB free space
- Network: TCP/IP network connection

**Software:**
- Rust 1.85+ (for building from source)
- Terminal emulator with ANSI support
- Telnet or SSH client

### Recommended Requirements

- CPU: 2 GHz quad-core processor
- RAM: 1 GB available
- Disk: 500 MB free space
- Network: Broadband connection

---

## Installation

### Option 1: Build from Source (Recommended)

1. **Clone the repository:**
   ```bash
   git clone https://github.com/doublegate/Impulse-Next_BBS.git
   cd Impulse-Next_BBS
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Verify the build:**
   ```bash
   cargo test --workspace
   ```

4. **Install binaries:**
   ```bash
   cargo install --path crates/impulse-server
   cargo install --path crates/impulse-cli
   ```

### Option 2: Download Pre-built Binaries

*(Coming soon - binaries will be available for Linux, macOS, and Windows)*

---

## Configuration

### Initial Setup

1. **Generate default configuration:**
   ```bash
   impulse-cli config generate
   ```

   This creates `bbs.toml` in the current directory.

2. **Edit configuration:**
   ```toml
   [system]
   name = "My BBS"
   sysop_name = "Your Name"
   location = "Your City, State"

   [network]
   telnet_port = 2323
   ssh_port = 2222
   max_connections = 50

   [paths]
   data_dir = "./data"
   message_dir = "./messages"
   file_dir = "./files"
   log_dir = "./logs"

   [security]
   password_min_length = 8
   session_timeout_minutes = 30
   max_login_attempts = 3
   ```

3. **Validate configuration:**
   ```bash
   impulse-cli config validate
   ```

### First Run

1. **Start the server:**
   ```bash
   impulse-server --config bbs.toml
   ```

2. **Connect via telnet:**
   ```bash
   telnet localhost 2323
   ```

3. **Create first user (SysOp):**
   - Follow the registration prompts
   - First user automatically receives SysOp privileges

---

## Features to Test

### 1. User Authentication

**What to test:**
- User registration with validation
- Login with correct/incorrect credentials
- Password change flow
- Session timeout and reconnection
- Multiple concurrent sessions

**How to test:**
- Create several test users
- Try invalid usernames/passwords
- Leave session idle to test timeout
- Connect from multiple terminals simultaneously

### 2. Menu System

**What to test:**
- Menu navigation
- Hot key selection
- Menu timeout
- Display on different screen sizes
- ANSI art rendering

**How to test:**
- Navigate through all menus
- Try both hot keys and menu numbers
- Test on 80x25, 80x43, 132x60 terminals
- Verify ANSI colors display correctly

### 3. Message System

**What to test:**
- Message area listing
- Reading messages with threading
- Posting new messages
- Replying to messages with quoting
- Message search
- Pagination

**How to test:**
- Browse multiple message areas
- Read threaded discussions
- Post messages to different areas
- Reply with and without quoting
- Search by from, to, subject, body
- Navigate multi-page message lists

### 4. File Areas

**What to test:**
- File area browsing
- File search by name, description, uploader
- FILE_ID.DIZ display
- Pagination
- Sorting by name, date, size

**How to test:**
- Browse different file areas
- Search for files using various criteria
- View file details and descriptions
- Navigate multi-page file lists
- Test sorting options

### 5. User Profiles

**What to test:**
- Profile display
- Privacy settings
- Settings changes (screen size, colors)
- Statistics tracking
- Achievement notifications

**How to test:**
- View your own profile
- View other users' profiles
- Change privacy settings
- Update screen preferences
- Check statistics after activity
- Verify achievements are awarded

### 6. Concurrent Access

**What to test:**
- Multiple users online simultaneously
- Concurrent message posting
- Session isolation
- Performance under load

**How to test:**
- Connect 5-10 users simultaneously
- Have users post messages concurrently
- Verify each user's session is independent
- Monitor server performance

---

## Testing Scenarios

### Scenario 1: New User Experience

1. Connect to BBS as a new user
2. Register a new account
3. Navigate the main menu
4. Browse message areas
5. Read a few messages
6. Post a new message
7. Browse file areas
8. Search for files
9. View your profile
10. Logout and reconnect

**Expected Behavior:**
- Registration should validate all input
- Menu navigation should be intuitive
- All features should be accessible
- Statistics should update correctly

### Scenario 2: Message Thread Participation

1. Login as User A
2. Find a message area
3. Post a new topic
4. Logout

5. Login as User B
6. Read User A's message
7. Reply to the message
8. Logout

9. Login as User A
10. See the reply notification
11. Read User B's reply
12. Reply again with quoting

**Expected Behavior:**
- Messages should thread correctly
- Quoting should format properly
- Notifications should appear
- Thread view should be accurate

### Scenario 3: File Discovery

1. Login to BBS
2. Enter file areas menu
3. Browse "General Files" area
4. Use search to find ".zip" files
5. View file details
6. Check uploader and date info
7. Try pagination if >20 files
8. Switch to another area

**Expected Behavior:**
- File lists should load quickly
- Search should filter correctly
- Pagination should work smoothly
- File details should be accurate

### Scenario 4: Concurrent Users

1. Connect 5 users simultaneously
2. Have each user:
   - Navigate to message areas
   - Post a message
   - Search for files
   - View profiles
3. Verify no conflicts or errors

**Expected Behavior:**
- All users should operate independently
- No session crossover
- No data corruption
- Reasonable performance

### Scenario 5: Extended Session

1. Login and remain connected for 30+ minutes
2. Perform various activities:
   - Read messages
   - Post messages
   - Browse files
   - Update profile
3. Monitor for session issues

**Expected Behavior:**
- Session should remain stable
- No memory leaks
- No performance degradation
- Session timeout should work correctly

---

## Bug Reporting

### How to Report Bugs

1. **Check existing issues:**
   Visit https://github.com/doublegate/Impulse-Next_BBS/issues

2. **Create a new issue:**
   Click "New Issue" and select "Bug Report"

3. **Provide details:**
   - Clear description of the problem
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - System information (OS, terminal type)
   - Logs (if available)

### Bug Severity Levels

- **Critical:** System crash, data corruption, security vulnerability
- **High:** Feature completely broken, affects multiple users
- **Medium:** Feature partially broken, workaround available
- **Low:** Cosmetic issue, minor inconvenience

### Example Bug Report

```
**Description:** Message quoting includes extra blank lines

**Steps to Reproduce:**
1. Login to BBS
2. Navigate to message area
3. Read any message
4. Press 'R' to reply
5. Select 'Quote original'
6. Type reply
7. Post message

**Expected Behavior:**
Quoted text should have proper spacing (> prefix with original text)

**Actual Behavior:**
Extra blank lines appear between quoted lines

**System Info:**
- OS: Ubuntu 22.04
- Terminal: xterm-256color
- Server Version: 0.1.0
```

---

## Feedback Channels

### GitHub Issues
Primary channel for bugs and feature requests:
https://github.com/doublegate/Impulse-Next_BBS/issues

### Discussion Forum
For general feedback and questions:
https://github.com/doublegate/Impulse-Next_BBS/discussions

### Email
For private feedback or security issues:
parobek@gmail.com

### What Feedback We Need

**Usability:**
- Is the interface intuitive?
- Are menu options clear?
- Is navigation logical?

**Performance:**
- Response times
- Load times for large message/file lists
- Server resource usage

**Features:**
- What features are most useful?
- What features are missing?
- What improvements would you like?

**Bugs:**
- Anything that doesn't work as expected
- Edge cases or unusual scenarios
- Platform-specific issues

---

## Known Limitations

### Current Limitations

1. **No Upload Functionality:**
   - File uploads are validated but not yet implemented
   - Only browsing and downloading planned

2. **No Door Games:**
   - Door game interface not yet implemented
   - Coming in Phase 3

3. **No Networking:**
   - FidoNet and QWK networking not yet implemented
   - Coming in Phase 3

4. **Limited Terminal Emulation:**
   - Basic ANSI support
   - Advanced features (Avatar, RIP) in development

5. **No Web Admin:**
   - Web administration panel not yet implemented
   - Use CLI tools for administration

### Workarounds

**Issue:** Session timeout too short/long
**Workaround:** Edit `session_timeout_minutes` in config

**Issue:** Can't see ANSI colors
**Workaround:** Ensure terminal supports ANSI (use xterm, PuTTY, etc.)

**Issue:** Menu displays incorrectly on wide screens
**Workaround:** Set terminal to 80x25 or 80x43

---

## Testing Checklist

Use the [Beta Testing Checklist](beta-testing-checklist.md) to track your testing progress and ensure comprehensive coverage.

---

## Thank You!

Your participation in this beta test is invaluable. Every bug report, every piece of feedback, and every minute spent testing helps make Impulse-Next_BBS better.

**Questions?** Feel free to reach out through any of the feedback channels listed above.

**Happy Testing!**

---

**Document Version:** 1.0
**Last Updated:** 2025-11-25
**Phase:** 2 - Core Features (Complete)
