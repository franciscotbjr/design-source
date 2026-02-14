# Design Source - AI-Assisted Rust Library Development

A methodology framework for designing and implementing Rust libraries with AI assistance.

## Required Skills

The AI assistant must demonstrate proficiency in the following areas:

### Core Development Skills (6 required)
1. **Rust Language** - Edition 2021/2024, ownership, lifetimes, traits
2. **Async Programming** - tokio, async/await, Futures, Streams
3. **Error Handling** - thiserror, Result patterns, error propagation
4. **Serialization** - serde, JSON, custom serializers/deserializers
5. **HTTP Clients** - reqwest, async HTTP, request/response handling
6. **Testing** - Unit tests, integration tests, mocking with mockito

### Design Skills (4 required)
1. **API Design** - Ergonomic interfaces, builder patterns, with-method chains, type safety
2. **Module Organization** - Crate structure, feature flags, visibility, re-exports
3. **Documentation** - Rustdoc, examples, README patterns
4. **Versioning** - Semantic versioning, changelog management

### Process Skills (3 required)
1. **Specification Writing** - YAML specs, API analysis, implementation plans
2. **Iterative Development** - Phase-based delivery, incremental progress
3. **Session Management** - Context preservation, decision tracking

## Development Workflow

### 1. Analysis Phase
```
Input: API specification or feature request
Output: api-analysis.md with endpoint categorization
```

**Steps:**
1. Analyze target API/feature requirements
2. Categorize by complexity (simple, medium, complex)
3. Identify dependencies between components
4. Document in spec/api-analysis.md

### 2. Planning Phase
```
Input: Analysis document
Output: Implementation plan with phases
```

**Steps:**
1. Define implementation phases (typically 4)
2. Create milestone checklist
3. Identify blocking decisions
4. Document in spec/definition.md

### 3. Specification Phase
```
Input: API analysis
Output: YAML specification files
```

**Steps:**
1. Create spec/apis/ directory
2. Write YAML spec for each endpoint/feature
3. Define request/response types
4. Document validation rules
5. Include example payloads

### 4. Implementation Phase
```
Input: Specification files
Output: Rust source code
```

**Steps:**
1. Create implementation plan (impl/)
2. Implement types in src/inference/ (or src/model/, src/tools/)
3. Implement HTTP layer in src/http/
4. Write tests alongside code
5. Create examples in examples/

### 5. Review Phase
```
Input: Implemented code
Output: Verified, documented code
```

**Steps:**
1. Run cargo clippy --all-features and fix warnings
2. Run cargo test --all-features
3. Verify examples compile and run
4. Update CHANGELOG.md
5. Update README.md if needed

## File Organization

### Specification Files
```
spec/
├── definition.md          # Project definition and phases
├── api-analysis.md        # API complexity analysis
└── apis/                  # Per-endpoint YAML specs
    ├── 01-endpoint.yaml
    └── ...

impl/                      # Implementation plans
├── 01-feature-plan.md
└── ...
```

### Source Files
```
src/
├── lib.rs                 # Crate root, feature-gated re-exports
├── error.rs               # Error enum with {Type}Error variants
├── inference/             # Feature: "inference" (default)
│   ├── mod.rs             # Facade: mod + pub use only
│   ├── chat_request.rs    # One type per file
│   ├── chat_response.rs
│   ├── chat_message.rs
│   ├── generate_request.rs
│   └── ...
├── model/                 # Feature: "model"
│   ├── mod.rs
│   ├── show_request.rs
│   ├── delete_request.rs
│   └── ...
├── tools/                 # Feature: "tools"
│   ├── mod.rs
│   ├── tool_definition.rs
│   ├── tool_trait.rs
│   ├── tool_registry.rs
│   └── ...
├── http/                  # Feature: "http" (default)
│   ├── mod.rs
│   ├── client.rs          # OllamaClient struct + retry helpers
│   ├── api_async.rs       # OllamaApiAsync trait + impl
│   ├── api_sync.rs        # OllamaApiSync trait + impl
│   ├── endpoints.rs       # Endpoint constants
│   └── config.rs          # ClientConfig struct
└── conveniences/          # Feature: "conveniences"
    └── mod.rs
```

### Documentation Files
```
README.md                  # Project overview, quick start
CHANGELOG.md               # Version history
CONTRIBUTING.md            # How to contribute
DEV_NOTES.md               # Development decisions
DECISIONS.md               # Architectural decisions
BLOCKERS.md                # Pending decisions
ARCHITECTURE.md            # Module structure
```

## Session Management

### Starting a Session
1. Run `/continue-session` to load previous context
2. Review current phase and pending tasks
3. Check BLOCKERS.md for decisions needed

### During a Session
1. Use TODO list for task tracking
2. Document decisions in DECISIONS.md
3. Update BLOCKERS.md when stuck
4. Commit logical units of work

### Ending a Session
1. Run `/save-session-cache` to preserve context
2. Update CHANGELOG.md with progress
3. Note next steps in DEV_NOTES.md

## Quality Standards

### Code Quality
- `cargo clippy --all-features -- -D warnings` must pass
- `cargo fmt --check` must pass
- All public items documented
- No unsafe code without justification

### Test Coverage
- Unit tests in source files (`#[cfg(test)] mod tests`)
- Integration tests in `tests/client_{operation}_tests.rs`
- Example files that compile and run
- Feature-gated tests with `#[cfg(feature = "...")]`

### Documentation
- README with quick start and examples
- Rustdoc for all public APIs (use `no_run` on doc examples)
- CHANGELOG following Keep a Changelog format
- Architecture documentation for complex modules

## Decision Framework

When facing implementation choices:

1. **Check existing patterns** - Look at similar code in the project
2. **Prefer simplicity** - Choose the simpler solution
3. **Document trade-offs** - Record in DECISIONS.md
4. **Defer when uncertain** - Add to BLOCKERS.md and ask

## Common Patterns

### With-Method Chain (preferred over Builder)
```rust
ChatRequest::new("llama3", [ChatMessage::user("Hello")])
    .with_format(FormatSetting::json())
    .with_options(ModelOptions::default().with_temperature(0.7))
    .with_think(ThinkSetting::enabled())
```

### Async/Sync Variants
Provide both variants with `_blocking` suffix for sync:
```rust
client.chat(&request).await          // async (OllamaApiAsync)
client.chat_blocking(&request)       // sync  (OllamaApiSync)
```

### Error Handling
Use thiserror with `{Type}Error` suffix on variants:
```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpError(String),
    #[error("HTTP status error: {0}")]
    HttpStatusError(u16),
    #[error("Request timeout after {0} seconds")]
    TimeoutError(u64),
    #[error("Maximum retry attempts ({0}) exceeded")]
    MaxRetriesExceededError(u32),
}

// Manual From (avoid exposing external types)
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err.to_string())
    }
}
```

### Feature Flags
Three levels of conditional compilation:
```rust
// Module level (lib.rs)
#[cfg(feature = "model")]
pub mod model;

// Field level (struct)
#[cfg(feature = "tools")]
#[serde(skip_serializing_if = "Option::is_none")]
pub tools: Option<Vec<ToolDefinition>>,

// Method level (trait)
#[cfg(feature = "model")]
async fn list_models(&self) -> Result<ListResponse>;
```

### Trait-Based API
```rust
#[async_trait]
pub trait OllamaApiAsync: Send + Sync {
    async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse>;
    #[cfg(feature = "model")]
    async fn list_models(&self) -> Result<ListResponse>;
}

pub trait OllamaApiSync: Send + Sync {
    fn chat_blocking(&self, request: &ChatRequest) -> Result<ChatResponse>;
    #[cfg(feature = "model")]
    fn list_models_blocking(&self) -> Result<ListResponse>;
}
```

## Commands Reference

| Command | Purpose |
|---------|---------|
| `/continue-session` | Load previous context |
| `/save-session-cache` | Save current context |
| `/review-changes` | Review uncommitted changes |
| `/execute-impl` | Execute implementation instructions |
| `/update-docs` | Update documentation |
| `/write-commit-text` | Generate commit message |

## Iteration Cycle

```
┌─────────────────────────────────────────┐
│                ANALYZE                   │
│  (Understand requirements, categorize)   │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│                 PLAN                     │
│  (Write specs, create implementation    │
│   plans, identify blockers)             │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│               IMPLEMENT                  │
│  (Code types, HTTP layer, tests)        │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│                VERIFY                    │
│  (Run tests, clippy, examples)          │
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│               DOCUMENT                   │
│  (Update changelog, notes, decisions)   │
└─────────────────────────────────────────┘
```

Repeat for each feature/endpoint.
