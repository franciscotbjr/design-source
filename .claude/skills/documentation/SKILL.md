# Documentation - Standards and Practices

## Documentation Files

### Required Files

| File | Purpose |
|------|---------|
| `README.md` | Project overview, quick start, feature flags |
| `CHANGELOG.md` | Version history |
| `CONTRIBUTING.md` | How to contribute, development setup |
| `ARCHITECTURE.md` | Module structure, design patterns, dependency rules |
| `LICENSE` | License terms |

### Development Files

| File | Purpose |
|------|---------|
| `DEV_NOTES.md` | Development notes, discoveries |
| `DECISIONS.md` | Architectural decisions log |
| `BLOCKERS.md` | Pending decisions, blockers |

## README.md Structure

```markdown
<div align="center">

# Project Name

Brief tagline describing the project.

[![Crates.io](https://img.shields.io/crates/v/crate-name.svg)](https://crates.io/crates/crate-name)
[![Documentation](https://docs.rs/crate-name/badge.svg)](https://docs.rs/crate-name)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

</div>

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

Add to your `Cargo.toml`:

\`\`\`toml
[dependencies]
crate-name = "0.1"
\`\`\`

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `http` | Yes | HTTP client layer |
| `inference` | Yes | Inference types (chat, generate, embed) |
| `model` | No | Model management operations |
| `tools` | No | Function calling with schema generation |

\`\`\`toml
# All features
crate-name = { version = "0.1", features = ["model", "tools"] }
\`\`\`

## Quick Start

\`\`\`rust
use crate_name::{OllamaClient, OllamaApiAsync};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::default()?;
    let version = client.version().await?;
    println!("Version: {}", version.version);
    Ok(())
}
\`\`\`

## Documentation

- [API Documentation](https://docs.rs/crate-name)
- [Architecture Guide](ARCHITECTURE.md)
- [Examples](examples/)
- [Contributing](CONTRIBUTING.md)

## License

MIT
```

## ARCHITECTURE.md Structure

ARCHITECTURE.md is a **required** file that documents the module organization, design patterns, and dependency rules. It serves as the primary reference for understanding the codebase structure.

### Required Sections

```markdown
# Architecture Guide

## Design Principles
- Single Concern Per File (one type per .rs file)
- Module as Facade (mod.rs = declarations + re-exports only)
- Explicit Over Implicit (file names match primary types)

## Module Organization Rule
- Show the canonical module structure with mod.rs + type files
- Include a concrete example from the project

## Current Structure
- Full `src/` tree showing all modules, files, and their purposes
- Feature annotations: which feature flag gates each module

## Feature Flag Architecture
- Feature table from Cargo.toml
- Feature dependency graph (ASCII or Mermaid)
- Conditional compilation patterns (module, field, method levels)

## Adding New Components
- Step-by-step for new type, new endpoint, new configuration
- Reference the pattern of existing implementations

## Design Patterns
- Visibility control (pub(super), pub(crate))
- Trait per concern (OllamaApiAsync / OllamaApiSync)
- Dependency hierarchy with rules table

## Architecture Diagrams (Mermaid)
- Module and component relations (class diagram)
- API call flow (state diagram)
- Request/Response type patterns
- Tool execution flow (if applicable)

## Testing Architecture
- Unit tests in source files (internal behavior)
- Public interface tests in tests/ (mockito, no external deps)
- Integration tests as examples/ (require running server)

## Version History
- Track architectural changes with dates
```

### Key Principles for ARCHITECTURE.md

1. **Keep it current** - Update when module structure changes
2. **Show real code** - Use actual project examples, not generic placeholders
3. **Document rules** - Dependency direction, visibility, feature flag patterns
4. **Include diagrams** - Mermaid diagrams for complex relationships
5. **Version history** - Track architectural evolution with dates

## CHANGELOG.md Format

Follow [Keep a Changelog](https://keepachangelog.com) format:

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New feature description

### Changed
- Modified behavior description

### Fixed
- Bug fix description

## [0.1.0] - 2024-01-15

### Added
- Initial release
- Feature A
- Feature B
```

### Change Categories

| Category | Usage |
|----------|-------|
| Added | New features |
| Changed | Changes in existing functionality |
| Deprecated | Soon-to-be removed features |
| Removed | Now removed features |
| Fixed | Bug fixes |
| Security | Vulnerability fixes |

## DECISIONS.md Format

```markdown
# Architectural Decisions Log

## Decision Format

[YYYY-MM-DD] **Decision title**
- Context: Why this decision was needed
- Decision: What was decided
- Consequences: What results from this decision

---

## Decisions

[2024-01-15] **With-method chain pattern over Builder pattern**
- Context: Requests have many optional fields, need ergonomic construction
- Decision: Use `with_*()` methods returning Self instead of separate Builder types
- Consequences: Simpler API, no separate builder struct, chainable calls

[2024-01-16] **Trait-based API with async_trait**
- Context: Need both async and sync interfaces for the HTTP client
- Decision: Define OllamaApiAsync and OllamaApiSync traits, implement on OllamaClient
- Consequences: Clear API contract, testable via mock implementations

[2024-01-17] **{Type}Error suffix for error variants**
- Context: Need clear, descriptive error variant names
- Decision: Use descriptive suffixes like HttpError, TimeoutError, MaxRetriesExceededError
- Consequences: Self-documenting error types, easy to match on
```

## BLOCKERS.md Format

```markdown
# Active Blockers

Pending decisions and blockers that need resolution.

## Format
- **[Category]** Description of blocker
  - Options considered
  - Impact if not resolved

---

## Current Blockers

- **[Streaming]** Choose streaming implementation approach
  - Option A: tokio-stream
  - Option B: async-stream
  - Blocks: All streaming endpoints

## Resolved

- ~~**[Architecture]** Module organization~~ - Resolved: Feature-flagged modules
```

## Rustdoc Guidelines

### Module Documentation
```rust
//! # Module Name
//!
//! Brief description of the module's purpose.
//!
//! ## Overview
//!
//! More detailed explanation of what this module provides.
//!
//! ## Examples
//!
//! ```no_run
//! use crate::module::Type;
//!
//! let instance = Type::new("param");
//! let configured = instance.with_option("value");
//! ```
//!
//! ## Related
//!
//! - [`other_module`] - Related functionality
```

### Struct Documentation
```rust
/// A request to generate chat completions.
///
/// Uses with-method chain pattern for optional fields.
///
/// # Examples
///
/// ## Basic usage
///
/// ```no_run
/// let request = ChatRequest::new("model", [ChatMessage::user("Hello")]);
/// ```
///
/// ## With options
///
/// ```no_run
/// let request = ChatRequest::new("model", [ChatMessage::user("Hello")])
///     .with_options(ModelOptions::new().with_temperature(0.7))
///     .with_format(FormatSetting::json());
/// ```
pub struct ChatRequest { ... }
```

### Method Documentation
```rust
/// Sends a chat request and returns the complete response.
///
/// For streaming responses, use [`chat_stream`](Self::chat_stream).
///
/// # Arguments
///
/// * `request` - The chat parameters including model and messages
///
/// # Returns
///
/// The complete chat response with content and metadata.
///
/// # Errors
///
/// * [`HttpError`](crate::Error::HttpError) - HTTP communication failure
/// * [`TimeoutError`](crate::Error::TimeoutError) - Request exceeded timeout
/// * [`MaxRetriesExceededError`](crate::Error::MaxRetriesExceededError) - All retries failed
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), ollama_oxide::Error> {
/// let response = client.chat(&request).await?;
/// println!("{}", response.content().unwrap_or("No response"));
/// # Ok(())
/// # }
/// ```
pub async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse> { ... }
```

### Doc Test Convention

All doc examples use `no_run` attribute:
```rust
/// ```no_run
/// let client = OllamaClient::default()?;
/// ```
```

**Rationale:** Doc examples require a running Ollama server. Real test coverage comes from unit tests (mockito) and integration test examples.

## Example Files

### Naming Convention

Pattern: `{feature}_{variant}_{mode}.rs`

| Component | Values | Description |
|-----------|--------|-------------|
| `feature` | `chat`, `generate`, `embed`, `model_create`, etc. | The API feature being demonstrated |
| `variant` | (optional) `with_tools`, `cli`, `concise`, `custom` | Specific variation or configuration |
| `mode` | `async`, `sync` | Async or sync execution mode |

### Real Examples from ollama-oxide

```
examples/
├── chat_async.rs                      # Basic chat (async)
├── chat_sync.rs                       # Basic chat (sync)
├── chat_cli_async.rs                  # Interactive CLI chat
├── chat_with_tools_async.rs           # Chat with tool definitions
├── chat_with_tools_registry_async.rs  # Chat with ToolRegistry dispatch
├── generate_async.rs                  # Text generation (async)
├── generate_sync.rs                   # Text generation (sync)
├── generate_concise.rs                # Minimal generation example
├── embed_async.rs                     # Embeddings (async)
├── embed_sync.rs                      # Embeddings (sync)
├── get_version_async.rs               # Server version (async)
├── get_version_sync.rs                # Server version (sync)
├── get_version_custom.rs              # Version with custom config
├── list_models_async.rs               # List models (async)
├── list_models_sync.rs                # List models (sync)
├── list_running_models_async.rs       # Running models (async)
├── list_running_models_sync.rs        # Running models (sync)
├── show_model_async.rs                # Show model info (async)
├── show_model_sync.rs                 # Show model info (sync)
├── copy_model_async.rs                # Copy model (async)
├── copy_model_sync.rs                 # Copy model (sync)
├── model_create_async.rs              # Create model (async)
├── model_delete_async.rs              # Delete model (async)
├── model_delete_sync.rs               # Delete model (sync)
├── model_cleanup_async.rs             # Delete multiple models
├── pull_model_async.rs                # Pull model (async)
├── pull_model_sync.rs                 # Pull model (sync)
├── push_model_async.rs                # Push model (async)
├── push_model_sync.rs                 # Push model (sync)
└── tools_async.rs                     # Tools module standalone
```

### Example File Structure

```rust
//! Example: Chat completion (async)
//!
//! This example demonstrates how to use the chat API
//! for conversational interactions.
//!
//! Run with: cargo run --example chat_async
//!
//! Note: Requires a running Ollama server with a model installed
//! (e.g., qwen3:0.6b, llama3.2, etc.)

use ollama_oxide::{
    ChatMessage, ChatRequest, OllamaApiAsync, OllamaClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default configuration
    let client = OllamaClient::default()?;

    // Model to use
    let model = "qwen3:0.6b";

    // Build request with with-method chain
    let request = ChatRequest::new(
        model,
        [ChatMessage::user("Hello! What can you help me with?")],
    );

    // Execute
    let response = client.chat(&request).await?;

    // Access response via helper methods
    println!("Assistant: {}", response.content().unwrap_or("No response"));
    println!("Done: {}", response.is_done());

    Ok(())
}
```

### Feature-Gated Examples in Cargo.toml

```toml
[[example]]
name = "chat_async"
# No required-features: uses default features

[[example]]
name = "chat_with_tools_async"
required-features = ["tools"]

[[example]]
name = "model_create_async"
required-features = ["model"]
```

## Specification Files

### Directory Structure

```
spec/
├── apis/                    # API endpoint specifications
│   ├── 01-version.yaml
│   ├── 02-generate.yaml
│   ├── 03-chat.yaml
│   └── ...
└── api-analysis.md          # Endpoint complexity analysis

impl/                        # Implementation plans
├── phase-1.md
├── phase-2.md
└── ...
```

### YAML Spec Format

```yaml
# spec/apis/01-endpoint-name.yaml

endpoint: POST /api/endpoint
complexity: simple | medium | complex
streaming: true | false
feature: inference | model | tools

description: |
  Brief description of what this endpoint does.

request:
  type: EndpointRequest
  fields:
    - name: required_field
      type: String
      required: true
      description: Description of the field

    - name: optional_field
      type: Option<String>
      required: false
      description: Optional field description

  example: |
    {
      "required_field": "value"
    }

response:
  type: EndpointResponse
  fields:
    - name: result
      type: String
      description: The result

  example: |
    {
      "result": "success"
    }

errors:
  - status: 400
    description: Invalid request
  - status: 404
    description: Resource not found

trait_methods:
  async: endpoint_name
  sync: endpoint_name_blocking
```
