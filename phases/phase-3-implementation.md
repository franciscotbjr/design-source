# Phase 3: API Implementation

**Version Target:** v0.3.0
**Focus:** Trait-based API, async/sync variants, retry helpers, endpoint constants

## Objectives

1. Implement trait methods for all endpoints on `OllamaApiAsync` and `OllamaApiSync`
2. Use generic retry helpers for all HTTP operations
3. Use `_blocking` suffix for all sync methods
4. Add streaming support (where applicable)

## Prerequisites

- Phase 1 completed (foundation, client, retry helpers)
- Phase 2 completed (all types defined and tested)
- All types defined and tested

## Implementation Order

Process endpoints by complexity:

| Order | Complexity | Endpoints | Retry Helper |
|-------|-----------|-----------|--------------|
| 1 | Simple GET | version, tags, ps | `get_with_retry` |
| 2 | Simple POST (empty response) | copy, delete | `post_empty_with_retry`, `delete_empty_with_retry` |
| 3 | Medium POST | show, embed | `post_with_retry` |
| 4 | Complex POST | generate, chat | `post_with_retry` |
| 5 | Complex POST (model ops) | create, pull, push | `post_with_retry` |

## Trait-Based API Pattern

### Async Trait Definition

```rust
// src/http/api_async.rs

use async_trait::async_trait;
use crate::{Result, VersionResponse, ChatRequest, ChatResponse};

#[async_trait]
pub trait OllamaApiAsync {
    /// Get the Ollama server version
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let version = client.version().await?;
    /// println!("Version: {}", version.version);
    /// # Ok(())
    /// # }
    /// ```
    async fn version(&self) -> Result<VersionResponse>;

    /// Send a chat request
    async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse>;

    // Feature-gated methods
    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    async fn copy_model(&self, request: &CopyRequest) -> Result<()>;

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: &DeleteRequest) -> Result<()>;
}
```

### Async Implementation

```rust
#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn version(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);
        self.get_with_retry(&url).await
    }

    async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse> {
        let url = self.config.url(Endpoints::CHAT);
        self.post_with_retry(&url, request).await
    }

    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse> {
        let url = self.config.url(Endpoints::TAGS);
        self.get_with_retry(&url).await
    }

    #[cfg(feature = "model")]
    async fn copy_model(&self, request: &CopyRequest) -> Result<()> {
        let url = self.config.url(Endpoints::COPY);
        self.post_empty_with_retry(&url, request).await
    }

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: &DeleteRequest) -> Result<()> {
        let url = self.config.url(Endpoints::DELETE);
        self.delete_empty_with_retry(&url, request).await
    }
}
```

### Sync Trait Definition (`_blocking` suffix)

```rust
// src/http/api_sync.rs

pub trait OllamaApiSync {
    /// Get the Ollama server version (blocking)
    fn version_blocking(&self) -> Result<VersionResponse>;

    /// Send a chat request (blocking)
    fn chat_blocking(&self, request: &ChatRequest) -> Result<ChatResponse>;

    #[cfg(feature = "model")]
    fn list_models_blocking(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    fn copy_model_blocking(&self, request: &CopyRequest) -> Result<()>;

    #[cfg(feature = "model")]
    fn delete_model_blocking(&self, request: &DeleteRequest) -> Result<()>;
}
```

### Sync Implementation

```rust
impl OllamaApiSync for OllamaClient {
    fn version_blocking(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);
        self.get_blocking_with_retry(&url)
    }

    fn chat_blocking(&self, request: &ChatRequest) -> Result<ChatResponse> {
        let url = self.config.url(Endpoints::CHAT);
        self.post_blocking_with_retry(&url, request)
    }

    #[cfg(feature = "model")]
    fn list_models_blocking(&self) -> Result<ListResponse> {
        let url = self.config.url(Endpoints::TAGS);
        self.get_blocking_with_retry(&url)
    }

    #[cfg(feature = "model")]
    fn copy_model_blocking(&self, request: &CopyRequest) -> Result<()> {
        let url = self.config.url(Endpoints::COPY);
        self.post_empty_blocking_with_retry(&url, request)
    }

    #[cfg(feature = "model")]
    fn delete_model_blocking(&self, request: &DeleteRequest) -> Result<()> {
        let url = self.config.url(Endpoints::DELETE);
        self.delete_empty_blocking_with_retry(&url, request)
    }
}
```

## Generic Retry Helpers

All HTTP operations use generic retry helpers defined in `client.rs`:

| Helper | HTTP Method | Response | Use Case |
|--------|-----------|----------|----------|
| `get_with_retry<T>` | GET | Deserialized `T` | version, tags, ps |
| `post_with_retry<R, T>` | POST | Deserialized `T` | chat, generate, embed, show |
| `post_empty_with_retry<R>` | POST | `()` | copy |
| `delete_empty_with_retry<R>` | DELETE | `()` | delete |

### Retry Logic Pattern

```rust
pub(super) async fn post_with_retry<R, T>(&self, url: &str, body: &R) -> Result<T>
where
    R: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    for attempt in 0..=self.config.max_retries {
        match self.client.post(url).json(body).send().await {
            Ok(response) => {
                if response.status().is_server_error() && attempt < self.config.max_retries {
                    tokio::time::sleep(Duration::from_millis(100 * (attempt as u64 + 1))).await;
                    continue;
                }
                if response.status().is_success() {
                    return response.json().await.map_err(Into::into);
                }
                return Err(Error::HttpStatusError(response.status().as_u16()));
            }
            Err(_e) => {
                if attempt < self.config.max_retries {
                    tokio::time::sleep(Duration::from_millis(100 * (attempt as u64 + 1))).await;
                }
            }
        }
    }
    Err(Error::MaxRetriesExceededError(self.config.max_retries))
}
```

## Endpoint Constant Usage

All methods reference endpoints via `Endpoints` constants:

```rust
// NOT this:
let url = format!("{}/api/chat", self.config.base_url);

// THIS:
let url = self.config.url(Endpoints::CHAT);
```

## Streaming Endpoints (Future)

```rust
// src/http/api_async.rs

#[async_trait]
pub trait OllamaApiAsync {
    // ... non-streaming methods ...

    /// Generate with streaming
    async fn generate_stream(
        &self,
        request: &GenerateRequest,
    ) -> Result<impl Stream<Item = Result<GenerateResponse>>>;
}
```

## Module Organization

```
src/http/
├── mod.rs           # Re-exports: ClientConfig, OllamaClient, traits
├── config.rs        # ClientConfig + impl Default
├── client.rs        # OllamaClient + constructors + retry helpers
├── endpoints.rs     # Endpoint constants
├── api_async.rs     # OllamaApiAsync trait + impl for OllamaClient
└── api_sync.rs      # OllamaApiSync trait + impl for OllamaClient
```

**Key:** Methods are defined in the trait, not as direct `impl Client` blocks. All API surface is through the traits.

## Tasks Checklist

### Per Endpoint
- [ ] Write implementation plan (in `impl/`)
- [ ] Add endpoint constant to `Endpoints`
- [ ] Add async trait method to `OllamaApiAsync`
- [ ] Add async implementation using retry helper
- [ ] Add sync trait method to `OllamaApiSync` (`_blocking` suffix)
- [ ] Add sync implementation using blocking retry helper
- [ ] Feature-gate if model/tools endpoint

### Integration Tests (`tests/client_{operation}_tests.rs`)
- [ ] Async success test with mockito
- [ ] Async error test (404, etc.)
- [ ] Async retry on server error (mock_fail + mock_success with expect(1))
- [ ] Async max retries exceeded (expect(N+1))
- [ ] Sync success test
- [ ] Sync error test
- [ ] Sync retry test
- [ ] Sync max retries exceeded test
- [ ] Section separators between test groups

### Examples (`examples/{feature}_{mode}.rs`)
- [ ] Async example
- [ ] Sync example
- [ ] Cargo.toml `[[example]]` with `required-features` if needed
- [ ] Cargo.toml `[[test]]` with `required-features` if needed

### Quality
- [ ] All endpoints implemented with both async and sync
- [ ] Retry logic consistent across all helpers
- [ ] `cargo build --all-features` succeeds
- [ ] `cargo test --all-features` passes
- [ ] `cargo clippy --all-features` passes
- [ ] `cargo fmt` applied
- [ ] Doc examples with `no_run`

## Testing Strategy

### Integration Tests with Mockito

**File:** `tests/client_{operation}_tests.rs`

```rust
//! Tests for operation API methods (METHOD /api/endpoint)

use ollama_oxide::{ClientConfig, EndpointRequest, OllamaApiAsync, OllamaApiSync, OllamaClient};
use std::time::Duration;

// ============================================================================
// Async API Tests
// ============================================================================

#[tokio::test]
async fn test_operation_async_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/endpoint")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "model": "llama3"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"model":"llama3","response":"Hello","done":true}"#)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = EndpointRequest::new("llama3", "prompt");
    let result = client.operation(&request).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

// ============================================================================
// Sync API Tests
// ============================================================================

#[test]
fn test_operation_sync_success() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/api/endpoint")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"model":"llama3","response":"Hello","done":true}"#)
        .create();

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = EndpointRequest::new("llama3", "prompt");
    let result = client.operation_blocking(&request);

    assert!(result.is_ok());
    mock.assert();
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_operation_async_retry_on_server_error() {
    let mut server = mockito::Server::new_async().await;

    let mock_fail = server
        .mock("POST", "/api/endpoint")
        .with_status(500)
        .expect(1)
        .create_async()
        .await;

    let mock_success = server
        .mock("POST", "/api/endpoint")
        .with_status(200)
        .with_body(r#"{"done":true}"#)
        .expect(1)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 1,
    };

    let client = OllamaClient::new(config).unwrap();
    let result = client.operation(&request).await;

    assert!(result.is_ok());
    mock_fail.assert_async().await;
    mock_success.assert_async().await;
}
```

## Examples

### Async Example
```rust
//! Example: Chat completion (async)
//!
//! Run with: cargo run --example chat_async

use ollama_oxide::{ChatMessage, ChatRequest, OllamaApiAsync, OllamaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;

    let request = ChatRequest::new(
        "qwen3:0.6b",
        [ChatMessage::user("Hello!")],
    );

    let response = client.chat(&request).await?;
    println!("{}", response.content().unwrap_or("No response"));

    Ok(())
}
```

### Sync Example
```rust
//! Example: Chat completion (sync)
//!
//! Run with: cargo run --example chat_sync

use ollama_oxide::{ChatMessage, ChatRequest, OllamaApiSync, OllamaClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;

    let request = ChatRequest::new(
        "qwen3:0.6b",
        [ChatMessage::user("Hello!")],
    );

    let response = client.chat_blocking(&request)?;
    println!("{}", response.content().unwrap_or("No response"));

    Ok(())
}
```

## Completion Criteria

1. All endpoints have `OllamaApiAsync` trait methods + implementation
2. All endpoints have `OllamaApiSync` trait methods + implementation (`_blocking` suffix)
3. All methods use generic retry helpers and `Endpoints` constants
4. Feature-gated methods for model/tools operations
5. Integration tests pass with mockito (async, sync, retry, max retries)
6. Examples compile and demonstrate real usage
7. `cargo test --all-features` passes

## Next Phase

After completing Phase 3, proceed to [Phase 4: Conveniences](phase-4-conveniences.md).
