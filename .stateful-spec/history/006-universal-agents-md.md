# Iteration: 006 — Universal AGENTS.md Creation

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** fix
- **Status:** done
- **Created:** 2026-05-03
- **Completed:** 2026-05-03
- **Author:** AI assistant

## Description

Make `AGENTS.md` a universal "Always create" requirement across all initialization flows (`new-project.md`, `onboard-existing.md`, `update-project.md`) regardless of the native agent chosen. Previously, `AGENTS.md` was only required for Cursor and Codex — OpenCode, Claude Code, Windsurf, and Antigravity had no instruction to create it.

## Acceptance Criteria

- [ ] `new-project.md` STEP 8.6: `AGENTS.md` is item 5 in "Always create", referencing canonical template
- [ ] `onboard-existing.md` STEP 4.5: Same universal requirement as `new-project.md`
- [ ] `update-project.md` STEP 6: `AGENTS.md` refresh/creation in "Always (regardless of scope)" block
- [ ] `update-project.md` STEP 3: Scope options no longer duplicate `AGENTS.md` refresh instruction
- [ ] `update-project.md` STEP 2: Inventory flags missing `AGENTS.md` for creation
- [ ] `templates/project/agents-md.md` canonical template exists
- [ ] Per-agent instructions (Codex, Cursor) updated to reference "Always create" instead of duplicating
- [ ] Discoverability rationale preserved in Cursor section (`@name` matching)
- [ ] Agent invocation notes cover all supported agents in the ordering note

## Implementation Tasks

- [ ] Create `templates/project/agents-md.md` canonical template (based on root `AGENTS.md`)
- [ ] Update `new-project.md` — add AGENTS.md to "Always create", reference template, ordering note
- [ ] Update `onboard-existing.md` — same as new-project.md
- [ ] Update `update-project.md` — "Always (regardless of scope)" block for AGENTS.md
- [ ] Update `update-project.md` — remove redundant AGENTS.md mentions from scope options
- [ ] Update `update-project.md` — inventory row flags missing AGENTS.md
- [ ] Add discoverability rationale back to Cursor sections
- [ ] Add ordering note (create template first, then update after native commands placed)

## Quality Checks

- [ ] All three prompts consistently reference `templates/project/agents-md.md`
- [ ] No duplicate AGENTS.md creation instructions remain in per-agent sections
- [ ] Developer messages mention AGENTS.md creation in all flows
- [ ] Paths to template correct (relative from `prompts/initialization/`)

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| AGENTS.md as "Always create" + canonical template | Single source of truth; avoids content drift across 3 prompts | 2026-05-03 |
| Remove AGENTS.md mentions from update-project scope options | Avoid duplication with "Always (regardless of scope)" block | 2026-05-03 |
| "Refresh or create (if absent)" wording in update-project | Covers both update and first-install scenarios | 2026-05-03 |

## Blockers & Notes

- _(none)_

## References

- **Issue:** `impl.md` (root) — "AGENTS.md creation not mandatory across all agents"
- **Code Review:** Feedback from detailed review with A1-A6, B1-B3 recommendations
