# Project Definition

> This is the single source of technology-specific information that all prompts and AI interactions reference.

---

## Project Identity

- **Project Name:** Stateful Spec
- **Description:** A specification-driven development methodology for AI-assisted software projects
- **Project Type:** Documentation / Methodology
- **Repository URL:** https://github.com/franciscotbjr/stateful-spec
- **License:** MIT

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| Markdown | N/A | Primary — all content is `.md` files |

### Framework(s)

None — this is a documentation-only project.

### Key Dependencies

None — no runtime or build-time dependencies.

### Build System & Package Manager

- **Package Manager:** None
- **Build Tool:** None
- **Task Runner:** None

## Repository Structure

```
stateful-spec/
├── methodology/            # Core process documentation
│   ├── overview.md         # Philosophy, principles, iteration cycle
│   ├── roles.md            # Human vs AI responsibilities
│   ├── decision-framework.md
│   └── phases/             # 5 phase guides (01-analyze through 05-verify)
├── prompts/                # LLM-ready prompts
│   ├── initialization/     # new-project, onboard-existing
│   ├── phase-transitions/  # start-analysis through start-verification
│   └── operations/         # 9 operational prompts
├── templates/              # Fill-in templates
│   ├── project/            # project-definition, memory, architecture-decision
│   ├── specification/      # feature, endpoint, component, bugfix, refactor
│   └── implementation/     # implementation-plan, test-plan, iteration
├── presets/                # Pre-filled project definitions (Rust, Node, Python, React, Go)
├── examples/               # Community examples (placeholder)
├── .stateful-spec/         # Project memory instance (this project's own Stateful Spec)
├── .cursor/rules/          # Cursor-native operation prompts
├── .claude/commands/       # Claude Code operation commands
├── CLAUDE.md               # Claude Code entry point (imports AGENTS.md)
├── AGENTS.md
├── CHANGELOG.md
├── LICENSE
└── README.md
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| `methodology/` | Core process documentation — the source of truth for the methodology |
| `prompts/` | Ready-to-use prompts organized by category (initialization, phase-transitions, operations) |
| `templates/` | Fill-in templates for project setup, specifications, and implementation plans |
| `presets/` | Pre-filled Project Definitions for common technology stacks |
| `examples/` | Placeholder for community-contributed applied examples |
| `.stateful-spec/` | This project's own memory, project definition, and iteration history |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | kebab-case.md | `project-definition.md` |
| Directories | lowercase | `phase-transitions/` |
| Phase files | NN-kebab-case.md | `01-analyze.md` |
| Iteration files | NNN-kebab-case.md | `001-feature-name.md` |

### Content Style

- **Formatter:** None
- **Max Line Length:** No formal limit
- **Indentation:** 2 spaces for nested lists, 4 spaces for code blocks
- **Import Order:** N/A

### Patterns & Conventions

- Some prompt files use YAML frontmatter with a `description:` field
- Templates use placeholder syntax: `[e.g., example value]` or `{{VARIABLE}}`
- Prompts use step-by-step wizard format with developer confirmation gates
- Section headers use ATX-style Markdown (`#`, `##`, `###`)
- Tables use GitHub Flavored Markdown pipe syntax

## Testing

Not applicable — documentation-only project.

## Quality Gates

> No automated quality gates. Quality is maintained through manual review.

```bash
# No linter, formatter, type checker, tests, or build commands
```

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| `README.md` | Project overview, get-started guide, operation/preset listing |
| `CHANGELOG.md` | Version history (Keep a Changelog format) |
| `methodology/overview.md` | Philosophy, principles, iteration cycle |

### Documentation Style

- **Code Comments:** N/A
- **Doc Examples:** Inline in Markdown with fenced code blocks

## Deployment

- **Target Environment:** GitHub repository (public)
- **CI/CD:** None
- **Branch Strategy:** main + feature branches with PRs

## Constraints & Non-Negotiables

- All content must be Markdown — no application code in the repository
- File naming must follow existing kebab-case convention
- CHANGELOG entries must follow Keep a Changelog format
- New presets must match the structure of `templates/project/project-definition.md`
- New prompts must include clear instructions, expected inputs, and expected outputs
- When modifying source prompts in `prompts/operations/`, the AI must also update the corresponding `.cursor/rules/<name>.mdc` and `.claude/commands/<name>.md` files
- When modifying methodology source files in `methodology/`, no sync is needed — `.stateful-spec/methodology/` references the source directly
- Non-trivial work must use an iteration file under `.stateful-spec/history/` (see `AGENTS.md` Iteration tracking and `prompts/operations/resume-session.md`); update `memory.md` when starting or completing work
