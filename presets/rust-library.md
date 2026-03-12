# Project Definition — Rust Library

> Pre-filled preset for a Rust library/crate. Customize for your project.

---

## Project Identity

- **Project Name:** [your-crate-name]
- **Description:** [One-sentence description]
- **Project Type:** library
- **Repository URL:** [https://github.com/user/repo]
- **License:** MIT OR Apache-2.0

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| Rust | Edition 2021 / 2024 | Primary |

### Framework(s)

| Framework | Version | Purpose |
|-----------|---------|---------|
| tokio | 1.x | Async runtime (if async) |

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| serde | 1.x | Serialization / deserialization |
| serde_json | 1.x | JSON support |
| thiserror | 2.x | Error derive macros |
| reqwest | 0.12.x | HTTP client (if needed) |
| async-trait | 0.1.x | Async trait support |

### Build System & Package Manager

- **Package Manager:** cargo
- **Build Tool:** cargo
- **Task Runner:** cargo (via Cargo.toml scripts / aliases)

## Repository Structure

```
project/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── error.rs
│   └── [modules]/
├── tests/
├── examples/
├── README.md
├── CHANGELOG.md
└── LICENSE
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| src/ | Library source code |
| tests/ | Integration tests |
| examples/ | Usage examples |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | snake_case | user_service.rs |
| Functions/Methods | snake_case | get_user_by_id |
| Types/Structs/Enums | PascalCase | UserService |
| Constants | SCREAMING_SNAKE_CASE | MAX_RETRIES |
| Modules | snake_case | http_client |
| Lifetimes | lowercase, short | 'a, 'de |

### Code Style

- **Formatter:** rustfmt (default settings)
- **Max Line Length:** 100 (rustfmt default)
- **Indentation:** 4 spaces
- **Import Order:** std → external crates → crate internal → super/self

### Patterns & Conventions

- **Error Handling:** Custom Error enum with thiserror, Result<T> type alias, manual From impls for external types
- **API Design:** Constructor with required fields + with_* method chain for optional fields
- **Visibility:** Start private, expose only what's needed. Use pub(crate) and pub(super) for internal sharing
- **Async/Sync:** Provide both variants; async methods have no suffix, blocking methods use `_blocking` suffix
- **Documentation:** Rustdoc on all public items with `no_run` examples

## Testing

### Strategy

- **Unit Tests:** Co-located with source in `#[cfg(test)] mod tests`
- **Integration Tests:** In tests/ directory, one file per feature area
- **Test Framework:** cargo test
- **Mocking:** mockito for HTTP mocking
- **Coverage Target:** No formal target; focus on behavior coverage

### Test Naming Convention

`test_{what_is_being_tested}_{scenario}` — e.g., `test_serialize_minimal`, `test_retry_on_server_error`

## Quality Gates

```bash
# Linter
cargo clippy --all-features -- -D warnings

# Formatter check
cargo fmt --check

# Tests
cargo test --all-features

# Build
cargo build --all-features

# Doc build
cargo doc --all-features --no-deps
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Project overview, usage, feature flags |
| CHANGELOG.md | Version history (Keep a Changelog format) |
| LICENSE | License file |

### Documentation Style

- **Code Comments:** Rustdoc (`///` for public, `//` for internal)
- **Doc Examples:** `no_run` attribute on doc examples

## Deployment

- **Target Environment:** crates.io
- **CI/CD:** GitHub Actions
- **Branch Strategy:** main + feature branches

## Constraints & Non-Negotiables

- No unsafe code without justification and documentation
- All public items must have rustdoc documentation
- All types must be Send + Sync (verified by compile-time test)
- No `#[from]` on error variants that expose external types — use manual From impls
- Feature flags for optional functionality
