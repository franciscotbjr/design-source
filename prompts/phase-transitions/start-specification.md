# Prompt: Start Specification Phase

## Context

Use this prompt to begin Phase 3 (Specify) after completing the plan for a work unit. The AI will help you write detailed technical specifications for each unit of work in the current milestone.

## Prerequisites

- Phase 2 (Plan) is complete — architecture defined, milestones ordered, blockers resolved
- AI has access to the **Project Definition**
- You know which **specification template** to use (feature, endpoint, component, bugfix, or refactor)

## Input

Paste the following alongside this prompt:

1. The **plan output** from Phase 2 (or confirm the AI has it in context)
2. The **milestone** you're specifying (if the plan has multiple milestones)
3. Optionally: a specification template from `templates/specification/` for the AI to follow

## Prompt

<!-- Copy from here -->

We are entering **Phase 3: Specify** of the Stateful Spec methodology.

Based on the plan we created, please write a detailed technical specification for the current milestone.

**Current milestone:** {{MILESTONE_DESCRIPTION}}

**Specification template to follow:** {{TEMPLATE_NAME — e.g., feature-spec, endpoint-spec, component-spec, bugfix-spec, or refactor-spec}}

The specification must include:
1. **Description** — What is being built or changed
2. **Acceptance Criteria** — Measurable, testable criteria for "done"
3. **Data/Type Definitions** — Inputs, outputs, data structures (using our project's language and conventions)
4. **Interface/API** — How consumers will interact with this (public API, props, route, CLI usage)
5. **Test Scenarios** — Happy path, edge cases, and error cases
6. **Dependencies** — What this depends on and what it impacts
7. **Implementation Notes** — Hints, constraints, suggested approach

Follow the Project Definition's conventions for all code examples and naming.

Do NOT write implementation code yet. This phase is specification only.

When the specification is complete, confirm it covers all acceptance criteria and test scenarios, then state we can proceed to Phase 4 (Implement).

<!-- To here -->

## Expected Output

A complete specification document following the selected template, ready to hand off for implementation.

## Next Steps

- Review the specification — ensure acceptance criteria are complete and testable
- Use `start-implementation.md` to advance to Phase 4
