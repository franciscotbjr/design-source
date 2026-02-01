# Testing - Rust Testing Strategies

## Test Organization

### Location Rules
- Unit tests: In source files with `#[cfg(test)]`
- Integration tests: In `tests/` directory
- No doc tests (per project convention)

### Directory Structure
```
src/
├── lib.rs
├── primitives/
│   ├── mod.rs
│   ├── request.rs      # Contains unit tests
│   └── response.rs     # Contains unit tests
└── http/
    └── client.rs       # Contains unit tests

tests/
├── integration/
│   ├── mod.rs
│   └── api_tests.rs
└── common/
    └── mod.rs          # Shared test utilities
```

## Unit Testing

### Test Module Pattern
```rust
// src/primitives/request.rs

pub struct Request { ... }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_valid_instance() {
        let request = Request::new("model");
        assert_eq!(request.model, "model");
    }

    #[test]
    fn test_builder_sets_all_fields() {
        let request = Request::builder()
            .model("llama3")
            .temperature(0.7)
            .build()
            .unwrap();

        assert_eq!(request.model, "llama3");
        assert_eq!(request.temperature, Some(0.7));
    }
}
```

### Test Naming Convention
```
test_<function>_<condition>_<expected>
```

Examples:
- `test_new_with_valid_input_succeeds`
- `test_parse_empty_string_returns_error`
- `test_serialize_skips_none_fields`

## Serialization Tests

### Request Serialization
```rust
#[test]
fn test_serialize_request_minimal() {
    let request = Request::new("model");
    let json = serde_json::to_string(&request).unwrap();

    assert!(json.contains(r#""model":"model""#));
    // Optional fields should not appear
    assert!(!json.contains("temperature"));
}

#[test]
fn test_serialize_request_full() {
    let request = Request {
        model: "model".to_string(),
        temperature: Some(0.7),
        stream: Some(false),
    };
    let json = serde_json::to_string(&request).unwrap();

    assert!(json.contains(r#""model":"model""#));
    assert!(json.contains(r#""temperature":0.7"#));
    assert!(json.contains(r#""stream":false"#));
}
```

### Response Deserialization
```rust
#[test]
fn test_deserialize_response_minimal() {
    let json = r#"{"result": "success"}"#;
    let response: Response = serde_json::from_str(json).unwrap();
    assert_eq!(response.result, "success");
}

#[test]
fn test_deserialize_response_with_extra_fields() {
    // Should ignore unknown fields
    let json = r#"{"result": "success", "unknown": true}"#;
    let response: Response = serde_json::from_str(json).unwrap();
    assert_eq!(response.result, "success");
}
```

## Async Testing

### tokio::test Attribute
```rust
#[tokio::test]
async fn test_async_operation() {
    let client = TestClient::new();
    let result = client.fetch().await;
    assert!(result.is_ok());
}
```

### Testing with Timeout
```rust
#[tokio::test]
async fn test_with_timeout() {
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        async_operation(),
    )
    .await;

    assert!(result.is_ok());
}
```

## Mocking

### Mock HTTP Responses
```rust
use mockito::{mock, server_url};

#[tokio::test]
async fn test_api_call() {
    let _m = mock("POST", "/api/endpoint")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"result": "success"}"#)
        .create();

    let client = Client::new(&server_url());
    let response = client.call().await.unwrap();

    assert_eq!(response.result, "success");
}
```

### Mock Error Responses
```rust
#[tokio::test]
async fn test_api_error() {
    let _m = mock("POST", "/api/endpoint")
        .with_status(400)
        .with_body(r#"{"error": "Bad request"}"#)
        .create();

    let client = Client::new(&server_url());
    let result = client.call().await;

    assert!(matches!(result, Err(Error::Api { .. })));
}
```

## Test Fixtures

### Shared Test Data
```rust
// tests/common/mod.rs

pub fn sample_request() -> Request {
    Request {
        model: "test-model".to_string(),
        prompt: "test prompt".to_string(),
        ..Default::default()
    }
}

pub fn sample_response() -> Response {
    Response {
        result: "test result".to_string(),
        done: true,
    }
}
```

### JSON Fixtures
```rust
const SAMPLE_REQUEST_JSON: &str = r#"{
    "model": "llama3",
    "prompt": "Hello"
}"#;

const SAMPLE_RESPONSE_JSON: &str = r#"{
    "result": "Hi there!",
    "done": true
}"#;
```

## Integration Testing

### Test Against Real Service
```rust
// tests/integration/api_tests.rs

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_real_api_generate() {
    let client = OllamaClient::new();

    let request = GenerateRequest::new("llama3", "Say hello");
    let response = client.generate(request).await;

    assert!(response.is_ok());
}
```

### Running Integration Tests
```bash
# Run unit tests only
cargo test

# Run integration tests (requires running service)
cargo test -- --ignored

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Test Coverage Checklist

For each type:
- [ ] Serialization with minimal fields
- [ ] Serialization with all fields
- [ ] Deserialization from valid JSON
- [ ] Deserialization handles extra fields
- [ ] Default values work correctly
- [ ] Builder pattern validation

For each HTTP method:
- [ ] Success case
- [ ] Error response handling
- [ ] Network error handling
- [ ] Timeout handling (if applicable)

## Assertion Patterns

### Common Assertions
```rust
// Equality
assert_eq!(actual, expected);
assert_ne!(actual, not_expected);

// Boolean
assert!(condition);
assert!(!condition);

// Pattern matching
assert!(matches!(result, Ok(_)));
assert!(matches!(error, Error::Validation { .. }));

// Contains
assert!(string.contains("substring"));

// Result unwrapping in tests
let value = result.expect("should succeed");
```

### Custom Assertions
```rust
fn assert_valid_response(response: &Response) {
    assert!(!response.result.is_empty());
    assert!(response.done);
}
```
