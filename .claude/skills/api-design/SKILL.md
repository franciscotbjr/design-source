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
// Simple case - one line
let response = client.generate("llama3", "Hello").await?;

// Complex case - full control
let response = client.generate(
    GenerateRequest::builder()
        .model("llama3")
        .prompt("Hello")
        .temperature(0.7)
        .num_ctx(4096)
        .build()?
).await?;
```

### 3. Flexible Input
Accept various input types for convenience.

```rust
// Accept anything that converts to String
pub fn new(model: impl Into<String>) -> Self;

// Accept owned or borrowed
pub fn with_prompt(prompt: impl AsRef<str>) -> Self;

// Accept iterators
pub fn with_messages(messages: impl IntoIterator<Item = Message>) -> Self;
```

## Constructor Patterns

### Simple Constructor
```rust
impl Request {
    /// Create a new request with required fields only.
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            ..Default::default()
        }
    }
}
```

### Builder Pattern
```rust
impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }
}

#[derive(Default)]
pub struct RequestBuilder {
    model: Option<String>,
    prompt: Option<String>,
    temperature: Option<f32>,
}

impl RequestBuilder {
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn build(self) -> Result<Request, ValidationError> {
        Ok(Request {
            model: self.model.ok_or(ValidationError::missing("model"))?,
            prompt: self.prompt.ok_or(ValidationError::missing("prompt"))?,
            temperature: self.temperature,
        })
    }
}
```

### With-Method Pattern
```rust
impl Request {
    pub fn new(model: impl Into<String>) -> Self { ... }

    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }
}

// Usage
let request = Request::new("llama3")
    .with_prompt("Hello")
    .with_temperature(0.7);
```

## Method Naming

### Async/Sync Variants
```rust
// Async is the default (no suffix)
pub async fn generate(&self, req: Request) -> Result<Response>;

// Sync variant has _sync suffix
pub fn generate_sync(&self, req: Request) -> Result<Response>;
```

### Streaming Variants
```rust
// Non-streaming
pub async fn generate(&self, req: Request) -> Result<Response>;

// Streaming returns a Stream
pub async fn generate_stream(&self, req: Request) -> Result<impl Stream<Item = Result<Chunk>>>;
```

### Naming Conventions
| Pattern | Example |
|---------|---------|
| Get single | `get_model()` |
| Get collection | `list_models()` |
| Create | `create_model()` |
| Update | `update_model()` |
| Delete | `delete_model()` |
| Action | `run_model()`, `stop_model()` |

## Error Design

### Error Enum
```rust
#[derive(Debug, thiserror::Error)]
pub enum OllamaError {
    /// HTTP transport error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    /// Request validation failed
    #[error("Validation error: {0}")]
    Validation(String),

    /// Model not found
    #[error("Model not found: {0}")]
    ModelNotFound(String),
}
```

### Error Helpers
```rust
impl OllamaError {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::Api { status: 404, .. } | Self::ModelNotFound(_))
    }

    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_))
    }
}
```

## Optional Fields

### Serde Configuration
```rust
#[derive(Serialize, Deserialize)]
pub struct Request {
    /// Required field - always present
    pub model: String,

    /// Optional field - omitted when None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Optional with default - uses default when missing
    #[serde(default)]
    pub stream: bool,

    /// Optional with custom default
    #[serde(default = "default_num_ctx")]
    pub num_ctx: u32,
}

fn default_num_ctx() -> u32 { 2048 }
```

### Flexible Enums
```rust
/// Format can be known variants or custom string
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Json,
    #[serde(untagged)]
    Custom(String),
}

impl Format {
    pub fn json() -> Self { Self::Json }
    pub fn custom(s: impl Into<String>) -> Self { Self::Custom(s.into()) }
}
```

## Conversion Traits

### From/Into
```rust
impl From<&str> for ModelName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ModelName {
    fn from(s: String) -> Self {
        Self(s)
    }
}
```

### TryFrom for Validation
```rust
impl TryFrom<String> for ValidatedEmail {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains('@') {
            Ok(Self(value))
        } else {
            Err(ValidationError::invalid("email"))
        }
    }
}
```

## Documentation Standards

### Type Documentation
```rust
/// A request to generate text from a model.
///
/// # Examples
///
/// ```
/// use ollama_oxide::GenerateRequest;
///
/// // Simple usage
/// let request = GenerateRequest::new("llama3", "Hello, world!");
///
/// // With options
/// let request = GenerateRequest::builder()
///     .model("llama3")
///     .prompt("Hello")
///     .temperature(0.7)
///     .build()?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest { ... }
```

### Method Documentation
```rust
/// Generate a completion from the model.
///
/// # Arguments
///
/// * `request` - The generation request containing model and prompt
///
/// # Returns
///
/// The complete response after generation finishes.
///
/// # Errors
///
/// Returns [`OllamaError::ModelNotFound`] if the model doesn't exist.
/// Returns [`OllamaError::Api`] for other API errors.
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), ollama_oxide::OllamaError> {
/// let client = OllamaClient::new();
/// let response = client.generate(GenerateRequest::new("llama3", "Hi")).await?;
/// println!("{}", response.response);
/// # Ok(())
/// # }
/// ```
pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse>;
```
