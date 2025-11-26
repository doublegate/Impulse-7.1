# Phase 9: Next-Generation Platform
## Impulse-Next BBS Post-v2.0.0 Development

**Phase**: 9 of 12
**Duration**: 6 months (Sprints 65-72)
**Version Range**: v2.1.0 → v2.2.0
**Status**: Planning
**Last Updated**: 2025-11-26

---

## Phase Overview

Phase 9 transforms Impulse-Next BBS from an enterprise-ready platform into a next-generation decentralized communication hub. By integrating ActivityPub/Fediverse, Nostr protocol, and decentralized identity (DID/VC), this phase positions the BBS as a node in the open social web. Advanced terminal graphics, WebAssembly clients, and internationalization lay the foundation for global adoption.

**Theme**: "Joining the Decentralized Web"

**Primary Goals**:
1. ActivityPub/Fediverse integration for federated messaging
2. Nostr protocol support with key-based identity
3. Decentralized Identity (DID/VC) framework
4. Advanced terminal graphics (Sixel, Kitty, iTerm2)
5. WebAssembly-based browser client
6. Internationalization foundation (20+ languages)

---

## Business Objectives

### Strategic Goals
- **Decentralization Leadership**: First BBS platform with full fediverse integration
- **Global Reach**: International language support for worldwide adoption
- **Modern Access**: Browser-based client for zero-install experience
- **Future-Proof Identity**: Key-based authentication replaces passwords

### Success Metrics
- 10,000+ fediverse followers of BBS content within 6 months
- 5,000+ Nostr-based identities created
- 50+ BBSes joining ActivityPub federation
- 80% of users on graphics-capable terminals
- 20+ language translations completed
- Sub-100ms WebAssembly client latency

---

## Sprint Breakdown

### Sprint 65: ActivityPub Foundation (Weeks 1-3)
**Objective**: Implement core ActivityPub protocol for user accounts and content

**Deliverables**:
- Actor model (Person, Group, Service types)
- Inbox/Outbox endpoints (HTTP POST handlers)
- WebFinger discovery (.well-known/webfinger)
- ActivityPub JSON-LD serialization
- HTTP Signatures (RFC 9421) for authentication

**Key Features**:
- Users get ActivityPub URIs (user@bbs.example.com)
- Follow/Accept/Reject activities
- Create/Update/Delete for content
- Public/Followers/Direct visibility levels

**Testing**: Interoperability with Mastodon, Pixelfed, Pleroma

---

### Sprint 66: Fediverse Message Federation (Weeks 4-6)
**Objective**: Federate BBS messages across the fediverse

**Deliverables**:
- Note object mapping (BBS message ↔ ActivityPub Note)
- Thread/reply federation (inReplyTo chains)
- Hashtag and mention support
- Media attachments (FILE_ID.DIZ, images)
- Federated timeline view

**Key Features**:
- Post BBS messages to fediverse
- Receive fediverse replies as BBS messages
- Hashtag discovery and following
- Cross-platform @ mentions

**Success Criteria**:
- 100+ federated messages per day
- 50+ external fediverse interactions
- Zero message loss in federation

---

### Sprint 67: Nostr Protocol Integration (Weeks 7-9)
**Objective**: Implement Nostr relay and key-based identity

**Deliverables**:
- Nostr relay server (NIP-01 compliant)
- Key generation and management (secp256k1)
- Event signing and verification
- Relay subscription filters
- NIP-05 identifier verification

**Key Features**:
- Users can import Nostr keypairs
- BBS acts as Nostr relay for messages
- Cross-protocol identity (ActivityPub + Nostr)
- Lightning Network zaps for tipping

**Technologies**:
- **Crypto**: secp256k1 for key operations
- **Protocol**: WebSocket for Nostr relay
- **NIPs**: NIP-01 (basic), NIP-05 (verification), NIP-57 (zaps)

---

### Sprint 68: Decentralized Identity (DID/VC) (Weeks 10-12)
**Objective**: Implement W3C DID/VC for self-sovereign identity

**Deliverables**:
- DID document generation (did:web, did:key methods)
- Verifiable Credential issuance
- Credential verification system
- DID resolver integration
- Privacy-preserving selective disclosure

**Key Features**:
- Self-sovereign user identities
- Portable reputation credentials
- Age verification without revealing birthdate
- Cross-platform identity verification

**Use Cases**:
- BBS issues "Trusted User" credentials
- Users prove identity across BBSes
- Privacy-preserving adult content access

---

### Sprint 69: Advanced Terminal Graphics (Weeks 13-15)
**Objective**: Implement Sixel, Kitty, and iTerm2 graphics protocols

**Deliverables**:
- Sixel encoder/decoder
- Kitty Graphics Protocol implementation
- iTerm2 inline images support
- Terminal capability auto-detection
- Fallback to ANSI for legacy terminals

**Key Features**:
- Inline image viewing (file previews)
- User avatars in terminal
- Charts and graphs for statistics
- Enhanced ANSI art with graphics
- Door game enhancements

**Technologies**:
- **rasterm**: Rust library for Sixel/Kitty/iTerm2
- **image**: Image processing and format conversion
- **sixel-sys**: Low-level Sixel bindings

**Success Criteria**:
- Support 3 protocols (Sixel, Kitty, iTerm2)
- 80% of users on capable terminals
- <100ms image rendering latency

---

### Sprint 70: WebAssembly Browser Client (Weeks 16-18)
**Objective**: Create WASM-based BBS client for browsers

**Deliverables**:
- Rust → WASM compilation pipeline
- xterm.js terminal integration
- WebSocket connection handler
- Progressive Web App (PWA) manifest
- Offline mode with service worker

**Key Features**:
- Zero-install browser access
- Full BBS functionality in WASM
- Offline message reading/composing
- Add to Home Screen (mobile)
- Push notifications for new mail

**Technologies**:
- **wasm-bindgen**: Rust/JavaScript interop
- **xterm.js**: Terminal emulation
- **workbox**: Service worker management

**Success Metrics**:
- <500KB WASM bundle size
- <100ms first input delay
- 10,000+ browser sessions/month

---

### Sprint 71: Internationalization Foundation (Weeks 19-21)
**Objective**: UTF-8 encoding and locale system for 20+ languages

**Deliverables**:
- Full UTF-8 encoding throughout codebase
- Locale detection and selection
- Message catalogs (gettext/Fluent)
- Date/time/number formatting
- Translation workflow (Crowdin/Weblate)

**Key Features**:
- 20 initial languages (priority: EN, ES, FR, DE, PT, IT, RU, CN, JP, KR)
- Right-to-left (RTL) text support
- Pluralization rules per language
- Locale-specific formatting

**Technologies**:
- **ICU**: International Components for Unicode
- **fluent-rs**: Mozilla Fluent localization
- **unic**: Unicode utilities

**Success Criteria**:
- 20+ languages 80%+ translated
- Zero mojibake (encoding errors)
- <50ms locale switching

---

### Sprint 72: Phase 9 Integration Testing (Weeks 22-24)
**Objective**: Comprehensive testing and documentation

**Deliverables**:
- Fediverse interoperability testing
- Nostr relay stress testing
- Graphics protocol compatibility testing
- WASM client performance testing
- i18n completeness audit
- Security audit (DID/VC, federation)
- Administrator documentation
- Developer API documentation

**Testing Matrix**:
- ActivityPub: Mastodon, Pixelfed, Pleroma, Misskey
- Nostr: Damus, Snort, Iris, Amethyst clients
- Terminals: Kitty, WezTerm, iTerm2, xterm (Sixel)
- Browsers: Chrome, Firefox, Safari, Edge (WASM)
- Languages: 20 locales with native speakers

---

## Technical Architecture

### Decentralized Identity Stack

```
┌─────────────────────────────────────────────────────────┐
│                  Impulse-Next BBS                       │
│                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ ActivityPub  │  │    Nostr     │  │   DID/VC     │ │
│  │   Actor      │  │  Identity    │  │  Resolver    │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘ │
│         │                  │                  │         │
│         └──────────────────┼──────────────────┘         │
│                            │                            │
│                    ┌───────▼────────┐                   │
│                    │  Identity      │                   │
│                    │  Abstraction   │                   │
│                    │  Layer         │                   │
│                    └───────┬────────┘                   │
│                            │                            │
│                    ┌───────▼────────┐                   │
│                    │  User Store    │                   │
│                    │  (PostgreSQL)  │                   │
│                    └────────────────┘                   │
└─────────────────────────────────────────────────────────┘
```

### Fediverse Integration

```
┌──────────────┐     ActivityPub     ┌──────────────────┐
│ Mastodon/    │◄───────────────────►│  Impulse-Next    │
│ Fediverse    │   (HTTP + JSON-LD)  │  BBS (Actor)     │
└──────────────┘                      └────────┬─────────┘
                                               │
                                       ┌───────▼────────┐
                                       │  Inbox/Outbox  │
                                       │   (WebFinger)  │
                                       └───────┬────────┘
                                               │
                                       ┌───────▼────────┐
                                       │  Message Base  │
                                       │  (Federation)  │
                                       └────────────────┘
```

### Nostr Relay Architecture

```
┌──────────────┐     WebSocket      ┌──────────────────┐
│ Nostr Clients│◄──────────────────►│  Impulse Relay   │
│ (Damus, etc.)│   (NIP-01 Events)  │  (BBS as Relay)  │
└──────────────┘                     └────────┬─────────┘
                                              │
                                      ┌───────▼────────┐
                                      │  Event Store   │
                                      │  (PostgreSQL)  │
                                      └───────┬────────┘
                                              │
                                      ┌───────▼────────┐
                                      │  Subscription  │
                                      │  Filters       │
                                      └────────────────┘
```

---

## Key Technologies

### Decentralization Stack
- **activitypub-rs**: Rust ActivityPub library (or custom implementation)
- **jsonwebtoken**: JWT for OAuth flows
- **ssi**: Self-Sovereign Identity (DID/VC Rust library)
- **secp256k1**: Elliptic curve cryptography (Nostr keys)
- **nostr-sdk**: Nostr protocol Rust implementation

### Terminal Graphics
- **rasterm**: Sixel/Kitty/iTerm2 encoder
- **image**: Image processing (resize, format conversion)
- **sixel**: Sixel graphics encoding
- **termwiz**: Terminal capability detection

### WebAssembly
- **wasm-bindgen**: Rust/JS interop
- **wasm-pack**: WASM build tool
- **web-sys**: Web API bindings
- **js-sys**: JavaScript standard library bindings
- **xterm.js**: Terminal emulator (JavaScript)

### Internationalization
- **fluent-rs**: Mozilla Fluent localization
- **icu**: ICU bindings for Rust
- **unic**: Unicode utilities
- **encoding_rs**: Character encoding conversion

---

## Data Models

### ActivityPub Actor
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "@context")]
    pub context: Vec<String>, // ["https://www.w3.org/ns/activitystreams"]
    pub id: String,            // https://bbs.example.com/users/alice
    #[serde(rename = "type")]
    pub actor_type: String,    // "Person"
    pub name: String,          // Display name
    pub preferred_username: String, // username
    pub inbox: String,         // https://bbs.example.com/users/alice/inbox
    pub outbox: String,        // https://bbs.example.com/users/alice/outbox
    pub followers: Option<String>,
    pub following: Option<String>,
    pub public_key: PublicKey,
    pub icon: Option<Image>,   // Avatar
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String, // RSA public key for HTTP Signatures
}
```

### Nostr Event
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NostrEvent {
    pub id: String,            // 32-byte hex event ID
    pub pubkey: String,        // 32-byte hex pubkey
    pub created_at: i64,       // Unix timestamp
    pub kind: u16,             // Event kind (1 = note, 0 = metadata)
    pub tags: Vec<Vec<String>>, // [["e", "<event-id>"], ["p", "<pubkey>"]]
    pub content: String,       // Event content
    pub sig: String,           // Schnorr signature
}

impl NostrEvent {
    /// Verify event signature
    pub fn verify(&self) -> Result<bool, NostrError>;

    /// Sign event with private key
    pub fn sign(event: &mut Self, privkey: &SecretKey) -> Result<(), NostrError>;
}
```

### DID Document
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,            // did:web:bbs.example.com:users:alice
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>, // Reference to verification methods
    pub assertion_method: Option<Vec<String>>,
    pub service: Option<Vec<ServiceEndpoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub method_type: String,   // "JsonWebKey2020"
    pub controller: String,    // DID of controller
    pub public_key_jwk: JsonWebKey,
}
```

---

## Security Considerations

### Federation Security
- **HTTP Signatures**: Verify all incoming ActivityPub activities
- **Content Validation**: Sanitize federated content (XSS prevention)
- **Rate Limiting**: Per-instance limits to prevent DoS
- **Blocklists**: Defederate from abusive instances
- **Content Filtering**: User-controlled instance blocks

### Nostr Security
- **Event Verification**: Always verify Schnorr signatures
- **Relay Spam**: Proof-of-work for event submission (NIP-13)
- **Key Management**: Secure storage of private keys (keyring integration)
- **Relay Selection**: User controls which relays to trust

### DID/VC Security
- **Key Rotation**: Support key updates without breaking identity
- **Revocation**: Credential revocation mechanisms
- **Selective Disclosure**: Zero-knowledge proofs for privacy
- **Verification**: Always verify credential signatures

---

## Performance Targets

### Federation
- **ActivityPub Inbox**: < 100ms processing time per activity
- **Outbox Delivery**: < 500ms to federated instances
- **WebFinger**: < 50ms lookup time

### Nostr
- **Event Verification**: < 10ms per event
- **Subscription Filters**: < 100ms query execution
- **Relay Throughput**: 1,000+ events/second

### Graphics
- **Sixel Encoding**: < 100ms for typical image
- **Terminal Detection**: < 50ms on connection
- **Image Cache**: 90%+ hit rate

### WASM Client
- **Bundle Size**: < 500KB compressed
- **First Load**: < 2 seconds (3G connection)
- **Input Latency**: < 100ms
- **Offline Mode**: 100% functionality for cached content

---

## Testing Strategy

### Interoperability Testing
- **ActivityPub**: Mastodon 4.x, Pixelfed, Pleroma, Misskey
- **Nostr**: Damus, Snort, Iris, Amethyst, Nostros
- **DID Resolvers**: Universal Resolver, did:web, did:key

### Graphics Testing
- **Terminals**: Kitty, WezTerm, iTerm2, Ghostty, xterm
- **Protocols**: Sixel, Kitty Graphics, iTerm2 Inline
- **Fallback**: Verify ANSI fallback on legacy terminals

### WASM Testing
- **Browsers**: Chrome, Firefox, Safari, Edge (latest 2 versions)
- **Mobile**: iOS Safari, Chrome Mobile, Firefox Mobile
- **Performance**: Lighthouse score 90+ for performance
- **Offline**: Service worker functionality across browsers

### i18n Testing
- **Translation Completeness**: 80%+ for 20 languages
- **RTL Testing**: Arabic, Hebrew text rendering
- **CJK Testing**: Chinese, Japanese, Korean rendering (preview for Phase 11)
- **Locale Formatting**: Date/time/number correctness

---

## Migration & Compatibility

### Backward Compatibility
- All Phase 9 features optional (can be disabled)
- v2.0.0 LTS users can upgrade with zero config changes
- Traditional username/password auth still supported
- ANSI-only mode for legacy terminals
- Monolingual operation (English only) still possible

### Data Migration
- Automatic user DID generation on upgrade
- Opt-in for ActivityPub federation
- Nostr keypair generation (optional)
- Graphics auto-detection, no user action needed

---

## Documentation Deliverables

### Administrator Guides
- ActivityPub federation setup and troubleshooting
- Nostr relay configuration
- DID/VC key management
- Graphics protocol debugging
- WASM client deployment (CDN, self-hosted)
- Translation workflow for new languages

### Developer Guides
- ActivityPub API reference
- Nostr relay implementation details
- DID/VC integration patterns
- Terminal graphics API
- WASM client architecture
- Localization contribution guide

### User Guides
- Decentralized identity explained (non-technical)
- Nostr key management best practices
- Graphics-enabled terminal setup
- Browser client usage
- Language selection and preferences

---

## Dependencies

### Upstream Dependencies
- **Phase 8 Complete**: v2.0.0 LTS released and stable
- **impulse-web**: Web server foundation for ActivityPub endpoints
- **impulse-message**: Message base for federated content
- **impulse-user**: User management for decentralized identities

### External Dependencies
- **PostgreSQL 14+**: JSON/JSONB for ActivityPub objects
- **Redis 7+**: Caching for federation, relay events
- **Modern Browsers**: WASM support (Chrome 91+, Firefox 89+, Safari 15+)

---

## Risks & Mitigation

### Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| ActivityPub spec ambiguities | Medium | High | Reference implementations (Mastodon), extensive testing |
| Nostr protocol evolution (NIPs) | Medium | Medium | Conservative NIP adoption, versioning |
| DID/VC complexity | High | Medium | Phase as optional, comprehensive docs, expert consultation |
| Terminal graphics incompatibility | Medium | Low | Auto-detection, graceful fallback to ANSI |
| WASM performance issues | Medium | Low | Benchmarking, native client fallback option |
| Translation quality | Low | Medium | Community review, professional translators for top 5 languages |

### Operational Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Fediverse defederation | Medium | Medium | Multi-protocol strategy, own network |
| Nostr relay spam | High | Medium | Proof-of-work (NIP-13), user filtering |
| DID key loss (users) | High | Low | Key backup education, recovery mechanisms |
| Translation maintenance | Medium | High | Active translator community, tooling (Weblate) |

---

## Success Criteria

### Functional Requirements
- [ ] ActivityPub federation with 3+ major platforms (Mastodon, Pixelfed, Pleroma)
- [ ] Nostr relay operational with 1,000+ events/day
- [ ] 100+ users using DID-based authentication
- [ ] Graphics protocols working on 5+ terminals
- [ ] WASM client functional in 4 major browsers
- [ ] 20 languages 80%+ translated

### Non-Functional Requirements
- [ ] 99.9% uptime for federation endpoints
- [ ] < 100ms ActivityPub inbox processing
- [ ] < 500KB WASM bundle size
- [ ] WCAG 2.1 A compliance (AA in Phase 10)
- [ ] Zero critical security vulnerabilities

### Business Requirements
- [ ] 50+ BBSes joining ActivityPub federation
- [ ] 10,000+ fediverse followers
- [ ] 5,000+ Nostr identities created
- [ ] 20+ languages actively used
- [ ] Community satisfaction > 4.5/5

---

## Timeline & Milestones

| Milestone | Target Date | Deliverables |
|-----------|-------------|--------------|
| Sprint 65 Complete | Month 1 | ActivityPub foundation |
| Sprint 67 Complete | Month 2 | Nostr relay operational |
| Sprint 69 Complete | Month 3 | Graphics protocols working |
| Sprint 71 Complete | Month 5 | 20 languages available |
| Sprint 72 Complete | Month 6 | Phase 9 done, v2.2.0 release |

---

## Next Phase Preview

**Phase 10: Immersive Experience** will build on this decentralized foundation by adding:
- AI-powered content moderation and natural language interfaces
- WCAG 2.1 AA accessibility compliance
- Gamification systems for engagement
- Experimental VR/AR text interfaces
- Rich media in terminals (audio previews, animations)

---

**For detailed sprint plans, see** `to-dos/phase-9-nextgen/sprint-65-*.md` through `sprint-72-*.md`

**Related Documentation**:
- [Post-v2.0.0 Roadmap](post-v2-roadmap.md)
- [Phase 10 Overview](phase-10-overview.md)
- [Research Summary](../../POST-V2-ROADMAP-RESEARCH.md)
