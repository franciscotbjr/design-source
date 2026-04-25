# Iteration: 004 — Add Claude Code support

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** done
- **Created:** 2026-04-25
- **Completed:** 2026-04-25
- **Author:** (project)

## Description

Add Claude Code agent configuration alongside the existing Cursor setup so the project works with both tools. Create `CLAUDE.md` (imports `AGENTS.md`), `.claude/commands/*.md` (9 operation prompts invokable with `/name`), and update `AGENTS.md`, `README.md`, and `project-definition.md` to reflect dual-tool support.

## Acceptance Criteria

- [x] `CLAUDE.md` exists at project root, imports `@AGENTS.md`, adds Claude Code section
- [x] `.claude/commands/*.md` contains all 9 operation prompts (body matches source, no Cursor frontmatter)
- [x] `AGENTS.md` mentions Claude Code commands alongside Cursor rules
- [x] `.stateful-spec/project-definition.md` includes `.claude/` in repo tree and sync constraint
- [x] `README.md` mentions Claude Code as a supported tool

## Implementation Tasks

- [x] Create `CLAUDE.md`
- [x] Create 9 `.claude/commands/*.md` files
- [x] Update `AGENTS.md`
- [x] Update `.stateful-spec/project-definition.md`
- [x] Update `README.md`

## Quality Checks

- [x] Manual review of cross-references and links
- [x] No debug code or TODOs left behind

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Use `.claude/commands/` not `.claude/rules/` | Rules without `paths` load unconditionally; commands load on `/name` invocation, matching Cursor's `alwaysApply: false` | 2026-04-25 |
| `CLAUDE.md` imports `@AGENTS.md` | Official recommendation; avoids duplicating shared instructions | 2026-04-25 |

## Blockers & Notes

- _(none)_

## References

- **Plan:** `add_claude_code_support` plan
- **Files:** `CLAUDE.md`, `.claude/commands/*.md`, `AGENTS.md`, `README.md`, `.stateful-spec/project-definition.md`
- **Commits:** —
