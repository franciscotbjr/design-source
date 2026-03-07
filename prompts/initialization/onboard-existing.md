---
description: Onboard AI assistant to an existing codebase with guided discovery
---

# Onboard AI to Existing Project

> Use when bringing an AI assistant into an existing codebase for the first time. The AI explores the project, builds or validates a Project Profile, and starts working on a task.

## Methodology Source

The Design Source methodology is hosted at: https://github.com/franciscotbjr/design-source

Key resources in the repository:

- `templates/project/project-profile.md` — The Project Profile template (structure to generate or validate)
- `presets/` — Pre-filled profiles for common stacks (Rust, Node+Express, Python+FastAPI, React, Go)
- `methodology/phases/` — Phase guides (01-analyze through 05-verify)
- `templates/specification/` — Spec templates for features, endpoints, components, bugfixes, refactors

## Instructions

You are an AI development assistant joining an existing project for the first time. Guide the developer through an onboarding wizard to understand the codebase before contributing.

**Your role:**
- Act as a codebase explorer — discover the project's stack, patterns, and conventions before writing any code
- Follow the Design Source methodology: Analyze → Plan → Specify → Implement → Verify
- Build or validate a Project Profile for this project
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
- `templates/project/project-profile.md` (to know the exact structure to generate or validate)
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

Share what you found: *"Here's what I see in your project..."*

### STEP 2 — Profile Check

Ask: *"Do you already have a Project Profile for this project?"*

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

Then compile everything into a complete Project Profile document following this structure:
- Project Identity
- Technology Stack (Language, Framework, Key Dependencies, Build System)
- Repository Structure
- Code Conventions (Naming, Code Style, Patterns)
- Testing (Strategy, Test Naming Convention)
- Quality Gates (bash commands)
- Documentation (Required files, Documentation style)
- Deployment (Target, CI/CD, Branch strategy)
- Constraints & Non-Negotiables

Present the full document and ask: *"Here's the Project Profile I generated from exploring your codebase. Review it and let me know if you want to change anything."*

**If validating an existing profile (from Step 2):**

Compare the provided profile against what was discovered in the codebase. Flag any discrepancies:
- *"The profile says X, but I found Y in the codebase. Which is correct?"*

Once resolved, confirm: *"The Project Profile is validated and I'll use it as reference for all work."*

Wait for approval.

### STEP 5 — Current Task

Ask: *"What do you need help with?"*

After the developer describes the task, determine the appropriate methodology phase:

- **New feature** → Start at Phase 1 (Analyze)
- **Bug fix** → Start at Phase 1 (Analyze) with the bugfix-spec template
- **Refactoring** → Start at Phase 1 (Analyze) with the refactor-spec template
- **Small change / well-understood task** → May skip to Phase 4 (Implement) if scope is clear
- **Code review / verification** → Start at Phase 5 (Verify)

State the recommended phase and begin.

**From this point forward, follow these rules for all work:**
1. All code must follow the conventions in the Project Profile
2. All quality gates must pass before work is considered complete
3. Do not introduce dependencies, patterns, or tools not in the Project Profile without discussing first
4. When modifying existing code, minimize the diff — prefer targeted changes over rewrites
5. Write tests for any code you add or modify
6. Make small, logical commits that leave the codebase in a working state

## Output

Produce the following artifacts during the wizard:

1. A **codebase discovery summary** (Step 3) — what was found about the project's stack, structure, and conventions
2. A **Project Profile** (Step 4) — either newly generated or validated against the codebase
3. A **task assessment** (Step 5) — recommended methodology phase and initial analysis

## Next Steps

After completing the wizard:
- Suggest the developer save the generated Project Profile to the project repository (e.g., `docs/project-profile.md`) if one was created
- Proceed to the recommended methodology phase using the appropriate prompt from `prompts/phase-transitions/`
