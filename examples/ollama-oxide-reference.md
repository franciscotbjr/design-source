# Reference Implementation: ollama-oxide

This document provides reference examples from the ollama-oxide project, demonstrating the patterns and conventions described in this methodology guide.

## Project Structure

```
ollama-oxide/
├── Cargo.toml                    # Single crate configuration
├── src/
│   ├── lib.rs                    # Crate root with re-exports
│   ├── primitives/               # Type definitions
│   │   ├── mod.rs
│   │   ├── generate.rs           # Generate endpoint types
│   │   ├── chat.rs               # Chat endpoint types
│   │   ├── embed.rs              # Embed endpoint types
│   │   └── ...
│   ├── http/                     # HTTP client
│   │   ├── mod.rs
│   │   ├── client.rs             # OllamaClient struct
│   │   └── endpoints/            # Per-endpoint implementations
│   └── conveniences/             # High-level APIs (feature-gated)
├── spec/
│   ├── definition.md             # Project definition
│   ├── api-analysis.md           # API complexity analysis
│   ├── primitives/               # YAML specs per endpoint
│   └── impl-plans/               # Implementation plans
├── examples/
│   ├── generate_basic_async.rs
│   ├── chat_with_tools_async.rs
│   └── ...
└── tests/
    └── integration/
```

## Specification Example

From `spec/primitives/01-ollama_api_generate.yaml`:

```yaml
endpoint: POST /api/generate
complexity: complex
streaming: true

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

From `src/primitives/generate.rs`:

```rust
use serde::{Deserialize, Serialize};

/// Request for text generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// The model to use for generation.
    pub model: String,

    /// The prompt to generate from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Whether to stream the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Sampling temperature (0.0-2.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl GenerateRequest {
    /// Create a new generate request.
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: Some(prompt.into()),
            stream: None,
            temperature: None,
        }
    }

    /// Create a builder for more complex requests.
    pub fn builder() -> GenerateRequestBuilder {
        GenerateRequestBuilder::default()
    }
}

/// Response from text generation.
#[derive(Debug, Clone, Deserialize)]
pub struct GenerateResponse {
    /// The model that was used.
    pub model: String,

    /// The generated text.
    pub response: String,

    /// Whether generation is complete.
    pub done: bool,

    /// Context tokens for continuation.
    #[serde(default)]
    pub context: Vec<i64>,
}
```

## Builder Pattern Example

```rust
#[derive(Default)]
pub struct GenerateRequestBuilder {
    model: Option<String>,
    prompt: Option<String>,
    stream: Option<bool>,
    temperature: Option<f32>,
}

impl GenerateRequestBuilder {
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn build(self) -> Result<GenerateRequest, ValidationError> {
        let model = self.model
            .ok_or_else(|| ValidationError::missing_field("model"))?;

        Ok(GenerateRequest {
            model,
            prompt: self.prompt,
            stream: self.stream,
            temperature: self.temperature,
        })
    }
}
```

## HTTP Client Example

From `src/http/client.rs`:

```rust
use crate::error::{OllamaError, Result};
use crate::primitives::*;
use reqwest::Client as HttpClient;
use std::time::Duration;

/// Client for the Ollama API.
pub struct OllamaClient {
    http: HttpClient,
    base_url: String,
}

impl OllamaClient {
    /// Create a new client with default settings.
    pub fn new() -> Self {
        Self::with_base_url("http://localhost:11434")
    }

    /// Create a client with a custom base URL.
    pub fn with_base_url(url: impl Into<String>) -> Self {
        Self {
            http: HttpClient::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: url.into(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

impl Default for OllamaClient {
    fn default() -> Self {
        Self::new()
    }
}
```

## Endpoint Implementation Example

From `src/http/endpoints/generate.rs`:

```rust
impl OllamaClient {
    /// Generate text from a prompt.
    pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        // Ensure stream is false for non-streaming
        let mut request = request;
        request.stream = Some(false);

        let url = self.url("/api/generate");

        let response = self.http
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.extract_error(response).await);
        }

        response.json().await.map_err(Into::into)
    }

    /// Generate text (blocking version).
    pub fn generate_sync(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        tokio::runtime::Runtime::new()
            .expect("Failed to create runtime")
            .block_on(self.generate(request))
    }
}
```

## Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_request_serialization() {
        let request = GenerateRequest::new("llama3", "Hello");
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains(r#""model":"llama3""#));
        assert!(json.contains(r#""prompt":"Hello""#));
        // Optional None fields should be omitted
        assert!(!json.contains("temperature"));
    }

    #[test]
    fn test_generate_response_deserialization() {
        let json = r#"{
            "model": "llama3",
            "response": "Hi there!",
            "done": true,
            "context": [1, 2, 3]
        }"#;

        let response: GenerateResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.model, "llama3");
        assert_eq!(response.response, "Hi there!");
        assert!(response.done);
        assert_eq!(response.context, vec![1, 2, 3]);
    }

    #[test]
    fn test_builder_validates_required_fields() {
        let result = GenerateRequestBuilder::default()
            .prompt("Hello")
            .build();

        assert!(result.is_err());
    }
}
```

## Example File

From `examples/generate_basic_async.rs`:

```rust
//! Basic async generation example.
//!
//! Demonstrates simple text generation using the Ollama API.
//!
//! ## Running
//!
//! ```bash
//! # Start Ollama server first
//! ollama serve
//!
//! # Run the example
//! cargo run --example generate_basic_async
//! ```

use ollama_oxide::{OllamaClient, GenerateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default settings (localhost:11434)
    let client = OllamaClient::new();

    // Create a simple request
    let request = GenerateRequest::new("llama3", "Why is the sky blue?");

    // Generate response
    println!("Generating response...");
    let response = client.generate(request).await?;

    // Print result
    println!("\nModel: {}", response.model);
    println!("Response: {}", response.response);

    Ok(())
}
```

## Decision Log Example

From `DECISIONS.md`:

```markdown
[2024-01-15] **Use single crate with feature flags**
- Context: Choosing between workspace (multiple crates) vs single crate
- Decision: Single crate with primitives/http/conveniences as modules
- Consequences: Simpler dependency management, easier versioning

[2024-01-16] **Builder pattern for 3+ optional fields**
- Context: Many request types have numerous optional parameters
- Decision: Implement builder pattern when type has 3 or more optional fields
- Consequences: Ergonomic construction, type-safe, clear API

[2024-01-17] **ChatRequest::new() accepts IntoIterator<Item=ChatMessage>**
- Context: Users need to pass message history
- Decision: Accept any iterator that yields ChatMessage
- Consequences: Flexible API, works with Vec, arrays, iterators
```

## Session Cache Script

From `.claude/scripts/read_cache.rs`:

```rust
//! Session cache reader for context restoration.
//!
//! Reads and displays cached session information to help
//! the AI assistant restore context between sessions.

use std::fs;
use std::path::Path;

fn main() {
    let cache_path = ".claude/cache/session.json";

    if !Path::new(cache_path).exists() {
        println!("❌ No previous conversation found");
        println!("💡 Tip: Run /save-session-cache to create a cache");
        return;
    }

    let content = fs::read_to_string(cache_path)
        .expect("Failed to read cache");

    println!("🔍 Loading previous conversation context...");
    println!();
    // ... display cache contents
}
```

## Key Patterns Summary

| Pattern | When to Use | Example |
|---------|-------------|---------|
| `new()` constructor | Required fields only | `Request::new("model")` |
| Builder pattern | 3+ optional fields | `Request::builder().field(v).build()` |
| `impl Into<String>` | String-like parameters | `fn new(s: impl Into<String>)` |
| `skip_serializing_if` | Optional serde fields | `#[serde(skip_serializing_if = "Option::is_none")]` |
| `_sync` suffix | Blocking variants | `client.generate_sync(req)` |
| `_stream` suffix | Streaming variants | `client.generate_stream(req)` |

This reference implementation demonstrates the complete workflow from specification to working code, following the methodology defined in this guide.
