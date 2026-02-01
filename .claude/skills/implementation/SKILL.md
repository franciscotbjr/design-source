# Implementation - Development Workflow

## Implementation Plan Structure

Each feature/endpoint requires an implementation plan before coding.

### Plan File Template
Location: `spec/impl-plans/NN-feature-implementation-plan.md`

```markdown
# Feature Implementation Plan

**Endpoint:** METHOD /api/endpoint
**Complexity:** Simple | Medium | Complex
**Streaming:** Yes | No
**Status:** Planning | In Progress | Complete

## Overview
Brief description of what this feature does.

## Request Type

### Fields
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | String | Yes | Field description |
| options | Option<T> | No | Optional field |

### Validation Rules
- Rule 1
- Rule 2

### Example Request
```json
{
  "name": "value"
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

### 1. Types (primitives/)
- [ ] Create request struct
- [ ] Create response struct
- [ ] Implement builders if needed
- [ ] Add serde attributes

### 2. HTTP Layer (http/)
- [ ] Add endpoint method to client
- [ ] Implement async version
- [ ] Implement sync version
- [ ] Add error handling

### 3. Tests
- [ ] Request serialization tests
- [ ] Response deserialization tests
- [ ] Validation tests
- [ ] Integration tests (optional)

### 4. Examples
- [ ] Basic usage example
- [ ] Advanced example (if applicable)

## Dependencies
- Depends on: [list features]
- Blocks: [list features]

## Notes
Additional implementation notes.
```

## Implementation Workflow

### Step 1: Write Types
```rust
// src/primitives/feature.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRequest {
    pub required_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeatureResponse {
    pub result: String,
}
```

### Step 2: Add to Module
```rust
// src/primitives/mod.rs
mod feature;
pub use feature::{FeatureRequest, FeatureResponse};
```

### Step 3: Implement HTTP Method
```rust
// src/http/endpoints/feature.rs
impl OllamaClient {
    pub async fn feature(&self, request: FeatureRequest) -> Result<FeatureResponse> {
        let url = format!("{}/api/feature", self.base_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(OllamaError::api_error(response).await);
        }

        Ok(response.json().await?)
    }

    pub fn feature_sync(&self, request: FeatureRequest) -> Result<FeatureResponse> {
        self.runtime.block_on(self.feature(request))
    }
}
```

### Step 4: Write Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_serialization() {
        let request = FeatureRequest {
            required_field: "value".to_string(),
            optional_field: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("required_field"));
        assert!(!json.contains("optional_field"));
    }

    #[test]
    fn response_deserialization() {
        let json = r#"{"result": "success"}"#;
        let response: FeatureResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.result, "success");
    }
}
```

### Step 5: Create Example
```rust
// examples/feature_basic_async.rs
use ollama_oxide::{OllamaClient, FeatureRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::new();

    let request = FeatureRequest {
        required_field: "example".to_string(),
        optional_field: Some("optional".to_string()),
    };

    let response = client.feature(request).await?;
    println!("Result: {}", response.result);

    Ok(())
}
```

## Checklist Before Moving On

- [ ] Types compile without warnings
- [ ] Serde serialization works correctly
- [ ] HTTP method returns expected results
- [ ] Tests pass
- [ ] Example compiles and runs
- [ ] Documentation is complete
- [ ] CHANGELOG updated

## Streaming Implementation

For streaming endpoints:

```rust
use futures_util::Stream;
use tokio_stream::StreamExt;

impl OllamaClient {
    pub async fn feature_stream(
        &self,
        request: FeatureRequest,
    ) -> Result<impl Stream<Item = Result<FeatureChunk>>> {
        let url = format!("{}/api/feature", self.base_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        let stream = response
            .bytes_stream()
            .map(|chunk| {
                let bytes = chunk?;
                let line = std::str::from_utf8(&bytes)?;
                serde_json::from_str(line)
                    .map_err(Into::into)
            });

        Ok(stream)
    }
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
```
