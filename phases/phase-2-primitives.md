# Phase 2: Inference & Model Types

**Version Target:** v0.2.0
**Focus:** Type definitions per file, with-method chain, feature-gated fields

## Objectives

1. Define all request/response types (one type per file)
2. Implement with-method chain pattern for optional fields
3. Add feature-gated fields (`#[cfg(feature = "tools")]`)
4. Add response helper methods
5. Write comprehensive tests

## Prerequisites

- Phase 1 completed
- API specifications available in `spec/apis/`

## Workflow

### Step 1: Analyze API Specifications

Read each YAML specification from `spec/apis/` and categorize:

| Complexity | Criteria | Feature | Examples |
|------------|----------|---------|----------|
| Simple | Few fields, GET or empty response | inference | version, tags, ps |
| Medium | POST with body, optional fields | inference/model | embed, show, copy, delete |
| Complex | Streaming, nested types, many options | inference | generate, chat, create, pull, push |

### Step 2: Create Types (One Per File)

For each endpoint specification:

1. **Read the YAML spec**
   ```yaml
   # spec/apis/03-chat.yaml
   request:
     type: ChatRequest
     fields:
       - name: model
         type: String
         required: true
       - name: messages
         type: Vec<ChatMessage>
         required: true
       - name: tools
         type: Option<Vec<ToolDefinition>>
         feature_gate: tools
   ```

2. **Create the request file** (`src/inference/chat_request.rs`)
   ```rust
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
   pub struct ChatRequest {
       pub model: String,
       pub messages: Vec<ChatMessage>,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub format: Option<FormatSetting>,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub options: Option<ModelOptions>,
       #[cfg(feature = "tools")]
       #[serde(skip_serializing_if = "Option::is_none")]
       pub tools: Option<Vec<ToolDefinition>>,
   }
   ```

3. **Add constructor + with-method chain**
   ```rust
   impl ChatRequest {
       pub fn new(
           model: impl Into<String>,
           messages: impl IntoIterator<Item = ChatMessage>,
       ) -> Self {
           Self {
               model: model.into(),
               messages: messages.into_iter().collect(),
               format: None,
               options: None,
               #[cfg(feature = "tools")]
               tools: None,
           }
       }

       pub fn with_format(mut self, format: FormatSetting) -> Self {
           self.format = Some(format);
           self
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

4. **Create the response file** (`src/inference/chat_response.rs`)
   ```rust
   use serde::Deserialize;

   #[derive(Debug, Clone, PartialEq, Deserialize)]
   pub struct ChatResponse {
       pub model: String,
       #[serde(default)]
       pub message: Option<ChatMessage>,
       #[serde(default)]
       pub done: bool,
       #[serde(default)]
       pub total_duration: Option<u64>,
       #[serde(default)]
       pub eval_count: Option<u64>,
       #[serde(default)]
       pub eval_duration: Option<u64>,
   }
   ```

5. **Add response helper methods**
   ```rust
   impl ChatResponse {
       pub fn content(&self) -> Option<&str> {
           self.message.as_ref().map(|m| m.content.as_str())
       }

       pub fn is_done(&self) -> bool {
           self.done
       }

       pub fn total_duration_ms(&self) -> Option<f64> {
           self.total_duration.map(|d| d as f64 / 1_000_000.0)
       }

       pub fn tokens_per_second(&self) -> Option<f64> {
           match (self.eval_count, self.eval_duration) {
               (Some(count), Some(dur)) if dur > 0 => {
                   Some(count as f64 / (dur as f64 / 1_000_000_000.0))
               }
               _ => None,
           }
       }
   }
   ```

6. **Write unit tests** (inside the source file)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_serialize_minimal() {
           let req = ChatRequest::new("model", [ChatMessage::user("Hello")]);
           let json = serde_json::to_string(&req).unwrap();
           assert!(json.contains(r#""model":"model""#));
       }

       #[test]
       fn test_is_send_sync() {
           fn assert_send_sync<T: Send + Sync>() {}
           assert_send_sync::<ChatRequest>();
       }
   }
   ```

### Step 3: Module Organization (Facade Pattern)

```rust
// src/inference/mod.rs — declarations + re-exports ONLY

mod chat_message;
mod chat_request;
mod chat_response;
mod generate_request;
mod generate_response;
mod embed_request;
mod embed_response;
mod version;
mod format_setting;
mod model_options;

pub use chat_message::ChatMessage;
pub use chat_request::ChatRequest;
pub use chat_response::ChatResponse;
pub use generate_request::GenerateRequest;
pub use generate_response::GenerateResponse;
pub use embed_request::EmbedRequest;
pub use embed_response::EmbedResponse;
pub use version::VersionResponse;
pub use format_setting::FormatSetting;
pub use model_options::ModelOptions;
```

```rust
// src/model/mod.rs — feature-gated module

mod copy_request;
mod delete_request;
mod show_request;
mod show_response;
mod create_request;
mod create_response;
mod pull_request;
mod pull_response;
mod push_request;
mod push_response;
mod list_response;
mod model_summary;
mod model_details;
mod ps_response;
mod running_model;

pub use copy_request::CopyRequest;
pub use delete_request::DeleteRequest;
// ... all re-exports
```

### Step 4: Re-export from lib.rs (Feature-Gated)

```rust
// src/lib.rs

#[cfg(feature = "inference")]
pub mod inference;

#[cfg(feature = "model")]
pub mod model;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "tools")]
pub mod tools;

// Convenient re-exports
#[cfg(feature = "inference")]
pub use inference::*;

#[cfg(feature = "model")]
pub use model::*;

#[cfg(feature = "http")]
pub use http::{ClientConfig, OllamaClient, OllamaApiAsync, OllamaApiSync};

pub use error::{Error, Result};
```

## Type Patterns

### Request with With-Method Chain
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub required: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<String>,
    #[serde(default)]
    pub with_default: bool,
}

impl Request {
    pub fn new(required: impl Into<String>) -> Self {
        Self {
            required: required.into(),
            optional: None,
            with_default: false,
        }
    }

    pub fn with_optional(mut self, value: impl Into<String>) -> Self {
        self.optional = Some(value.into());
        self
    }
}
```

### Response with Helper Methods
```rust
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Response {
    pub model: String,
    #[serde(default)]
    pub data: Option<String>,
    #[serde(default)]
    pub total_duration: Option<u64>,
}

impl Response {
    pub fn content(&self) -> Option<&str> {
        self.data.as_deref()
    }

    pub fn total_duration_ms(&self) -> Option<f64> {
        self.total_duration.map(|d| d as f64 / 1_000_000.0)
    }
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

### FormatSetting (Flexible Enum)
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormatSetting {
    Known(String),
    Schema(serde_json::Value),
}

impl FormatSetting {
    pub fn json() -> Self {
        FormatSetting::Known("json".to_string())
    }
}
```

## Tasks Checklist

### Analysis
- [ ] Review all YAML specifications in `spec/apis/`
- [ ] List all types to create (one file per type)
- [ ] Identify shared/reusable types (ModelOptions, FormatSetting, ChatMessage)
- [ ] Identify feature-gated fields

### Inference Types (per endpoint)
- [ ] Create request type file (`src/inference/{type}_request.rs`)
- [ ] Create response type file (`src/inference/{type}_response.rs`)
- [ ] Add serde attributes (`skip_serializing_if`, `default`)
- [ ] Implement constructor with required fields
- [ ] Add with-method chain for optional fields
- [ ] Add feature-gated fields (`#[cfg(feature = "tools")]`)
- [ ] Add response helper methods
- [ ] Write unit tests in source file

### Model Types (feature-gated)
- [ ] Create all model types in `src/model/`
- [ ] Gate entire module with `#[cfg(feature = "model")]`
- [ ] Follow same one-file-per-type pattern

### Organization
- [ ] Module facade: `mod.rs` with `mod` + `pub use` only
- [ ] Feature-gated re-exports from `lib.rs`
- [ ] No circular dependencies

### Quality
- [ ] All types derive Debug, Clone, PartialEq
- [ ] All types are Send + Sync (verified by test)
- [ ] All public items documented with `no_run` examples
- [ ] `cargo build --all-features` passes
- [ ] `cargo test --all-features` passes
- [ ] `cargo clippy --all-features` passes

## Testing Strategy

### Unit Tests (inside source files)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_minimal() {
        let req = Request::new("model");
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""model":"model""#));
        assert!(!json.contains("optional"));
    }

    #[test]
    fn test_serialize_with_options() {
        let req = Request::new("model")
            .with_optional("value");
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""optional":"value""#));
    }

    #[test]
    fn test_deserialize_minimal() {
        let json = r#"{"model": "llama3"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
        assert_eq!(resp.model, "llama3");
    }

    #[test]
    fn test_roundtrip() {
        let original = Request::new("model").with_optional("val");
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Request = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Request>();
        assert_send_sync::<Response>();
    }
}
```

### Feature-Gated Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "tools")]
    fn test_with_tools() {
        let req = ChatRequest::new("model", [ChatMessage::user("Hi")])
            .with_tools(vec![]);
        assert!(req.tools.is_some());
    }
}
```

## Completion Criteria

1. All request types implemented (one file per type)
2. All response types implemented with helper methods
3. With-method chain pattern on all types with optional fields
4. Feature-gated fields working correctly
5. Types properly re-exported through module facade
6. All tests pass with `cargo test --all-features`
7. Documentation complete with `no_run` examples

## Next Phase

After completing Phase 2, proceed to [Phase 3: API Implementation](phase-3-implementation.md).
