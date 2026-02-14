# Testing - Rust Testing Strategies

## Test Organization

### Location Rules
- Unit tests: In source files with `#[cfg(test)]`
- Integration tests: In `tests/` directory (one file per operation)
- No doc tests (feature flag complexity makes doc tests hard to maintain)
- All coverage via unit + integration tests

### Directory Structure
```
src/
├── lib.rs
├── error.rs
├── inference/
│   ├── mod.rs
│   ├── chat_request.rs       # Contains #[cfg(test)] unit tests
│   ├── chat_response.rs      # Contains #[cfg(test)] unit tests
│   ├── generate_request.rs   # Contains #[cfg(test)] unit tests
│   └── ...
├── model/
│   ├── show_request.rs       # Contains #[cfg(test)] unit tests
│   └── ...
├── tools/
│   ├── tool_trait.rs          # Contains #[cfg(test)] unit tests
│   ├── erased_tool.rs         # Contains #[cfg(test)] unit tests
│   └── ...
└── http/
    ├── client.rs              # Contains #[cfg(test)] unit tests
    └── ...

tests/
├── client_async_tests.rs          # Version async tests
├── client_sync_tests.rs           # Version sync tests
├── client_chat_tests.rs           # Chat type + API tests
├── client_generate_tests.rs       # Generate type + API tests
├── client_embed_tests.rs          # Embed type + API tests
├── client_list_models_tests.rs    # List models API tests
├── client_show_model_tests.rs     # Show model API tests
├── client_copy_model_tests.rs     # Copy model API tests
├── client_delete_model_tests.rs   # Delete model API tests
├── client_create_model_tests.rs   # Create model API tests
├── client_pull_tests.rs           # Pull model API tests
├── client_push_tests.rs           # Push model API tests
├── client_config_tests.rs         # Client configuration tests
├── client_construction_tests.rs   # Client constructor tests
├── error_tests.rs                 # Error type tests
└── primitives_tests.rs            # Shared type tests
```

### File Naming Convention
```
tests/client_{operation}_tests.rs
```

## Unit Testing

### Test Module Pattern
```rust
// src/inference/chat_request.rs

pub struct ChatRequest { ... }

impl ChatRequest { ... }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_request_new() {
        let request = ChatRequest::new("model", [ChatMessage::user("Hello")]);
        assert_eq!(request.model, "model");
        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.stream, Some(false));
    }

    #[test]
    fn test_chat_request_serialize() {
        let request = ChatRequest::new("model", [ChatMessage::user("Hello")]);
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["model"], "model");
        assert_eq!(json["stream"], false);
        assert!(json.get("format").is_none()); // Omitted when None
    }

    #[test]
    fn test_chat_request_with_format() {
        let request = ChatRequest::new("model", [ChatMessage::user("Hello")])
            .with_format(FormatSetting::json());
        assert!(request.format.is_some());
    }

    #[test]
    fn test_chat_request_clone() {
        let request = ChatRequest::new("model", [ChatMessage::user("Hello")]);
        let cloned = request.clone();
        assert_eq!(request, cloned);
    }
}
```

### Test Naming Convention
```
test_{type}_{action}_{variant}
```

Examples:
- `test_chat_request_new`
- `test_chat_request_new_with_iterator`
- `test_chat_request_serialize_minimal`
- `test_chat_request_serialize_with_tools`
- `test_chat_request_with_format`
- `test_chat_request_clone`
- `test_chat_request_debug`
- `test_chat_role_serialization`
- `test_chat_role_default`

## Integration Test Structure

### Section Organization with Separators
```rust
//! Tests for chat API methods (POST /api/chat)

use ollama_oxide::{
    ChatMessage, ChatRequest, ChatResponse, ChatRole, ClientConfig,
    FormatSetting, OllamaApiAsync, OllamaApiSync, OllamaClient,
};
#[cfg(feature = "tools")]
use ollama_oxide::{ToolCall, ToolCallFunction, ToolDefinition, ToolFunction};
use serde_json::json;
use std::time::Duration;

// ============================================================================
// Type Tests (serialization, deserialization, constructors)
// ============================================================================

#[test]
fn test_chat_role_serialization() { /* ... */ }

// ============================================================================
// Async API Tests
// ============================================================================

#[tokio::test]
async fn test_chat_async_success() { /* ... */ }

// ============================================================================
// Sync API Tests
// ============================================================================

#[test]
fn test_chat_sync_success() { /* ... */ }

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_chat_async_max_retries_exceeded() { /* ... */ }

// ============================================================================
// Type Safety Tests
// ============================================================================

#[test]
fn test_chat_request_is_send_sync() { /* ... */ }

// ============================================================================
// Clone and Debug Tests
// ============================================================================

#[test]
fn test_chat_request_clone() { /* ... */ }
```

## Serialization Tests

### Request Serialization (serde_json::to_value)
```rust
#[test]
fn test_chat_request_serialization_minimal() {
    let request = ChatRequest::new("qwen3:0.6b", [ChatMessage::user("Hello")]);
    let json = serde_json::to_string(&request).unwrap();

    assert!(json.contains(r#""model":"qwen3:0.6b""#));
    assert!(json.contains(r#""stream":false"#));
    assert!(json.contains(r#""role":"user""#));
    assert!(json.contains(r#""content":"Hello""#));
}

#[test]
fn test_chat_request_serialization_with_options() {
    let request = ChatRequest::new("model", [ChatMessage::user("Hi")])
        .with_format(FormatSetting::json())
        .with_options(ModelOptions::default().with_temperature(0.7));

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["model"], "model");
    assert!(json.get("format").is_some());
    assert_eq!(json["options"]["temperature"], 0.7);
}
```

### Response Deserialization (serde_json::from_str)
```rust
#[test]
fn test_chat_response_deserialization_full() {
    let json = r#"{
        "model": "qwen3:0.6b",
        "created_at": "2025-10-17T23:14:07.414671Z",
        "message": {
            "role": "assistant",
            "content": "Hello! How can I help you today?"
        },
        "done": true,
        "done_reason": "stop",
        "total_duration": 174560334,
        "load_duration": 101397084,
        "prompt_eval_count": 11,
        "prompt_eval_duration": 13074791,
        "eval_count": 18,
        "eval_duration": 52479709
    }"#;

    let response: ChatResponse = serde_json::from_str(json).unwrap();

    assert_eq!(response.model(), Some("qwen3:0.6b"));
    assert_eq!(response.content(), Some("Hello! How can I help you today?"));
    assert!(response.is_done());
    assert_eq!(response.done_reason(), Some("stop"));
    assert_eq!(response.prompt_tokens(), Some(11));
    assert_eq!(response.completion_tokens(), Some(18));
}
```

### Response Helper Method Tests
```rust
#[test]
fn test_chat_response_duration_conversions() {
    let json = r#"{
        "total_duration": 1000000000,
        "load_duration": 500000000,
        "prompt_eval_duration": 200000000,
        "eval_duration": 300000000
    }"#;
    let response: ChatResponse = serde_json::from_str(json).unwrap();

    assert_eq!(response.total_duration_ms(), Some(1000.0));
    assert_eq!(response.load_duration_ms(), Some(500.0));
}

#[test]
fn test_chat_response_tokens_per_second() {
    let json = r#"{"eval_count": 100, "eval_duration": 2000000000}"#;
    let response: ChatResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.tokens_per_second(), Some(50.0));
}
```

## Mocking with mockito

### Async Mock Pattern (POST with JSON body)
```rust
#[tokio::test]
async fn test_chat_async_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/chat")
        .match_body(mockito::Matcher::Json(json!({
            "model": "qwen3:0.6b",
            "messages": [{"role": "user", "content": "Hello"}],
            "stream": false
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "model": "qwen3:0.6b",
            "message": {"role": "assistant", "content": "Hello!"},
            "done": true
        }"#)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = ChatRequest::new("qwen3:0.6b", [ChatMessage::user("Hello")]);
    let response = client.chat(&request).await.unwrap();

    assert_eq!(response.content(), Some("Hello!"));
    mock.assert_async().await;
}
```

### Async Mock Pattern (GET)
```rust
#[tokio::test]
async fn test_list_models_async_with_mock() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/api/tags")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "models": [{"name": "test-model", "size": 1000000}]
        }"#)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
    };

    let client = OllamaClient::new(config).unwrap();
    let response = client.list_models().await.unwrap();

    assert_eq!(response.models.len(), 1);
    assert_eq!(response.models[0].name, "test-model");
    mock.assert_async().await;
}
```

### Async Mock Pattern (DELETE with body)
```rust
#[tokio::test]
async fn test_delete_model_async_success() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("DELETE", "/api/delete")
        .match_body(mockito::Matcher::Json(json!({
            "model": "llama3.1-backup"
        })))
        .with_status(200)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = DeleteRequest::new("llama3.1-backup");
    let result = client.delete_model(&request).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
```

### Sync Mock Pattern
```rust
#[test]
fn test_chat_sync_success() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/api/chat")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "message": {"role": "assistant", "content": "Hello!"},
            "done": true
        }"#)
        .create();

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = ChatRequest::new("qwen3:0.6b", [ChatMessage::user("Hello")]);
    let response = client.chat_blocking(&request).unwrap();

    assert_eq!(response.content(), Some("Hello!"));
    mock.assert();
}
```

### Config Helper Pattern (for tests with many test functions)
```rust
fn make_config(base_url: String) -> ClientConfig {
    ClientConfig {
        base_url,
        timeout: Duration::from_secs(30),
        max_retries: 3,
    }
}

#[tokio::test]
async fn test_push_model_success() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", "/api/push")
        .with_status(200)
        .with_body(r#"{"status": "success"}"#)
        .create_async()
        .await;

    let config = make_config(server.url());
    let client = OllamaClient::new(config).unwrap();
    // ...
}
```

## Error and Retry Testing

### Error Response (4xx)
```rust
#[tokio::test]
async fn test_chat_async_model_not_found() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/chat")
        .with_status(404)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = ChatRequest::new("nonexistent", [ChatMessage::user("Hello")]);
    let result = client.chat(&request).await;

    assert!(result.is_err());
    mock.assert_async().await;
}
```

### Retry on Server Error (5xx with recovery)
```rust
#[tokio::test]
async fn test_chat_async_retry_on_server_error() {
    let mut server = mockito::Server::new_async().await;

    // First request fails with 500
    let mock_fail = server
        .mock("POST", "/api/chat")
        .with_status(500)
        .expect(1)
        .create_async()
        .await;

    // Second request succeeds
    let mock_success = server
        .mock("POST", "/api/chat")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": {"content": "Ok"}, "done": true}"#)
        .expect(1)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 1,  // Allow 1 retry
    };

    let client = OllamaClient::new(config).unwrap();
    let request = ChatRequest::new("model", [ChatMessage::user("Hello")]);
    let result = client.chat(&request).await;

    assert!(result.is_ok());

    mock_fail.assert_async().await;
    mock_success.assert_async().await;
}
```

### Max Retries Exceeded
```rust
#[tokio::test]
async fn test_delete_model_async_max_retries_exceeded() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("DELETE", "/api/delete")
        .with_status(500)
        .expect(3)  // 1 initial + 2 retries
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(1),
        max_retries: 2,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = DeleteRequest::new("model");
    let result = client.delete_model(&request).await;

    assert!(result.is_err());
    mock.assert_async().await;
}
```

## Feature-Gated Tests

### Conditional Imports
```rust
// At the top of the test file
use ollama_oxide::{ChatMessage, ChatRequest, ChatResponse, OllamaApiAsync, OllamaClient};

// Feature-gated imports
#[cfg(feature = "tools")]
use ollama_oxide::{ToolCall, ToolCallFunction, ToolDefinition, ToolFunction};
```

### Feature-Gated Unit Tests
```rust
#[cfg(feature = "tools")]
#[test]
fn test_chat_request_with_tools() {
    let tool = ToolDefinition::function("test", json!({}));
    let request = ChatRequest::new("model", [ChatMessage::user("Hi")])
        .with_tools(vec![tool]);

    assert!(request.has_tools());
    assert_eq!(request.tools().unwrap().len(), 1);
}

#[cfg(feature = "tools")]
#[test]
fn test_chat_request_with_tool() {
    let request = ChatRequest::new("model", [ChatMessage::user("Hi")])
        .with_tool(ToolDefinition::function("a", json!({})))
        .with_tool(ToolDefinition::function("b", json!({})));

    assert_eq!(request.tools().unwrap().len(), 2);
}
```

### Feature-Gated Integration Tests
```rust
#[cfg(feature = "tools")]
#[tokio::test]
async fn test_chat_async_with_tools() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/chat")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "message": {
                "role": "assistant",
                "content": "",
                "tool_calls": [
                    {"function": {"name": "get_weather", "arguments": {"location": "Paris"}}}
                ]
            },
            "done": true
        }"#)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 0,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = ChatRequest::new("model", [ChatMessage::user("Weather?")])
        .with_tools(vec![ToolDefinition::function(
            "get_weather",
            json!({"type": "object", "properties": {"location": {"type": "string"}}}),
        )]);

    let response = client.chat(&request).await.unwrap();

    assert!(response.has_tool_calls());
    let calls = response.tool_calls().unwrap();
    assert_eq!(calls[0].function_name(), Some("get_weather"));
    assert_eq!(calls[0].arguments().unwrap()["location"], "Paris");

    mock.assert_async().await;
}
```

### Feature-Gated Type Safety Tests
```rust
#[cfg(feature = "tools")]
#[test]
fn test_tool_definition_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ToolDefinition>();
}

#[cfg(feature = "tools")]
#[test]
fn test_tool_call_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ToolCall>();
}
```

## Type Safety Tests

### Send + Sync Assertion Pattern
```rust
#[test]
fn test_chat_request_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ChatRequest>();
}

#[test]
fn test_chat_response_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ChatResponse>();
}
```

## Clone, Debug, and Equality Tests

### Clone + PartialEq
```rust
#[test]
fn test_chat_request_clone() {
    let request = ChatRequest::new("model", [ChatMessage::user("Hi")])
        .with_format(FormatSetting::json());
    let cloned = request.clone();
    assert_eq!(request, cloned);
}
```

### Debug Output
```rust
#[test]
fn test_chat_message_debug() {
    let msg = ChatMessage::user("Hello");
    let debug_str = format!("{:?}", msg);
    assert!(debug_str.contains("Hello"));
    assert!(debug_str.contains("User"));
}
```

## Doc Test Convention

### No Doc Tests - Rationale
Feature flag complexity makes doc tests hard to maintain:
- Types gated behind `#[cfg(feature = "tools")]` require specific feature combinations
- Cross-feature doc examples (e.g., chat with tools) would need complex configuration
- `no_run` or `ignore` on all doc examples

### Documentation Examples Use `no_run`
```rust
/// Request body for POST /api/chat endpoint.
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::{ChatRequest, ChatMessage};
///
/// let request = ChatRequest::new("model", [
///     ChatMessage::user("Hello!")
/// ]);
/// ```
pub struct ChatRequest { ... }
```

All real test coverage exists in:
1. `#[cfg(test)] mod tests { ... }` in source files (unit tests)
2. `tests/client_{operation}_tests.rs` files (integration tests)

## Test Coverage Checklist

### For Each Type:
- [ ] Constructor with required fields (`test_{type}_new`)
- [ ] Serialization minimal (`test_{type}_serialize_minimal` or `test_{type}_serialization_minimal`)
- [ ] Serialization full / with options (`test_{type}_serialize_full`)
- [ ] Deserialization from valid JSON (`test_{type}_deserialization_full`)
- [ ] With-method chain setters (`test_{type}_with_{field}`)
- [ ] Accessor/helper methods (`test_{type}_{accessor}`)
- [ ] Clone + PartialEq (`test_{type}_clone`)
- [ ] Debug output (`test_{type}_debug`)
- [ ] Send + Sync (`test_{type}_is_send_sync`)

### For Each HTTP Operation (async):
- [ ] Success case (`test_{op}_async_success`)
- [ ] Multi-variant success (e.g., multi-turn chat)
- [ ] Error response 4xx (`test_{op}_async_model_not_found`)
- [ ] Retry on 5xx (`test_{op}_async_retry_on_server_error`)
- [ ] Max retries exceeded (`test_{op}_async_max_retries_exceeded`)

### For Each HTTP Operation (sync):
- [ ] Success case (`test_{op}_sync_success`)
- [ ] Error response 4xx (`test_{op}_sync_model_not_found`)
- [ ] Retry on 5xx (`test_{op}_sync_retry_on_server_error`)
- [ ] Max retries exceeded (`test_{op}_sync_max_retries_exceeded`)

### Feature-Gated (when applicable):
- [ ] Feature-specific with-methods (`#[cfg(feature = "tools")]`)
- [ ] Feature-specific serialization
- [ ] Feature-specific API responses
- [ ] Feature-specific Send + Sync

## Running Tests

```bash
# Run all tests (default features)
cargo test

# Run with all features
cargo test --all-features

# Run specific feature combination
cargo test --features "model,tools"

# Run specific test file
cargo test --test client_chat_tests

# Run specific test
cargo test test_chat_async_success

# Run with output
cargo test -- --nocapture

# Run clippy
cargo clippy --all-features

# Check formatting
cargo fmt --check
```

## Assertion Patterns

### Common Assertions
```rust
// Equality
assert_eq!(actual, expected);
assert_ne!(actual, not_expected);

// Boolean
assert!(condition);
assert!(!condition);

// Pattern matching for errors
assert!(result.is_err());
assert!(result.is_ok());
assert!(matches!(result, Err(Error::HttpStatusError(404))));

// JSON value checks
let json = serde_json::to_value(&request).unwrap();
assert_eq!(json["model"], "model");
assert!(json.get("format").is_none()); // Omitted when None

// String contains (for serialization)
let json_str = serde_json::to_string(&request).unwrap();
assert!(json_str.contains(r#""model":"qwen3:0.6b""#));

// Option checks
assert_eq!(response.content(), Some("Hello!"));
assert!(response.content().is_none());

// Accessor method checks
assert_eq!(response.total_duration_ms(), Some(1000.0));
assert_eq!(response.tokens_per_second(), Some(50.0));
```
