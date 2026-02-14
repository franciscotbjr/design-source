# Conventions - Rust Coding Standards

## Code Style

### Formatting
- Use `rustfmt` with default settings
- Maximum line length: 100 characters
- Use 4-space indentation

### Naming Conventions
| Item | Style | Example |
|------|-------|---------|
| Types | PascalCase | `ChatRequest`, `ModelOptions` |
| Functions | snake_case | `send_message`, `with_format` |
| Constants | SCREAMING_SNAKE_CASE | `DEFAULT_TIMEOUT` |
| Modules | snake_case | `chat_request`, `api_async` |
| Lifetimes | lowercase, short | `'a`, `'de` |
| Error variants | `{Type}Error` suffix | `HttpError`, `TimeoutError` |
| Async methods | No suffix (default) | `version()`, `chat()` |
| Sync methods | `_blocking` suffix | `version_blocking()`, `chat_blocking()` |
| Streaming methods | `_stream` suffix | `chat_stream()`, `generate_stream()` |

### Import Organization
```rust
// 1. Standard library
use std::sync::Arc;
use std::time::Duration;

// 2. External crates
use reqwest::Client;
use serde::{Deserialize, Serialize};

// 3. Internal modules (crate)
use crate::{Error, Result};

// 4. Parent/sibling modules (super)
use super::ClientConfig;

// 5. Feature-gated imports
#[cfg(feature = "tools")]
use crate::tools::ToolDefinition;
```

## Type Definitions

### Struct Patterns
```rust
// Derive macros - consistent order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    /// Required field - always present
    pub model: String,

    /// Optional field - omitted from JSON when None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatSetting>,

    /// Feature-gated field
    #[cfg(feature = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}
```

### Enum Patterns
```rust
// Simple enums with string serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

// Untagged enums for flexible JSON
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormatSetting {
    /// Simple string format (e.g., "json")
    Named(String),
    /// JSON Schema for structured output
    Schema(serde_json::Value),
}
```

### With-Method Chain Pattern (Preferred over Builder)
```rust
impl Request {
    /// Constructor with required fields only
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            format: None,
            options: None,
            stream: Some(false),
            #[cfg(feature = "tools")]
            tools: None,
        }
    }

    /// Fluent optional field setters (with_ prefix)
    pub fn with_format(mut self, format: impl Into<FormatSetting>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn with_options(mut self, options: ModelOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Feature-gated methods
    #[cfg(feature = "tools")]
    pub fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Accessor methods (immutable references)
    pub fn model(&self) -> &str {
        &self.model
    }
}
```

### Constructor Flexibility
```rust
/// Accept any type that converts to String
pub fn new(model: impl Into<String>) -> Self { ... }

/// Accept any iterator for collection fields
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
```

## Single Concern Per File

### One Primary Type Per File
- Each `.rs` file contains one main type with its implementations
- File named after the type: `chat_request.rs` contains `ChatRequest`
- Includes `impl` blocks, trait implementations, and `#[cfg(test)]` module

### mod.rs as Facade Only
```rust
// src/inference/mod.rs - ONLY re-exports, no logic
mod chat_message;
mod chat_request;
mod chat_response;
mod chat_role;
mod embed_request;
mod embed_response;
mod generate_request;
mod generate_response;
// ... one mod per file

pub use chat_message::ChatMessage;
pub use chat_request::ChatRequest;
pub use chat_response::ChatResponse;
pub use chat_role::ChatRole;
pub use embed_request::EmbedRequest;
pub use embed_response::EmbedResponse;
pub use generate_request::GenerateRequest;
pub use generate_response::GenerateResponse;
// ... one pub use per type
```

## Error Handling

### Error Enum with {Type}Error Suffix
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
```

### From Implementations
```rust
// Manual From for types converted to String (avoid exposing external types)
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

// #[from] only when preserving original error type
#[error("Invalid URL: {0}")]
InvalidUrlError(#[from] url::ParseError),
```

### Result Type Alias
```rust
pub type Result<T> = std::result::Result<T, Error>;
```

### Error Propagation
```rust
// Use ? operator with automatic From conversion
async fn version(&self) -> Result<VersionResponse> {
    let url = self.config.url(Endpoints::VERSION);
    self.get_with_retry(&url).await
}
```

## Conditional Compilation

### Module Level
```rust
// In lib.rs - gate entire modules
#[cfg(feature = "inference")]
pub mod inference;

#[cfg(feature = "model")]
pub mod model;

#[cfg(feature = "tools")]
pub mod tools;
```

### Struct Field Level
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,

    // Field only exists when "tools" feature is enabled
    #[cfg(feature = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}
```

### Method Level
```rust
impl ChatRequest {
    // Method only available with "tools" feature
    #[cfg(feature = "tools")]
    pub fn with_tools(mut self, tools: Vec<ToolDefinition>) -> Self {
        self.tools = Some(tools);
        self
    }

    #[cfg(feature = "tools")]
    pub fn has_tools(&self) -> bool {
        self.tools.as_ref().map(|t| !t.is_empty()).unwrap_or(false)
    }
}
```

### Trait Method Level
```rust
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    async fn version(&self) -> Result<VersionResponse>;
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    // Only available with "model" feature
    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse>;

    #[cfg(feature = "model")]
    async fn delete_model(&self, request: DeleteRequest) -> Result<()>;
}
```

### Import Level
```rust
// Conditional imports
#[cfg(feature = "model")]
use crate::{
    CopyRequest, CreateRequest, DeleteRequest, ListResponse,
    PullRequest, PullResponse, ShowRequest, ShowResponse,
};
```

### Test Level
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() { /* ... */ }

    // Test only runs when "tools" feature is enabled
    #[cfg(feature = "tools")]
    #[test]
    fn test_with_tools() { /* ... */ }

    // Test for both feature states
    #[cfg(not(feature = "tools"))]
    #[test]
    fn test_without_tools() { /* ... */ }
}
```

### Example/Binary Level (Cargo.toml)
```toml
[[example]]
name = "chat_with_tools_async"
required-features = ["tools"]

[[example]]
name = "model_create_async"
required-features = ["model"]
```

## Serde Patterns

### Optional Fields
```rust
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub enabled: bool,

    #[serde(default = "default_count")]
    pub count: u32,
}

fn default_count() -> u32 { 10 }
```

### Untagged Enums for Flexible JSON
```rust
// Accept multiple JSON formats for the same field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThinkSetting {
    /// Boolean: true/false
    Toggle(bool),
    /// String budget: "1024" tokens
    Budget(String),
}

impl ThinkSetting {
    pub fn enabled() -> Self { Self::Toggle(true) }
    pub fn disabled() -> Self { Self::Toggle(false) }
}
```

### Enum with rename_all
```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Json,
    Text,
}
```

## Documentation

### Module Documentation
```rust
//! Chat request type for POST /api/chat endpoint.
```

### Type Documentation with Examples
```rust
/// Request body for POST /api/chat endpoint.
///
/// Generates the next message in a chat conversation.
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
```

### Method Documentation
```rust
/// Create a new chat request.
///
/// # Arguments
///
/// * `model` - Name of the model to use
/// * `messages` - Conversation history as an iterator of messages
///
/// # Errors
///
/// Returns an error if the request cannot be serialized
///
/// # Examples
///
/// ```no_run
/// let request = ChatRequest::new("model", [
///     ChatMessage::user("Hello!")
/// ]);
/// ```
```

### No Doc Tests Convention
- Feature flag complexity makes doc tests hard to maintain
- Use `no_run` or `ignore` on doc examples
- All test coverage exists in `#[cfg(test)]` modules and `tests/` directory

## Testing Conventions

### Test Organization
```rust
// Tests in same file, after all implementations
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = create_input();

        // Act
        let result = function(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

### Test Naming
```rust
// Pattern: test_{type}_{action}_{variant}
#[test]
fn test_chat_request_new_with_vec() { ... }
fn test_chat_request_new_with_array() { ... }
fn test_chat_request_serialize() { ... }
fn test_chat_request_serialize_full() { ... }
fn test_chat_request_deserialize() { ... }
fn test_chat_request_clone() { ... }
fn test_chat_request_equality() { ... }
```

### Async Tests
```rust
#[tokio::test]
async fn test_version_async() {
    let result = client.version().await;
    assert!(result.is_ok());
}
```

### Feature-Gated Tests
```rust
#[cfg(feature = "tools")]
#[test]
fn test_chat_request_with_tools() {
    let tool = ToolDefinition::function("test", json!({}));
    let request = ChatRequest::new("model", [ChatMessage::user("Hi")])
        .with_tools(vec![tool]);
    assert!(request.has_tools());
}
```

### Integration Tests (tests/ directory)
```rust
// tests/client_chat_tests.rs
// Naming: client_{feature}_tests.rs

use mockito;
use ollama_oxide::{OllamaClient, OllamaApiAsync, ChatRequest, ChatMessage};

#[tokio::test]
async fn test_chat_async_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("POST", "/api/chat")
        .with_status(200)
        .with_body(json_response)
        .create_async()
        .await;
    // ...
}
```

## Visibility Rules

### Default to Private
- Start with private (`fn`, `struct`)
- Only expose what's needed (`pub`)
- Use `pub(super)` for module-internal sharing (e.g., client internals)
- Use `pub(crate)` for crate-internal sharing

### Module Internal Fields
```rust
pub struct OllamaClient {
    pub(super) config: ClientConfig,    // Accessible within http module
    pub(super) client: Arc<Client>,     // Not exposed to library users
}
```

### Re-exports in lib.rs
```rust
// Expose clean public API - users import from crate root
pub use error::{Error, Result};

#[cfg(feature = "http")]
pub use http::{ClientConfig, OllamaApiAsync, OllamaApiSync, OllamaClient};

#[cfg(feature = "inference")]
pub use inference::{ChatMessage, ChatRequest, ChatResponse, /* ... */};
```
