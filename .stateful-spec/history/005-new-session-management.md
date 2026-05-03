# Iteration: 005 — New Session Management

> One file per feature, bugfix, or refactor. Track progress and decisions here.

## Metadata

- **Type:** feature
- **Status:** in-progress
- **Created:** 2026-05-03
- **Completed:** —
- **Author:** AI assistant

## Description

Introduce explicit `start-session` / `end-session` lifecycle so that multiple agent instances working on different operations (create-technical-spec, review-changes, update-documentation, etc.) all register contributions under the same iteration record. This solves the problem of fragmented tracking when the developer uses separate agent sessions for different phases of an implementation cycle.

## Acceptance Criteria

- [ ] `start-session.md` and `end-session.md` source prompts exist under `prompts/operations/`
- [ ] Tool-specific mirrors exist for Cursor (`.cursor/rules/`), Claude Code (`.claude/commands/`), and OpenCode (`.opencode/commands/`)
- [ ] `templates/implementation/iteration.md` has a **Session Log** section
- [ ] `templates/project/memory.md` has an **Open Session** section
- [ ] `resume-session.md` highlights open session in state summary
- [ ] `save-session.md` registers contributions under the open session iteration
- [ ] All 7 other operation prompts include session-tracking directive
- [ ] `AGENTS.md`, `CLAUDE.md`, `README.md` list the new operations
- [ ] `methodology/overview.md`, `roles.md`, `05-verify.md` reference session lifecycle
- [ ] All quality checks pass (manual review of each file)

## Implementation Tasks

- [ ] Create `prompts/operations/start-session.md` source prompt
- [ ] Create `prompts/operations/end-session.md` source prompt
- [ ] Create 6 tool-specific mirrors (`.cursor/rules/`, `.claude/commands/`, `.opencode/commands/`)
- [ ] Update `templates/implementation/iteration.md` — add Session Log
- [ ] Update `templates/project/memory.md` — add Open Session section
- [ ] Update `prompts/operations/resume-session.md` — session awareness
- [ ] Update `prompts/operations/save-session.md` — open session integration
- [ ] Update `prompts/operations/create-technical-spec.md` — session tracking
- [ ] Update `prompts/operations/review-changes.md` — session tracking
- [ ] Update `prompts/operations/update-documentation.md` — session tracking
- [ ] Update `prompts/operations/write-commit-message.md` — session tracking
- [ ] Update `prompts/operations/debug-issue.md` — session tracking
- [ ] Update `prompts/operations/refactor-code.md` — session tracking
- [ ] Update `prompts/operations/write-tests.md` — session tracking
- [ ] Update `AGENTS.md` — add session operations and rules
- [ ] Update `CLAUDE.md` — add new commands
- [ ] Update `README.md` — add new operations
- [ ] Update `methodology/overview.md` — session lifecycle
- [ ] Update `methodology/roles.md` — session start/end references
- [ ] Update `methodology/phases/05-verify.md` — end-session reference

## Quality Checks

- [ ] All operation prompts follow the same structure as existing ones
- [ ] All mirrors match sources (except Cursor `.mdc` frontmatter)
- [ ] No duplicate content between prompts
- [ ] No new patterns or conventions not already established

## Session Log

> Timestamped entries recording each operation performed during this session.

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-05-03 | start-session | Session opened. Implementing New Session Management feature per impl.md. |

## Decisions Made

| Decision | Rationale | Date |
|----------|-----------|------|
| Dedicated `## Open Session` section in memory.md | Clearer than overloading Active Work section | 2026-05-03 |
| `## Session Log` in iteration file with timestamped entries | Provides audit trail of multi-agent contributions | 2026-05-03 |
| Update all 7 other operation prompts | Consistency — any operation may run during an open session | 2026-05-03 |

## Blockers & Notes

- _(none)_

## References

- **Specification:** `impl.md` (root)
- **Commits:** _(to be filled)_
