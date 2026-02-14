# Phase 1: Foundation

**Version Target:** v0.1.0
**Focus:** Project structure, feature flags, error handling, HTTP client with retry

## Objectives

1. Establish project structure with feature flag architecture
2. Define error handling patterns with `{Type}Error` suffix convention
3. Create HTTP client with trait-based API and retry helpers
4. Implement basic configuration with `ClientConfig`

## Deliverables

### Project Structure
```
project/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Module declarations + re-exports + prelude
│   ├── error.rs            # Error enum + manual From impls + Result alias
│   ├── inference/           # Feature: "inference" (default)
│   │   └── mod.rs
│   ├── http/               # Feature: "http" (default)
│   │   ├── mod.rs          # Re-exports: ClientConfig, OllamaClient, traits
│   │   ├── config.rs       # ClientConfig + impl Default
│   │   ├── client.rs       # OllamaClient + constructors + retry helpers
│   │   ├── endpoints.rs    # Endpoint constants
│   │   ├── api_async.rs    # OllamaApiAsync trait + impl
│   │   └── api_sync.rs     # OllamaApiSync trait + impl
│   ├── model/              # Feature: "model" (optional)
│   │   └── mod.rs
│   └── tools/              # Feature: "tools" (optional)
│       └── mod.rs
├── tests/
├── examples/
├── spec/
│   └── apis/               # API endpoint specifications
├── impl/                   # Implementation plans
├── ARCHITECTURE.md
├── CONTRIBUTING.md
├── CHANGELOG.md
└── README.md
```

### Cargo.toml Configuration
```toml
[package]
name = "library-name"
version = "0.1.0"
edition = "2024"
authors = ["Author Name <email@example.com>"]
description = "Brief description"
license = "MIT"
repository = "https://github.com/user/repo"
documentation = "https://docs.rs/library-name"
keywords = ["keyword1", "keyword2"]
categories = ["category"]

[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
reqwest = { version = "0.12", features = ["json", "blocking"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
async-trait = "0.1"

# Optional dependencies for feature-gated modules
schemars = { version = "1", optional = true }
futures = { version = "0.3", optional = true }

[dev-dependencies]
mockito = "1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

[features]
default = ["http", "inference"]
http = []
inference = []
model = ["http", "inference"]
tools = ["dep:schemars", "dep:futures"]
conveniences = ["http", "inference"]
```

### Feature Flag Architecture

Three levels of conditional compilation:

**1. Module Level** (in `lib.rs`):
```rust
#[cfg(feature = "inference")]
pub mod inference;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "model")]
pub mod model;

#[cfg(feature = "tools")]
pub mod tools;
```

**2. Struct Field Level** (in type definitions):
```rust
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[cfg(feature = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}
```

**3. Method Level** (in implementations):
```rust
impl ChatRequest {
    #[cfg(feature = "tools")]
    pub fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = Some(tools);
        self
    }
}
```

### Error Type
```rust
// src/error.rs

use std::fmt;

/// Error types for the library
///
/// Uses `{Type}Error` suffix convention for all variants.
/// Manual `From` implementations avoid exposing external crate types.
#[derive(Debug)]
pub enum Error {
    /// HTTP request/response error
    HttpError(String),

    /// HTTP status code error
    HttpStatusError(u16),

    /// JSON serialization/deserialization error
    JsonError(String),

    /// Request timeout
    TimeoutError(String),

    /// Maximum retries exceeded
    MaxRetriesExceededError(u32),

    /// URL parsing error
    UrlParseError(String),

    /// Invalid configuration
    ConfigError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            Error::HttpStatusError(status) => write!(f, "HTTP status error: {}", status),
            Error::JsonError(msg) => write!(f, "JSON error: {}", msg),
            Error::TimeoutError(msg) => write!(f, "Timeout: {}", msg),
            Error::MaxRetriesExceededError(n) => write!(f, "Max retries exceeded: {}", n),
            Error::UrlParseError(msg) => write!(f, "URL parse error: {}", msg),
            Error::ConfigError(msg) => write!(f, "Config error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Manual From implementations avoid exposing external types
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
```

### Client Configuration
```rust
// src/http/config.rs

use std::time::Duration;

/// Configuration for the HTTP client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}
```

### HTTP Client with Retry Helpers
```rust
// src/http/client.rs

use std::sync::Arc;
use reqwest::Client;

/// Thread-safe HTTP client for API communication
pub struct OllamaClient {
    pub(super) config: ClientConfig,
    pub(super) client: Arc<Client>,
}

impl OllamaClient {
    pub fn new(config: ClientConfig) -> Result<Self> { ... }

    pub fn default() -> Result<Self> {
        Self::new(ClientConfig::default())
    }

    /// Generic GET with retry logic
    pub(super) async fn get_with_retry<T: DeserializeOwned>(&self, url: &str) -> Result<T> { ... }

    /// Generic POST with retry logic (returns deserialized response)
    pub(super) async fn post_with_retry<R: Serialize, T: DeserializeOwned>(
        &self, url: &str, body: &R,
    ) -> Result<T> { ... }

    /// POST with retry logic (empty response body)
    pub(super) async fn post_empty_with_retry<R: Serialize>(
        &self, url: &str, body: &R,
    ) -> Result<()> { ... }

    /// DELETE with retry logic (empty response body)
    pub(super) async fn delete_empty_with_retry<R: Serialize>(
        &self, url: &str, body: &R,
    ) -> Result<()> { ... }

    // Blocking variants for sync API
    pub(super) fn get_blocking_with_retry<T: DeserializeOwned>(&self, url: &str) -> Result<T> { ... }
    pub(super) fn post_blocking_with_retry<R: Serialize, T: DeserializeOwned>(...) -> Result<T> { ... }
    pub(super) fn post_empty_blocking_with_retry<R: Serialize>(...) -> Result<()> { ... }
    pub(super) fn delete_empty_blocking_with_retry<R: Serialize>(...) -> Result<()> { ... }
}
```

### Endpoint Constants
```rust
// src/http/endpoints.rs

/// API endpoint paths relative to base URL
pub struct Endpoints;

impl Endpoints {
    pub const VERSION: &'static str = "/api/version";
    pub const GENERATE: &'static str = "/api/generate";
    pub const CHAT: &'static str = "/api/chat";
    pub const EMBED: &'static str = "/api/embed";
    pub const TAGS: &'static str = "/api/tags";
    pub const PS: &'static str = "/api/ps";
    pub const SHOW: &'static str = "/api/show";
    pub const CREATE: &'static str = "/api/create";
    pub const COPY: &'static str = "/api/copy";
    pub const PULL: &'static str = "/api/pull";
    pub const PUSH: &'static str = "/api/push";
    pub const DELETE: &'static str = "/api/delete";
}
```

### Trait-Based API (initial scaffold)
```rust
// src/http/api_async.rs

use async_trait::async_trait;

#[async_trait]
pub trait OllamaApiAsync {
    /// Get server version
    async fn version(&self) -> Result<VersionResponse>;
}

#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn version(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);
        self.get_with_retry(&url).await
    }
}
```

```rust
// src/http/api_sync.rs

pub trait OllamaApiSync {
    /// Get server version (blocking)
    fn version_blocking(&self) -> Result<VersionResponse>;
}

impl OllamaApiSync for OllamaClient {
    fn version_blocking(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);
        self.get_blocking_with_retry(&url)
    }
}
```

## Tasks Checklist

### Setup
- [ ] Initialize Cargo project
- [ ] Configure Cargo.toml with metadata and feature flags
- [ ] Create directory structure (src/, tests/, examples/, spec/apis/, impl/)
- [ ] Add .gitignore

### Feature Flags
- [ ] Define feature flags in Cargo.toml
- [ ] Set up conditional compilation in lib.rs
- [ ] Verify default features build correctly
- [ ] Verify `--all-features` builds correctly

### Error Handling
- [ ] Create error.rs with `{Type}Error` suffix variants
- [ ] Define Result type alias
- [ ] Add manual From implementations (reqwest, serde_json)
- [ ] Implement Display and std::error::Error

### HTTP Client
- [ ] Create ClientConfig with Default
- [ ] Create OllamaClient with Arc<Client>
- [ ] Implement generic retry helpers (get, post, post_empty, delete_empty)
- [ ] Implement blocking retry helpers
- [ ] Add Endpoint constants struct

### Trait-Based API
- [ ] Define OllamaApiAsync trait with async_trait
- [ ] Define OllamaApiSync trait with `_blocking` suffix
- [ ] Implement both traits on OllamaClient
- [ ] Implement first endpoint (version) as proof of concept

### Documentation
- [ ] Create README.md with feature flags section
- [ ] Create ARCHITECTURE.md with module structure
- [ ] Create CHANGELOG.md
- [ ] Create CONTRIBUTING.md
- [ ] Add LICENSE file

### Quality
- [ ] `cargo build --all-features` succeeds
- [ ] `cargo test --all-features` passes
- [ ] `cargo clippy --all-features -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] All types are Send + Sync

## Completion Criteria

1. `cargo build --all-features` succeeds without warnings
2. `cargo test --all-features` passes
3. `cargo clippy --all-features -- -D warnings` passes
4. All documentation files exist (README, ARCHITECTURE, CHANGELOG, CONTRIBUTING)
5. Feature flag architecture is working (default, model, tools)
6. OllamaClient can be instantiated with default and custom config
7. Retry helpers are functional
8. Version endpoint works as proof of concept

## Next Phase

After completing Phase 1, proceed to [Phase 2: Inference Types](phase-2-primitives.md).
