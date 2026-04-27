# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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
   - `.cursor/rules/*.mdc` — Cursor-native versions of the operation prompts (see Sync rule below)

2. **Self-application** (this repo using its own methodology):
   - `.stateful-spec/memory.md` — current state, active work, history index. **AI's entry point.**
   - `.stateful-spec/project-definition.md` — tech stack and conventions (this is the canonical Project Definition for this repo)
   - `.stateful-spec/history/NNN-*.md` — one file per iteration
   - `.stateful-spec/methodology/README.md` — a pointer that says "read from `methodology/` at repo root, not from here." Do not duplicate methodology files into `.stateful-spec/methodology/`.

`AGENTS.md` at the root is the AI-agent entrypoint and should be read alongside this file.

## Working in this repo

### Iteration tracking is mandatory for non-trivial work

Before substantive edits for any feature, bugfix, refactor, or methodology/documentation change:

1. Find the next `NNN` from existing `.stateful-spec/history/*.md` files
2. Create `.stateful-spec/history/NNN-[kebab-name].md` from `templates/implementation/iteration.md`
3. Update **Active Work** and **History Index** in `.stateful-spec/memory.md`
4. On completion, move it to **Recent Completions** and mark status `done`

Trivial edits (typo, one-line obvious fix) may skip this. When in doubt, create the file. This applies even when the session starts with a direct task ("implement this") rather than a `@resume-session` dialog — see the **Direct-task entry** section in `prompts/operations/resume-session.md`.

### Sync rule: `prompts/operations/` ↔ `.cursor/rules/`

The files in `.cursor/rules/*.mdc` are Cursor-native ports of `prompts/operations/*.md`. **When you modify a source prompt in `prompts/operations/`, also update the matching `.cursor/rules/<name>.mdc`.** They drift quickly otherwise.

Note: `.cursor/rules/*.mdc` files have YAML frontmatter (`description:`, `alwaysApply:`) that the source `.md` files do not. Preserve it when editing.

### Methodology source location

Some prompts (notably `prompts/operations/resume-session.md`) instruct the AI to "read `methodology/`." In a downstream project that copied the methodology under `.stateful-spec/methodology/`, that copy is authoritative. **In this repo, read from `methodology/` at the root** — `.stateful-spec/methodology/` is intentionally a stub.

### Conventions

- Files: `kebab-case.md`. Phase files: `NN-kebab-case.md`. Iteration files: `NNN-kebab-case.md`.
- Templates use `[e.g., ...]` or `{{VARIABLE}}` placeholder syntax.
- Tables are GitHub-flavored Markdown pipes; headers are ATX (`#`, `##`).
- CHANGELOG follows Keep a Changelog format.
- New presets must mirror the structure of `templates/project/project-definition.md`.
- Branch strategy: `main` + feature branches via PRs. Don't push or open PRs without explicit instruction.

### Constraints to honor

- Do not introduce application code, build tooling, dependencies, or CI — this repo is documentation-only by design.
- Do not duplicate methodology content into `.stateful-spec/methodology/`.
- When modifying existing files, prefer minimal targeted diffs over rewrites.
