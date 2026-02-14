# Phase 4: Conveniences

**Version Target:** v0.4.0 → v1.0.0
**Focus:** High-level APIs, helpers, tools module, documentation polish

## Objectives

1. Create ergonomic high-level APIs (feature-gated under `conveniences`)
2. Implement tools module (feature-gated under `tools`)
3. Polish documentation with `no_run` examples
4. Prepare for release

## Prerequisites

- Phases 1-3 completed
- Core functionality working with trait-based API
- All tests passing with `--all-features`

## Convenience API Patterns

### Simple Wrappers
```rust
// src/conveniences/mod.rs

use crate::{
    ChatMessage, ChatRequest, ChatResponse, GenerateRequest, GenerateResponse,
    OllamaApiAsync, OllamaClient, Result,
};

impl OllamaClient {
    /// Generate text with a simple prompt.
    ///
    /// Convenience wrapper around [`generate`](OllamaApiAsync::generate).
    ///
    /// # Example
    ///
    /// ```no_run
    /// let text = client.prompt("qwen3:0.6b", "Hello!").await?;
    /// ```
    pub async fn prompt(&self, model: &str, prompt: &str) -> Result<String> {
        let request = GenerateRequest::new(model, prompt);
        let response = self.generate(&request).await?;
        Ok(response.content().unwrap_or_default().to_string())
    }
}
```

### Conversation Helper
```rust
/// A conversation builder for multi-turn chat interactions.
pub struct Conversation<'a> {
    client: &'a OllamaClient,
    model: String,
    messages: Vec<ChatMessage>,
}

impl<'a> Conversation<'a> {
    pub fn new(client: &'a OllamaClient, model: impl Into<String>) -> Self {
        Self {
            client,
            model: model.into(),
            messages: Vec::new(),
        }
    }

    /// Add a system message.
    pub fn with_system(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::system(content));
        self
    }

    /// Send a user message and get response.
    pub async fn send(&mut self, message: impl Into<String>) -> Result<String> {
        self.messages.push(ChatMessage::user(message));

        let request = ChatRequest::new(&self.model, self.messages.clone());
        let response = self.client.chat(&request).await?;

        let content = response.content().unwrap_or_default().to_string();
        self.messages.push(ChatMessage::assistant(&content));

        Ok(content)
    }
}
```

## Tools Module (Feature: "tools")

The tools module provides ergonomic function calling with automatic schema generation:

### Type Hierarchy
```
Tool (typed, not object-safe)
  → ToolWrapper<T> (bridge)
    → ErasedTool (object-safe, type-erased)
      → ToolRegistry (stores Arc<dyn ErasedTool>)
```

### Tool Trait
```rust
// src/tools/tool_trait.rs

pub trait Tool: Send + Sync + 'static {
    type Params: DeserializeOwned + JsonSchema;
    type Output: Serialize;

    fn name(&self) -> &str;
    fn description(&self) -> &str;

    fn execute(
        &self,
        params: Self::Params,
    ) -> impl Future<Output = ToolResult<Self::Output>> + Send;

    fn parameters_schema(&self) -> serde_json::Value { ... }
    fn to_definition(&self) -> ToolDefinition { ... }
}
```

### ToolRegistry
```rust
// src/tools/tool_registry.rs

pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn ErasedTool>>>>,
}

impl ToolRegistry {
    pub fn new() -> Self { ... }
    pub fn register<T: Tool>(&self, tool: T) { ... }
    pub fn definitions(&self) -> Vec<ToolDefinition> { ... }
    pub async fn execute(&self, call: &ToolCall) -> ToolResult<serde_json::Value> { ... }
    pub async fn execute_all(&self, response: &ChatResponse) -> Vec<ToolResult<serde_json::Value>> { ... }
}
```

### Usage Pattern
```rust
use ollama_oxide::{
    ChatMessage, ChatRequest, OllamaApiAsync, OllamaClient,
    Tool, ToolRegistry, ToolResult,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
struct WeatherParams {
    city: String,
}

#[derive(Debug, Serialize)]
struct WeatherResult {
    temperature: f64,
    condition: String,
}

struct GetWeather;

impl Tool for GetWeather {
    type Params = WeatherParams;
    type Output = WeatherResult;

    fn name(&self) -> &str { "get_weather" }
    fn description(&self) -> &str { "Get weather for a city" }

    async fn execute(&self, params: WeatherParams) -> ToolResult<WeatherResult> {
        Ok(WeatherResult {
            temperature: 22.0,
            condition: format!("Sunny in {}", params.city),
        })
    }
}

// Registration and dispatch
let registry = ToolRegistry::new();
registry.register(GetWeather);

let request = ChatRequest::new("qwen3:0.6b", [ChatMessage::user("Weather in Tokyo?")])
    .with_tools(registry.definitions());

let response = client.chat(&request).await?;
let results = registry.execute_all(&response).await;
```

## Feature Gating

```toml
# Cargo.toml

[features]
default = ["http", "inference"]
http = []
inference = []
model = ["http", "inference"]
tools = ["dep:schemars", "dep:futures"]
conveniences = ["http", "inference"]
```

```rust
// src/lib.rs

#[cfg(feature = "inference")]
pub mod inference;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "model")]
pub mod model;

#[cfg(feature = "tools")]
pub mod tools;

#[cfg(feature = "conveniences")]
pub mod conveniences;
```

## Documentation Polish

### Doc Test Convention
All doc examples use `no_run`:
```rust
/// ```no_run
/// let client = OllamaClient::default()?;
/// ```
```

### README Updates
- Add feature flags table
- Update quick start with trait imports
- Add section for tools feature
- Add troubleshooting section

### ARCHITECTURE.md Updates
- Add tools module class hierarchy diagram (Mermaid)
- Add tool execution flow sequence diagram
- Update dependency hierarchy

### Examples
- Create example for each major feature
- Use `{feature}_{variant}_{mode}.rs` naming
- Add `required-features` in Cargo.toml for non-default features

## Release Checklist

### Code Quality
- [ ] `cargo build --all-features` succeeds
- [ ] `cargo test --all-features` passes
- [ ] `cargo clippy --all-features -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] No compiler warnings

### Documentation
- [ ] All public items documented with `no_run` examples
- [ ] README.md comprehensive with feature flags
- [ ] ARCHITECTURE.md up to date with all modules
- [ ] CHANGELOG.md current
- [ ] CONTRIBUTING.md with dev setup

### Package Metadata
- [ ] Cargo.toml metadata complete
- [ ] License file present
- [ ] Repository URL correct
- [ ] Documentation URL set
- [ ] Feature flags documented

### Testing
- [ ] Unit tests pass (inside source files)
- [ ] Integration tests pass (`tests/client_*.rs` with mockito)
- [ ] Examples compile (`cargo build --examples --all-features`)
- [ ] Manual testing against real Ollama server

### Version
- [ ] Version number updated
- [ ] CHANGELOG entry added
- [ ] Git tag created

## Completion Criteria

1. Convenience APIs implemented and feature-gated
2. Tools module working with Tool trait, ToolRegistry, type erasure
3. All documentation complete with `no_run` examples
4. All examples follow naming convention
5. Release checklist passed
6. `cargo test --all-features` passes
7. Version 1.0.0 ready

## Post-Release

After v1.0.0:

1. Monitor for issues
2. Gather feedback
3. Plan streaming support (v1.1.0)
4. Maintain backward compatibility
