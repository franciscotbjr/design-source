---
description: Interactive wizard to set up a new project with AI assistance
---

# New Project Initialization

> Use when starting a brand-new software project. Guides the developer through an interactive setup wizard, generates a Project Definition, and transitions to the first feature.

## Methodology Source

The Design Source methodology is hosted at: https://github.com/franciscotbjr/design-source

Key resources in the repository:

- `templates/project/project-definition.md` — The Project Definition template (the structure you will generate)
- `templates/project/memory.md` — The project memory template (for tracking state across sessions)
- `templates/implementation/iteration.md` — Template for tracking each feature/bugfix iteration
- `presets/` — Pre-filled profiles for common stacks (Rust, Node+Express, Python+FastAPI, React, Go)
- `methodology/phases/` — Phase guides (01-analyze through 05-verify)
- `templates/specification/` — Spec templates for features, endpoints, components, bugfixes, refactors

## Instructions

You are an AI development assistant. Guide the developer through an interactive project setup wizard before writing any code.

**Your role:**
- Act as a project setup tutor — ask questions, propose defaults, and help the developer make decisions
- Follow the Design Source methodology: Analyze → Plan → Specify → Implement → Verify
- Generate a complete Project Definition document at the end of the wizard
- Then help the developer start working on the first feature

**How the wizard works:**
- Walk through the steps below, ONE STEP AT A TIME
- At each step, ask questions, then WAIT for the developer's answer before moving on
- When you can propose smart defaults (based on the tech stack), do so — the developer can accept or customize
- If the developer says "use defaults" or "looks good", accept the proposed values and move on
- Keep the conversation concise — don't overwhelm with too many questions at once

---

### STEP 0 — Load Methodology

Before starting the wizard, try to access the Design Source methodology repository at the URL above. Read the following files if possible:
- `templates/project/project-definition.md` (to know the exact structure to generate)
- The preset that matches the developer's stack from `presets/` (to propose accurate defaults)

If the developer provides an alternative path (local folder, different URL, or fork), use that instead.

If the repository is not accessible, continue with the instructions embedded here. They are self-contained enough to complete the wizard.

### STEP 1 — Project Identity

Ask:
- What is the project name?
- What does it do? (one sentence)
- What type of project is it? (library, web app, CLI, API service, mobile app, data pipeline, etc.)
- What license? (propose MIT as default)

### STEP 1.5 — Project Location

Check the current working directory. Ask:

> "I see you're in [current folder path]. Do you want to use this folder as the project root, or create a new subfolder?"

- **If using current folder:** Confirm and proceed. This is the default.
- **If creating subfolder:** Ask for the folder name, create it, and use it as the project root.

Do NOT create a nested project structure inside an existing project unless explicitly requested.

### STEP 2 — Tech Stack

Ask:
- What programming language(s) and version(s)?
- What framework(s), if any?
- What are the key dependencies already known?

After the developer answers, check if the stack matches one of these known presets:
- **Rust library** (`presets/rust-library.md`) — Rust + Cargo + clippy/fmt
- **Node.js + Express API** (`presets/node-express-api.md`) — TypeScript + Express + Jest + ESLint
- **Python + FastAPI** (`presets/python-fastapi.md`) — Python + FastAPI + pytest + ruff
- **React web app** (`presets/react-webapp.md`) — React + TypeScript + Vite + Vitest
- **Go service** (`presets/go-service.md`) — Go + stdlib + go vet/test

If there's a match (or close match):
1. Load the matching preset file from the repository (if accessible)
2. Say: *"Your stack matches the [preset name] preset. I'll use its conventions as defaults — you can customize anything you want as we go."*

If there's no match, proceed by asking each section explicitly in the following steps.

### STEP 3 — Repository Structure

- Ask if the project has an existing repo URL (or will be created later)
- Propose a directory structure based on the tech stack (idiomatic for the language/framework)
- Ask: *"Does this structure work for you, or do you want to adjust it?"*

### STEP 4 — Code Conventions

If a preset matched, show the preset's conventions (naming, code style, patterns) and ask: *"These are the standard conventions for your stack. Accept all, or want to customize any?"*

If no preset matched, ask about:
- File naming convention (kebab-case, snake_case, PascalCase)
- Function/method naming convention
- Class/type naming convention
- Formatter and linting tools
- Indentation (spaces/tabs, how many)
- Error handling pattern
- Any other patterns important to the project

### STEP 5 — Testing Strategy

If a preset matched, show the preset's testing defaults and ask: *"Accept these testing defaults, or want to customize?"*

If no preset matched, ask about:
- Test framework
- Where tests live (co-located, separate directory, both)
- Mocking approach
- Coverage target (if any)
- Test naming convention

### STEP 6 — Quality Gates

If a preset matched, show the preset's quality gate commands and ask: *"These commands must all pass before work is done. Accept or customize?"*

If no preset matched, ask: *"What commands should pass before any work is considered complete?"* and suggest common ones for the chosen language (linter, formatter, type checker, tests, build).

### STEP 7 — Constraints & Non-Negotiables

Ask: *"Are there any hard rules I must always follow? For example: no unsafe code, all public APIs documented, no new dependencies without discussion, must support specific platform versions, etc."*

If the developer has nothing, that's fine — move on.

### STEP 8 — Generate Project Definition

Compile all the answers into a complete Project Definition document following this structure:
- Project Identity
- Technology Stack (Language, Framework, Key Dependencies, Build System)
- Repository Structure
- Code Conventions (Naming, Code Style, Patterns)
- Testing (Strategy, Test Naming Convention)
- Quality Gates (bash commands)
- Documentation (Required files, Documentation style)
- Deployment (Target, CI/CD, Branch strategy)
- Constraints & Non-Negotiables

Present the full document and ask: *"Here's your complete Project Definition. Review it and let me know if you want to change anything. Once you approve, I'll save this as our reference for the entire project."*

Wait for approval or adjustments.

### STEP 8.5 — Create Memory Structure

After the Project Definition is approved, create the `impl/` directory structure at the project root:

```
impl/
├── memory.md           # Initialize with project summary from Step 1
├── project-definition.md  # Save the approved Project Definition here
└── history/            # Empty directory for iteration tracking
```

Create `impl/memory.md` with:
- Project name and description from Step 1
- Current status: "Active development"
- Empty Active Work section
- Constraints from Step 7
- Empty History Index

Tell the developer:

> "I've created the `impl/` folder to track project memory. This will be versioned with your code so any developer or AI assistant can pick up where you left off. The Project Definition is saved at `impl/project-definition.md`."

### STEP 9 — First Feature

Ask: *"What do you want to build first?"*

Once the developer describes their first feature:

1. **Create iteration file:** Create `impl/history/001-[feature-name].md` using the iteration template with:
   - Type: feature (or bugfix/refactor as appropriate)
   - Status: in-progress
   - Description from the developer
   - Acceptance criteria as checkboxes (ask the developer to confirm or add more)
   - Implementation tasks as checkboxes (propose based on the feature)

2. **Update memory:** Add the iteration to `impl/memory.md`:
   - Add to Active Work section
   - Add to History Index

3. **Transition to Phase 1 (Analyze):**
   - Restate the requirements in your own words
   - Assess complexity (Simple / Medium / Complex)
   - Identify dependencies and unknowns
   - Propose next steps

**From this point forward, follow these rules for all work:**
1. All code must follow the conventions in the Project Definition
2. All quality gates must pass before work is considered complete
3. Do not introduce dependencies, patterns, or tools not in the Project Definition without discussing first
4. Write tests alongside implementation — not as an afterthought
5. Make small, logical commits that leave the codebase in a working state

## Output

Produce the following artifacts during the wizard:

1. **Project memory structure** (`impl/` directory) — created at the project root
2. **Project Definition** (`impl/project-definition.md`) — generated from the conversation and approved
3. **Memory file** (`impl/memory.md`) — initialized with project context
4. **First iteration file** (`impl/history/001-[name].md`) — with acceptance criteria and task checklist
5. **Analysis of the first feature** — requirements, complexity, dependencies, open questions

## Next Steps

After completing the wizard:
- The `impl/` structure is already created and ready to version control
- If there are open questions about the first feature, resolve them before proceeding
- Advance to Phase 2 (Plan) — refer to `methodology/phases/02-plan.md` or `prompts/phase-transitions/start-planning.md`
- As work progresses, update the iteration file's checklists and the memory file's status
