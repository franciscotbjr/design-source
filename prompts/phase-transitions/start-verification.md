# Prompt: Start Verification Phase

## Context

Use this prompt to begin Phase 5 (Verify) after implementation is complete. The AI will help run quality gates, verify acceptance criteria, review changes, and update documentation.

## Prerequisites

- Phase 4 (Implement) is complete — code and tests are written
- AI has access to the **Project Definition** (specifically the Quality Gates section)
- AI has access to the **specification** (for acceptance criteria verification)

## Input

Paste the following alongside this prompt:

1. Confirm the **Project Definition** and **specification** are in context
2. Optionally: the output of quality gate commands if you've already run them
3. Optionally: a diff or summary of changes made

## Prompt

<!-- Copy from here -->

We are entering **Phase 5: Verify** of the Stateful Spec methodology.

Please help me verify the implementation is complete and meets quality standards.

**Step 1: Quality Gates**
List every quality gate from the Project Definition and confirm whether each one passes. If you can run commands, run them. If not, tell me which commands to run and I'll provide the output.

**Step 2: Acceptance Criteria**
Go through every acceptance criterion from the specification and confirm each is met, referencing the specific test or code that verifies it.

**Step 3: Change Review**
Review the changes made during implementation:
- Are all changes intentional? Any debug code, TODO comments, or temporary hacks remaining?
- Do all changes follow the Project Definition's conventions?
- Is the scope within the specification's boundaries?
- Are there any security concerns (exposed secrets, unsafe inputs)?

**Step 4: Documentation**
Identify what documentation needs updating:
- README (if setup, usage, or API surface changed)
- CHANGELOG (add entry for this change)
- API docs (if public interfaces changed)
- Architecture docs (if structure changed)
- Any new ADRs needed for decisions made during implementation?

**Step 5: Delivery**
Recommend the appropriate delivery action:
- Commit message (suggest one following the project's conventions)
- PR description (if applicable)
- Version bump (if this completes a release milestone)

<!-- To here -->

## Expected Output

1. Quality gate status (pass/fail for each)
2. Acceptance criteria verification (each criterion checked)
3. Change review findings (issues or "all clear")
4. Documentation update list
5. Suggested commit message and delivery action

## Next Steps

- Fix any quality gate failures or review findings
- Update documentation as recommended
- Commit/PR/release as appropriate
- If more milestones remain → return to `start-specification.md` for the next one
- If done → use `prompts/operations/save-session.md` to save session context
