# API Design - Ergonomic Rust APIs

## Design Principles

### 1. Type Safety
Leverage Rust's type system to prevent invalid states at compile time.

```rust
// Bad: String accepts any value
pub fn set_format(format: String);

// Good: Enum restricts to valid values
pub fn set_format(format: Format);

pub enum Format {
    Json,
    Text,
}
```

### 2. Progressive Disclosure
Simple things should be simple; complex things should be possible.

```rust
// Simple case - required fields only
let request = ChatRequest::new("llama3", [
    ChatMessage::user("Hello")
]);

// Complex case - full control with chaining
let request = ChatRequest::new("llama3", [
    ChatMessage::system("Be concise."),
    ChatMessage::user("Hello"),
])
.with_format(FormatSetting::json())
.with_options(ModelOptions::default().with_temperature(0.7))
.with_think(ThinkSetting::enabled())
.with_keep_alive(KeepAliveSetting::duration("5m"));
```

### 3. Flexible Input
Accept various input types for convenience.

```rust
// Accept anything that converts to String
pub fn new(model: impl Into<String>) -> Self;

// Accept any iterator for collection fields
pub fn new<M, I>(model: M, messages: I) -> Self
where
    M: Into<String>,
    I: IntoIterator<Item = ChatMessage>,
{
    Self {
        model: model.into(),
        messages: messages.into_iter().collect(),
        // ...
    }
}

// Accept impl Into for optional setters
pub fn with_format(mut self, format: impl Into<FormatSetting>) -> Self {
    self.format = Some(format.into());
    self
}
```

## Constructor Patterns

### Simple Constructor (Required Fields Only)
```rust
impl Request {
    /// Create a new request with required fields only.
    /// Optional fields default to None.
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            format: None,
            options: None,
            stream: Some(false),   // Sensible default
            #[cfg(feature = "tools")]
            tools: None,
        }
    }
}
```

### With-Method Chain Pattern (Preferred)
```rust
impl Request {
    pub fn new(model: impl Into<String>) -> Self { ... }

    // Fluent setters return Self for chaining
    pub fn with_format(mut self, format: impl Into<FormatSetting>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn with_options(mut self, options: ModelOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub fn with_think(mut self, think: impl Into<ThinkSetting>) -> Self {
        self.think = Some(think.into());
        self
    }

    // Feature-gated optional setter
    #[cfg(feature = "tools")]
    pub fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = Some(tools);
        self
    }

    // Append-style setter (for collections)
    #[cfg(feature = "tools")]
    pub fn with_tool(mut self, tool: ToolDefinition) -> Self {
        self.tools.get_or_insert_with(Vec::new).push(tool);
        self
    }
}

// Usage
let request = Request::new("llama3")
    .with_format(FormatSetting::json())
    .with_options(ModelOptions::default().with_temperature(0.7));
```

### Accessor Methods
```rust
impl Request {
    /// Immutable reference to required field
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Slice access for collections
    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }

    /// Derived accessor
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Feature-gated accessor
    #[cfg(feature = "tools")]
    pub fn has_tools(&self) -> bool {
        self.tools.as_ref().map(|t| !t.is_empty()).unwrap_or(false)
    }

    #[cfg(feature = "tools")]
    pub fn tools(&self) -> Option<&[ToolDefinition]> {
        self.tools.as_deref()
    }
}
```

## Trait-Based API Design

### Separate Async and Sync Traits
```rust
use async_trait::async_trait;

/// Async API operations
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    async fn version(&self) -> Result<VersionResponse>;
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse>;
    async fn embed(&self, request: EmbedRequest) -> Result<EmbedResponse>;

    // Feature-gated methods
    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    async fn show_model(&self, request: ShowRequest) -> Result<ShowResponse>;

    #[cfg(feature = "model")]
    async fn copy_model(&self, request: CopyRequest) -> Result<()>;

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: DeleteRequest) -> Result<()>;
}

/// Sync (blocking) API operations
pub trait OllamaApiSync: Send + Sync {
    fn version_blocking(&self) -> Result<VersionResponse>;
    fn chat_blocking(&self, request: ChatRequest) -> Result<ChatResponse>;
    fn generate_blocking(&self, request: GenerateRequest) -> Result<GenerateResponse>;

    #[cfg(feature = "model")]
    fn list_models_blocking(&self) -> Result<ListResponse>;
}
```

### Method Naming

| Pattern | Async (default) | Sync | Streaming |
|---------|----------------|------|-----------|
| Get single | `version()` | `version_blocking()` | - |
| Get collection | `list_models()` | `list_models_blocking()` | - |
| Create/Action | `chat(req)` | `chat_blocking(req)` | `chat_stream(req)` |
| Delete | `delete_model(req)` | `delete_model_blocking(req)` | - |
| Copy | `copy_model(req)` | `copy_model_blocking(req)` | - |

### Trait Implementation with Retry Helpers
```rust
#[async_trait]
impl OllamaApiAsync for OllamaClient {
    async fn version(&self) -> Result<VersionResponse> {
        let url = self.config.url(Endpoints::VERSION);
        self.get_with_retry(&url).await
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = self.config.url(Endpoints::CHAT);
        self.post_with_retry(&url, &request).await
    }

    #[cfg(feature = "model")]
    async fn copy_model(&self, request: CopyRequest) -> Result<()> {
        let url = self.config.url(Endpoints::COPY);
        self.post_empty_with_retry(&url, &request).await
    }

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: DeleteRequest) -> Result<()> {
        let url = self.config.url(Endpoints::DELETE);
        self.delete_empty_with_retry(&url, &request).await
    }
}
```

## Endpoint Constants

### Centralized Endpoint Definitions
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
    pub const COPY: &'static str = "/api/copy";
    pub const DELETE: &'static str = "/api/delete";
    pub const CREATE: &'static str = "/api/create";
    pub const PULL: &'static str = "/api/pull";
    pub const PUSH: &'static str = "/api/push";
}

// Usage in endpoint implementations
let url = self.config.url(Endpoints::CHAT);
```

## Generic Retry Helpers

### Helper Categories

| Helper | HTTP Method | Returns | Use Case |
|--------|------------|---------|----------|
| `get_with_retry<T>` | GET | `Result<T>` | Fetch JSON data |
| `post_with_retry<R,T>` | POST | `Result<T>` | Send body, receive JSON |
| `post_empty_with_retry<R>` | POST | `Result<()>` | Send body, expect 200 OK |
| `delete_empty_with_retry<R>` | DELETE | `Result<()>` | Send body, expect 200 OK |

### Signatures
```rust
impl OllamaClient {
    /// GET with retry - returns deserialized JSON
    pub(super) async fn get_with_retry<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    { /* ... */ }

    /// POST with retry - sends JSON body, returns deserialized response
    pub(super) async fn post_with_retry<R, T>(&self, url: &str, body: &R) -> Result<T>
    where
        R: serde::Serialize,
        T: serde::de::DeserializeOwned,
    { /* ... */ }

    /// POST with retry - sends JSON body, expects empty 200 OK
    #[cfg(feature = "model")]
    pub(super) async fn post_empty_with_retry<R>(&self, url: &str, body: &R) -> Result<()>
    where
        R: serde::Serialize,
    { /* ... */ }

    /// DELETE with retry - sends JSON body, expects empty 200 OK
    #[cfg(feature = "model")]
    pub(super) async fn delete_empty_with_retry<R>(&self, url: &str, body: &R) -> Result<()>
    where
        R: serde::Serialize,
    { /* ... */ }
}
```

### Retry Strategy
- **Exponential backoff**: `100ms * (attempt + 1)`
- **Server errors (5xx)**: Retry up to `max_retries`
- **Client errors (4xx)**: Fail immediately with `HttpStatusError` (no retry)
- **Network errors**: Retry up to `max_retries`
- **Exceeded retries**: Return `MaxRetriesExceededError`

### Blocking Variants
Each async helper has a corresponding `_blocking_` variant using `reqwest::blocking::Client`:
- `get_blocking_with_retry<T>`
- `post_blocking_with_retry<R,T>`
- `post_empty_blocking_with_retry<R>`
- `delete_empty_blocking_with_retry<R>`

### Design Benefits
- 90% code reduction vs. per-endpoint implementations
- Consistent retry behavior across all endpoints
- Type-safe through generic bounds
- `pub(super)` visibility keeps helpers internal to `http` module

## Type Erasure Pattern

### Problem
Different `Tool` implementations have different `Params` and `Output` types,
making them impossible to store in a single collection.

### Solution: ErasedTool + ToolWrapper

**Layer 1 - Typed Tool trait:**
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

    /// Auto-implemented: generate JSON schema from Params type
    fn parameters_schema(&self) -> serde_json::Value {
        let schema = schema_for!(Self::Params);
        serde_json::to_value(schema).unwrap_or_else(|_| json!({}))
    }

    /// Auto-implemented: convert to ToolDefinition for chat requests
    fn to_definition(&self) -> ToolDefinition {
        ToolDefinition::function(self.name(), self.parameters_schema())
            .with_description(self.description())
    }
}
```

**Layer 2 - Object-safe erased trait:**
```rust
pub trait ErasedTool: Send + Sync {
    fn name(&self) -> &'static str;
    fn definition(&self) -> ToolDefinition;

    /// Execute with JSON in, JSON out (type-erased)
    fn execute_erased<'a>(
        &'a self,
        args: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = ToolResult<serde_json::Value>> + Send + 'a>>;
}
```

**Layer 3 - Bridge wrapper:**
```rust
pub(crate) struct ToolWrapper<T> {
    tool: T,
}

impl<T: Tool + 'static> ErasedTool for ToolWrapper<T> {
    fn name(&self) -> &'static str {
        self.tool.name()
    }

    fn definition(&self) -> ToolDefinition {
        self.tool.to_definition()
    }

    fn execute_erased<'a>(
        &'a self,
        args: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = ToolResult<serde_json::Value>> + Send + 'a>> {
        Box::pin(async move {
            let params: T::Params = serde_json::from_value(args)?;
            let output = self.tool.execute(params).await?;
            Ok(serde_json::to_value(output)?)
        })
    }
}
```

**Layer 4 - Registry stores erased tools:**
```rust
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ErasedTool>>,
}

impl ToolRegistry {
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        let name = tool.name().to_string();
        self.tools.insert(name, Box::new(ToolWrapper::new(tool)));
    }

    pub fn definitions(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|t| t.definition()).collect()
    }
}
```

### Usage Flow
```rust
// 1. Define typed tool
struct GetWeather;
impl Tool for GetWeather {
    type Params = WeatherParams;  // Has JsonSchema
    type Output = WeatherResult;  // Has Serialize
    // ...
}

// 2. Register (type-erased automatically)
let mut registry = ToolRegistry::new();
registry.register(GetWeather);   // ToolWrapper<GetWeather> created internally

// 3. Get definitions for ChatRequest
let request = ChatRequest::new("model", messages)
    .with_tools(registry.definitions());

// 4. Dispatch tool calls from response
let results = registry.execute_all(&response).await;
```

## Error Design

### Error Enum with {Type}Error Suffix
```rust
#[derive(Debug, thiserror::Error)]
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
```

### Separate Error Types for Modules
```rust
// Tool-specific errors (in tools module)
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Tool not found: {0}")]
    ToolNotFoundError(String),

    #[error("Failed to deserialize arguments: {0}")]
    DeserializationError(String),

    #[error("Failed to serialize output: {0}")]
    SerializationError(String),

    #[error("Tool execution failed: {0}")]
    ExecutionError(String),
}

pub type ToolResult<T> = std::result::Result<T, ToolError>;
```

## Optional Fields

### Serde Configuration
```rust
#[derive(Serialize, Deserialize)]
pub struct Request {
    /// Required field - always present
    pub model: String,

    /// Optional field - omitted from JSON when None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Optional with default - uses default when missing in deserialization
    #[serde(default)]
    pub stream: bool,

    /// Optional with custom default
    #[serde(default = "default_num_ctx")]
    pub num_ctx: u32,
}

fn default_num_ctx() -> u32 { 2048 }
```

### Flexible Enums (Untagged)
```rust
/// Accept multiple JSON representations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormatSetting {
    /// String format: "json"
    Named(String),
    /// JSON Schema object
    Schema(serde_json::Value),
}

impl FormatSetting {
    pub fn json() -> Self { Self::Named("json".to_string()) }
}
```

## Response Helper Methods

```rust
impl ChatResponse {
    /// Get the response content
    pub fn content(&self) -> &str {
        &self.message.content
    }

    /// Check if generation is complete
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Calculate tokens per second
    pub fn tokens_per_second(&self) -> Option<f64> {
        self.eval_count.and_then(|count| {
            self.eval_duration.map(|duration| {
                count as f64 / (duration as f64 / 1_000_000_000.0)
            })
        })
    }

    /// Convert nanosecond duration to milliseconds
    pub fn total_duration_ms(&self) -> Option<f64> {
        self.total_duration.map(|ns| ns as f64 / 1_000_000.0)
    }
}
```

## Documentation Standards

### Type Documentation
```rust
/// Request body for POST /api/chat endpoint.
///
/// Generates the next message in a chat conversation.
/// This type always sets `stream: false` for non-streaming responses.
///
/// # Examples
///
/// ## Basic Request
///
/// ```no_run
/// use ollama_oxide::{ChatRequest, ChatMessage};
///
/// let request = ChatRequest::new("model", [
///     ChatMessage::user("Hello!")
/// ]);
/// ```
///
/// ## With Options
///
/// ```no_run
/// use ollama_oxide::{ChatRequest, ChatMessage, ModelOptions};
///
/// let request = ChatRequest::new("model", [
///     ChatMessage::user("Hello!")
/// ]).with_options(ModelOptions::default().with_temperature(0.7));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatRequest { ... }
```

### Method Documentation
```rust
/// Create a new chat request.
///
/// Creates a non-streaming request with the specified model and messages.
///
/// # Arguments
///
/// * `model` - Name of the model to use (e.g., "qwen3:0.6b")
/// * `messages` - Conversation history as an iterator of messages
///
/// # Errors
///
/// Returns [`Error::HttpError`] if the network request fails.
/// Returns [`Error::HttpStatusError`] for 4xx client errors.
/// Returns [`Error::MaxRetriesExceededError`] if all retries fail.
///
/// # Examples
///
/// ```no_run
/// use ollama_oxide::{ChatRequest, ChatMessage};
///
/// // With array
/// let request = ChatRequest::new("model", [
///     ChatMessage::user("Hello!")
/// ]);
///
/// // With iterator
/// let messages = vec![ChatMessage::user("Hi")];
/// let request = ChatRequest::new("model", messages);
/// ```
```
