# Implementation - Development Workflow

## Implementation Plan Structure

Each feature/endpoint requires an implementation plan before coding.

### Plan File Template
Location: `impl/NN-feature-implementation-plan.md`

```markdown
# Feature Implementation Plan

**Endpoint:** METHOD /api/endpoint
**Complexity:** Simple | Medium | Complex
**Streaming:** Yes | No
**Feature Flag:** inference | model | tools
**Status:** Planning | In Progress | Complete

## Overview
Brief description of what this feature does.

## Request Type

### Fields
| Field | Type | Required | Feature | Description |
|-------|------|----------|---------|-------------|
| model | String | Yes | - | Model name |
| options | Option<ModelOptions> | No | - | Runtime options |
| tools | Option<Vec<ToolDefinition>> | No | tools | Function tools |

### Example Request
```json
{
  "model": "llama3",
  "prompt": "Hello"
}
```

## Response Type

### Fields
| Field | Type | Description |
|-------|------|-------------|
| result | String | Field description |

### Example Response
```json
{
  "result": "value"
}
```

## Implementation Steps

### 1. Types (inference/ or model/)
- [ ] Create request struct in `{type_name}_request.rs`
- [ ] Create response struct in `{type_name}_response.rs`
- [ ] Add with-method chain setters
- [ ] Add serde attributes
- [ ] Add accessor methods
- [ ] Re-export in `mod.rs`
- [ ] Re-export in `lib.rs`

### 2. HTTP Layer (http/)
- [ ] Add Endpoint constant to `endpoints.rs`
- [ ] Add async method to `OllamaApiAsync` trait
- [ ] Add blocking method to `OllamaApiSync` trait
- [ ] Implement async with retry helper
- [ ] Implement blocking with retry helper

### 3. Tests
- [ ] Request serialization tests (minimal + full)
- [ ] Response deserialization tests
- [ ] With-method chain tests
- [ ] Clone/equality tests
- [ ] HTTP client tests with mockito (tests/ directory)
- [ ] Feature-gated tests (if applicable)

### 4. Examples
- [ ] Basic usage example in `examples/`
- [ ] Register in Cargo.toml `[[example]]`

## Dependencies
- Depends on: [list features]
- Blocks: [list features]

## Notes
Additional implementation notes.
```

## Implementation Workflow

### Step 1: Write Types (One File Per Type)

**Request type** (`src/inference/feature_request.rs`):
```rust
//! Feature request type for POST /api/feature endpoint.

use serde::{Deserialize, Serialize};
use super::{FormatSetting, ModelOptions};

#[cfg(feature = "tools")]
use crate::tools::ToolDefinition;

/// Request body for POST /api/feature endpoint.
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::FeatureRequest;
///
/// let request = FeatureRequest::new("model", "Hello!");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureRequest {
    /// Required field
    pub model: String,

    /// Required field
    pub prompt: String,

    /// Optional field - omitted from JSON when None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSetting>,

    /// Feature-gated optional field
    #[cfg(feature = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// Stream control - always false for non-streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl FeatureRequest {
    /// Create a new request with required fields only.
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            format: None,
            #[cfg(feature = "tools")]
            tools: None,
            stream: Some(false),
        }
    }

    /// Set the output format.
    pub fn with_format(mut self, format: impl Into<FormatSetting>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Accessor: get the model name.
    pub fn model(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_request_new() {
        let request = FeatureRequest::new("model", "Hello");
        assert_eq!(request.model, "model");
        assert_eq!(request.prompt, "Hello");
        assert_eq!(request.stream, Some(false));
    }

    #[test]
    fn test_feature_request_serialize() {
        let request = FeatureRequest::new("model", "Hello");
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["model"], "model");
        assert_eq!(json["stream"], false);
        assert!(json.get("format").is_none()); // Omitted when None
    }

    #[test]
    fn test_feature_request_with_format() {
        let request = FeatureRequest::new("model", "Hello")
            .with_format(FormatSetting::json());
        assert!(request.format.is_some());
    }

    #[test]
    fn test_feature_request_clone() {
        let request = FeatureRequest::new("model", "Hello");
        let cloned = request.clone();
        assert_eq!(request, cloned);
    }
}
```

**Response type** (`src/inference/feature_response.rs`):
```rust
//! Feature response type for POST /api/feature endpoint.

use serde::{Deserialize, Serialize};

/// Response body from POST /api/feature endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureResponse {
    /// The generated content
    pub response: String,

    /// Whether generation is complete
    pub done: bool,

    /// Total duration in nanoseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<u64>,
}

impl FeatureResponse {
    /// Get the response content.
    pub fn content(&self) -> &str {
        &self.response
    }

    /// Check if generation is complete.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Total duration in milliseconds.
    pub fn total_duration_ms(&self) -> Option<f64> {
        self.total_duration.map(|ns| ns as f64 / 1_000_000.0)
    }
}
```

### Step 2: Add to Module Facade
```rust
// src/inference/mod.rs - add mod + pub use (facade only)
mod feature_request;
mod feature_response;

pub use feature_request::FeatureRequest;
pub use feature_response::FeatureResponse;
```

```rust
// src/lib.rs - add re-export
#[cfg(feature = "inference")]
pub use inference::{FeatureRequest, FeatureResponse, /* ... */};
```

### Step 3: Add Endpoint Constant
```rust
// src/http/endpoints.rs
impl Endpoints {
    pub const FEATURE: &'static str = "/api/feature";
}
```

### Step 4: Add to API Traits
```rust
// src/http/api_async.rs
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    // ... existing methods ...

    /// Feature operation (async)
    async fn feature(&self, request: FeatureRequest) -> Result<FeatureResponse>;
}

#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn feature(&self, request: FeatureRequest) -> Result<FeatureResponse> {
        let url = self.config.url(Endpoints::FEATURE);
        self.post_with_retry(&url, &request).await
    }
}
```

```rust
// src/http/api_sync.rs
pub trait OllamaApiSync: Send + Sync {
    // ... existing methods ...

    /// Feature operation (blocking)
    fn feature_blocking(&self, request: FeatureRequest) -> Result<FeatureResponse>;
}

impl OllamaApiSync for OllamaClient {
    fn feature_blocking(&self, request: FeatureRequest) -> Result<FeatureResponse> {
        let url = self.config.url(Endpoints::FEATURE);
        self.post_blocking_with_retry(&url, &request)
    }
}
```

### Step 5: Write Integration Tests
```rust
// tests/client_feature_tests.rs
use mockito;
use ollama_oxide::{OllamaClient, OllamaApiAsync, FeatureRequest};

#[tokio::test]
async fn test_feature_async_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/feature")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"response":"Hello!","done":true}"#)
        .create_async()
        .await;

    let client = OllamaClient::with_base_url(server.url()).unwrap();
    let request = FeatureRequest::new("model", "Hello");
    let response = client.feature(request).await.unwrap();

    assert_eq!(response.content(), "Hello!");
    assert!(response.is_done());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_feature_async_server_error_retries() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/feature")
        .with_status(500)
        .expect_at_least(2)
        .create_async()
        .await;

    let client = OllamaClient::with_base_url(server.url()).unwrap();
    let request = FeatureRequest::new("model", "Hello");
    let result = client.feature(request).await;

    assert!(result.is_err());
    mock.assert_async().await;
}
```

### Step 6: Create Example
```rust
// examples/feature_async.rs
//! Example: Basic feature usage
//!
//! Requires a running Ollama instance.
//! Run with: cargo run --example feature_async

use ollama_oxide::{OllamaClient, OllamaApiAsync, FeatureRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;

    let request = FeatureRequest::new("qwen3:0.6b", "Hello, how are you?");
    let response = client.feature(request).await?;

    println!("Response: {}", response.content());

    if let Some(ms) = response.total_duration_ms() {
        println!("Duration: {:.1}ms", ms);
    }

    Ok(())
}
```

```toml
# Cargo.toml
[[example]]
name = "feature_async"
required-features = []  # or ["model"] if feature-gated
```

## Feature-Gated Implementation

### When to Use Feature Gates

| Module | Feature | When to Gate |
|--------|---------|-------------|
| inference | `inference` | Always (default feature) |
| model | `model` | Model management operations |
| tools | `tools` | Tool types and function calling |
| http | `http` | HTTP client (default feature) |

### Gating Trait Methods

When an endpoint requires a feature (e.g., `model`):

```rust
// In trait definition
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    // Always available (inference feature)
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    // Only with "model" feature
    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: DeleteRequest) -> Result<()>;
}

// In trait implementation
#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = self.config.url(Endpoints::CHAT);
        self.post_with_retry(&url, &request).await
    }

    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse> {
        let url = self.config.url(Endpoints::TAGS);
        self.get_with_retry(&url).await
    }

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: DeleteRequest) -> Result<()> {
        let url = self.config.url(Endpoints::DELETE);
        self.delete_empty_with_retry(&url, &request).await
    }
}
```

### Gating Imports
```rust
// In api_async.rs - conditional imports for feature-gated types
use crate::{ChatRequest, ChatResponse, Result};

#[cfg(feature = "model")]
use crate::{DeleteRequest, ListResponse, ShowRequest, ShowResponse};
```

### Gating Retry Helpers
```rust
// In client.rs - some helpers only needed with certain features
impl OllamaClient {
    // Always available
    pub(super) async fn get_with_retry<T>(&self, url: &str) -> Result<T> { ... }
    pub(super) async fn post_with_retry<R, T>(&self, url: &str, body: &R) -> Result<T> { ... }

    // Only needed for model operations (copy, delete)
    #[cfg(feature = "model")]
    pub(super) async fn post_empty_with_retry<R>(&self, url: &str, body: &R) -> Result<()> { ... }

    #[cfg(feature = "model")]
    pub(super) async fn delete_empty_with_retry<R>(&self, url: &str, body: &R) -> Result<()> { ... }
}
```

## Endpoint Complexity Categories

### Simple (GET, no body)
```rust
// version, list_models, list_running_models
async fn version(&self) -> Result<VersionResponse> {
    let url = self.config.url(Endpoints::VERSION);
    self.get_with_retry(&url).await
}
```

### Medium (POST, body → JSON response)
```rust
// chat, generate, embed, show, create, pull, push
async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
    let url = self.config.url(Endpoints::CHAT);
    self.post_with_retry(&url, &request).await
}
```

### Empty Response (POST/DELETE, body → 200 OK)
```rust
// copy, delete
async fn copy_model(&self, request: CopyRequest) -> Result<()> {
    let url = self.config.url(Endpoints::COPY);
    self.post_empty_with_retry(&url, &request).await
}

async fn delete_model(&self, request: DeleteRequest) -> Result<()> {
    let url = self.config.url(Endpoints::DELETE);
    self.delete_empty_with_retry(&url, &request).await
}
```

## Checklist Before Moving On

- [ ] Types compile without warnings
- [ ] Single concern per file (one type per .rs file)
- [ ] mod.rs only contains re-exports
- [ ] lib.rs re-exports all public types
- [ ] Serde serialization works correctly (skip_serializing_if for Option)
- [ ] With-method chain setters for all optional fields
- [ ] Accessor methods for key fields
- [ ] Endpoint constant added to Endpoints struct
- [ ] Async trait method + implementation
- [ ] Blocking trait method + implementation
- [ ] Feature gates applied correctly (if applicable)
- [ ] Unit tests in source file (#[cfg(test)])
- [ ] Integration tests in tests/ with mockito
- [ ] Example in examples/ with Cargo.toml entry
- [ ] Documentation complete (module doc, type doc, method doc)
- [ ] CHANGELOG updated
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes
- [ ] `cargo fmt --check` passes

## Streaming Implementation (Future - Phase 2)

For streaming endpoints:

```rust
use futures::Stream;
use std::pin::Pin;

// In trait
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    /// Non-streaming (default)
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    /// Streaming variant
    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatResponse>> + Send>>>;
}
```

## Common Patterns

### Optional Field Handling
```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub field: Option<Type>,
```

### Default Values
```rust
#[serde(default = "default_value")]
pub field: Type,

fn default_value() -> Type { ... }
```

### Enum Fields
```rust
#[serde(rename_all = "lowercase")]
pub enum Variant { A, B, C }
```

### Flexible Input
```rust
pub fn new(model: impl Into<String>) -> Self {
    Self { model: model.into() }
}

pub fn new<M, I>(model: M, messages: I) -> Self
where
    M: Into<String>,
    I: IntoIterator<Item = ChatMessage>,
{ ... }
```

### Feature-Gated Constructor Fields
```rust
impl Request {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            format: None,
            #[cfg(feature = "tools")]
            tools: None,
            stream: Some(false),
        }
    }
}
```
