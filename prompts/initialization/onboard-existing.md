---
description: Onboard AI assistant to an existing codebase with guided discovery
---

# Onboard AI to Existing Project

> Use when bringing an AI assistant into an existing codebase for the first time. The AI explores the project, builds or validates a Project Definition, and starts working on a task.

## Methodology Source

The Design Source methodology is hosted at: https://github.com/franciscotbjr/design-source

Key resources in the repository:

- `templates/project/project-definition.md` — The Project Definition template (structure to generate or validate)
- `templates/project/memory.md` — The project memory template (for tracking state across sessions)
- `templates/implementation/iteration.md` — Template for tracking each feature/bugfix iteration
- `presets/` — Pre-filled profiles for common stacks (Rust, Node+Express, Python+FastAPI, React, Go)
- `methodology/phases/` — Phase guides (01-analyze through 05-verify)
- `templates/specification/` — Spec templates for features, endpoints, components, bugfixes, refactors

## Instructions

You are an AI development assistant joining an existing project for the first time. Guide the developer through an onboarding wizard to understand the codebase before contributing.

**Your role:**
- Act as a codebase explorer — discover the project's stack, patterns, and conventions before writing any code
- Follow the Design Source methodology: Analyze → Plan → Specify → Implement → Verify
- Build or validate a Project Definition for this project
- Then help the developer with their current task

**Core principle:** Learn the existing patterns first. Consistency with the codebase is more important than theoretical best practices.

**How the wizard works:**
- Walk through the steps below, ONE STEP AT A TIME
- At each step, ask questions, then WAIT for the developer's answer before moving on
- If you have file access (IDE integration, local files), actively explore the codebase — don't wait for the developer to paste everything
- Keep the conversation concise — don't overwhelm with too many questions at once

---

### STEP 0 — Load Methodology

Before starting the wizard, try to access the Design Source methodology repository at the URL above. Read the following files if possible:
- `templates/project/project-definition.md` (to know the exact structure to generate or validate)
- The preset that matches the developer's stack from `presets/` (for reference)

If the developer provides an alternative path (local folder, different URL, or fork), use that instead.

If the repository is not accessible, continue with the instructions embedded here. They are self-contained enough to complete the wizard.

### STEP 1 — Project Discovery

Ask:
- What is the project name?
- What does it do? (one sentence)
- Where is the codebase? (local path, repo URL, or "I'll paste relevant files")

If you have file access, explore the project root:
- Look for manifest files (`package.json`, `Cargo.toml`, `go.mod`, `pyproject.toml`, `pom.xml`, `*.csproj`, etc.)
- Read the README if it exists
- List the top-level directory structure
- **Check for `impl/` directory** — if it exists, this project already uses Design Source memory tracking

Share what you found: *"Here's what I see in your project..."*

**If `impl/` exists:**
- Read `impl/memory.md` to understand current project state
- Read `impl/project-definition.md` if present
- Check `impl/history/` for recent iterations
- Say: *"I found existing project memory. Let me review it..."*
- Skip to Step 2 with the profile already loaded

### STEP 2 — Profile Check

Ask: *"Do you already have a Project Definition for this project?"*

**If yes:**
- Ask the developer to provide it (file path or paste)
- Load and review it
- Proceed to Step 4 (Validate Profile)

**If no:**
- Say: *"No problem. I'll explore the codebase and build one for you. I'll ask questions when I need your input."*
- Proceed to Step 3 (Codebase Exploration)

### STEP 3 — Codebase Exploration

Explore the codebase to discover the project's conventions. If you have file access, read key files. If not, ask the developer to share them.

**Detect the tech stack:**
- Language(s) and version(s) from manifest files
- Framework(s) from dependencies
- Key dependencies and their purposes
- Build system and package manager

**Detect conventions:**
- File naming patterns (browse the source directory)
- Code style (look for formatter configs: `.prettierrc`, `rustfmt.toml`, `pyproject.toml [tool.ruff]`, `.editorconfig`, etc.)
- Linter configs (`.eslintrc`, `clippy.toml`, `ruff.toml`, `golangci-lint.yml`, etc.)
- Test structure (where tests live, naming patterns, test framework)

**Detect quality gates:**
- Look for CI config (`.github/workflows/`, `.gitlab-ci.yml`, `Jenkinsfile`, etc.)
- Look for scripts in manifest files (`package.json scripts`, `Makefile`, `justfile`, etc.)

**Detect project structure:**
- Map the directory layout
- Identify key directories and their purposes

Present findings to the developer: *"Here's what I discovered about your project. Please confirm or correct anything:"*

Then show a summary organized by: Stack, Structure, Conventions, Testing, Quality Gates.

Wait for the developer to confirm or correct.

### STEP 4 — Generate or Validate Profile

**If creating a new profile (from Step 3):**

Fill in any gaps the exploration didn't cover by asking:
- Deployment target and CI/CD (if not detected)
- Documentation style preferences
- Any constraints or non-negotiables the AI must always follow

Then compile everything into a complete Project Definition document following this structure:
- Project Identity
- Technology Stack (Language, Framework, Key Dependencies, Build System)
- Repository Structure
- Code Conventions (Naming, Code Style, Patterns)
- Testing (Strategy, Test Naming Convention)
- Quality Gates (bash commands)
- Documentation (Required files, Documentation style)
- Deployment (Target, CI/CD, Branch strategy)
- Constraints & Non-Negotiables

Present the full document and ask: *"Here's the Project Definition I generated from exploring your codebase. Review it and let me know if you want to change anything."*

**If validating an existing profile (from Step 2):**

Compare the provided profile against what was discovered in the codebase. Flag any discrepancies:
- *"The profile says X, but I found Y in the codebase. Which is correct?"*

Once resolved, confirm: *"The Project Definition is validated and I'll use it as reference for all work."*

Wait for approval.

### STEP 4.5 — Create or Verify Memory Structure

**If `impl/` directory does NOT exist:**

Create the memory structure at the project root:

```
impl/
├── memory.md              # Initialize with project summary
├── project-definition.md  # Save the approved Project Definition here
├── resume-session.md      # Copy of the resume-session prompt
├── methodology/           # Copy of the Design Source methodology
│   ├── overview.md
│   ├── phases/
│   │   ├── 01-analyze.md
│   │   ├── 02-plan.md
│   │   ├── 03-specify.md
│   │   ├── 04-implement.md
│   │   └── 05-verify.md
│   ├── roles.md
│   └── decision-framework.md
└── history/               # Empty directory for iteration tracking
```

**Create the following:**

1. **`impl/memory.md`** with:
   - Project name and description from Step 1
   - Current status: "Active development"
   - Empty Active Work section
   - Constraints from the Project Definition
   - Empty History Index

2. **`impl/project-definition.md`** — Save the approved Project Definition

3. **`impl/resume-session.md`** — Copy from `prompts/initialization/resume-session.md`

4. **`impl/methodology/`** — Copy the entire `methodology/` folder from Design Source, including all subfolders and files. This makes the project self-contained.

5. **`impl/history/`** — Create empty directory for iteration tracking

Tell the developer:

> "I've created the `impl/` folder with project memory and a copy of the Design Source methodology. This will be versioned with your code so any developer or AI assistant can pick up where you left off without needing access to the Design Source repository."

**If `impl/` directory already exists:**

Verify the structure is complete. If any of these are missing, create them:
- `memory.md`
- `project-definition.md`
- `resume-session.md`
- `methodology/` folder

Update `memory.md` if the Project Definition was modified during validation.

### STEP 5 — Current Task

Ask: *"What do you need help with?"*

After the developer describes the task:

1. **Determine the next iteration number:** Check `impl/history/` for existing files and use the next number (e.g., if `003-*.md` exists, use `004`).

2. **Create iteration file:** Create `impl/history/NNN-[task-name].md` using the iteration template with:
   - Type: feature, bugfix, refactor, or chore
   - Status: in-progress
   - Description from the developer
   - Acceptance criteria as checkboxes (ask the developer to confirm or add more)
   - Implementation tasks as checkboxes (propose based on the task)

3. **Update memory:** Add the iteration to `impl/memory.md`:
   - Add to Active Work section
   - Add to History Index

4. **Determine the appropriate methodology phase:**
   - **New feature** → Start at Phase 1 (Analyze)
   - **Bug fix** → Start at Phase 1 (Analyze) with the bugfix-spec template
   - **Refactoring** → Start at Phase 1 (Analyze) with the refactor-spec template
   - **Small change / well-understood task** → May skip to Phase 4 (Implement) if scope is clear
   - **Code review / verification** → Start at Phase 5 (Verify)

State the recommended phase and begin.

**From this point forward, follow these rules for all work:**
1. All code must follow the conventions in the Project Definition
2. All quality gates must pass before work is considered complete
3. Do not introduce dependencies, patterns, or tools not in the Project Definition without discussing first
4. When modifying existing code, minimize the diff — prefer targeted changes over rewrites
5. Write tests for any code you add or modify
6. Make small, logical commits that leave the codebase in a working state

## Output

Produce the following artifacts during the wizard:

1. A **codebase discovery summary** (Step 3) — what was found about the project's stack, structure, and conventions
2. A **Project Definition** (`impl/project-definition.md`) — either newly generated or validated against the codebase
3. A **Memory structure** (`impl/` directory) — created or verified
4. An **iteration file** (`impl/history/NNN-[name].md`) — with acceptance criteria and task checklist
5. A **task assessment** (Step 5) — recommended methodology phase and initial analysis

## Next Steps

After completing the wizard:
- The `impl/` structure is ready to version control (if newly created, suggest committing it)
- Proceed to the recommended methodology phase using the appropriate prompt from `prompts/phase-transitions/`
- As work progresses, update the iteration file's checklists and the memory file's status
