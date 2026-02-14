# Template: Implementation Plan

**Endpoint:** METHOD /api/endpoint
**Complexity:** Simple | Medium | Complex
**Streaming:** Yes | No
**Feature:** inference | model | tools
**Status:** Planning | In Progress | Complete

---

## Overview

Brief description of what this endpoint does and its purpose in the API.

## Feature Flag Requirements

| Requirement | Value |
|-------------|-------|
| Feature gate | `feature = "inference"` (or `model`, `tools`) |
| Module location | `src/inference/` (or `src/model/`, `src/tools/`) |
| Trait methods | `OllamaApiAsync::method_name`, `OllamaApiSync::method_name_blocking` |
| Endpoint constant | `Endpoints::ENDPOINT_NAME` |
| Retry helper | `post_with_retry` (or `get_with_retry`, `post_empty_with_retry`, `delete_empty_with_retry`) |
| Cargo.toml gating | `required-features = ["feature"]` for tests/examples (if not default) |

## API Reference

### Request

**Type:** `EndpointRequest`

| Field | Type | Required | Default | Feature | Description |
|-------|------|----------|---------|---------|-------------|
| `model` | `String` | Yes | - | - | The model name |
| `prompt` | `Option<String>` | No | `None` | - | Input prompt |
| `options` | `Option<ModelOptions>` | No | `None` | - | Model parameters |
| `tools` | `Option<Vec<ToolDefinition>>` | No | `None` | `tools` | Tool definitions |

### Response

**Type:** `EndpointResponse`

| Field | Type | Description |
|-------|------|-------------|
| `model` | `String` | Model that was used |
| `response` | `String` | Generated output |
| `done` | `bool` | Completion status |

### Response Helper Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `content()` | `Option<&str>` | Convenience accessor for response text |
| `is_done()` | `bool` | Whether generation is complete |
| `total_duration_ms()` | `Option<f64>` | Duration in ms (from nanoseconds) |
| `tokens_per_second()` | `Option<f64>` | Calculated throughput metric |

### Errors

| Status | Condition | Response |
|--------|-----------|----------|
| 400 | Invalid input | `{"error": "description"}` |
| 404 | Model not found | `{"error": "model not found"}` |

---

## Implementation Plan

### Step 1: Types (src/inference/ or src/model/)

**File:** `src/inference/endpoint_request.rs` (one type per file)

#### Request Type
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModelOptions>,
    // Feature-gated field
    #[cfg(feature = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}
```

#### Response Type

**File:** `src/inference/endpoint_response.rs`

```rust
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EndpointResponse {
    pub model: String,
    #[serde(default)]
    pub response: String,
    #[serde(default)]
    pub done: bool,
}
```

#### Constructor + With-Method Chain
```rust
impl EndpointRequest {
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: Some(prompt.into()),
            options: None,
            #[cfg(feature = "tools")]
            tools: None,
        }
    }

    pub fn with_options(mut self, options: ModelOptions) -> Self {
        self.options = Some(options);
        self
    }

    #[cfg(feature = "tools")]
    pub fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = Some(tools);
        self
    }
}
```

#### Response Helper Methods
```rust
impl EndpointResponse {
    pub fn content(&self) -> Option<&str> {
        if self.response.is_empty() { None } else { Some(&self.response) }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}
```

### Step 2: Update Module Facade

**File:** `src/inference/mod.rs`

```rust
mod endpoint_request;
mod endpoint_response;

pub use endpoint_request::EndpointRequest;
pub use endpoint_response::EndpointResponse;
```

### Step 3: Add Endpoint Constant

**File:** `src/http/endpoints.rs`

```rust
impl Endpoints {
    /// METHOD /api/endpoint - Description
    pub const ENDPOINT_NAME: &'static str = "/api/endpoint";
}
```

### Step 4: Add Trait Methods

#### Async Trait (src/http/api_async.rs)

```rust
#[async_trait]
pub trait OllamaApiAsync {
    /// Method description (async)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiAsync, EndpointRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = EndpointRequest::new("model", "prompt");
    /// let response = client.endpoint_name(&request).await?;
    /// println!("{}", response.content().unwrap_or("No response"));
    /// # Ok(())
    /// # }
    /// ```
    async fn endpoint_name(&self, request: &EndpointRequest) -> Result<EndpointResponse>;
}

#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn endpoint_name(&self, request: &EndpointRequest) -> Result<EndpointResponse> {
        let url = self.config.url(Endpoints::ENDPOINT_NAME);
        self.post_with_retry(&url, request).await
    }
}
```

#### Sync Trait (src/http/api_sync.rs)

```rust
pub trait OllamaApiSync {
    /// Method description (blocking)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ollama_oxide::{OllamaClient, OllamaApiSync, EndpointRequest};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OllamaClient::default()?;
    /// let request = EndpointRequest::new("model", "prompt");
    /// let response = client.endpoint_name_blocking(&request)?;
    /// println!("{}", response.content().unwrap_or("No response"));
    /// # Ok(())
    /// # }
    /// ```
    fn endpoint_name_blocking(&self, request: &EndpointRequest) -> Result<EndpointResponse>;
}

impl OllamaApiSync for OllamaClient {
    fn endpoint_name_blocking(&self, request: &EndpointRequest) -> Result<EndpointResponse> {
        let url = self.config.url(Endpoints::ENDPOINT_NAME);
        self.post_blocking_with_retry(&url, request)
    }
}
```

### Step 5: Update lib.rs Re-exports

**File:** `src/lib.rs`

```rust
#[cfg(feature = "inference")]
pub use inference::{EndpointRequest, EndpointResponse};
```

### Step 6: Integration Tests

**File:** `tests/client_endpoint_name_tests.rs`

```rust
//! Tests for endpoint_name API methods (METHOD /api/endpoint)

use ollama_oxide::{ClientConfig, EndpointRequest, OllamaApiAsync, OllamaApiSync, OllamaClient};
use std::time::Duration;

// ============================================================================
// Async API Tests
// ============================================================================

#[tokio::test]
async fn test_endpoint_name_async_success() {
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
    let result = client.endpoint_name(&request).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_endpoint_name_async_retry_on_server_error() {
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
        .with_header("content-type", "application/json")
        .with_body(r#"{"model":"llama3","response":"Hello","done":true}"#)
        .expect(1)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 1,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = EndpointRequest::new("llama3", "prompt");
    let result = client.endpoint_name(&request).await;

    assert!(result.is_ok());
    mock_fail.assert_async().await;
    mock_success.assert_async().await;
}

// ============================================================================
// Sync API Tests
// ============================================================================

#[test]
fn test_endpoint_name_sync_success() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/api/endpoint")
        .match_body(mockito::Matcher::Json(serde_json::json!({
            "model": "llama3"
        })))
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
    let result = client.endpoint_name_blocking(&request);

    assert!(result.is_ok());
    mock.assert();
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_endpoint_name_async_max_retries_exceeded() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/endpoint")
        .with_status(500)
        .expect(3)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(1),
        max_retries: 2,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = EndpointRequest::new("llama3", "prompt");
    let result = client.endpoint_name(&request).await;

    assert!(result.is_err());
    mock.assert_async().await;
}

// ============================================================================
// Type Tests
// ============================================================================

#[test]
fn test_endpoint_request_debug_impl() {
    let request = EndpointRequest::new("test-model", "prompt");
    let debug_str = format!("{:?}", request);
    assert!(debug_str.contains("test-model"));
}

#[test]
fn test_endpoint_request_clone_impl() {
    let request = EndpointRequest::new("original", "prompt");
    let cloned = request.clone();
    assert_eq!(request, cloned);
}
```

### Step 7: Examples

**File:** `examples/endpoint_name_async.rs`

```rust
//! Example: Endpoint description (async)
//!
//! This example demonstrates how to use the endpoint API.
//!
//! Run with: cargo run --example endpoint_name_async
//!
//! Note: Requires a running Ollama server with a model installed
//! (e.g., qwen3:0.6b, llama3.2, etc.)

use ollama_oxide::{EndpointRequest, OllamaApiAsync, OllamaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;

    let model = "qwen3:0.6b";
    let request = EndpointRequest::new(model, "Hello!");

    let response = client.endpoint_name(&request).await?;
    println!("Response: {}", response.content().unwrap_or("No response"));

    Ok(())
}
```

### Step 8: Update Cargo.toml (if feature-gated)

```toml
[[test]]
name = "client_endpoint_name_tests"
required-features = ["model"]  # Only if endpoint requires non-default feature

[[example]]
name = "endpoint_name_async"
required-features = ["model"]  # Only if endpoint requires non-default feature
```

---

## Checklist

### Types (Single Concern Per File)
- [ ] Request struct in `src/{module}/endpoint_request.rs`
- [ ] Response struct in `src/{module}/endpoint_response.rs`
- [ ] Serde attributes correct (`skip_serializing_if`, `default`)
- [ ] Constructor with required fields
- [ ] With-method chain for optional fields
- [ ] Feature-gated fields with `#[cfg(feature = "...")]`
- [ ] Response helper methods (`content()`, `is_done()`, etc.)

### Module Facade
- [ ] `mod.rs` updated with `mod` + `pub use`
- [ ] `lib.rs` re-export added (with feature gate if needed)

### Trait-Based API
- [ ] Endpoint constant in `Endpoints` struct
- [ ] `OllamaApiAsync` trait method + implementation
- [ ] `OllamaApiSync` trait method + implementation (`_blocking` suffix)
- [ ] Uses correct retry helper (`get_with_retry`, `post_with_retry`, etc.)
- [ ] Doc examples with `no_run` attribute

### Tests (`tests/client_{operation}_tests.rs`)
- [ ] Async success test
- [ ] Async error test (404, etc.)
- [ ] Async retry on server error test
- [ ] Async max retries exceeded test
- [ ] Sync success test
- [ ] Sync error test
- [ ] Sync retry on server error test
- [ ] Sync max retries exceeded test
- [ ] Debug impl test
- [ ] Clone/PartialEq test
- [ ] Section separators between test groups

### Examples (`examples/{feature}_{mode}.rs`)
- [ ] Async example created
- [ ] Sync example created
- [ ] Cargo.toml `[[example]]` entries (with `required-features` if needed)

### Quality
- [ ] `cargo build --all-features` succeeds
- [ ] `cargo test --all-features` passes
- [ ] `cargo clippy --all-features` has no warnings
- [ ] `cargo fmt` applied
- [ ] Send + Sync verified for all types

---

## File Changes Summary

### New Files
| File | Description |
|------|-------------|
| `src/{module}/endpoint_request.rs` | Request type |
| `src/{module}/endpoint_response.rs` | Response type |
| `tests/client_endpoint_name_tests.rs` | Integration tests with mockito |
| `examples/endpoint_name_async.rs` | Async usage example |
| `examples/endpoint_name_sync.rs` | Sync usage example |

### Modified Files
| File | Changes |
|------|---------|
| `src/{module}/mod.rs` | Add module declaration and re-export |
| `src/lib.rs` | Add public re-export (feature-gated) |
| `src/http/endpoints.rs` | Add endpoint constant |
| `src/http/api_async.rs` | Add trait method + implementation |
| `src/http/api_sync.rs` | Add trait method + implementation |
| `Cargo.toml` | Add `[[test]]` and `[[example]]` entries |

---

## Notes

Additional implementation notes, edge cases, or decisions.

---

## References

- [API Specification](../spec/apis/NN-endpoint.yaml)
- [Architecture Guide](../ARCHITECTURE.md)
- [Related Endpoint Plan](./related-plan.md)
