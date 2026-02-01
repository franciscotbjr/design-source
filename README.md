# Design Source

A methodology guide for designing and implementing Rust libraries with AI assistance.

## Overview

This project provides a structured approach to library development, derived from real-world experience building the [ollama-oxide](https://github.com/franciscotbjr/ollama-oxide) library. It serves as a blueprint for conducting iterative design and implementation cycles with an AI assistant.

## Purpose

- **Consistent Delivery**: Establish repeatable patterns for library design and implementation
- **AI Collaboration**: Define clear protocols for working with AI assistants on complex projects
- **Quality Assurance**: Ensure consistent code quality through defined conventions and testing practices
- **Documentation First**: Prioritize specification and planning before implementation

## Project Structure

```
design-source/
├── .claude/
│   ├── skills/           # AI assistant skill definitions
│   │   ├── architecture/    # Architectural patterns
│   │   ├── conventions/     # Rust coding standards
│   │   ├── implementation/  # Implementation workflows
│   │   ├── testing/         # Testing strategies
│   │   ├── api-design/      # API design principles
│   │   └── documentation/   # Documentation standards
│   └── scripts/          # Automation scripts
├── phases/               # Implementation phase guides
├── templates/            # Reusable templates
└── examples/             # Reference implementations
```

## Methodology Phases

### Phase 1: Foundation
- Project structure definition
- Core abstractions design
- Error handling strategy
- Configuration patterns

### Phase 2: API Primitives
- Type definitions (requests, responses)
- Validation rules
- Serialization patterns
- Builder patterns

### Phase 3: Implementation
- HTTP client integration
- Async/sync variants
- Streaming support
- Retry mechanisms

### Phase 4: Conveniences
- High-level APIs
- Common use-case helpers
- Extension traits
- Examples and documentation

## Key Principles

1. **Specification First**: Always write specs before code
2. **Incremental Delivery**: Ship working code in small iterations
3. **Type Safety**: Leverage Rust's type system for compile-time guarantees
4. **Ergonomic APIs**: Design for the common case, allow escape hatches
5. **Comprehensive Testing**: Unit, integration, and example-based tests

## Getting Started

1. Read the [CLAUDE.md](CLAUDE.md) for AI assistant integration
2. Review phases in [phases/](phases/) for implementation workflow
3. Check [templates/](templates/) for reusable patterns
4. See [examples/](examples/) for reference implementations

## License

MIT
