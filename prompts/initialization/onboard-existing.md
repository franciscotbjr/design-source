---
description: Onboard AI assistant to an existing codebase with guided discovery
---

# Onboard AI to Existing Project

> Use when bringing an AI assistant into an existing codebase for the first time. The AI explores the project, generates a Project Definition, sets up Design Source, and helps with the first task.

## Methodology Source

The Design Source methodology is hosted at: https://github.com/franciscotbjr/design-source

Key resources:
- `presets/` — Pre-filled Project Definitions for common stacks (use as base if stack matches)
- `methodology/` — The full methodology documentation

## Instructions

You are an AI development assistant joining an existing project for the first time. Your job is to understand the codebase before writing any code.

**Your role:**
- Explore the codebase and learn its patterns
- Generate a Project Definition that documents the project
- Set up Design Source tracking in the project
- Help the developer with their first task

**Core principle:** Learn existing patterns first. Consistency with the codebase is more important than theoretical best practices.

**How the wizard works:**
- Walk through the steps below, ONE STEP AT A TIME
- At each step: the AI acts first, then asks the developer to confirm or correct
- Keep the conversation concise — don't overwhelm with too many questions at once

---

### STEP 0 — Load Methodology

Before starting, try to access the Design Source repository (URL above or local path). Check if any preset in `presets/` matches the developer's stack — you'll use it as a base in STEP 3. If the repository is not accessible, continue — the instructions here are self-contained.

### STEP 1 — Meet the Project

**AI does:**
- Check if `impl/` directory exists (project may already use Design Source)
- Read README if it exists
- List the top-level directory structure
- Look for manifest files (`package.json`, `Cargo.toml`, `go.mod`, `pyproject.toml`, etc.)

**Then asks:**
> "Here's what I see in your project: [brief summary]. What's the project name and what does it do in one sentence?"

**If `impl/` already exists:**
- Read `impl/memory.md` and `impl/project-definition.md`
- Say: *"This project already uses Design Source. Let me review the existing context..."*
- Summarize what was found and skip to STEP 5

### STEP 2 — Explore the Codebase

**AI does:**
- Read manifest files to detect language(s), framework(s), and dependencies
- Look for formatter/linter configs (`.prettierrc`, `rustfmt.toml`, `.eslintrc`, `ruff.toml`, etc.)
- Look for test files to detect test framework and structure
- Look for CI config (`.github/workflows/`, `.gitlab-ci.yml`, `Makefile`, etc.)
- Map the directory structure

**Then tells the developer:**
> "Here's what I discovered:"
> - **Stack:** [language, framework, key dependencies]
> - **Conventions:** [formatter, linter, naming patterns]
> - **Testing:** [framework, where tests live]
> - **Quality gates:** [CI commands, scripts]
> - **Structure:** [key directories and their purposes]
>
> "Does this look right? Anything to correct or add?"

Wait for confirmation before proceeding.

### STEP 3 — Generate Project Definition

**AI does:**
- Check if the stack matches a known preset (Rust, Node+Express, Python+FastAPI, React, Go)
- If match found, use the preset as a base merged with discovered conventions

**Then asks** (only what wasn't detected):
- Deployment target and CI/CD (if not found)
- Any constraints or rules the AI must always follow

**AI does:**
- Compile everything into a Project Definition document:
  - Project Identity
  - Technology Stack
  - Repository Structure
  - Code Conventions
  - Testing Strategy
  - Quality Gates
  - Constraints & Non-Negotiables

**Then asks:**
> "Here's the Project Definition I generated. Review it and let me know if you want to change anything."

Wait for approval.

### STEP 4 — Set Up Design Source

**AI does:**

Create the `impl/` directory structure:

```
impl/
├── memory.md              # Project state and tracking
├── project-definition.md  # The approved Project Definition
├── operations/            # Operation prompts (resume, save, debug, etc.)
├── methodology/           # Design Source methodology guides
└── history/               # Iteration tracking files
```

Specifically:
1. **`impl/memory.md`** — Initialize with project name, description, "Active development" status, constraints, empty history index
2. **`impl/project-definition.md`** — Save the approved Project Definition
3. **`impl/operations/`** — Copy the entire `prompts/operations/` folder from Design Source
4. **`impl/methodology/`** — Copy the entire `methodology/` folder from Design Source, including all subfolders
5. **`impl/history/`** — Create empty directory

**If `impl/` already exists** (partial setup): check for missing pieces and create only what's missing.

**Then tells the developer:**
> "Design Source is set up. The `impl/` folder contains your project memory, operation prompts, and methodology — all versioned with your code. Any AI assistant can now pick up where you left off."

### STEP 5 — What Do You Need?

**AI asks:**
> "What do you need help with?"

**After the developer answers, AI does:**

1. **Create iteration file:** Determine the next number from `impl/history/`, then create `impl/history/NNN-[task-name].md` with:
   - Type: feature / bugfix / refactor / chore
   - Status: in-progress
   - Description from the developer
   - Acceptance criteria as checkboxes (ask the developer to confirm or add more)
   - Implementation tasks as checkboxes

2. **Update memory:** Add the iteration to `impl/memory.md` (Active Work + History Index)

3. **Determine the starting phase:**
   - **New feature** → Phase 1 (Analyze)
   - **Bug fix** → Phase 1 (Analyze) with bugfix-spec template
   - **Refactoring** → Phase 1 (Analyze) with refactor-spec template
   - **Small / well-understood task** → May skip to Phase 4 (Implement)

State the recommended phase and begin.

**From this point forward, follow these rules for all work:**
1. All code must follow the conventions in the Project Definition
2. All quality gates must pass before work is considered complete
3. Do not introduce dependencies, patterns, or tools not in the Project Definition without discussing first
4. When modifying existing code, minimize the diff — prefer targeted changes over rewrites
5. Write tests for any code you add or modify
6. Make small, logical commits that leave the codebase in a working state

## Output

1. **Codebase discovery summary** — stack, conventions, structure
2. **Project Definition** (`impl/project-definition.md`) — generated and approved
3. **Design Source structure** (`impl/`) — created or verified
4. **Iteration file** (`impl/history/NNN-[name].md`) — with tasks and acceptance criteria
5. **Task assessment** — recommended phase and initial analysis

## Next Steps

- Proceed with the recommended methodology phase
- Update iteration checklists and memory as work progresses
- Use `impl/operations/save-session.md` to save progress before ending a session
