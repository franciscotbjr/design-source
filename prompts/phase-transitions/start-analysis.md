# Prompt: Start Analysis Phase

## Context

Use this prompt to begin Phase 1 (Analyze) for a new work unit. This is the entry point for any new feature, bugfix, refactoring, or change request.

## Prerequisites

- AI has access to the **Project Definition**
- You have a description of what needs to be built or changed

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. The **work request** (feature description, bug report, change request, etc.)

## Prompt

<!-- Copy from here -->

We are entering **Phase 1: Analyze** of the Design Source methodology.

**Work request:**

{{WORK_REQUEST}}

Please analyze this request by producing:

1. **Requirements Summary** — Restate what needs to be built or changed in your own words
2. **Complexity Assessment** — Categorize as Simple, Medium, or Complex, and explain why
3. **Sub-tasks** — Break the work into discrete pieces (if applicable)
4. **Dependencies** — What does this work depend on? What does it affect?
5. **Open Questions** — List anything that is unclear or needs confirmation before we can plan

Do NOT start planning or writing code. This phase is analysis only.

When analysis is complete, state whether we can proceed to Phase 2 (Plan) or if open questions need resolution first.

<!-- To here -->

## Expected Output

1. Requirements restated clearly
2. Complexity rating with justification
3. Sub-task breakdown
4. Dependency map
5. Open questions (if any)
6. Recommendation: proceed to Plan or resolve questions first

## Next Steps

- Resolve open questions if any
- Use `start-planning.md` to advance to Phase 2
