# Template: Implementation Plan

**Endpoint:** METHOD /api/endpoint
**Complexity:** Simple | Medium | Complex
**Streaming:** Yes | No
**Status:** Planning | In Progress | Complete

---

## Overview

Brief description of what this endpoint does and its purpose in the API.

## API Reference

### Request

**Type:** `EndpointRequest`

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `model` | `String` | Yes | - | The model name |
| `prompt` | `Option<String>` | No | `None` | Input prompt |
| `options` | `Option<Options>` | No | `None` | Model parameters |

### Response

**Type:** `EndpointResponse`

| Field | Type | Description |
|-------|------|-------------|
| `model` | `String` | Model that was used |
| `response` | `String` | Generated output |
| `done` | `bool` | Completion status |

### Errors

| Status | Condition | Response |
|--------|-----------|----------|
| 400 | Invalid input | `{"error": "description"}` |
| 404 | Model not found | `{"error": "model not found"}` |

---

## Implementation Plan

### Step 1: Types (src/primitives/)

**File:** `src/primitives/endpoint.rs`

#### Request Type
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
}
```

#### Response Type
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct EndpointResponse {
    pub model: String,
    pub response: String,
    pub done: bool,
}
```

#### Constructor
```rust
impl EndpointRequest {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: None,
            options: None,
        }
    }
}
```

### Step 2: HTTP Method (src/http/endpoints/)

**File:** `src/http/endpoints/endpoint.rs`

```rust
impl Client {
    pub async fn endpoint(&self, request: EndpointRequest) -> Result<EndpointResponse> {
        let url = self.url("/api/endpoint");

        let response = self.http()
            .post(&url)
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub fn endpoint_sync(&self, request: EndpointRequest) -> Result<EndpointResponse> {
        self.runtime.block_on(self.endpoint(request))
    }
}
```

### Step 3: Tests

**Location:** `src/primitives/endpoint.rs` or `tests/`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = EndpointRequest::new("model");
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""model":"model""#));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{"model":"llama3","response":"Hello","done":true}"#;
        let response: EndpointResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.model, "llama3");
    }
}
```

### Step 4: Example

**File:** `examples/endpoint_basic_async.rs`

```rust
use library::{Client, EndpointRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = EndpointRequest::new("llama3");
    let response = client.endpoint(request).await?;

    println!("Response: {}", response.response);
    Ok(())
}
```

---

## Checklist

### Types
- [ ] Request struct defined
- [ ] Response struct defined
- [ ] Serde attributes correct
- [ ] Constructor implemented
- [ ] Builder pattern (if needed)

### HTTP
- [ ] Async method implemented
- [ ] Sync method implemented
- [ ] Error handling complete

### Tests
- [ ] Serialization tests
- [ ] Deserialization tests
- [ ] HTTP method tests (mocked)

### Documentation
- [ ] Types documented
- [ ] Methods documented
- [ ] Example created

### Quality
- [ ] clippy passes
- [ ] fmt passes
- [ ] All tests pass
- [ ] Example runs

---

## Notes

Additional implementation notes, edge cases, or decisions.

---

## References

- [API Specification](../primitives/NN-endpoint.yaml)
- [Related Endpoint](./related-plan.md)
