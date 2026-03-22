# Prompt: Start Implementation Phase

## Context

Use this prompt to begin Phase 4 (Implement) after the specification is written and reviewed. The AI will write code, tests, and documentation following the spec and Project Definition conventions.

## Prerequisites

- Phase 3 (Specify) is complete — specification reviewed and approved
- AI has access to the **Project Definition** and the **specification**
- Optionally: an implementation plan from `templates/implementation/implementation-plan.md`

## Input

Paste the following alongside this prompt:

1. The **specification** from Phase 3 (or confirm the AI has it in context)
2. Optionally: an **implementation plan** if you created one
3. Any **additional constraints** (e.g., "implement only the first 3 steps today")

## Prompt

<!-- Copy from here -->

We are entering **Phase 4: Implement** of the Stateful Spec methodology.

Please implement the specification we wrote, following this order:
1. **Data/Types** — Define data structures, types, schemas, interfaces
2. **Core Logic** — Implement the business logic or algorithm
3. **Integration** — Wire into the rest of the system (routes, handlers, UI, etc.)
4. **Tests** — Write tests that verify each acceptance criterion from the spec
5. **Documentation** — Add inline documentation for non-obvious logic

**Rules for implementation:**
- Follow ALL conventions from the Project Definition (naming, style, patterns, file organization)
- Write tests alongside code — not after everything is done
- Make each change committable independently when possible
- Stay within the scope of the specification — do not add features or improvements not in the spec
- If you discover something the spec didn't cover, flag it and ask before proceeding

**Specification:**

{{SPECIFICATION_OR_CONFIRM_IN_CONTEXT}}

Start with step 1 (Data/Types) and proceed through each step. After each step, briefly confirm what was done before moving to the next.

<!-- To here -->

## Expected Output

Working code that:
1. Implements all acceptance criteria from the spec
2. Includes tests for happy path, edge cases, and error cases
3. Follows Project Definition conventions
4. Is documented where non-obvious

## Next Steps

- Review the implementation
- Use `start-verification.md` to advance to Phase 5
- If issues are found, iterate within this phase
