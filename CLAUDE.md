@AGENTS.md

## Claude Code / OpenClaude

Operation prompts are available as Claude Code commands. Invoke them with `/command-name`:

| Command | Purpose |
|---------|---------|
| `/start-session` | Start a new session — creates iteration file and marks it as open |
| `/end-session` | End the current session — summarizes work and closes the iteration |
| `/resume-session` | Resume work — loads project context and picks up where you left off |
| `/save-session` | Save session progress — updates memory.md and iteration files |
| `/create-technical-spec` | Write a technical specification for new work |
| `/write-tests` | Generate tests for existing or new code |
| `/debug-issue` | Diagnose and fix a bug with structured root cause analysis |
| `/refactor-code` | Safely restructure code without changing behavior |
| `/review-changes` | Self-review code changes before committing |
| `/write-commit-message` | Generate a well-structured commit message |
| `/update-documentation` | Update docs after implementing a change |

These commands live in `.claude/commands/` and mirror the source prompts in `prompts/operations/`.

## What this repository is

This is the **source repository for the Stateful Spec methodology** — a documentation-only project. There is no application code, build system, package manager, linter, test runner, or CI. Every file is Markdown. "Quality gates" are manual review.

Because this repo IS the methodology, it consumes itself: the project's own `.stateful-spec/` folder uses the methodology files at the repo root rather than copying them.

## Common commands

There are no build/lint/test commands. The only tooling is `git`. When the user says something like "run tests" or "build", clarify — there is nothing to run.

## High-level architecture

Two layers coexist in this repo and must not be confused:

1. **Source artifacts** (the product this repo ships):
   - `methodology/` — process docs: `overview.md`, `roles.md`, `decision-framework.md`, `phases/01-analyze.md` … `phases/05-verify.md`
   - `prompts/` — three categories: `initialization/` (new-project, onboard-existing, update-project), `phase-transitions/` (one per phase), `operations/` (resume-session, save-session, write-commit-message, etc.)
   - `templates/` — `project/`, `specification/`, `implementation/`
   - `presets/` — pre-filled Project Definitions (rust-library, node-express-api, python-fastapi, react-webapp, go-service)
   - `.cursor/rules/*.mdc` — Cursor-native versions of the operation prompts
   - `.claude/commands/*.md` — Claude Code / OpenClaude command versions
   - `.opencode/commands/*.md` — OpenCode command versions

2. **Self-application** (this repo using its own methodology):
   - `.stateful-spec/memory.md` — current state, active work, history index. **AI's entry point.**
   - `.stateful-spec/project-definition.md` — tech stack and conventions (this is the canonical Project Definition for this repo)
   - `.stateful-spec/history/NNN-*.md` — one file per iteration
   - `.stateful-spec/methodology/README.md` — a pointer that says "read from `methodology/` at repo root, not from here." Do not duplicate methodology files into `.stateful-spec/methodology/`.

`AGENTS.md` at the root is the AI-agent entrypoint and should be read alongside this file.

## Sync rule: `prompts/operations/` ↔ tool-specific files

The files in `.cursor/rules/*.mdc`, `.claude/commands/*.md`, and `.opencode/commands/*.md` are tool-native ports of `prompts/operations/*.md`. **When you modify a source prompt in `prompts/operations/`, also update each matching tool-specific file.** They drift quickly otherwise.

Note: `.cursor/rules/*.mdc` files have YAML frontmatter (`description:`, `alwaysApply:`) that the source `.md` files do not. Preserve it when editing.

## Methodology source location

Some prompts (notably `prompts/operations/resume-session.md`) instruct the AI to "read `methodology/`." In a downstream project that copied the methodology under `.stateful-spec/methodology/`, that copy is authoritative. **In this repo, read from `methodology/` at the root** — `.stateful-spec/methodology/` is intentionally a stub.

## Conventions

- Files: `kebab-case.md`. Phase files: `NN-kebab-case.md`. Iteration files: `NNN-kebab-case.md`.
- Templates use `[e.g., ...]` or `{{VARIABLE}}` placeholder syntax.
- Tables are GitHub-flavored Markdown pipes; headers are ATX (`#`, `##`).
- CHANGELOG follows Keep a Changelog format.
- New presets must mirror the structure of `templates/project/project-definition.md`.
- Branch strategy: `main` + feature branches via PRs. Don't push or open PRs without explicit instruction.

## Constraints to honor

- Do not introduce application code, build tooling, dependencies, or CI — this repo is documentation-only by design.
- Do not duplicate methodology content into `.stateful-spec/methodology/`.
- When modifying existing files, prefer minimal targeted diffs over rewrites.
