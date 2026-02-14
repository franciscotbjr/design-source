# Architecture - Rust Library Structure

## Module Organization

### Layered Architecture

```
src/
├── lib.rs           # Entry point: module declarations, re-exports, prelude
├── error.rs         # Error enum + Result type alias (base layer)
├── inference/       # Feature: "inference" (default) - Domain types
├── http/            # Feature: "http" (default) - HTTP client
├── tools/           # Feature: "tools" (optional) - Ergonomic function calling
├── model/           # Feature: "model" (optional) - Model management
└── conveniences/    # Feature: "conveniences" (optional) - High-level APIs
```

### Layer Responsibilities

**error.rs** (Base Layer)
- Error enum with all variants
- `From` implementations for external error types
- `Result<T>` type alias
- No dependencies on other internal modules

**inference** (Domain Layer)
- Request/response type definitions for inference operations (chat, generate, embed)
- Shared types (ModelOptions, Logprob, enums)
- Serialization/deserialization with serde
- No external dependencies beyond serde
- Feature: `inference` (default)

**http** (Infrastructure Layer)
- HTTP client wrapper (`OllamaClient`)
- Client configuration (`ClientConfig`)
- Async/sync API traits (`OllamaApiAsync`, `OllamaApiSync`)
- Endpoint constants
- Generic retry helpers
- Feature: `http` (default)

**tools** (Extension Layer)
- Tool types for function calling (ToolCall, ToolDefinition)
- `Tool` trait for type-safe tool definitions
- `ToolRegistry` for automatic dispatch
- Type erasure via `ErasedTool` / `ToolWrapper`
- Auto-generated JSON schemas via `schemars`
- Feature: `tools` (optional, requires `schemars` + `futures`)

**model** (Extension Layer)
- Model management operations (create, delete, copy, pull, push)
- Model info types (list, show, ps)
- Opt-in feature isolating destructive operations
- Feature: `model` (optional, requires `http` + `inference`)

**conveniences** (Application Layer)
- High-level helper functions
- Common use-case implementations
- Builder extensions
- Feature: `conveniences` (optional, requires `http` + `inference`)

### Dependency Flow

```
conveniences → http → inference
                ↓
tools ────────→ (independent, schemars + futures)
model ────────→ http + inference
                ↓
             external (reqwest, tokio, serde)
                ↓
             error.rs (base layer, no internal deps)
```

- **error.rs**: No internal dependencies
- **inference**: No internal dependencies (standalone types)
- **http**: Depends on inference, model (via feature flags)
- **tools**: Independent (standalone types + traits)
- **model**: Depends on http + inference
- **conveniences**: Depends on http + inference

## Feature Flags

### Configuration

```toml
[features]
default = ["http", "inference"]       # Standard usage
http = []                             # HTTP client layer
inference = []                        # Inference types (chat, generate, embed)
tools = ["dep:schemars", "dep:futures"] # Ergonomic function calling
model = ["http", "inference"]         # Model management (opt-in)
conveniences = ["http", "inference"]  # High-level APIs
```

### Three-Level Conditional Compilation

**Level 1 - Module Level** (in `lib.rs`):
```rust
#[cfg(feature = "inference")]
pub mod inference;

#[cfg(feature = "model")]
pub mod model;

#[cfg(feature = "tools")]
pub mod tools;
```

**Level 2 - Struct Field Level** (in request types):
```rust
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,

    #[cfg(feature = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}
```

**Level 3 - Method Level** (in traits and impls):
```rust
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    async fn version(&self) -> Result<VersionResponse>;
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: DeleteRequest) -> Result<()>;
}
```

### Feature Matrix

| Feature | Dependencies | Purpose | Default |
|---------|-------------|---------|---------|
| `inference` | - | Standalone inference types | Yes |
| `http` | - | HTTP client implementation | Yes |
| `tools` | `schemars`, `futures` | Tool types + function calling | No |
| `model` | `http`, `inference` | Model management operations | No |
| `conveniences` | `http`, `inference` | High-level ergonomic APIs | No |

### Usage Scenarios

```toml
# Default (inference types + HTTP client)
ollama-oxide = "0.1.0"

# With function calling
ollama-oxide = { version = "0.1.0", features = ["tools"] }

# With model management
ollama-oxide = { version = "0.1.0", features = ["model"] }

# Full featured
ollama-oxide = { version = "0.1.0", features = ["tools", "model"] }

# Types only (no HTTP client)
ollama-oxide = { version = "0.1.0", default-features = false, features = ["inference"] }
```

## Module Visibility

### Public API Surface
- Re-export key types from `lib.rs`
- Use `pub use` for clean API
- Hide implementation details with `pub(super)` within modules
- Provide a `prelude` module for convenient glob imports

### lib.rs Structure
```rust
//! Library documentation

// Base layer (always available)
mod error;
pub use error::{Error, Result};

// Feature-gated modules
#[cfg(feature = "inference")]
pub mod inference;

#[cfg(feature = "inference")]
pub use inference::{
    ChatMessage, ChatRequest, ChatResponse, ChatRole,
    GenerateRequest, GenerateResponse,
    EmbedRequest, EmbedResponse,
    // ... other types
};

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "http")]
pub use http::{ClientConfig, OllamaApiAsync, OllamaApiSync, OllamaClient};

#[cfg(feature = "model")]
pub mod model;

#[cfg(feature = "model")]
pub use model::{
    CopyRequest, CreateRequest, DeleteRequest,
    ListResponse, ShowRequest, ShowResponse,
    // ... other types
};

#[cfg(feature = "tools")]
pub mod tools;

#[cfg(feature = "tools")]
pub use tools::{ToolCall, ToolCallFunction, ToolDefinition, ToolFunction};

// Prelude for convenient imports
pub mod prelude {
    pub use crate::{Error, Result};

    #[cfg(feature = "http")]
    pub use crate::{ClientConfig, OllamaApiAsync, OllamaApiSync, OllamaClient};

    #[cfg(feature = "inference")]
    pub use crate::{ChatMessage, ChatRequest, ChatResponse, /* ... */};

    #[cfg(feature = "tools")]
    pub use crate::{ToolCall, ToolDefinition};

    #[cfg(feature = "model")]
    pub use crate::{ListResponse, ShowRequest, ShowResponse, /* ... */};
}
```

## Error Handling Architecture

### Error Enum Pattern
```rust
use thiserror::Error;

/// Error type for all library operations
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpError(String),

    #[error("HTTP status error: {0}")]
    HttpStatusError(u16),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("API error: {message}")]
    ApiError { message: String },

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrlError(#[from] url::ParseError),

    #[error("Request timeout after {0} seconds")]
    TimeoutError(u64),

    #[error("Maximum retry attempts ({0}) exceeded")]
    MaxRetriesExceededError(u32),
}
```

### Naming Convention
- All variants use `{Type}Error` suffix (e.g., `HttpError`, `TimeoutError`)
- String-wrapped errors for external types (reqwest, serde_json)
- `#[from]` only for types where original error is preserved (url::ParseError)
- Manual `From` implementations for types converted to String

### From Implementations
```rust
// Manual From - convert to String to avoid exposing external types
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}
```

### Result Type Alias
```rust
pub type Result<T> = std::result::Result<T, Error>;
```

## Configuration Pattern

### ClientConfig Struct
```rust
use std::time::Duration;

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

impl ClientConfig {
    /// Build full URL for an endpoint
    pub fn url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }
}
```

### Client Construction (Three Constructors)
```rust
use std::sync::Arc;
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct OllamaClient {
    pub(super) config: ClientConfig,
    pub(super) client: Arc<Client>,  // Thread-safe shared client
}

impl OllamaClient {
    /// Create with custom configuration
    pub fn new(config: ClientConfig) -> Result<Self> {
        // Validate URL, build reqwest client
        let client = Client::builder()
            .timeout(config.timeout)
            .build()?;
        Ok(Self { config, client: Arc::new(client) })
    }

    /// Create with custom base URL and default settings
    pub fn with_base_url(base_url: impl Into<String>) -> Result<Self> {
        Self::new(ClientConfig {
            base_url: base_url.into(),
            ..Default::default()
        })
    }

    /// Create with default configuration (localhost:11434)
    pub fn default() -> Result<Self> {
        Self::new(ClientConfig::default())
    }
}
```

### Design Decisions
- **No builder pattern** for client - `ClientConfig` struct is simple enough
- **`Arc<Client>`** for efficient cloning and thread safety
- **URL validation** in constructor (fail early)
- **`pub(super)`** for internal fields - only accessible within `http` module

## Trait-Based API Design

### Separate Async and Sync Traits
```rust
use async_trait::async_trait;

/// Async API operations
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    async fn version(&self) -> Result<VersionResponse>;
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse>;
    async fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse>;

    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    async fn copy_model(&self, request: CopyRequest) -> Result<()>;
}

/// Sync (blocking) API operations
pub trait OllamaApiSync: Send + Sync {
    fn version_blocking(&self) -> Result<VersionResponse>;
    fn chat_blocking(&self, request: ChatRequest) -> Result<ChatResponse>;
    fn generate_blocking(&self, request: GenerateRequest) -> Result<GenerateResponse>;

    #[cfg(feature = "model")]
    fn list_models_blocking(&self) -> Result<ListResponse>;
}
```

### Naming Convention
- **Async methods**: No suffix (default) - `version()`, `chat()`, `generate()`
- **Sync methods**: `_blocking` suffix - `version_blocking()`, `chat_blocking()`
- **Streaming methods** (future): `_stream` suffix - `chat_stream()`, `generate_stream()`

### Implementation with Generic Retry Helpers
```rust
#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn version(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);
        self.get_with_retry(&url).await
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = self.config.url(Endpoints::CHAT);
        self.post_with_retry(&url, &request).await
    }
}
```

### Endpoint Constants
```rust
pub struct Endpoints;

impl Endpoints {
    pub const VERSION: &'static str = "/api/version";
    pub const GENERATE: &'static str = "/api/generate";
    pub const CHAT: &'static str = "/api/chat";
    pub const EMBED: &'static str = "/api/embed";
    pub const TAGS: &'static str = "/api/tags";
    pub const PS: &'static str = "/api/ps";
    pub const SHOW: &'static str = "/api/show";
    pub const COPY: &'static str = "/api/copy";
    pub const DELETE: &'static str = "/api/delete";
    pub const CREATE: &'static str = "/api/create";
    pub const PULL: &'static str = "/api/pull";
    pub const PUSH: &'static str = "/api/push";
}
```

## Generic Retry Helpers

### Pattern
All HTTP methods use generic retry helpers with exponential backoff:

```rust
impl OllamaClient {
    /// GET with retry (returns deserialized JSON)
    pub(super) async fn get_with_retry<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    { /* ... */ }

    /// POST with retry (sends JSON, returns deserialized JSON)
    pub(super) async fn post_with_retry<R, T>(&self, url: &str, body: &R) -> Result<T>
    where
        R: serde::Serialize,
        T: serde::de::DeserializeOwned,
    { /* ... */ }

    /// POST with retry (sends JSON, expects empty 200 OK)
    pub(super) async fn post_empty_with_retry<R>(&self, url: &str, body: &R) -> Result<()>
    where
        R: serde::Serialize,
    { /* ... */ }

    /// DELETE with retry (sends JSON, expects empty 200 OK)
    pub(super) async fn delete_empty_with_retry<R>(&self, url: &str, body: &R) -> Result<()>
    where
        R: serde::Serialize,
    { /* ... */ }
}
```

### Retry Strategy
- **Exponential backoff**: `100ms * (attempt + 1)`
- **Server errors (5xx)**: Retry up to `max_retries`
- **Client errors (4xx)**: Fail immediately (no retry)
- **Network errors**: Retry up to `max_retries`
- **Async**: Uses `tokio::time::sleep`
- **Sync**: Uses `std::thread::sleep` with `reqwest::blocking::Client`

### Blocking Variants
Each async helper has a corresponding blocking variant:
- `get_with_retry` / `get_blocking_with_retry`
- `post_with_retry` / `post_blocking_with_retry`
- `post_empty_with_retry` / `post_empty_blocking_with_retry`
- `delete_empty_with_retry` / `delete_empty_blocking_with_retry`

## File Naming Conventions

| Type | File Pattern | Example |
|------|--------------|---------|
| Single type | `{type_name}.rs` | `chat_request.rs`, `chat_response.rs` |
| Module facade | `mod.rs` | Re-exports only, no logic |
| API traits | `api_async.rs`, `api_sync.rs` | Trait + impl per file |
| Client | `client.rs` | Client struct + helpers |
| Config | `config.rs` | Configuration struct |
| Endpoints | `endpoints.rs` | Endpoint constants |
| Error | `error.rs` | Error enum + From impls |
| Tests | `tests/{feature}_tests.rs` | `client_chat_tests.rs` |
| Examples | `{feature}_{variant}_{mode}.rs` | `chat_with_tools_async.rs` |
| Specs | `NN-endpoint_name.yaml` | `01-ollama_api_generate.yaml` |

### Single Concern Per File
- One primary type per file with its implementations
- `mod.rs` serves exclusively as re-export facade (no logic)
- Example: `chat_request.rs` contains `ChatRequest` struct + `impl` blocks
