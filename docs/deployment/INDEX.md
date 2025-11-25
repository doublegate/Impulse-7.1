# Deployment Documentation

Deployment guides, migration tools, and operational procedures.

**Last Updated:** 2025-11-24

---

## Overview

This directory contains deployment and migration documentation for Impulse-Next BBS, including installation procedures, configuration management, and migration from classic Impulse 7.1.

---

## Files

### [deployment-guide.md](deployment-guide.md)

**Installation, configuration, and deployment procedures**

Comprehensive guide for deploying Impulse-Next BBS in production.

**Topics:**
- **System Requirements:** Hardware, OS, dependencies
- **Installation Methods:**
  - Binary installation (pre-compiled releases)
  - Source compilation
  - Docker/Podman containers
  - Kubernetes deployment
- **Configuration:**
  - Initial setup wizard
  - Configuration file structure
  - Environment variables
  - Security hardening
- **Database Setup:**
  - SQLite for single-node
  - PostgreSQL for multi-node
  - Schema initialization
  - Backup and restore
- **Network Configuration:**
  - Telnet port setup (default: 23)
  - SSH port setup (default: 22)
  - Firewall rules
  - Reverse proxy (nginx, caddy)
- **Service Management:**
  - Systemd service files
  - Automatic startup
  - Process monitoring
  - Log rotation
- **Monitoring:**
  - Health checks
  - Metrics (Prometheus)
  - Alerting
  - Performance tuning

### [migration-guide.md](migration-guide.md)

**Migrating from classic Impulse 7.1 to Impulse-Next**

Step-by-step guide for SysOps migrating existing Impulse installations.

**Topics:**
- **Pre-Migration:**
  - Backup procedures
  - Compatibility assessment
  - Data inventory
  - Risk evaluation
- **Data Migration:**
  - User database (USER.LST)
  - Message bases (JAM, Hudson)
  - File areas and descriptions
  - Configuration files
  - Theme and ANSI files
- **Migration Tools:**
  - imp-migrate utility
  - Data format converters
  - Validation tools
  - Rollback procedures
- **Testing Migration:**
  - Test environment setup
  - Parallel operation
  - User acceptance testing
  - Performance comparison
- **Cutover Process:**
  - Downtime planning
  - Final data sync
  - Service switchover
  - Post-migration verification
- **Troubleshooting:**
  - Common migration issues
  - Data integrity checks
  - Performance problems
  - Rollback procedures

---

## Deployment Scenarios

### Single-Node BBS (Small Scale)

**Recommended for:** Home hobbyists, small communities (1-10 users)

**Setup:**
- Single server (Raspberry Pi 4+, or any Linux/Windows machine)
- SQLite database
- Binary installation or Docker
- Local file storage

**Resources:**
- RAM: 512MB minimum, 1GB recommended
- CPU: 1 core minimum, 2+ cores recommended
- Disk: 5GB minimum, 20GB+ recommended
- Network: Home internet connection

### Multi-Node BBS (Medium Scale)

**Recommended for:** Active communities (10-100 users)

**Setup:**
- Dedicated server (VPS, bare metal)
- PostgreSQL database
- Docker/Podman with orchestration
- Network file storage (NFS, S3)

**Resources:**
- RAM: 2GB minimum, 4GB+ recommended
- CPU: 2 cores minimum, 4+ cores recommended
- Disk: 20GB minimum, 100GB+ recommended
- Network: 100Mbps+ connection

### Enterprise BBS (Large Scale)

**Recommended for:** Large communities (100+ users)

**Setup:**
- Kubernetes cluster
- PostgreSQL HA cluster
- S3-compatible object storage
- Load balancer
- CDN for static assets

**Resources:**
- RAM: 8GB+ per node
- CPU: 4+ cores per node
- Disk: 100GB+ per node
- Network: 1Gbps+ connection

---

## Migration Timeline

**Phase 1: Planning (1-2 weeks)**
- Backup existing system
- Inventory data and configurations
- Test migration tools
- Prepare rollback plan

**Phase 2: Test Migration (1-2 weeks)**
- Set up test environment
- Migrate test data
- Validate migrated data
- User acceptance testing

**Phase 3: Production Migration (1-3 days)**
- Final backup
- Service cutover
- Data migration
- Verification and testing

**Phase 4: Post-Migration (1-2 weeks)**
- Monitor for issues
- Performance tuning
- User support
- Documentation updates

---

## Support Resources

**Documentation:**
- [Getting Started Guide](../getting-started/)
- [Architecture Overview](../architecture/)
- [Configuration Reference](../reference/)

**Community:**
- GitHub Issues: Bug reports and feature requests
- GitHub Discussions: Q&A and support
- IRC/Discord: Real-time community help

**Commercial Support:**
- Available for enterprise deployments
- Contact via GitHub Discussions

---

[← Back to Documentation Index](../INDEX.md)
