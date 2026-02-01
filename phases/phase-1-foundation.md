# Phase 1: Foundation

**Version Target:** v0.1.0
**Focus:** Project structure, core abstractions, error handling

## Objectives

1. Establish project structure and configuration
2. Define error handling patterns
3. Create HTTP client abstraction
4. Implement basic configuration

## Deliverables

### Project Structure
```
project/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── error.rs
│   ├── primitives/
│   │   └── mod.rs
│   └── http/
│       ├── mod.rs
│       └── client.rs
├── tests/
│   └── common/
│       └── mod.rs
└── examples/
```

### Cargo.toml Configuration
```toml
[package]
name = "library-name"
version = "0.1.0"
edition = "2021"
authors = ["Author Name <email@example.com>"]
description = "Brief description"
license = "MIT"
repository = "https://github.com/user/repo"
documentation = "https://docs.rs/library-name"
keywords = ["keyword1", "keyword2"]
categories = ["category"]

[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"

[dev-dependencies]
mockito = "1"

[features]
default = ["http"]
http = []
```

### Error Type
```rust
// src/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, LibraryError>;
```

### HTTP Client
```rust
// src/http/client.rs

use crate::error::Result;
use reqwest::Client as HttpClient;
use std::time::Duration;

pub struct Client {
    http: HttpClient,
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        Self::with_base_url("http://localhost:11434")
    }

    pub fn with_base_url(url: impl Into<String>) -> Self {
        Self {
            http: HttpClient::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: url.into(),
        }
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub(crate) fn http(&self) -> &HttpClient {
        &self.http
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
```

## Tasks Checklist

### Setup
- [ ] Initialize Cargo project
- [ ] Configure Cargo.toml with metadata
- [ ] Create directory structure
- [ ] Add .gitignore

### Error Handling
- [ ] Create error.rs with error enum
- [ ] Define Result type alias
- [ ] Add error conversion traits

### HTTP Client
- [ ] Create client struct
- [ ] Implement new() and builder pattern
- [ ] Add configuration options
- [ ] Write unit tests

### Documentation
- [ ] Create README.md
- [ ] Create CHANGELOG.md
- [ ] Create CONTRIBUTING.md
- [ ] Add LICENSE file

### Quality
- [ ] Run cargo clippy
- [ ] Run cargo fmt
- [ ] Ensure cargo build succeeds
- [ ] Ensure cargo test passes

## Completion Criteria

1. `cargo build` succeeds without warnings
2. `cargo test` passes
3. `cargo clippy -- -D warnings` passes
4. All documentation files exist
5. Basic client can be instantiated

## Next Phase

After completing Phase 1, proceed to [Phase 2: API Primitives](phase-2-primitives.md).
