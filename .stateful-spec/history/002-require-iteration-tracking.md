# Iteration: 002 — Require iteration tracking (non-onboarding work)

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** chore
- **Status:** done
- **Created:** 2026-04-04
- **Completed:** 2026-04-04
- **Author:** (project)

## Description

Document and enforce that non-trivial work uses `.stateful-spec/history/NNN-*.md` and `memory.md` updates even when the session does not start from onboard/new-project wizards. Adds **Iteration tracking** to AGENTS.md, **direct-task entry** and methodology path fix to `resume-session`, **STEP 2.5** fallback to `save-session`, constraint in project-definition, one sentence in `methodology/overview.md`, and syncs `resume-session.mdc` / `save-session.mdc` with source prompts.

## Acceptance Criteria

- [x] AGENTS.md states when to create iteration files and trivial exceptions
- [x] `resume-session.md` covers direct-task entry and conditional methodology location
- [x] `save-session.md` handles missing iteration (retroactive or memory-only)
- [x] `.stateful-spec/project-definition.md` mirrors policy
- [x] `methodology/overview.md` notes work-unit iterations beyond onboarding
- [x] Cursor rules match updated operation prompts

## Implementation Tasks

- [x] Edit AGENTS.md, prompts/operations/resume-session.md, save-session.md
- [x] Edit .stateful-spec/project-definition.md, methodology/overview.md
- [x] Sync .cursor/rules/resume-session.mdc and save-session.mdc

## Quality Checks

- [x] Manual review of cross-references

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Retroactive iteration 002 | Session implemented plan without prior history file; record for audit | 2026-04-04 |

## Blockers & Notes

- _(none)_

## References

- **Plan:** `require_iteration_tracking` (conceptual)
- **Files:** `AGENTS.md`, `prompts/operations/resume-session.md`, `prompts/operations/save-session.md`, `.stateful-spec/project-definition.md`, `methodology/overview.md`, `.cursor/rules/resume-session.mdc`, `.cursor/rules/save-session.mdc`
- **Commits:** `9499bc4` — Require iteration tracking for non-onboarding work
