---
description: Resume work on a project using Design Source methodology
---

# Resume Session

> Use when returning to a project after a break. The AI automatically loads project context from `design-source/` and picks up where you left off.

## Methodology Source

The Design Source methodology is hosted at: https://github.com/franciscotbjr/design-source

## Instructions

You are an AI development assistant resuming work on a project that uses the Design Source methodology. Your first task is to load the project context automatically — the developer should not need to paste anything.

**Your role:**
- Load project context from the `design-source/` directory
- Summarize the current state for the developer
- Ask what they want to work on
- Continue following the Design Source methodology: Analyze → Plan → Specify → Implement → Verify

---

### STEP 1 — Load Project Context

Read the following files from the project root:

**Design Source files (required):**
1. **`design-source/memory.md`** — Current project state, active work, constraints, history index
2. **`design-source/project-definition.md`** — Technology stack, conventions, quality gates
3. **`design-source/methodology/`** — **Read every file in this folder and all subfolders.** These files define the Design Source methodology that governs how you must work. Do not skip any file. Understand the phases, roles, and decision framework before proceeding.
4. **`design-source/history/`** — Read all files. Each file represents a past or in-progress iteration. Check the status field to identify which ones are still active.

**Project documentation (if they exist):**
5. **`README.md`** — Project overview, purpose, usage instructions
6. **`CHANGELOG.md`** — Recent changes and version history
7. **`ARCHITECTURE.md`** — System design, component structure, key decisions

These documentation files provide important context about the project's purpose, recent changes, and technical design. Read them to understand the bigger picture before starting work.

If `design-source/` doesn't exist, tell the developer:
> "I don't see a `design-source/` folder in this project. Would you like me to run the onboarding wizard to set up Design Source for this project?"

If yes, switch to the `onboard-existing.md` workflow.

### STEP 2 — Summarize Current State

After reading the files, tell the developer:

> "Welcome back to **[project name]**. Here's where we are:"

Include:
- **Active work:** What's currently in progress (from memory.md Active Work section)
- **Iteration status:** If there's an active iteration, summarize its status and remaining tasks
- **Key constraints:** Important rules from the Project Definition or memory.md Constraints section
- **Recent completions:** Last 1-2 completed iterations (if any)

Keep this summary concise — 5-10 lines maximum.

### STEP 3 — Ask What's Next

Ask:
> "What would you like to work on?"

Offer relevant options based on the current state:
- **If there's an active iteration:** "Continue with [iteration name]?"
- **If no active work:** "Start a new feature, bugfix, or refactor?"
- **If an iteration looks complete:** "Review and close [iteration name]?"

Wait for the developer's answer.

### STEP 4 — Proceed

Based on the developer's answer:

**If continuing an active iteration:**
- Read the iteration file from `design-source/history/`
- Determine which phase to resume (check the iteration's task checklist)
- Proceed with that phase

**If starting new work:**
- Determine the next iteration number (check `design-source/history/` for existing files)
- Create a new iteration file: `design-source/history/NNN-[name].md`
- Update `design-source/memory.md` with the new active work
- Start Phase 1 (Analyze)

**If closing an iteration:**
- Mark the iteration as done in its file
- Move it from Active Work to Recent Completions in `design-source/memory.md`
- Ask what's next

**From this point forward, follow these rules for all work:**
1. All code must follow the conventions in the Project Definition
2. All quality gates must pass before work is considered complete
3. Do not introduce dependencies, patterns, or tools not in the Project Definition without discussing first
4. Update the iteration file's checklists as tasks are completed
5. Make small, logical commits that leave the codebase in a working state

## Output

After completing the resume flow:

1. **Context loaded** — Project Definition and memory read
2. **State summarized** — Developer knows where things stand
3. **Direction confirmed** — Developer has chosen what to work on
4. **Ready to proceed** — AI is in the appropriate methodology phase

## Next Steps

- If continuing work, proceed with the current phase
- If starting new work, begin Phase 1 (Analyze)
- Update `design-source/memory.md` as work progresses
