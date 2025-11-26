# Beta Testing Checklist - Impulse-Next_BBS

This checklist helps ensure comprehensive testing coverage for all Phase 2 features.

**Version:** 0.1.0
**Last Updated:** 2025-11-25

---

## Installation & Setup

- [ ] Clone repository successfully
- [ ] Build project without errors
- [ ] All tests pass (`cargo test --workspace`)
- [ ] Generate default configuration
- [ ] Validate configuration file
- [ ] Start server successfully
- [ ] Connect via telnet
- [ ] Connect via SSH (if enabled)
- [ ] Stop server gracefully

---

## User Authentication

### Registration
- [ ] Register new user with valid data
- [ ] Registration rejects invalid username (too short/long, special chars)
- [ ] Registration rejects weak password
- [ ] Registration rejects invalid email
- [ ] Registration validates all required fields
- [ ] Duplicate username prevented
- [ ] First user receives SysOp privileges

### Login
- [ ] Login with valid credentials succeeds
- [ ] Login with invalid password fails
- [ ] Login with non-existent username fails
- [ ] Case-insensitive username matching works
- [ ] Session token generated
- [ ] Session persists across menu navigation
- [ ] Rate limiting triggers after failed attempts
- [ ] Account lockout after max failures

### Session Management
- [ ] Session remains valid during activity
- [ ] Session expires after timeout period
- [ ] Reconnect after session expiry works
- [ ] Multiple concurrent sessions for same user
- [ ] Session isolation (users can't see each other's data)
- [ ] Logout invalidates session
- [ ] Forced logout (admin function) works

### Password Management
- [ ] Password change requires current password
- [ ] Password change validates new password
- [ ] New password works immediately
- [ ] Old password no longer works

---

## Menu System

### Navigation
- [ ] Main menu displays correctly
- [ ] All menu options accessible
- [ ] Hot keys work (single letter selection)
- [ ] Menu numbers work (numbered selection)
- [ ] Invalid selection shows error
- [ ] Menu timeout returns to previous menu
- [ ] Nested menu navigation works
- [ ] Return to main menu works from any submenu

### Display
- [ ] ANSI colors display correctly
- [ ] ANSI art renders properly
- [ ] Menu fits in 80x25 terminal
- [ ] Menu adapts to 80x43 terminal
- [ ] Menu adapts to 132x60 terminal
- [ ] Menu displays correctly without color (mono mode)
- [ ] Help text displays for each menu option

---

## Message System

### Message Areas
- [ ] List message areas
- [ ] Select message area
- [ ] Display area description
- [ ] Show message count for each area
- [ ] Indicate areas with new messages
- [ ] Access control by security level works

### Reading Messages
- [ ] List messages in area
- [ ] Display message header (from, to, subject, date)
- [ ] Display message body
- [ ] Display threaded messages correctly
- [ ] Navigate message list (next/previous)
- [ ] Jump to message number
- [ ] Mark message as read
- [ ] Display message numbers correctly

### Posting Messages
- [ ] Post new message
- [ ] Validate required fields (from, to, subject)
- [ ] Validate message length limits
- [ ] Sanitize HTML/special characters
- [ ] Message appears in area immediately
- [ ] Message number assigned correctly

### Replying
- [ ] Reply to message
- [ ] Reply links to original (reply_to field)
- [ ] Reply subject prefixed with "Re:"
- [ ] Quote original message
- [ ] Quote attribution shows sender name
- [ ] Quote formatting (> prefix) correct
- [ ] Reply threads display correctly

### Search
- [ ] Search by sender (from field)
- [ ] Search by recipient (to field)
- [ ] Search by subject
- [ ] Search by body text
- [ ] Search by date range
- [ ] Search case-insensitive
- [ ] Search results accurate
- [ ] Search results paginated

### Pagination
- [ ] Message list shows page indicators
- [ ] Navigate to next page
- [ ] Navigate to previous page
- [ ] Jump to specific page
- [ ] First/last page shortcuts work

---

## File Areas

### File Area Browsing
- [ ] List file areas
- [ ] Select file area
- [ ] Display area description
- [ ] Show file count for each area
- [ ] Access control by security level works

### File Listing
- [ ] List files in area
- [ ] Display filename, size, date, uploader
- [ ] Display file description
- [ ] Sort by filename
- [ ] Sort by date
- [ ] Sort by size
- [ ] Pagination works (next/previous page)

### File Details
- [ ] View detailed file information
- [ ] Display FILE_ID.DIZ if present
- [ ] Display full description
- [ ] Show upload date and time
- [ ] Show uploader name
- [ ] Show download count
- [ ] Show file size in appropriate units (B, KB, MB)

### File Search
- [ ] Search by filename (wildcards supported)
- [ ] Search by description
- [ ] Search by uploader
- [ ] Search by date range
- [ ] Search by size range
- [ ] Search results accurate
- [ ] Search results paginated

### FILE_ID.DIZ
- [ ] Extract FILE_ID.DIZ from ZIP files
- [ ] Display extracted description
- [ ] Handle missing FILE_ID.DIZ gracefully
- [ ] Handle corrupt ZIP files gracefully

---

## User Profiles

### Profile Display
- [ ] View own profile
- [ ] View other users' profiles
- [ ] Display username, real name, location
- [ ] Display security level
- [ ] Display join date, last login
- [ ] Display statistics (logins, messages, files)
- [ ] Privacy settings honored
- [ ] Hidden fields not shown to other users

### Statistics
- [ ] Login count accurate
- [ ] Messages posted count accurate
- [ ] Messages read count accurate
- [ ] Files uploaded count accurate
- [ ] Files downloaded count accurate
- [ ] Total time online accurate
- [ ] Upload/download ratio calculated correctly

### Settings
- [ ] Change screen width
- [ ] Change screen height
- [ ] Enable/disable color
- [ ] Change date format
- [ ] Change time format
- [ ] Settings persist after logout
- [ ] Settings apply immediately

### Privacy Settings
- [ ] Hide real name
- [ ] Hide email address
- [ ] Hide location
- [ ] Hide statistics
- [ ] Privacy settings persist
- [ ] Privacy settings enforced

### Achievements
- [ ] Achievements display correctly
- [ ] Achievement notifications appear
- [ ] Achievement progression tracked
- [ ] No duplicate achievements awarded

---

## Concurrent Access

### Multiple Users
- [ ] 5 users connected simultaneously
- [ ] 10 users connected simultaneously
- [ ] 20 users connected simultaneously (if supported)
- [ ] Each user session independent
- [ ] No session crossover or data leakage

### Concurrent Operations
- [ ] Multiple users posting messages simultaneously
- [ ] Message numbers sequential and unique
- [ ] No duplicate message numbers
- [ ] Concurrent file browsing works
- [ ] Concurrent profile updates work
- [ ] No data corruption under concurrent load

### Performance
- [ ] Response time acceptable (<100ms for typical operations)
- [ ] Message list loads quickly (<50ms)
- [ ] File list loads quickly (<50ms)
- [ ] Search completes quickly (<200ms)
- [ ] No performance degradation over time
- [ ] Memory usage stable
- [ ] CPU usage reasonable

---

## Error Handling

### Network Errors
- [ ] Server handles connection drops gracefully
- [ ] Client reconnect works after network interruption
- [ ] Timeout errors display user-friendly message
- [ ] Server logs network errors

### Input Validation
- [ ] Empty input handled gracefully
- [ ] Oversized input rejected
- [ ] Special characters sanitized
- [ ] SQL injection prevented (if using database)
- [ ] XSS prevented (if displaying user content)

### File System Errors
- [ ] Missing configuration file handled
- [ ] Read-only filesystem handled
- [ ] Disk full handled
- [ ] Missing message/file areas handled
- [ ] Corrupt data files handled

### Error Messages
- [ ] Error messages clear and helpful
- [ ] Error messages don't expose sensitive info
- [ ] Error messages suggest resolution
- [ ] Errors logged appropriately

---

## Terminal Compatibility

### ANSI Support
- [ ] xterm
- [ ] PuTTY
- [ ] SecureCRT
- [ ] mTerm (macOS)
- [ ] Windows Terminal
- [ ] Linux console

### Screen Sizes
- [ ] 80x25 (standard)
- [ ] 80x43 (EGA)
- [ ] 80x50 (VGA)
- [ ] 132x25 (wide)
- [ ] 132x43 (wide EGA)
- [ ] 132x60 (wide VGA)

### Color Modes
- [ ] Full ANSI color (16 colors)
- [ ] Monochrome mode
- [ ] Color/mono preference persists

---

## Platform Testing

### Linux
- [ ] Ubuntu 22.04 LTS
- [ ] Debian 12
- [ ] Fedora 38
- [ ] Arch Linux
- [ ] CentOS/RHEL 8

### macOS
- [ ] macOS 12 (Monterey)
- [ ] macOS 13 (Ventura)
- [ ] macOS 14 (Sonoma)

### Windows
- [ ] Windows 10 (21H2 or later)
- [ ] Windows 11
- [ ] Windows Server 2019
- [ ] Windows Server 2022

---

## Security Testing

### Authentication Security
- [ ] Passwords hashed (not stored plaintext)
- [ ] Session tokens unpredictable
- [ ] Session tokens invalidated on logout
- [ ] Rate limiting prevents brute force
- [ ] Account lockout prevents brute force

### Input Sanitization
- [ ] User input sanitized before display
- [ ] SQL injection prevented
- [ ] Command injection prevented
- [ ] Path traversal prevented

### Access Control
- [ ] Users can only access allowed areas
- [ ] Security levels enforced
- [ ] SysOp commands require SysOp level
- [ ] Users can't access other users' data

---

## Logging & Monitoring

### System Logs
- [ ] Server startup logged
- [ ] User logins logged
- [ ] Failed login attempts logged
- [ ] Errors logged with context
- [ ] Log rotation works
- [ ] Log files don't grow unbounded

### Audit Logs
- [ ] Security events logged
- [ ] Admin actions logged
- [ ] Log tampering prevented
- [ ] Logs include timestamps
- [ ] Logs include user context

---

## Edge Cases

### Empty States
- [ ] Empty message area handled
- [ ] Empty file area handled
- [ ] New user (no history) handled
- [ ] No search results handled

### Boundary Conditions
- [ ] Maximum message length enforced
- [ ] Maximum subject length enforced
- [ ] Maximum username length enforced
- [ ] Maximum concurrent users enforced
- [ ] Very large file lists handled (1000+ files)
- [ ] Very large message bases handled (10000+ messages)

### Special Characters
- [ ] Unicode in messages
- [ ] Unicode in filenames
- [ ] Emoji in messages (if supported)
- [ ] Special punctuation handled

---

## Documentation

### User Documentation
- [ ] README accurate and up-to-date
- [ ] Installation instructions work
- [ ] Configuration documented
- [ ] Feature list complete
- [ ] Troubleshooting section helpful

### Beta Testing Documentation
- [ ] Beta testing guide clear
- [ ] Testing scenarios realistic
- [ ] Bug reporting process clear
- [ ] Known limitations documented

---

## Summary

**Total Checklist Items:** 250+

**Completion Tracking:**
- [ ] Installation & Setup (9 items)
- [ ] User Authentication (28 items)
- [ ] Menu System (14 items)
- [ ] Message System (42 items)
- [ ] File Areas (32 items)
- [ ] User Profiles (30 items)
- [ ] Concurrent Access (15 items)
- [ ] Error Handling (18 items)
- [ ] Terminal Compatibility (18 items)
- [ ] Platform Testing (14 items)
- [ ] Security Testing (13 items)
- [ ] Logging & Monitoring (11 items)
- [ ] Edge Cases (14 items)
- [ ] Documentation (9 items)

**Notes:**
Use this checklist to systematically test all features. Mark items as you complete them. Report any failures as bugs through the issue tracker.

---

**Document Version:** 1.0
**Last Updated:** 2025-11-25
**Phase:** 2 - Core Features (Complete)
