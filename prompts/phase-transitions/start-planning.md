# Prompt: Start Planning Phase

## Context

Use this prompt to begin Phase 2 (Plan) after completing the analysis of a work unit.

## Prerequisites

- Phase 1 (Analyze) is complete — requirements understood, complexity assessed, open questions resolved
- AI has access to the **Project Definition**

## Input

Paste the following alongside this prompt:

1. The **analysis output** from Phase 1 (or confirm the AI has it in context)
2. Any **resolved questions** or new constraints since the analysis

## Prompt

<!-- Copy from here -->

We are entering **Phase 2: Plan** of the Design Source methodology.

Based on the analysis we completed, please create an implementation plan:

1. **Architecture Design** — Where does this code live? What modules, files, or components are affected? What patterns from the Project Definition apply?
2. **Milestones** — Break the work into ordered, deliverable milestones. Each milestone must be independently testable and produce working software.
3. **File Changes** — For each milestone, list which files will be created, modified, or deleted
4. **Blockers** — Are there any decisions, dependencies, or unknowns that could prevent progress?
5. **Risks** — For complex work, identify what could go wrong and what the fallback is

For any significant architectural decisions, present the alternatives and recommend one. I will approve before we proceed.

Do NOT write specifications or code yet. This phase is planning only.

When planning is complete, state whether we can proceed to Phase 3 (Specify) or if blockers need resolution first.

<!-- To here -->

## Expected Output

1. Architecture description aligned with Project Definition
2. Ordered milestone list with file changes per milestone
3. Blockers and resolution paths (if any)
4. Risk assessment (for complex work)
5. Architectural decisions presented for approval (if any)
6. Recommendation: proceed to Specify or resolve blockers first

## Next Steps

- Approve any architectural decisions (record as ADRs if significant)
- Resolve blockers if any
- Use `start-specification.md` to advance to Phase 3
