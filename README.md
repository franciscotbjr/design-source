# Design Source

A technology-agnostic methodology for building software with AI assistance.

## What Is This?

Design Source provides a structured, repeatable process for developing software with AI coding assistants. It includes:

- **A 5-phase methodology** — Analyze → Plan → Specify → Implement → Verify
- **LLM-ready prompts** — Copy-paste Markdown prompts that work with any AI (Claude, GPT, Gemini, local models)
- **Templates** — Fillable templates for project setup, technical specifications, and implementation planning
- **Presets** — Pre-filled Project Definitions for common tech stacks (Rust, Node.js, Python, React, Go)

The methodology is **technology-agnostic** — it works for any language, framework, or project type.

## Quick Start

### 1. Set Up Your Project Definition

Copy a preset from [`presets/`](presets/) or fill the template at [`templates/project/project-definition.md`](templates/project/project-definition.md). The Project Definition captures your tech stack, conventions, and quality gates — it's the single source of truth the AI references.

### 2. Initialize AI Collaboration

Use one of the initialization prompts from [`prompts/initialization/`](prompts/initialization/):

| Prompt | When to Use |
|--------|-------------|
| [`new-project.md`](prompts/initialization/new-project.md) | Starting a brand-new project |
| [`onboard-existing.md`](prompts/initialization/onboard-existing.md) | Bringing AI into an existing codebase |
| [`resume-session.md`](prompts/initialization/resume-session.md) | Resuming work from a previous session |

### 3. Follow the Methodology

For each unit of work, follow the 5-phase cycle using the phase transition prompts in [`prompts/phase-transitions/`](prompts/phase-transitions/):

```
Analyze → Plan → Specify → Implement → Verify
```

### 4. Use Operation Prompts as Needed

Common tasks have ready-made prompts in [`prompts/operations/`](prompts/operations/):

| Prompt | Purpose |
|--------|---------|
| `write-commit-message.md` | Generate a structured commit message |
| `update-documentation.md` | Update docs after a change |
| `review-changes.md` | Self-review code before committing |
| `create-technical-spec.md` | Write a spec for new work |
| `write-tests.md` | Generate tests for existing code |
| `refactor-code.md` | Safely restructure code |
| `debug-issue.md` | Diagnose and fix a bug |
| `save-session.md` | Generate a session summary for next time |

## Project Structure

```
design-source/
├── methodology/          # Core process documentation
│   ├── overview.md       # Philosophy, principles, iteration cycle
│   ├── phases/           # Detailed guide for each of the 5 phases
│   ├── roles.md          # Human and AI responsibilities
│   └── decision-framework.md  # How to make and record decisions
├── prompts/              # LLM-ready prompts (copy-paste into any AI)
│   ├── initialization/   # Start or resume projects
│   ├── phase-transitions/# Move between methodology phases
│   └── operations/       # Common development tasks
├── templates/            # User-fillable templates
│   ├── project/          # Project Definition, Architecture Decision Record
│   ├── specification/    # Feature, endpoint, component, bugfix, refactor specs
│   └── implementation/   # Implementation plan, test plan
├── presets/              # Pre-filled Project Definitions for common stacks
└── examples/             # Community-contributed applied examples
```

## Key Concepts

### The Project Definition

A single document that captures everything technology-specific about your project (stack, conventions, quality gates). All prompts reference it, keeping the methodology itself agnostic.

### The 5 Phases

| Phase | Purpose |
|-------|---------|
| **Analyze** | Understand requirements, break down complexity, identify unknowns |
| **Plan** | Define architecture, structure, milestones, and blockers |
| **Specify** | Write technical specs for each unit of work |
| **Implement** | Build, test, and integrate following the spec |
| **Verify** | Run quality gates, update docs, prepare delivery |

### Prompts as a Command Palette

Each prompt is a self-contained Markdown file. Copy the prompt section, paste your Project Definition and context, and send to any LLM. No vendor lock-in, no special tooling required.

## Available Presets

| Preset | Stack |
|--------|-------|
| [`rust-library.md`](presets/rust-library.md) | Rust + Cargo + clippy/fmt |
| [`node-express-api.md`](presets/node-express-api.md) | Node.js + Express + Jest + ESLint |
| [`python-fastapi.md`](presets/python-fastapi.md) | Python + FastAPI + pytest + ruff |
| [`react-webapp.md`](presets/react-webapp.md) | React + TypeScript + Vite + Vitest |
| [`go-service.md`](presets/go-service.md) | Go + stdlib + go vet/test |

## License

MIT
