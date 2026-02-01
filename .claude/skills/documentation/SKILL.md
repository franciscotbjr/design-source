# Documentation - Standards and Practices

## Documentation Files

### Required Files

| File | Purpose |
|------|---------|
| `README.md` | Project overview, quick start |
| `CHANGELOG.md` | Version history |
| `CONTRIBUTING.md` | How to contribute |
| `LICENSE` | License terms |

### Development Files

| File | Purpose |
|------|---------|
| `DEV_NOTES.md` | Development notes, discoveries |
| `ARCHITECTURE.md` | Module structure, design |
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

## Quick Start

\`\`\`rust
use crate_name::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // Example code
    Ok(())
}
\`\`\`

## Documentation

- [API Documentation](https://docs.rs/crate-name)
- [Examples](examples/)
- [Contributing](CONTRIBUTING.md)

## License

MIT
```

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

[2024-01-15] **Use thiserror for error handling**
- Context: Need a consistent error handling approach
- Decision: Use thiserror crate for derive macros
- Consequences: Clean error definitions, automatic Display impl

[2024-01-16] **Builder pattern for complex requests**
- Context: Requests have many optional fields
- Decision: Implement builder pattern for requests with 3+ optional fields
- Consequences: Ergonomic API, type-safe construction
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

- **[API]** Decide on error response format
  - Option A: Match upstream API exactly
  - Option B: Normalize errors
  - Blocks: Error handling implementation

## Resolved

- ~~**[Architecture]** Module organization~~ - Resolved: Three-layer architecture
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
//! ```rust
//! use crate::module::Type;
//!
//! let instance = Type::new();
//! instance.method();
//! ```
//!
//! ## Related
//!
//! - [`other_module`] - Related functionality
```

### Struct Documentation
```rust
/// A client for interacting with the API.
///
/// The client handles HTTP communication, authentication,
/// and request/response serialization.
///
/// # Examples
///
/// ## Creating a client
///
/// ```
/// let client = Client::new();
/// ```
///
/// ## With custom configuration
///
/// ```
/// let client = Client::builder()
///     .base_url("http://localhost:8080")
///     .timeout(Duration::from_secs(60))
///     .build()?;
/// ```
pub struct Client { ... }
```

### Method Documentation
```rust
/// Sends a request to generate text.
///
/// This method sends the request and waits for the complete response.
/// For streaming responses, use [`generate_stream`](Self::generate_stream).
///
/// # Arguments
///
/// * `request` - The generation parameters
///
/// # Returns
///
/// The complete generated response.
///
/// # Errors
///
/// * [`Error::ModelNotFound`] - If the specified model doesn't exist
/// * [`Error::Timeout`] - If the request exceeds the timeout
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), Error> {
/// let response = client.generate(request).await?;
/// println!("{}", response.text);
/// # Ok(())
/// # }
/// ```
///
/// # See Also
///
/// * [`generate_stream`](Self::generate_stream) - Streaming variant
pub async fn generate(&self, request: Request) -> Result<Response> { ... }
```

## Example Files

### Naming Convention
```
examples/
├── feature_basic_async.rs      # Basic async usage
├── feature_basic_sync.rs       # Basic sync usage
├── feature_advanced_async.rs   # Advanced async usage
├── feature_with_options.rs     # With various options
└── feature_streaming.rs        # Streaming example
```

### Example Structure
```rust
//! Example: Basic Feature Usage
//!
//! This example demonstrates how to use the feature API.
//!
//! ## Running
//!
//! ```bash
//! cargo run --example feature_basic_async
//! ```

use library::{Client, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let client = Client::new();

    // Create request
    let request = Request::new("param");

    // Execute
    let response = client.method(request).await?;

    // Output
    println!("Result: {}", response.result);

    Ok(())
}
```

## Specification Files

### YAML Spec Format
```yaml
# spec/primitives/01-endpoint-name.yaml

endpoint: POST /api/endpoint
complexity: simple | medium | complex
streaming: true | false

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
```
