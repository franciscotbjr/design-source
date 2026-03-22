# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [2.0.0] - 2026-03-06

### Changed

- Complete rewrite: transformed from Rust/Ollama-specific methodology to technology-agnostic framework
- Replaced 4-phase model (Foundation, Primitives, Implementation, Conveniences) with universal 5-phase model (Analyze, Plan, Specify, Implement, Verify)
- All artifacts now in English

### Added

- **Methodology core** — `methodology/overview.md`, 5 phase guides, roles definition, decision framework
- **LLM prompts** — 16 ready-to-use prompts organized in 3 categories:
  - Initialization (new project, onboard existing, resume session)
  - Phase transitions (start analysis, planning, specification, implementation, verification)
  - Operations (commit message, update docs, review changes, create spec, write tests, refactor, debug, save session)
- **Templates** — 10 user-fillable templates:
  - Project: Project Definition, Architecture Decision Record
  - Specification: feature, endpoint, component, bugfix, refactor
  - Implementation: implementation plan, test plan
- **Presets** — 5 pre-filled Project Definitions for common stacks:
  - Rust library, Node.js + Express API, Python + FastAPI, React web app, Go service

### Removed

- All Rust-specific content (phase guides, code patterns, type definitions)
- All Ollama-specific content (Modelfile, create-model-request.json)
- Claude Code-specific configuration (.claude/ directory, CLAUDE.md)
- UPDATE_PLAN.md (Rust-specific gap analysis)
- ollama-oxide reference example

## [1.0.0] - 2024-01-01

### Added

- Initial methodology for Rust library development with AI assistance
- Phase guides for Foundation, Primitives, Implementation, Conveniences
- Endpoint specification template (YAML)
- Implementation plan template
- Ollama Modelfile with embedded methodology
- ollama-oxide reference implementation example
