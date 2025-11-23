# Sprint To-Dos Directory

**Project:** Impulse 7.1 BBS Modernization
**Last Updated:** 2025-11-22

---

## Overview

This directory contains detailed sprint-level task tracking for the Impulse 7.1 BBS modernization project. Each sprint represents a 2-week development iteration with specific objectives, deliverables, and acceptance criteria.

## Directory Structure

```
to-dos/
├── README.md                        (This file)
├── CURRENT-PHASE.md                 (Active phase tracking)
├── COMPLETED-SPRINTS.md             (Progress history)
├── phase-1-foundation/              (Sprints 01-08: Foundation)
│   ├── sprint-01-project-setup.md
│   ├── sprint-02-core-types.md
│   ├── sprint-03-file-parsing.md
│   ├── sprint-04-ansi-engine.md
│   ├── sprint-05-telnet-basic.md
│   ├── sprint-06-user-system.md
│   ├── sprint-07-security-auth.md
│   └── sprint-08-testing-ci.md
├── phase-2-core-features/           (Sprints 09-16: Core Features)
│   ├── sprint-09-message-base.md
│   ├── sprint-10-file-areas.md
│   ├── sprint-11-menus-navigation.md
│   ├── sprint-12-door-games.md
│   ├── sprint-13-multinode.md
│   ├── sprint-14-file-transfers.md
│   ├── sprint-15-ssh-support.md
│   └── sprint-16-phase2-integration.md
├── phase-3-feature-completion/      (Sprints 17-24: Feature Completion)
│   ├── sprint-17-advanced-messaging.md
│   ├── sprint-18-mail-system.md
│   ├── sprint-19-bulletins-news.md
│   ├── sprint-20-voting-surveys.md
│   ├── sprint-21-user-profiles.md
│   ├── sprint-22-statistics-logs.md
│   ├── sprint-23-admin-tools.md
│   └── sprint-24-phase3-integration.md
└── phase-4-polish-launch/           (Sprints 25-32: Polish and Launch)
    ├── sprint-25-performance-optimization.md
    ├── sprint-26-security-hardening.md
    ├── sprint-27-deployment-automation.md
    ├── sprint-28-documentation-completion.md
    ├── sprint-29-user-acceptance-testing.md
    ├── sprint-30-migration-tooling.md
    ├── sprint-31-launch-preparation.md
    └── sprint-32-release-retrospective.md
```

## Sprint File Template

Each sprint file follows this structure:

- **Sprint Overview**: Purpose, context, and expected outcomes
- **Objectives**: Primary goals (checkboxes for tracking)
- **Deliverables**: Specific outputs with acceptance criteria
- **Detailed Tasks**: Categorized actionable items with time estimates
- **Technical Details**: Architecture considerations, dependencies, code patterns
- **Dependencies**: Upstream and downstream sprint relationships
- **Acceptance Criteria**: Completion requirements
- **Testing Requirements**: Unit, integration, and performance tests
- **Notes and Decisions**: Design decisions, lessons learned, risks
- **Progress Log**: Weekly progress tracking

## Usage

### Starting a New Sprint

1. Open the sprint file (e.g., `phase-1-foundation/sprint-01-project-setup.md`)
2. Review objectives and deliverables
3. Update **Status** field to "In Progress"
4. Update **Sprint Dates** with actual start/end dates
5. Use checkboxes to track task completion
6. Log progress in the **Progress Log** section

### Completing a Sprint

1. Verify all **Acceptance Criteria** are met
2. Run required tests and confirm passing
3. Update **Status** to "Completed"
4. Document lessons learned and decisions made
5. Update `CURRENT-PHASE.md` if moving to next phase
6. Add entry to `COMPLETED-SPRINTS.md`

### Tracking Current Work

- `CURRENT-PHASE.md` - Shows active phase and current sprint
- Individual sprint files - Track task-level progress
- `COMPLETED-SPRINTS.md` - Historical record of completed work

## Phase Overview

### Phase 1: Foundation (Months 1-4, Sprints 1-8)
**Focus:** Core infrastructure, basic types, file I/O, Telnet, and foundational systems

**Key Milestones:**
- Rust workspace and build system established
- Core type definitions and binary parsing functional
- ANSI rendering engine operational
- Basic Telnet connectivity working
- User authentication system complete
- CI/CD pipeline operational

### Phase 2: Core Features (Months 5-8, Sprints 9-16)
**Focus:** Essential BBS functionality - messaging, files, menus, multi-node

**Key Milestones:**
- JAM message base fully functional
- File areas with metadata and transfers
- Menu system and navigation complete
- Door game integration working
- Multi-node support operational
- SSH connectivity implemented

### Phase 3: Feature Completion (Months 9-16, Sprints 17-24)
**Focus:** Advanced features, polish, and feature parity with Pascal version

**Key Milestones:**
- Advanced messaging (threading, search)
- Email/netmail system
- Bulletins and news
- Voting and surveys
- User profiles and customization
- Statistics and logging
- Admin tools complete

### Phase 4: Polish and Launch (Months 17-24, Sprints 25-32)
**Focus:** Optimization, security, deployment, migration, and production readiness

**Key Milestones:**
- Performance optimized and benchmarked
- Security hardened and audited
- Deployment automation complete
- Documentation comprehensive
- User acceptance testing passed
- Migration tooling functional
- Production launch successful

## Velocity Tracking

Track completed story points or tasks per sprint to estimate future capacity.

**Example:**
```
Sprint 01: 40 tasks completed
Sprint 02: 35 tasks completed
Sprint 03: 42 tasks completed
Average: 39 tasks/sprint
```

Use this data for realistic sprint planning.

## Best Practices

### Task Granularity
- Break large tasks into sub-tasks (2-8 hours each)
- Use checkboxes for tracking
- Estimate hours conservatively

### Documentation
- Update progress log at least twice per week
- Document design decisions as they're made
- Capture lessons learned while fresh
- Note risks and mitigations proactively

### Quality Gates
- Never mark sprint complete with failing tests
- Ensure code review before marking deliverables done
- Verify acceptance criteria objectively
- Update documentation alongside code changes

### Communication
- Sprint files are team communication tools
- Use for stakeholder updates
- Reference in commit messages
- Link from pull requests

## Integration with Other Documentation

Sprint files complement other project documentation:

- **[docs/01-phase-sprint-plan.md](../docs/01-phase-sprint-plan.md)** - High-level sprint planning
- **[docs/04-development-guide.md](../docs/04-development-guide.md)** - Development workflows
- **[docs/05-testing-strategy.md](../docs/05-testing-strategy.md)** - Testing approach
- **[docs/06-deployment-guide.md](../docs/06-deployment-guide.md)** - Deployment procedures

## Sprint Timeline

| Sprint | Phase | Duration | Focus |
|--------|-------|----------|-------|
| 01-08 | Foundation | Weeks 1-16 | Infrastructure, core types, basic connectivity |
| 09-16 | Core Features | Weeks 17-32 | Messaging, files, menus, multi-node |
| 17-24 | Feature Completion | Weeks 33-48 | Advanced features, feature parity |
| 25-32 | Polish & Launch | Weeks 49-64 | Optimization, security, deployment, launch |

**Total Timeline:** 64 weeks (16 months with 4-week sprints) = 24 months (2-week sprints)

## Quick Reference

**Start sprint:**
```bash
# Open sprint file
vim to-dos/phase-1-foundation/sprint-01-project-setup.md

# Update status to "In Progress"
# Set sprint dates
# Begin working through tasks
```

**Complete sprint:**
```bash
# Verify all acceptance criteria met
# Run test suite: cargo test --workspace
# Update status to "Completed"
# Update CURRENT-PHASE.md and COMPLETED-SPRINTS.md
```

**Track progress:**
```bash
# View current work
cat to-dos/CURRENT-PHASE.md

# View history
cat to-dos/COMPLETED-SPRINTS.md
```

---

**This directory structure provides the operational framework for executing the 24-month modernization project. Use it actively, update it frequently, and let it guide the development process.**

---

**Questions about sprint planning?** Refer to [docs/01-phase-sprint-plan.md](../docs/01-phase-sprint-plan.md) for strategic planning details.
