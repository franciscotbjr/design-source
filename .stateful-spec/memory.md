# Project Memory

> This file is the AI's entry point for understanding the project's current state. Keep it updated as work progresses.

## Project Summary

- **Project:** Stateful Spec
- **Description:** A specification-driven development methodology for AI-assisted software projects
- **Last Updated:** 2026-04-03
- **Current Status:** Active development

## Active Work

> What is currently in progress? Reference the iteration file.

_(none)_

## Recent Completions

> Last 3-5 completed iterations for quick context.

| # | Name | Type | Completed |
|---|------|------|-----------|

## Key Decisions

> Important decisions that affect how the AI should work on this project. For detailed ADRs, see `history/` files.

- This project is documentation-only (Markdown). There is no application code, build system, or runtime dependencies.
- Operation prompts are placed as native Cursor rule files (`.cursor/rules/<name>.mdc`) rather than in `.stateful-spec/operations/`.
- `.stateful-spec/methodology/` references the root `methodology/` directory rather than copying it, because this repo IS the methodology source. This avoids duplication drift in a self-referential project.

## Constraints & Reminders

> Things the AI must always remember when working on this project.

- All files are Markdown (`.md`) using kebab-case naming
- Follow existing directory structure conventions
- CHANGELOG follows Keep a Changelog format
- No build system, no CI, no linter — quality is manual review
- Branch strategy: main + feature branches with PRs

## History Index

> Complete list of iterations. Newest first.

| # | Name | Type | Status | File |
|---|------|------|--------|------|
