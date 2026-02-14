# Reference Implementation: ollama-oxide

This document provides reference examples from the ollama-oxide project, demonstrating the patterns and conventions described in this methodology guide.

## Project Structure

```
ollama-oxide/
├── Cargo.toml                        # Single crate with feature flags
├── src/
│   ├── lib.rs                        # Crate root, feature-gated re-exports
│   ├── error.rs                      # Error enum with {Type}Error variants
│   ├── inference/                    # Feature: "inference" (default)
│   │   ├── mod.rs                    # Facade: mod + pub use only
│   │   ├── chat_request.rs           # One type per file
│   │   ├── chat_response.rs
│   │   ├── chat_message.rs
│   │   ├── chat_role.rs
│   │   ├── generate_request.rs
│   │   ├── generate_response.rs
│   │   ├── embed_request.rs
│   │   ├── embed_response.rs
│   │   ├── format_setting.rs
│   │   ├── model_options.rs
│   │   ├── think_setting.rs
│   │   ├── keep_alive_setting.rs
│   │   ├── response_message.rs
│   │   └── version_response.rs
│   ├── model/                        # Feature: "model"
│   │   ├── mod.rs
│   │   ├── show_request.rs
│   │   ├── show_response.rs
│   │   ├── list_response.rs
│   │   ├── copy_request.rs
│   │   ├── delete_request.rs
│   │   ├── create_request.rs
│   │   ├── pull_request.rs
│   │   ├── push_request.rs
│   │   └── ...
│   ├── tools/                        # Feature: "tools"
│   │   ├── mod.rs
│   │   ├── tool_definition.rs
│   │   ├── tool_function.rs
│   │   ├── tool_call.rs
│   │   ├── tool_call_function.rs
│   │   ├── tool_trait.rs
│   │   ├── tool_registry.rs
│   │   ├── erased_tool.rs
│   │   └── tool_error.rs
│   ├── http/                         # Feature: "http" (default)
│   │   ├── mod.rs
│   │   ├── client.rs                 # OllamaClient + retry helpers
│   │   ├── api_async.rs              # OllamaApiAsync trait + impl
│   │   ├── api_sync.rs               # OllamaApiSync trait + impl
│   │   ├── endpoints.rs              # Endpoint constants
│   │   └── config.rs                 # ClientConfig struct
│   └── conveniences/                 # Feature: "conveniences"
│       └── mod.rs
├── spec/
│   ├── definition.md                 # Project definition and phases
│   ├── api-analysis.md               # API complexity analysis
│   └── apis/                         # Per-endpoint YAML specs
│       ├── 01-generate.yaml
│       └── ...
├── impl/                             # Implementation plans
│   ├── 01-chat-plan.md
│   └── ...
├── examples/
│   ├── chat_async.rs
│   ├── chat_with_tools_async.rs
│   ├── generate_async.rs
│   ├── model_list_async.rs
│   └── ...
└── tests/
    ├── client_chat_tests.rs
    ├── client_generate_tests.rs
    ├── client_delete_model_tests.rs
    └── ...
```

## Cargo.toml Feature Flags

```toml
[features]
default = ["inference", "http"]
inference = []
model = []
tools = ["dep:schemars"]
http = ["inference", "dep:reqwest", "dep:tokio", "dep:async-trait"]
conveniences = ["http"]
full = ["inference", "model", "tools", "http", "conveniences"]
```

## Specification Example

From `spec/apis/01-generate.yaml`:

```yaml
endpoint: POST /api/generate
complexity: complex
streaming: true
feature: inference

description: |
  Generate a completion from a model. Supports both streaming
  and non-streaming modes.

request:
  type: GenerateRequest
  fields:
    - name: model
      type: String
      required: true
      description: The model name

    - name: prompt
      type: Option<String>
      required: false
      description: The prompt to generate from

    - name: stream
      type: Option<bool>
      required: false
      default: true
      description: Stream the response
```

## Type Definition Example

From `src/inference/generate_request.rs`:

```rust
//! Generate request primitive type

use serde::{Deserialize, Serialize};
use super::{FormatSetting, KeepAliveSetting, ModelOptions, ThinkSetting};

/// Request body for POST /api/generate endpoint
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::GenerateRequest;
///
/// let request = GenerateRequest::new("qwen3:0.6b", "Why is the sky blue?");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// Name of the model to use
    pub model: String,

    /// Text prompt to generate a response from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Output format (string like "json" or JSON schema object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSetting>,

    /// System prompt for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Whether to stream the response (always false for v0.1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Control thinking output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<ThinkSetting>,

    /// How long to keep the model loaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<KeepAliveSetting>,

    /// Runtime options for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModelOptions>,
}

impl GenerateRequest {
    /// Create a new generate request with required fields only.
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: Some(prompt.into()),
            format: None,
            system: None,
            stream: Some(false),
            think: None,
            keep_alive: None,
            options: None,
        }
    }

    /// Set the output format.
    pub fn with_format(mut self, format: impl Into<FormatSetting>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Set the system prompt.
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set runtime options.
    pub fn with_options(mut self, options: ModelOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Accessor: get the model name.
    pub fn model(&self) -> &str {
        &self.model
    }
}
```

## With-Method Chain Pattern (preferred over Builder)

```rust
// Simple case - minimal required fields
let request = GenerateRequest::new("qwen3:0.6b", "Hello");

// Complex case - with chaining
let request = GenerateRequest::new("qwen3:0.6b", "Tell me a joke")
    .with_system("You are a comedian.")
    .with_format(FormatSetting::json())
    .with_options(ModelOptions::default().with_temperature(0.9));
```

## Chat Request with Feature-Gated Tools

From `src/inference/chat_request.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSetting>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModelOptions>,

    /// Feature-gated field - only exists with "tools" feature
    #[cfg(feature = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl ChatRequest {
    pub fn new<M, I>(model: M, messages: I) -> Self
    where
        M: Into<String>,
        I: IntoIterator<Item = ChatMessage>,
    {
        Self {
            model: model.into(),
            messages: messages.into_iter().collect(),
            format: None,
            options: None,
            #[cfg(feature = "tools")]
            tools: None,
            stream: Some(false),
        }
    }

    /// Feature-gated method
    #[cfg(feature = "tools")]
    pub fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = Some(tools);
        self
    }
}
```

## Error Handling

From `src/error.rs`:

```rust
use thiserror::Error;

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

pub type Result<T> = std::result::Result<T, Error>;

// Manual From (avoid exposing external types)
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

## Endpoint Constants

From `src/http/endpoints.rs`:

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
    pub const CREATE: &'static str = "/api/create";
    pub const COPY: &'static str = "/api/copy";
    pub const PULL: &'static str = "/api/pull";
    pub const PUSH: &'static str = "/api/push";
    pub const DELETE: &'static str = "/api/delete";
}
```

## Trait-Based API Implementation

From `src/http/api_async.rs`:

```rust
use async_trait::async_trait;
use crate::{ChatRequest, ChatResponse, GenerateRequest, GenerateResponse, Result, VersionResponse};

#[cfg(feature = "model")]
use crate::{CopyRequest, DeleteRequest, ListResponse, ShowRequest, ShowResponse};

#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    async fn version(&self) -> Result<VersionResponse>;
    async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse>;
    async fn generate(&self, request: &GenerateRequest) -> Result<GenerateResponse>;

    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    async fn show_model(&self, request: &ShowRequest) -> Result<ShowResponse>;

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: &DeleteRequest) -> Result<()>;

    #[cfg(feature = "model")]
    async fn copy_model(&self, request: &CopyRequest) -> Result<()>;
}

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
    async fn delete_model(&self, request: &DeleteRequest) -> Result<()> {
        let url = self.config.url(Endpoints::DELETE);
        self.delete_empty_with_retry(&url, request).await
    }
}
```

From `src/http/api_sync.rs`:

```rust
pub trait OllamaApiSync: Send + Sync {
    fn version_blocking(&self) -> Result<VersionResponse>;
    fn chat_blocking(&self, request: &ChatRequest) -> Result<ChatResponse>;
    fn generate_blocking(&self, request: &GenerateRequest) -> Result<GenerateResponse>;

    #[cfg(feature = "model")]
    fn list_models_blocking(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    fn delete_model_blocking(&self, request: &DeleteRequest) -> Result<()>;
}
```

## Tool Trait (Type-Safe Tools API)

From `src/tools/tool_trait.rs`:

```rust
pub trait Tool: Send + Sync {
    type Params: for<'de> Deserialize<'de> + JsonSchema + Send + Debug;
    type Output: Serialize + Send;

    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn execute(
        &self,
        params: Self::Params,
    ) -> impl Future<Output = ToolResult<Self::Output>> + Send;

    /// Auto-implemented: generates JSON schema from Params type
    fn parameters_schema(&self) -> serde_json::Value {
        let schema = schema_for!(Self::Params);
        serde_json::to_value(schema).unwrap_or_else(|_| serde_json::json!({}))
    }

    /// Auto-implemented: converts to ToolDefinition for chat requests
    fn to_definition(&self) -> ToolDefinition {
        ToolDefinition::function(self.name(), self.parameters_schema())
            .with_description(self.description())
    }
}
```

## Integration Test Example

From `tests/client_chat_tests.rs`:

```rust
//! Tests for chat API methods (POST /api/chat)

use ollama_oxide::{
    ChatMessage, ChatRequest, ChatResponse, ClientConfig,
    OllamaApiAsync, OllamaApiSync, OllamaClient,
};
#[cfg(feature = "tools")]
use ollama_oxide::{ToolCall, ToolCallFunction, ToolDefinition};
use serde_json::json;
use std::time::Duration;

// ============================================================================
// Async API Tests
// ============================================================================

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
            "done": true,
            "done_reason": "stop"
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

    assert_eq!(response.model(), Some("qwen3:0.6b"));
    assert_eq!(response.content(), Some("Hello!"));
    assert!(response.is_done());
    mock.assert_async().await;
}

// ============================================================================
// Sync API Tests
// ============================================================================

#[test]
fn test_chat_sync_success() {
    let mut server = mockito::Server::new();

    let mock = server
        .mock("POST", "/api/chat")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": {"content": "Hello!"}, "done": true}"#)
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

// ============================================================================
// Error and Retry Tests
// ============================================================================

#[tokio::test]
async fn test_chat_async_retry_on_server_error() {
    let mut server = mockito::Server::new_async().await;

    let mock_fail = server
        .mock("POST", "/api/chat")
        .with_status(500)
        .expect(1)
        .create_async()
        .await;

    let mock_success = server
        .mock("POST", "/api/chat")
        .with_status(200)
        .with_body(r#"{"message": {"content": "Ok"}, "done": true}"#)
        .expect(1)
        .create_async()
        .await;

    let config = ClientConfig {
        base_url: server.url(),
        timeout: Duration::from_secs(5),
        max_retries: 1,
    };

    let client = OllamaClient::new(config).unwrap();
    let request = ChatRequest::new("model", [ChatMessage::user("Hello")]);
    let result = client.chat(&request).await;

    assert!(result.is_ok());
    mock_fail.assert_async().await;
    mock_success.assert_async().await;
}

// ============================================================================
// Feature-Gated Tests
// ============================================================================

#[cfg(feature = "tools")]
#[tokio::test]
async fn test_chat_async_with_tools() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/api/chat")
        .with_status(200)
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
    let request = ChatRequest::new("model", [ChatMessage::user("Weather in Paris?")])
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

// ============================================================================
// Type Safety Tests
// ============================================================================

#[test]
fn test_chat_request_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ChatRequest>();
}
```

## Example File

From `examples/chat_async.rs`:

```rust
//! Example: Chat completion (async)
//!
//! Run with: cargo run --example chat_async

use ollama_oxide::{
    ChatMessage, ChatRequest, FormatSetting, ModelOptions,
    OllamaApiAsync, OllamaClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;
    let model = "qwen3:0.6b";

    // Basic chat
    let request = ChatRequest::new(model, [
        ChatMessage::user("Hello! What can you help me with?"),
    ]);
    let response = client.chat(&request).await?;
    println!("Assistant: {}", response.content().unwrap_or("No response"));

    // Multi-turn with system message
    let request = ChatRequest::new(model, [
        ChatMessage::system("You are a helpful assistant."),
        ChatMessage::user("What is Rust?"),
        ChatMessage::assistant("Rust is a systems programming language."),
        ChatMessage::user("What are its main features?"),
    ]);
    let response = client.chat(&request).await?;
    println!("Assistant: {}", response.content().unwrap_or("No response"));

    // With options and format
    let request = ChatRequest::new(model, [
        ChatMessage::user("List 3 colors as JSON array."),
    ])
    .with_format(FormatSetting::json())
    .with_options(ModelOptions::new().with_temperature(0.7));

    let response = client.chat(&request).await?;
    println!("JSON: {}", response.content().unwrap_or("No JSON"));

    // Performance metrics
    if let Some(tps) = response.tokens_per_second() {
        println!("Tokens/sec: {:.2}", tps);
    }
    if let Some(ms) = response.total_duration_ms() {
        println!("Duration: {:.2} ms", ms);
    }

    Ok(())
}
```

## Decision Log Example

From `DECISIONS.md`:

```markdown
[2024-01-15] **Use single crate with feature flags**
- Context: Choosing between workspace (multiple crates) vs single crate
- Decision: Single crate with inference/model/tools/http as feature-gated modules
- Consequences: Simpler dependency management, fine-grained feature control

[2024-01-16] **With-method chain pattern over builder**
- Context: Many request types have numerous optional parameters
- Decision: Use with_* methods on the type itself (no separate builder struct)
- Consequences: Simpler API, no validation step needed, method chaining

[2024-01-17] **ChatRequest::new() accepts IntoIterator<Item=ChatMessage>**
- Context: Users need to pass message history
- Decision: Accept any iterator that yields ChatMessage
- Consequences: Flexible API, works with Vec, arrays, iterators

[2024-01-18] **Trait-based API with OllamaApiAsync / OllamaApiSync**
- Context: Need to separate async and sync API surfaces
- Decision: Two traits (OllamaApiAsync, OllamaApiSync) with _blocking suffix for sync
- Consequences: Clean trait-based dispatch, mockable, extensible

[2024-01-19] **{Type}Error suffix + manual From impls**
- Context: Error variant naming and external type exposure
- Decision: HttpError, TimeoutError, etc.; manual From<reqwest::Error> → HttpError(String)
- Consequences: No external types in public API, clear error naming
```

## Key Patterns Summary

| Pattern | When to Use | Example |
|---------|-------------|---------|
| `new()` with-method chain | All types with optional fields | `Request::new("model").with_format(...)` |
| `impl Into<String>` | String-like parameters | `fn new(s: impl Into<String>)` |
| `IntoIterator` | Collection parameters | `fn new<I: IntoIterator<Item = Msg>>(msgs: I)` |
| `skip_serializing_if` | Optional serde fields | `#[serde(skip_serializing_if = "Option::is_none")]` |
| `#[cfg(feature)]` | Conditional compilation | Fields, methods, imports, modules |
| `_blocking` suffix | Blocking variants | `client.chat_blocking(&request)` |
| `_stream` suffix | Streaming variants | `client.chat_stream(&request)` |
| `{Type}Error` suffix | Error variants | `HttpError`, `TimeoutError`, `MaxRetriesExceededError` |
| Trait-based API | HTTP methods | `OllamaApiAsync`, `OllamaApiSync` |
| Endpoint constants | API routes | `Endpoints::CHAT`, `Endpoints::VERSION` |
| Generic retry helpers | HTTP resilience | `post_with_retry`, `get_with_retry` |
| Type erasure | Heterogeneous tools | `Tool` → `ErasedTool` → `ToolWrapper` → `ToolRegistry` |
| Response helpers | Accessor methods | `response.content()`, `response.tokens_per_second()` |

This reference implementation demonstrates the complete workflow from specification to working code, following the methodology defined in this guide.
