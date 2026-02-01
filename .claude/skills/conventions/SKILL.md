# Conventions - Rust Coding Standards

## Code Style

### Formatting
- Use `rustfmt` with default settings
- Maximum line length: 100 characters
- Use 4-space indentation

### Naming Conventions
| Item | Style | Example |
|------|-------|---------|
| Types | PascalCase | `ChatRequest` |
| Functions | snake_case | `send_message` |
| Constants | SCREAMING_SNAKE_CASE | `DEFAULT_TIMEOUT` |
| Modules | snake_case | `http_client` |
| Lifetimes | lowercase, short | `'a`, `'de` |

### Import Organization
```rust
// 1. Standard library
use std::collections::HashMap;

// 2. External crates
use reqwest::Client;
use serde::{Deserialize, Serialize};

// 3. Internal modules
use crate::primitives::ChatRequest;
```

## Type Definitions

### Struct Patterns
```rust
// Use derive macros consistently
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<String>,
}
```

### Enum Patterns
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Inactive,
    Pending,
}
```

### Builder Pattern
```rust
impl Request {
    pub fn new(required: String) -> Self { ... }

    pub fn builder() -> RequestBuilder { ... }
}

#[derive(Default)]
pub struct RequestBuilder {
    required: Option<String>,
    optional: Option<String>,
}

impl RequestBuilder {
    pub fn required(mut self, value: impl Into<String>) -> Self {
        self.required = Some(value.into());
        self
    }

    pub fn optional(mut self, value: impl Into<String>) -> Self {
        self.optional = Some(value.into());
        self
    }

    pub fn build(self) -> Result<Request, ValidationError> {
        let required = self.required
            .ok_or(ValidationError::missing("required"))?;
        Ok(Request {
            required,
            optional: self.optional,
        })
    }
}
```

## Error Handling

### Use thiserror
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Invalid input: {field} - {message}")]
    Validation { field: String, message: String },
}
```

### Result Type Alias
```rust
pub type Result<T> = std::result::Result<T, LibError>;
```

### Error Propagation
```rust
// Use ? operator
fn process() -> Result<Data> {
    let response = client.get(url).send()?;
    let data: Data = response.json()?;
    Ok(data)
}
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

### Custom Serialization
```rust
#[derive(Serialize, Deserialize)]
pub struct Duration {
    #[serde(serialize_with = "serialize_duration")]
    #[serde(deserialize_with = "deserialize_duration")]
    pub value: std::time::Duration,
}
```

### Enum Variants
```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Json,
    #[serde(rename = "")]
    Empty,
    #[serde(untagged)]
    Custom(String),
}
```

## Documentation

### Module Documentation
```rust
//! # Module Name
//!
//! Brief description of the module.
//!
//! ## Examples
//!
//! ```rust
//! use crate::module::Type;
//! let instance = Type::new();
//! ```
```

### Function Documentation
```rust
/// Brief description.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Returns `Error::Variant` when condition occurs
///
/// # Examples
///
/// ```rust
/// let result = function(arg)?;
/// ```
pub fn function(param: Type) -> Result<Output> { ... }
```

## Testing Conventions

### Test Organization
```rust
// Tests in same file
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
#[test]
fn function_name_condition_expected_behavior() { ... }

// Examples:
fn parse_valid_json_returns_struct() { ... }
fn validate_empty_string_returns_error() { ... }
fn builder_without_required_fails() { ... }
```

### Async Tests
```rust
#[tokio::test]
async fn async_function_works() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

## Visibility Rules

### Default to Private
- Start with private (`fn`, `struct`)
- Only expose what's needed (`pub`)
- Use `pub(crate)` for internal sharing
- Use `pub(super)` for parent module access

### Re-exports
```rust
// In lib.rs - expose clean public API
pub use primitives::{Request, Response};
pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
```
