# Phase 5: Community & Connectivity
## Impulse-Next BBS Post-v1.0.0 Development

**Phase**: 5 of 8
**Duration**: 6 months (Sprints 33-40)
**Version Range**: v1.1.0 → v1.2.0
**Status**: Planning
**Last Updated**: 2025-11-26

---

## Phase Overview

Phase 5 focuses on transforming Impulse-Next BBS from a standalone system into a fully networked platform. By implementing FidoNet integration, inter-BBS networking, web gateways, and RESTful APIs, this phase enables the BBS to participate in the global BBS community while providing modern access methods for contemporary users.

**Theme**: "Connecting the BBS World"

**Primary Goals**:
1. Full FidoNet integration for classic BBS networking
2. Modern web gateway for browser-based access
3. Comprehensive REST API for third-party integrations
4. Mobile-friendly web client
5. Multi-network federation support

---

## Business Objectives

### Strategic Goals
- **Community Growth**: Enable network effects through BBS-to-BBS connectivity
- **User Accessibility**: Lower barriers to entry with web/mobile access
- **Developer Ecosystem**: Foster third-party development via APIs
- **Market Differentiation**: Stand out with superior networking capabilities

### Success Metrics
- 50+ BBSes networked via FidoNet within 6 months
- 10,000+ daily API requests from third-party applications
- 5+ message networks federated (FidoNet, RetroNet, FSXnet, etc.)
- 95%+ uptime for web gateway services
- Sub-200ms average API response time

---

## Sprint Breakdown

### Sprint 33: FidoNet Integration Foundation (Weeks 1-3)
**Objective**: Implement core FidoNet addressing and packet format support

**Deliverables**:
- FTN (FidoNet Technology Network) address parser
- PKT (Packet) format implementation
- Zone:Net/Node.Point addressing system
- Basic packet creation and parsing

**Key Features**:
- Parse 5D addresses (zone:net/node.point@domain)
- Create Type 2+ PKT packets
- CRC validation for packet integrity
- Time zone handling

**Dependencies**: Phase 4 complete, stable v1.0.0 release

---

### Sprint 34: FidoNet Message Transport (Weeks 4-6)
**Objective**: Implement mailer and session management for FidoNet connections

**Deliverables**:
- Binkley-style mailer implementation
- Session protocol (handshake, authentication)
- Packet queue management
- Mail routing tables

**Key Features**:
- EMSI (Electronic Mail Standard Identification) handshake
- Crash/Hold/Normal/Direct mail priorities
- Frequency (file request) support
- TIC (file distribution) protocol

**Dependencies**: Sprint 33

---

### Sprint 35: BBS-to-BBS Networking (Weeks 7-9)
**Objective**: Enable direct BBS-to-BBS message exchange beyond FidoNet

**Deliverables**:
- Inter-BBS protocol specification
- Message synchronization engine
- Network topology management
- Routing and forwarding logic

**Key Features**:
- Custom binary protocol for efficiency
- Incremental message synchronization
- Conflict resolution for duplicate messages
- Network health monitoring

**Dependencies**: Sprint 34

---

### Sprint 36: Web Gateway Interface (Weeks 10-12)
**Objective**: Create HTTP/HTTPS gateway for browser-based BBS access

**Deliverables**:
- Web server (Axum framework)
- Session bridging (web ↔ telnet)
- ANSI-to-HTML renderer
- WebSocket for real-time updates

**Key Features**:
- HTTPS with Let's Encrypt integration
- Session persistence across page loads
- ANSI art rendering in browser
- Real-time command execution via WebSocket

**Dependencies**: Phase 2 terminal system, impulse-web crate

---

### Sprint 37: RESTful API Foundation (Weeks 13-15)
**Objective**: Provide comprehensive REST API for external integrations

**Deliverables**:
- OpenAPI 3.0 specification
- Authentication (JWT tokens)
- Rate limiting and quotas
- Core resource endpoints

**Key Features**:
- `/api/v1/users`, `/api/v1/messages`, `/api/v1/files`
- OAuth 2.0 token exchange
- Per-user and per-IP rate limits
- Comprehensive error responses (RFC 7807)

**Dependencies**: Sprint 36 web infrastructure

---

### Sprint 38: Mobile Web Client (Weeks 16-18)
**Objective**: Build responsive, mobile-optimized web interface

**Deliverables**:
- Progressive Web App (PWA)
- Touch-optimized UI
- Offline capability
- Native-like experience

**Key Features**:
- Responsive design (mobile-first)
- Service worker for offline support
- App manifest for "Add to Home Screen"
- Push notifications for new messages

**Dependencies**: Sprint 36

---

### Sprint 39: Message Network Federation (Weeks 19-21)
**Objective**: Support multiple message networks simultaneously

**Deliverables**:
- Multi-network gateway
- Network-specific rules engine
- Message duplication prevention
- Unified message routing

**Key Features**:
- FidoNet, RetroNet, FSXnet, BBSNet support
- Per-network configuration
- Cross-network message threading
- Network-specific addressing schemes

**Dependencies**: Sprint 34, 35

---

### Sprint 40: Phase 5 Integration Testing (Weeks 22-24)
**Objective**: Comprehensive testing and documentation

**Deliverables**:
- End-to-end network tests
- Performance benchmarks
- Security audit
- Documentation

**Key Features**:
- FidoNet interoperability testing
- Load testing (1000+ concurrent web users)
- API security penetration testing
- Administrator and developer guides

**Dependencies**: Sprints 33-39

---

## Technical Architecture

### FidoNet Integration

```
┌─────────────────────────────────────────────────────────┐
│                  Impulse-Next BBS                       │
│                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   Message    │  │    Mailer    │  │   Tosser     │ │
│  │     Base     │←─┤   (Binkley)  │←─┤  (Scanner)   │ │
│  └──────────────┘  └───────┬──────┘  └──────────────┘ │
│                            │                           │
│                    ┌───────▼────────┐                  │
│                    │  PKT Queue     │                  │
│                    │ (Crash/Hold)   │                  │
│                    └───────┬────────┘                  │
│                            │                           │
│                    ┌───────▼────────┐                  │
│                    │  FidoNet Node  │                  │
│                    │ (1:123/456.0)  │                  │
│                    └────────────────┘                  │
└─────────────────────────────────────────────────────────┘
                           │
                           │ FidoNet Protocol
                           │ (EMSI Handshake)
                           ▼
               ┌────────────────────────┐
               │  Remote FidoNet Nodes  │
               └────────────────────────┘
```

### Web Gateway Architecture

```
┌─────────────┐     HTTPS      ┌──────────────────┐
│   Browser   │◄──────────────►│   Axum Web       │
│  (Client)   │                 │   Server         │
└─────────────┘                 └────────┬─────────┘
                                         │
                                         │ Bridge
                                         │
                                ┌────────▼─────────┐
                                │  Session Manager │
                                │  (Telnet Bridge) │
                                └────────┬─────────┘
                                         │
                                         │ Internal
                                         │ Telnet
                                         ▼
                                ┌────────────────┐
                                │  BBS Core      │
                                │  (Terminal IO) │
                                └────────────────┘
```

### API Architecture

```
┌──────────────┐    REST API    ┌──────────────────┐
│  3rd Party   │◄──────────────►│  API Gateway     │
│  Application │   (JWT Auth)   │  (Rate Limiting) │
└──────────────┘                 └────────┬─────────┘
                                          │
                         ┌────────────────┼────────────────┐
                         │                │                │
                  ┌──────▼──────┐  ┌──────▼──────┐ ┌──────▼──────┐
                  │   User API  │  │ Message API │ │  File API   │
                  │  Controller │  │  Controller │ │  Controller │
                  └──────┬──────┘  └──────┬──────┘ └──────┬──────┘
                         │                │                │
                         └────────────────┼────────────────┘
                                          │
                                  ┌───────▼────────┐
                                  │  BBS Core      │
                                  │  (Business     │
                                  │   Logic)       │
                                  └────────────────┘
```

---

## Key Technologies

### FidoNet Stack
- **PKT Parsing**: Custom Rust parser for Type 2+ packets
- **Mailer**: Binkley-style protocol implementation
- **Compression**: ARC, ZIP support for mail bundles
- **Time Zones**: Correct timezone handling per FTS-0001

### Web Stack
- **Web Framework**: Axum 0.8+ (Tokio-based)
- **WebSocket**: tokio-tungstenite for real-time communication
- **TLS**: rustls + acme for Let's Encrypt certificates
- **Template Engine**: Tera or Askama for server-side rendering

### API Stack
- **OpenAPI**: utoipa for automatic spec generation
- **Authentication**: jsonwebtoken for JWT
- **Rate Limiting**: governor crate with Redis backend
- **Validation**: validator crate for request validation

### Network Stack
- **Async I/O**: Tokio for all network operations
- **Connection Pooling**: bb8 for database connections
- **Caching**: moka for in-memory caching
- **Message Queue**: Optional RabbitMQ/NATS for scaling

---

## Data Models

### FidoNet Address
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FidoAddress {
    pub zone: u16,      // 1-6
    pub net: u16,       // Network number
    pub node: u16,      // Node number
    pub point: u16,     // Point number (0 = boss node)
    pub domain: Option<String>,  // Optional domain
}

impl FidoAddress {
    /// Parse "1:123/456.7@fidonet" format
    pub fn parse(addr: &str) -> Result<Self, FidoError>;

    /// Format as "1:123/456.7"
    pub fn to_string(&self) -> String;
}
```

### PKT Packet
```rust
#[derive(Debug, Clone)]
pub struct PktPacket {
    pub orig_node: u16,
    pub dest_node: u16,
    pub orig_net: u16,
    pub dest_net: u16,
    pub product_code: u16,
    pub revision: u8,
    pub password: [u8; 8],
    pub messages: Vec<PktMessage>,
}

#[derive(Debug, Clone)]
pub struct PktMessage {
    pub from_node: u16,
    pub to_node: u16,
    pub from_user: String,
    pub to_user: String,
    pub subject: String,
    pub body: String,
    pub attributes: MessageAttributes,
    pub timestamp: DateTime<Utc>,
}
```

### Web Session
```rust
#[derive(Debug, Clone)]
pub struct WebSession {
    pub session_id: Uuid,
    pub user_id: i32,
    pub telnet_session: SessionId,  // Bridge to telnet session
    pub jwt_token: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub user_agent: String,
    pub ip_address: IpAddr,
}
```

---

## Security Considerations

### FidoNet Security
- **Password Authentication**: Session passwords per FTS-0006
- **Packet Validation**: CRC checks, size limits
- **Rate Limiting**: Max packets per node per hour
- **Blacklist**: Block malicious nodes

### Web Security
- **HTTPS Only**: Enforce TLS 1.3
- **CORS**: Strict origin policies
- **CSP**: Content Security Policy headers
- **XSS Protection**: Input sanitization, output encoding

### API Security
- **JWT Tokens**: Short-lived (1 hour), refresh tokens
- **Rate Limiting**: 1000 req/hour per user, 100 req/hour per IP
- **Input Validation**: Comprehensive schema validation
- **SQL Injection**: Prepared statements only

---

## Performance Targets

### FidoNet
- **Mail Bundling**: < 1 second per 100 messages
- **Packet Processing**: 1000+ messages/second
- **Session Latency**: < 500ms handshake
- **Queue Processing**: < 5 second delay

### Web Gateway
- **Page Load**: < 2 seconds (95th percentile)
- **WebSocket Latency**: < 100ms
- **Concurrent Users**: 1000+ per instance
- **Throughput**: 10,000+ requests/second

### REST API
- **Response Time**: < 50ms (median), < 200ms (95th)
- **Rate Limit**: 1000 req/hour per user
- **Payload Size**: Max 10MB per request
- **Bulk Operations**: 1000+ records per batch

---

## Testing Strategy

### FidoNet Testing
- Interoperability with Mystic BBS, Synchronet
- Packet round-trip verification
- Network stress testing (1000+ nodes)
- Error recovery testing

### Web Testing
- Cross-browser compatibility (Chrome, Firefox, Safari, Edge)
- Mobile responsiveness (iOS, Android)
- WebSocket reliability under poor network
- Load testing (10,000+ concurrent users)

### API Testing
- OpenAPI spec validation
- Authentication edge cases
- Rate limiting enforcement
- Error response consistency

---

## Migration & Compatibility

### Backward Compatibility
- v1.0.0 installations can upgrade seamlessly
- Configuration migration automated
- Existing data preserved
- Optional features (FidoNet, web) can be disabled

### FidoNet Compatibility
- FTS-0001 (Basic Packet Format) compliant
- FTS-0006 (Session Handshake) support
- FTS-0009 (Message Attributes) implementation
- FTS-5001 (Control Paragraphs) support

---

## Documentation Deliverables

### Administrator Guides
- FidoNet setup and configuration
- Web gateway deployment
- API key management
- Network troubleshooting

### Developer Guides
- API reference (OpenAPI spec)
- Authentication flow
- Rate limiting details
- Example integrations

### User Guides
- Web client tutorial
- Mobile app usage
- API client setup
- Network participation

---

## Dependencies

### Upstream Dependencies
- **Phase 4 Complete**: Stable v1.0.0 release
- **impulse-web Crate**: Web admin foundation
- **impulse-message**: Message base infrastructure

### External Dependencies
- **acme**: Let's Encrypt certificate management
- **utoipa**: OpenAPI specification generation
- **jsonwebtoken**: JWT authentication
- **governor**: Rate limiting

---

## Risks & Mitigation

### Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| FidoNet spec ambiguities | Medium | High | Reference implementation testing |
| Web scaling challenges | High | Medium | Early load testing, caching strategy |
| API versioning complexity | Medium | Low | Semantic versioning, deprecation policy |
| Network partition issues | High | Low | Automated reconnection, message queuing |

### Operational Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Certificate expiration | High | Low | Automated renewal, monitoring |
| API rate limit tuning | Medium | Medium | Gradual rollout, metrics collection |
| FidoNet network politics | Low | Medium | Multiple network support, neutrality |

---

## Success Criteria

### Functional Requirements
- [ ] FidoNet mail successfully exchanges with 3+ other BBSes
- [ ] Web gateway handles 1000+ concurrent users
- [ ] API serves 10,000+ requests/day
- [ ] Mobile PWA installable on iOS and Android
- [ ] 5+ message networks federated

### Non-Functional Requirements
- [ ] 99.9% uptime for web services
- [ ] < 200ms API response time (95th percentile)
- [ ] Zero critical security vulnerabilities
- [ ] Comprehensive documentation published

### Business Requirements
- [ ] 50+ BBSes adopt FidoNet integration
- [ ] 100+ third-party applications use API
- [ ] 10,000+ web users registered
- [ ] Community satisfaction score > 4.5/5

---

## Timeline & Milestones

| Milestone | Target Date | Deliverables |
|-----------|-------------|--------------|
| Sprint 33 Complete | Month 1 | FidoNet foundation |
| Sprint 35 Complete | Month 2 | BBS networking operational |
| Sprint 37 Complete | Month 4 | API available |
| Sprint 39 Complete | Month 5 | Federation complete |
| Sprint 40 Complete | Month 6 | Phase 5 done, v1.2.0 release |

---

## Next Phase Preview

**Phase 6: Modern Enhancements** will build on this networking foundation by adding:
- Plugin system for extensibility
- Scripting engine for customization
- Modern door game framework
- WebSocket real-time features
- OAuth/SSO integration

---

**For detailed sprint plans, see** `to-dos/phase-5-community/sprint-33-*.md` through `sprint-40-*.md`

**Related Documentation**:
- [Post-v1.0.0 Roadmap](post-v1-roadmap.md)
- [Phase 6 Overview](phase-6-overview.md)
