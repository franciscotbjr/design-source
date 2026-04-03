# Iteration: 001 — Fix Cursor init prompts (per-operation rules)

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** chore
- **Status:** done
- **Created:** 2026-04-03
- **Completed:** 2026-04-03
- **Author:** (project)

## Description

Initialization prompts (`onboard-existing.md`, `new-project.md`) told agents to create native commands **for each** operation, but the Cursor subsection only described a single `stateful-spec.mdc` plus `AGENTS.md`. That mismatch caused Cursor to skip creating nine per-operation `.cursor/rules/<name>.mdc` files. This iteration documents the prompt fix: require one `.mdc` per `prompts/operations/` file, enumerate the nine basenames, clarify `@` invocation vs slash commands, and add Output verification for Cursor.

## Acceptance Criteria

- [x] `prompts/initialization/onboard-existing.md` STEP 4.5 Cursor block requires per-operation `.mdc` files
- [x] `prompts/initialization/new-project.md` STEP 8.6 matches the same Cursor guidance
- [x] No reliance on a single umbrella `stateful-spec.mdc` as the only Cursor artifact
- [x] History file records the change (this file)

## Implementation Tasks

- [x] Rewrite Cursor block in both init prompts (per-operation `.mdc`, table of nine, AGENTS.md)
- [x] Align STEP 4 / closing copy for Cursor `@` vs other agents
- [x] Extend Output sections with Cursor verification line
- [x] Add `.stateful-spec/history/001-fix-cursor-init-prompts.md` and update `memory.md`

## Quality Checks

> This project has no automated gates; manual review applies.

- [x] Prompts read consistently (no contradictory single-file Cursor path)
- [x] `memory.md` and history index updated

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| One pattern: nine operation `.mdc` files + AGENTS | Matches Claude/Windsurf/OpenCode “one file per prompt”; avoids model stopping after one rule | 2026-04-03 |

## Blockers & Notes

- _(none)_

## References

- **Specification:** Analysis in plan `fix_cursor_operations_in_prompts` (conceptual)
- **Files changed:** `prompts/initialization/onboard-existing.md`, `prompts/initialization/new-project.md`, `.stateful-spec/memory.md`, `.stateful-spec/history/001-fix-cursor-init-prompts.md`
- **Commits:** `55c4c5d` — Fix Cursor init prompts to require per-operation rules
- **Related:** Root cause — global “for each operation” vs old Cursor-only umbrella rule
