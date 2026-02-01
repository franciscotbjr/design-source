# Architecture - Rust Library Structure

## Module Organization

### Three-Layer Architecture

```
src/
├── primitives/     # Domain Layer - Types and validation
├── http/           # Infrastructure Layer - HTTP client
└── conveniences/   # Application Layer - High-level APIs
```

### Layer Responsibilities

**Primitives Layer** (Domain)
- Request/response type definitions
- Validation logic
- Serialization/deserialization
- No external dependencies beyond serde

**HTTP Layer** (Infrastructure)
- HTTP client wrapper
- Endpoint implementations
- Error handling and mapping
- Retry logic

**Conveniences Layer** (Application)
- High-level helper functions
- Common use-case implementations
- Builder extensions
- Feature-gated for optional inclusion

### Dependency Flow

```
conveniences → http → primitives
              ↓
           external (reqwest, tokio)
```

- Primitives: No internal dependencies
- HTTP: Depends on primitives
- Conveniences: Depends on http and primitives

## Feature Flags

```toml
[features]
default = ["http", "primitives"]
primitives = []           # Types only
http = ["primitives"]     # HTTP client
conveniences = ["http"]   # High-level APIs
```

## Module Visibility

### Public API Surface
- Re-export key types from lib.rs
- Use `pub use` for clean API
- Hide implementation details with `pub(crate)`

### Example lib.rs
```rust
//! Ollama API client library

pub mod primitives;
pub mod http;

#[cfg(feature = "conveniences")]
pub mod conveniences;

// Re-exports for convenience
pub use primitives::{ChatRequest, ChatResponse, GenerateRequest};
pub use http::{OllamaClient, OllamaError};
```

## Error Handling Architecture

### Error Enum Pattern
```rust
#[derive(Debug, thiserror::Error)]
pub enum LibraryError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error: {message}")]
    Api { status: u16, message: String },

    #[error("Validation error: {0}")]
    Validation(String),
}
```

### Result Type Alias
```rust
pub type Result<T> = std::result::Result<T, LibraryError>;
```

## Configuration Pattern

### Client Configuration
```rust
pub struct ClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub retry_count: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            timeout: Duration::from_secs(30),
            retry_count: 3,
        }
    }
}
```

### Builder Pattern for Client
```rust
impl Client {
    pub fn new() -> Self { ... }

    pub fn builder() -> ClientBuilder { ... }
}

impl ClientBuilder {
    pub fn base_url(mut self, url: impl Into<String>) -> Self { ... }
    pub fn timeout(mut self, duration: Duration) -> Self { ... }
    pub fn build(self) -> Result<Client> { ... }
}
```

## Async/Sync Strategy

### Provide Both Variants
```rust
impl Client {
    /// Async version (default)
    pub async fn request(&self, req: Request) -> Result<Response> { ... }

    /// Blocking version
    pub fn request_sync(&self, req: Request) -> Result<Response> { ... }
}
```

### Implementation Approach
- Async as the primary implementation
- Sync wraps async with `tokio::runtime::Runtime::block_on`
- Or use `reqwest::blocking` for sync variant

## File Naming Conventions

| Type | File Pattern | Example |
|------|--------------|---------|
| Types | `{feature}.rs` | `generate.rs`, `chat.rs` |
| Tests | In-file or `tests/` | `#[cfg(test)] mod tests` |
| Examples | `{feature}_{variant}_{mode}.rs` | `chat_tools_async.rs` |
| Specs | `NN-{endpoint}.yaml` | `01-api-generate.yaml` |
