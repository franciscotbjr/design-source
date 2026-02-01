# Phase 2: API Primitives

**Version Target:** v0.2.0
**Focus:** Type definitions, serialization, validation

## Objectives

1. Define all request/response types
2. Implement serialization with serde
3. Add builder patterns for complex types
4. Write comprehensive tests

## Prerequisites

- Phase 1 completed
- API specifications available in `spec/primitives/`

## Workflow

### Step 1: Analyze API

Read each YAML specification and categorize:

| Complexity | Criteria |
|------------|----------|
| Simple | Few fields, no optional parameters |
| Medium | Multiple optional fields, validation needed |
| Complex | Streaming, nested types, complex validation |

### Step 2: Create Types

For each endpoint specification:

1. **Read the YAML spec**
   ```yaml
   # spec/primitives/01-generate.yaml
   request:
     type: GenerateRequest
     fields:
       - name: model
         type: String
         required: true
   ```

2. **Create the Rust type**
   ```rust
   // src/primitives/generate.rs

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct GenerateRequest {
       pub model: String,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub prompt: Option<String>,
   }
   ```

3. **Add constructors**
   ```rust
   impl GenerateRequest {
       pub fn new(model: impl Into<String>) -> Self {
           Self {
               model: model.into(),
               prompt: None,
           }
       }
   }
   ```

4. **Add builder if needed**
   ```rust
   impl GenerateRequest {
       pub fn builder() -> GenerateRequestBuilder {
           GenerateRequestBuilder::default()
       }
   }
   ```

5. **Write tests**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_serialize_minimal() {
           let req = GenerateRequest::new("model");
           let json = serde_json::to_string(&req).unwrap();
           assert!(json.contains(r#""model":"model""#));
       }
   }
   ```

### Step 3: Module Organization

```rust
// src/primitives/mod.rs

mod generate;
mod chat;
mod embed;

pub use generate::{GenerateRequest, GenerateResponse};
pub use chat::{ChatRequest, ChatResponse, ChatMessage};
pub use embed::{EmbedRequest, EmbedResponse};
```

### Step 4: Re-export from lib.rs

```rust
// src/lib.rs

pub mod error;
pub mod primitives;
pub mod http;

// Convenient re-exports
pub use primitives::*;
pub use error::{Error, Result};
```

## Type Patterns

### Request with Optional Fields
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub required: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<String>,

    #[serde(default)]
    pub with_default: bool,

    #[serde(default = "default_value")]
    pub custom_default: u32,
}

fn default_value() -> u32 { 100 }
```

### Response with Nested Types
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub data: ResponseData,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseData {
    pub items: Vec<Item>,
}
```

### Enum Fields
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}
```

### Flexible Enum (String or Known Values)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Format {
    Known(KnownFormat),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KnownFormat {
    Json,
}
```

## Tasks Checklist

### Analysis
- [ ] Review all YAML specifications
- [ ] List all types to create
- [ ] Identify shared/reusable types
- [ ] Note validation requirements

### Implementation (per endpoint)
- [ ] Create request type
- [ ] Create response type
- [ ] Add serde attributes
- [ ] Implement constructors
- [ ] Add builder if needed (3+ optional fields)
- [ ] Write serialization tests
- [ ] Write deserialization tests

### Organization
- [ ] Create mod.rs with exports
- [ ] Re-export from lib.rs
- [ ] Ensure no circular dependencies

### Quality
- [ ] All types derive Debug, Clone
- [ ] All public items documented
- [ ] clippy passes
- [ ] All tests pass

## Testing Strategy

### Serialization Tests
```rust
#[test]
fn test_serialize_minimal() {
    let req = Request::new("value");
    let json = serde_json::to_string(&req).unwrap();

    assert!(json.contains(r#""field":"value""#));
    // Ensure optional fields are omitted
    assert!(!json.contains("optional"));
}

#[test]
fn test_serialize_full() {
    let req = Request {
        field: "value".to_string(),
        optional: Some("opt".to_string()),
    };
    let json = serde_json::to_string(&req).unwrap();

    assert!(json.contains(r#""field":"value""#));
    assert!(json.contains(r#""optional":"opt""#));
}
```

### Deserialization Tests
```rust
#[test]
fn test_deserialize_minimal() {
    let json = r#"{"field": "value"}"#;
    let resp: Response = serde_json::from_str(json).unwrap();
    assert_eq!(resp.field, "value");
}

#[test]
fn test_deserialize_ignores_unknown() {
    let json = r#"{"field": "value", "unknown": true}"#;
    let resp: Response = serde_json::from_str(json).unwrap();
    assert_eq!(resp.field, "value");
}
```

## Completion Criteria

1. All request types implemented
2. All response types implemented
3. Serialization tests pass
4. Deserialization tests pass
5. Types properly re-exported
6. Documentation complete

## Next Phase

After completing Phase 2, proceed to [Phase 3: Implementation](phase-3-implementation.md).
