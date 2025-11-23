# Deployment Guide

**Project:** Impulse 7.1 BBS Modernization
**Last Updated:** 2025-11-22
**Target Audience:** System Administrators, DevOps Engineers

---

## Table of Contents

1. [Deployment Overview](#deployment-overview)
2. [Container Deployment](#container-deployment)
3. [Kubernetes Deployment](#kubernetes-deployment)
4. [Bare Metal Deployment](#bare-metal-deployment)
5. [Configuration Management](#configuration-management)
6. [Monitoring and Logging](#monitoring-and-logging)
7. [Backup and Recovery](#backup-and-recovery)
8. [Security Hardening](#security-hardening)
9. [Performance Tuning](#performance-tuning)
10. [Upgrade and Rollback](#upgrade-and-rollback)
11. [Troubleshooting](#troubleshooting)

---

## Deployment Overview

### Deployment Models

| Model | Use Case | Complexity | Scalability |
|-------|----------|------------|-------------|
| **Docker Standalone** | Development, small deployments | Low | Limited |
| **Docker Compose** | Multi-node local testing | Low-Medium | Limited |
| **Kubernetes** | Production, high availability | High | Excellent |
| **Bare Metal** | Legacy compatibility, specialized hardware | Medium | Manual |
| **Cloud (AWS/GCP/Azure)** | Managed infrastructure | Medium | Excellent |

### System Requirements

**Minimum (Single Node):**
- CPU: 2 cores
- RAM: 2 GB
- Disk: 10 GB SSD
- Network: 10 Mbps

**Recommended (Production):**
- CPU: 4+ cores
- RAM: 8+ GB
- Disk: 50+ GB NVMe SSD
- Network: 100+ Mbps

**Multi-Node:**
- Additional 2 GB RAM per node
- Shared storage or network filesystem
- Low-latency network (< 5ms between nodes)

### Port Requirements

| Service | Port | Protocol | Purpose |
|---------|------|----------|---------|
| Telnet | 23 or 2323 | TCP | Legacy BBS access |
| SSH | 22 or 2222 | TCP | Secure BBS access |
| HTTP | 80 | TCP | Web interface (optional) |
| HTTPS | 443 | TCP | Secure web interface |
| WebSocket | 8080 | TCP | Real-time updates |
| Metrics | 9090 | TCP | Prometheus metrics |
| Admin | 8443 | TCP | Administrative interface |

---

## Container Deployment

### Docker Image Build

**Multi-stage Dockerfile:**

```dockerfile
# syntax=docker/dockerfile:1.4

# ============================================
# Stage 1: Build Environment
# ============================================
FROM rust:1.75-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency manifests first (for layer caching)
COPY Cargo.toml Cargo.lock ./
COPY crates/*/Cargo.toml ./crates/

# Create dummy source files to build dependencies
RUN mkdir -p crates/imp-cli/src && \
    echo "fn main() {}" > crates/imp-cli/src/main.rs && \
    find crates -name Cargo.toml -exec dirname {} \; | while read crate; do \
        mkdir -p "$crate/src" && echo "" > "$crate/src/lib.rs"; \
    done

# Build dependencies (cached layer)
RUN cargo build --release

# Remove dummy files
RUN find crates -name "*.rs" -delete

# Copy actual source code
COPY crates/ ./crates/

# Build application (only rebuilds if source changed)
RUN cargo build --release --bin imp-cli

# ============================================
# Stage 2: Runtime Environment
# ============================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash bbs && \
    mkdir -p /var/lib/impulse /etc/impulse && \
    chown -R bbs:bbs /var/lib/impulse /etc/impulse

# Copy binary from builder
COPY --from=builder /build/target/release/imp-cli /usr/local/bin/

# Copy default configuration
COPY docker/config.toml /etc/impulse/config.toml

# Set up volumes
VOLUME ["/var/lib/impulse", "/etc/impulse"]

# Expose ports
EXPOSE 2323 2222 8080 9090

USER bbs
WORKDIR /home/bbs

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD /usr/local/bin/imp-cli --health-check || exit 1

ENTRYPOINT ["/usr/local/bin/imp-cli"]
CMD ["--config", "/etc/impulse/config.toml"]
```

### Building the Image

```bash
# Build image
docker build -t impulse-bbs:latest .

# Build with build arguments
docker build \
    --build-arg RUST_VERSION=1.75 \
    --build-arg BUILD_TARGET=x86_64-unknown-linux-musl \
    -t impulse-bbs:alpine .

# Multi-platform build
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    -t impulse-bbs:multi .
```

### Running with Docker

```bash
# Run single container
docker run -d \
    --name impulse-bbs \
    -p 2323:2323 \
    -p 2222:2222 \
    -v impulse-data:/var/lib/impulse \
    -v impulse-config:/etc/impulse \
    --restart unless-stopped \
    impulse-bbs:latest

# View logs
docker logs -f impulse-bbs

# Execute admin command
docker exec -it impulse-bbs imp-cli admin --list-users

# Stop and remove
docker stop impulse-bbs
docker rm impulse-bbs
```

### Docker Compose

**docker-compose.yml:**

```yaml
version: '3.8'

services:
  impulse-bbs:
    image: impulse-bbs:latest
    build:
      context: .
      dockerfile: Dockerfile
      args:
        RUST_VERSION: "1.75"
    container_name: impulse-bbs
    restart: unless-stopped
    ports:
      - "2323:2323"   # Telnet
      - "2222:2222"   # SSH
      - "8080:8080"   # WebSocket
      - "9090:9090"   # Metrics
    volumes:
      - impulse-data:/var/lib/impulse
      - impulse-config:/etc/impulse
      - ./logs:/var/log/impulse
    environment:
      - RUST_LOG=info
      - BBS_NODE_COUNT=4
      - BBS_MAX_CONNECTIONS=100
    networks:
      - impulse-net
    healthcheck:
      test: ["CMD", "/usr/local/bin/imp-cli", "--health-check"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  # Optional: Prometheus monitoring
  prometheus:
    image: prom/prometheus:latest
    container_name: impulse-prometheus
    restart: unless-stopped
    ports:
      - "9091:9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
    networks:
      - impulse-net

  # Optional: Grafana dashboards
  grafana:
    image: grafana/grafana:latest
    container_name: impulse-grafana
    restart: unless-stopped
    ports:
      - "3000:3000"
    volumes:
      - grafana-data:/var/lib/grafana
      - ./monitoring/grafana/dashboards:/etc/grafana/provisioning/dashboards
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=changeme
      - GF_INSTALL_PLUGINS=grafana-piechart-panel
    networks:
      - impulse-net
    depends_on:
      - prometheus

networks:
  impulse-net:
    driver: bridge

volumes:
  impulse-data:
  impulse-config:
  prometheus-data:
  grafana-data:
```

**Running with Compose:**

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f impulse-bbs

# Scale nodes (if multi-node support implemented)
docker-compose up -d --scale impulse-bbs=4

# Stop all services
docker-compose down

# Remove volumes (DESTRUCTIVE)
docker-compose down -v
```

---

## Kubernetes Deployment

### Namespace and Resources

**namespace.yaml:**

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: impulse-bbs
  labels:
    app: impulse
    environment: production
```

### ConfigMap

**configmap.yaml:**

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: impulse-config
  namespace: impulse-bbs
data:
  config.toml: |
    [server]
    node_count = 4
    max_connections = 100

    [telnet]
    enabled = true
    port = 2323

    [ssh]
    enabled = true
    port = 2222

    [logging]
    level = "info"
    format = "json"

    [metrics]
    enabled = true
    port = 9090
```

### PersistentVolumeClaim

**pvc.yaml:**

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: impulse-data
  namespace: impulse-bbs
spec:
  accessModes:
    - ReadWriteOnce
  storageClassName: fast-ssd
  resources:
    requests:
      storage: 50Gi
```

### Deployment

**deployment.yaml:**

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: impulse-bbs
  namespace: impulse-bbs
  labels:
    app: impulse
    component: bbs-server
spec:
  replicas: 2
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: impulse
      component: bbs-server
  template:
    metadata:
      labels:
        app: impulse
        component: bbs-server
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "9090"
        prometheus.io/path: "/metrics"
    spec:
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        fsGroup: 1000

      containers:
      - name: impulse-bbs
        image: impulse-bbs:1.0.0
        imagePullPolicy: IfNotPresent

        ports:
        - name: telnet
          containerPort: 2323
          protocol: TCP
        - name: ssh
          containerPort: 2222
          protocol: TCP
        - name: websocket
          containerPort: 8080
          protocol: TCP
        - name: metrics
          containerPort: 9090
          protocol: TCP

        env:
        - name: RUST_LOG
          value: "info"
        - name: BBS_NODE_COUNT
          valueFrom:
            configMapKeyRef:
              name: impulse-config
              key: node_count
        - name: DATABASE_PASSWORD
          valueFrom:
            secretKeyRef:
              name: impulse-secrets
              key: db-password

        volumeMounts:
        - name: data
          mountPath: /var/lib/impulse
        - name: config
          mountPath: /etc/impulse
          readOnly: true

        resources:
          requests:
            memory: "2Gi"
            cpu: "1000m"
          limits:
            memory: "4Gi"
            cpu: "2000m"

        livenessProbe:
          exec:
            command:
            - /usr/local/bin/imp-cli
            - --health-check
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3

        readinessProbe:
          exec:
            command:
            - /usr/local/bin/imp-cli
            - --health-check
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 2

      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: impulse-data
      - name: config
        configMap:
          name: impulse-config
```

### Service

**service.yaml:**

```yaml
apiVersion: v1
kind: Service
metadata:
  name: impulse-bbs
  namespace: impulse-bbs
  labels:
    app: impulse
spec:
  type: LoadBalancer
  selector:
    app: impulse
    component: bbs-server
  ports:
  - name: telnet
    port: 23
    targetPort: 2323
    protocol: TCP
  - name: ssh
    port: 22
    targetPort: 2222
    protocol: TCP
  - name: websocket
    port: 8080
    targetPort: 8080
    protocol: TCP
  sessionAffinity: ClientIP
  sessionAffinityConfig:
    clientIP:
      timeoutSeconds: 3600
```

### Ingress (Optional)

**ingress.yaml:**

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: impulse-web
  namespace: impulse-bbs
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  ingressClassName: nginx
  tls:
  - hosts:
    - bbs.example.com
    secretName: impulse-tls
  rules:
  - host: bbs.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: impulse-bbs
            port:
              number: 8080
```

### Deploying to Kubernetes

```bash
# Apply all resources
kubectl apply -f namespace.yaml
kubectl apply -f configmap.yaml
kubectl apply -f pvc.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
kubectl apply -f ingress.yaml

# Verify deployment
kubectl get all -n impulse-bbs

# View logs
kubectl logs -n impulse-bbs -l app=impulse -f

# Execute command in pod
kubectl exec -n impulse-bbs -it deployment/impulse-bbs -- \
    /usr/local/bin/imp-cli admin --list-users

# Scale deployment
kubectl scale deployment/impulse-bbs -n impulse-bbs --replicas=4

# Rolling update
kubectl set image deployment/impulse-bbs -n impulse-bbs \
    impulse-bbs=impulse-bbs:1.1.0

# Rollback
kubectl rollout undo deployment/impulse-bbs -n impulse-bbs
```

---

## Bare Metal Deployment

### Systemd Service

**/etc/systemd/system/impulse-bbs.service:**

```ini
[Unit]
Description=Impulse 7.1 BBS Server
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=bbs
Group=bbs
WorkingDirectory=/opt/impulse

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/impulse /var/log/impulse

# Resource limits
LimitNOFILE=65536
LimitNPROC=512
TasksMax=512

# Environment
Environment="RUST_LOG=info"
Environment="BBS_CONFIG=/etc/impulse/config.toml"

# Execution
ExecStart=/usr/local/bin/imp-cli --config /etc/impulse/config.toml
ExecReload=/bin/kill -HUP $MAINPID
Restart=on-failure
RestartSec=5s

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=impulse-bbs

[Install]
WantedBy=multi-user.target
```

### Installation Steps

```bash
# Create user and directories
sudo useradd -r -s /bin/false bbs
sudo mkdir -p /opt/impulse /etc/impulse /var/lib/impulse /var/log/impulse
sudo chown -R bbs:bbs /opt/impulse /var/lib/impulse /var/log/impulse

# Copy binary
sudo cp target/release/imp-cli /usr/local/bin/
sudo chmod +x /usr/local/bin/imp-cli

# Copy configuration
sudo cp config.toml /etc/impulse/
sudo chown root:bbs /etc/impulse/config.toml
sudo chmod 640 /etc/impulse/config.toml

# Install systemd service
sudo cp impulse-bbs.service /etc/systemd/system/
sudo systemctl daemon-reload

# Enable and start
sudo systemctl enable impulse-bbs
sudo systemctl start impulse-bbs

# Check status
sudo systemctl status impulse-bbs

# View logs
sudo journalctl -u impulse-bbs -f
```

### Logrotate Configuration

**/etc/logrotate.d/impulse-bbs:**

```
/var/log/impulse/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0640 bbs bbs
    sharedscripts
    postrotate
        /bin/systemctl reload impulse-bbs > /dev/null 2>&1 || true
    endscript
}
```

---

## Configuration Management

### Configuration File Structure

**/etc/impulse/config.toml:**

```toml
[server]
node_count = 4
max_connections = 100
timeout_seconds = 300
data_directory = "/var/lib/impulse"

[telnet]
enabled = true
bind_address = "0.0.0.0"
port = 2323
encoding = "cp437"

[ssh]
enabled = true
bind_address = "0.0.0.0"
port = 2222
host_key = "/etc/impulse/ssh_host_key"
banner = "Welcome to Impulse BBS"

[logging]
level = "info"
format = "json"  # or "text"
file = "/var/log/impulse/impulse.log"
max_size_mb = 100
max_backups = 10

[metrics]
enabled = true
port = 9090
path = "/metrics"

[database]
# Future: Database connection settings
# url = "sqlite:///var/lib/impulse/impulse.db"

[security]
max_login_attempts = 3
login_timeout_seconds = 60
session_timeout_minutes = 60
require_strong_passwords = true

[performance]
connection_pool_size = 10
cache_size_mb = 256
worker_threads = 0  # 0 = auto-detect

[features]
enable_file_uploads = true
enable_door_games = true
enable_messaging = true
enable_web_interface = false
```

### Environment Variable Override

```bash
# Override any config value via environment
export BBS_SERVER__NODE_COUNT=8
export BBS_TELNET__PORT=2323
export BBS_LOGGING__LEVEL=debug
export BBS_DATABASE__PASSWORD=secure_password

# Start with overrides
imp-cli --config /etc/impulse/config.toml
```

---

## Monitoring and Logging

### Prometheus Metrics

**Exposed Metrics (Port 9090):**

```
# Connection metrics
impulse_connections_total
impulse_connections_active
impulse_connections_failed_total

# Node metrics
impulse_nodes_active
impulse_nodes_capacity

# Message metrics
impulse_messages_posted_total
impulse_messages_read_total

# Performance metrics
impulse_request_duration_seconds
impulse_ansi_render_duration_seconds

# System metrics
impulse_memory_usage_bytes
impulse_cpu_usage_percent
```

### Logging Strategy

**Structured JSON Logging:**

```json
{
  "timestamp": "2025-01-15T10:30:45Z",
  "level": "INFO",
  "target": "imp_telnet::connection",
  "message": "New connection established",
  "fields": {
    "node_id": 1,
    "remote_addr": "192.168.1.100:54321",
    "session_id": "abc123"
  }
}
```

**Log Aggregation (ELK Stack):**

```yaml
# filebeat.yml
filebeat.inputs:
- type: log
  enabled: true
  paths:
    - /var/log/impulse/*.log
  json.keys_under_root: true
  json.add_error_key: true

output.elasticsearch:
  hosts: ["elasticsearch:9200"]
  index: "impulse-logs-%{+yyyy.MM.dd}"
```

---

## Backup and Recovery

### Backup Strategy

```bash
#!/bin/bash
# /opt/impulse/backup.sh

BACKUP_DIR="/backup/impulse"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
DATA_DIR="/var/lib/impulse"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Stop service (optional, for consistency)
systemctl stop impulse-bbs

# Backup data files
tar -czf "$BACKUP_DIR/data_$TIMESTAMP.tar.gz" \
    -C "$DATA_DIR" .

# Backup configuration
tar -czf "$BACKUP_DIR/config_$TIMESTAMP.tar.gz" \
    /etc/impulse/

# Restart service
systemctl start impulse-bbs

# Keep only last 30 days
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +30 -delete

# Upload to S3 (optional)
# aws s3 cp "$BACKUP_DIR/data_$TIMESTAMP.tar.gz" \
#     s3://my-bucket/impulse-backups/
```

**Cron Schedule:**

```cron
# Daily backup at 2 AM
0 2 * * * /opt/impulse/backup.sh > /var/log/impulse/backup.log 2>&1
```

### Recovery Procedure

```bash
#!/bin/bash
# Restore from backup

BACKUP_FILE="$1"
DATA_DIR="/var/lib/impulse"

# Stop service
systemctl stop impulse-bbs

# Restore data
tar -xzf "$BACKUP_FILE" -C "$DATA_DIR"

# Fix permissions
chown -R bbs:bbs "$DATA_DIR"

# Start service
systemctl start impulse-bbs

# Verify
systemctl status impulse-bbs
```

---

## Security Hardening

### Firewall Configuration

```bash
# UFW (Ubuntu)
sudo ufw allow 2323/tcp comment 'Impulse Telnet'
sudo ufw allow 2222/tcp comment 'Impulse SSH'
sudo ufw enable

# iptables
sudo iptables -A INPUT -p tcp --dport 2323 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 2222 -j ACCEPT

# Rate limiting (prevent brute force)
sudo iptables -A INPUT -p tcp --dport 2323 -m state --state NEW \
    -m recent --set --name telnet_rl
sudo iptables -A INPUT -p tcp --dport 2323 -m state --state NEW \
    -m recent --update --seconds 60 --hitcount 10 --name telnet_rl \
    -j DROP
```

### SSL/TLS Certificates

```bash
# Generate self-signed certificate (development)
openssl req -x509 -newkey rsa:4096 -nodes \
    -keyout /etc/impulse/server.key \
    -out /etc/impulse/server.crt \
    -days 365 \
    -subj "/CN=bbs.example.com"

# Production: Use Let's Encrypt
certbot certonly --standalone -d bbs.example.com
```

### Fail2Ban Integration

**/etc/fail2ban/filter.d/impulse-bbs.conf:**

```ini
[Definition]
failregex = ^.*Failed login attempt.*remote_addr.*<HOST>.*$
ignoreregex =
```

**/etc/fail2ban/jail.d/impulse-bbs.conf:**

```ini
[impulse-bbs]
enabled = true
port = 2323,2222
filter = impulse-bbs
logpath = /var/log/impulse/impulse.log
maxretry = 5
bantime = 3600
findtime = 600
```

---

## Performance Tuning

### Kernel Parameters

**/etc/sysctl.d/99-impulse.conf:**

```ini
# Increase connection limits
net.core.somaxconn = 1024
net.ipv4.tcp_max_syn_backlog = 4096

# TCP performance
net.ipv4.tcp_fin_timeout = 30
net.ipv4.tcp_keepalive_time = 300
net.ipv4.tcp_keepalive_probes = 5
net.ipv4.tcp_keepalive_intvl = 15

# File descriptor limits
fs.file-max = 100000
```

Apply with: `sudo sysctl -p /etc/sysctl.d/99-impulse.conf`

### Resource Limits

**/etc/security/limits.d/impulse.conf:**

```
bbs soft nofile 65536
bbs hard nofile 65536
bbs soft nproc 512
bbs hard nproc 512
```

---

## Upgrade and Rollback

### Zero-Downtime Upgrade (Kubernetes)

```bash
# Update image
kubectl set image deployment/impulse-bbs -n impulse-bbs \
    impulse-bbs=impulse-bbs:1.1.0

# Monitor rollout
kubectl rollout status deployment/impulse-bbs -n impulse-bbs

# Rollback if issues
kubectl rollout undo deployment/impulse-bbs -n impulse-bbs
```

### Bare Metal Upgrade

```bash
# Backup current binary
sudo cp /usr/local/bin/imp-cli /usr/local/bin/imp-cli.backup

# Deploy new binary
sudo cp target/release/imp-cli /usr/local/bin/

# Restart service
sudo systemctl restart impulse-bbs

# Monitor logs
sudo journalctl -u impulse-bbs -f

# Rollback if needed
sudo cp /usr/local/bin/imp-cli.backup /usr/local/bin/imp-cli
sudo systemctl restart impulse-bbs
```

---

## Troubleshooting

### Common Issues

**Service won't start:**
```bash
# Check logs
sudo journalctl -u impulse-bbs -n 100

# Verify configuration
imp-cli --config /etc/impulse/config.toml --validate

# Check permissions
ls -la /var/lib/impulse /etc/impulse
```

**High memory usage:**
```bash
# Check metrics
curl http://localhost:9090/metrics | grep memory

# Reduce cache size in config.toml
performance.cache_size_mb = 128
```

**Connection timeouts:**
```bash
# Check network
telnet localhost 2323

# Verify firewall
sudo ufw status
sudo iptables -L

# Check service binding
sudo ss -tulpn | grep imp-cli
```

---

**For detailed operational procedures, see also:**
- [04-development-guide.md](./04-development-guide.md) - Build and development
- [05-testing-strategy.md](./05-testing-strategy.md) - Testing before deployment
- [08-security-architecture.md](./08-security-architecture.md) - Security details

---

**Questions or deployment issues?** Open an issue in the project repository.
