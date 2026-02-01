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
6. **Testing** - Unit tests, integration tests, mocking

### Design Skills (4 required)
1. **API Design** - Ergonomic interfaces, builder patterns, type safety
2. **Module Organization** - Crate structure, visibility, re-exports
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

### 3. Primitive Design Phase
```
Input: API analysis
Output: YAML specification files
```

**Steps:**
1. Create spec/primitives/ directory
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
1. Create implementation plan (spec/impl-plans/)
2. Implement types in src/primitives/
3. Implement HTTP layer in src/http/
4. Write tests alongside code
5. Create examples in examples/

### 5. Review Phase
```
Input: Implemented code
Output: Verified, documented code
```

**Steps:**
1. Run cargo clippy and fix warnings
2. Run cargo test
3. Verify examples compile and run
4. Update CHANGELOG.md
5. Update README.md if needed

## File Organization

### Specification Files
```
spec/
├── definition.md          # Project definition and phases
├── api-analysis.md        # API complexity analysis
├── primitives/            # Per-endpoint YAML specs
│   ├── 01-endpoint.yaml
│   └── ...
└── impl-plans/            # Implementation strategies
    ├── 01-feature-plan.md
    └── ...
```

### Source Files
```
src/
├── lib.rs                 # Crate root, re-exports
├── primitives/            # Type definitions
│   ├── mod.rs
│   ├── requests.rs
│   └── responses.rs
├── http/                  # HTTP client implementation
│   ├── mod.rs
│   ├── client.rs
│   └── endpoints/
└── conveniences/          # High-level helpers
    └── mod.rs
```

### Documentation Files
```
DEV_NOTES.md               # Development decisions
CHANGELOG.md               # Version history
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
- `cargo clippy -- -D warnings` must pass
- `cargo fmt --check` must pass
- All public items documented
- No unsafe code without justification

### Test Coverage
- Unit tests for all public functions
- Integration tests for HTTP endpoints
- Example files that compile and run
- Tests in source files or tests/ folder

### Documentation
- README with quick start and examples
- Rustdoc for all public APIs
- CHANGELOG following Keep a Changelog format
- Architecture documentation for complex modules

## Decision Framework

When facing implementation choices:

1. **Check existing patterns** - Look at similar code in the project
2. **Prefer simplicity** - Choose the simpler solution
3. **Document trade-offs** - Record in DECISIONS.md
4. **Defer when uncertain** - Add to BLOCKERS.md and ask

## Common Patterns

### Builder Pattern
Use for types with many optional fields:
```rust
ChatRequest::builder()
    .model("llama3")
    .messages(messages)
    .temperature(0.7)
    .build()
```

### Async/Sync Variants
Provide both variants with clear naming:
```rust
client.generate(request).await        // async
client.generate_sync(request)          // sync (blocking)
```

### Error Handling
Use thiserror for error types:
```rust
#[derive(Debug, thiserror::Error)]
pub enum OllamaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {message}")]
    Api { message: String },
}
```

### Response Types
Use Into/From for flexible conversions:
```rust
impl From<RawResponse> for CleanResponse {
    fn from(raw: RawResponse) -> Self { ... }
}
```

## Commands Reference

| Command | Purpose |
|---------|---------|
| `/continue-session` | Load previous context |
| `/save-session-cache` | Save current context |
| `/review-changes` | Review uncommitted changes |
| `/commit` | Create a commit with proper message |

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
