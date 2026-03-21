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

### STEP 1.5 — Detect Existing Structure & Project Location

**First, scan the current folder** for existing project indicators:
- `Cargo.toml` → Rust project
- `package.json` → Node.js project
- `pyproject.toml` or `setup.py` → Python project
- `go.mod` → Go project
- `.git/` → Git repository initialized
- `src/`, `lib/`, `cmd/`, `internal/` → Source directories
- `README.md`, `LICENSE` → Standard files

**If existing structure is found:**

Tell the developer what you found:
> "I found an existing project structure in this folder: [list what was found, e.g., 'Cargo.toml, src/, .git/']"

Then ask:
> "Do you want to configure Design Source for this existing project, or start fresh in a different location?"

- **If configuring existing:** Note what already exists (will skip creating those in Step 8.75). Proceed with the wizard to document the project.
- **If starting fresh:** Ask for a new folder name, create it, and use it as the project root.

**If no structure is found (empty or near-empty folder):**

Ask:
> "This folder appears empty. Do you want to use it as the project root?"

- **If yes:** Proceed. The wizard will offer to scaffold the project in Step 8.75.
- **If no:** Ask for a folder name to create.

**Remember what was detected** — you'll use this information in Step 8.75 to skip scaffolding steps that are already done.

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

### STEP 8.75 — Interactive Project Setup

Now that the Project Definition is approved, guide the developer through setting up the actual project. **Ask before doing** — the developer may have already done some of these steps, or may prefer to do them manually.

**Reference what you detected in Step 1.5.** Skip questions for things that already exist.

#### 8.75.1 — Project Structure

If no project manifest was detected in Step 1.5:

Ask:
> "Should I create the initial project structure for your [language/framework] project?"

If yes, ask:
> "What command should I run to initialize the project?"

Run the command the developer provides. If they're unsure, suggest the standard initialization command for their stack based on the Project Definition.

If the developer says no, skip — they'll set it up themselves.

#### 8.75.2 — Dependencies

If dependencies were discussed in Step 2:

Ask:
> "Should I add the dependencies we discussed to your project?"

List the dependencies for confirmation:
> "Dependencies to add: [list from Step 2]"

If yes, ask:
> "How should I add them? I can run a command (like your package manager's install command) or edit the manifest file directly. Which do you prefer?"

Follow the developer's preference to add the dependencies.

#### 8.75.3 — Standard Files

Check which standard files are missing and ask:

> "Should I create these missing files?"
> - README.md (with project name and description)
> - .gitignore (with language-appropriate defaults)
> - LICENSE (with the license from Step 1)

Create only the ones the developer approves.

#### 8.75.4 — Initialize Git

If `.git/` was not detected:

> "Should I initialize a git repository? (`git init`)"

If yes, run `git init`.

**Summary:** After completing these sub-steps, summarize what was created:
> "Project setup complete. Created: [list of files/directories created]"

### STEP 8.5 — Detect Code Agent

**AI does (silently, before asking the user):**

1. **Self-identify:** Determine which Code Agent you are by introspecting your own identity and system context. Every AI coding agent knows who it is:
   - If you are **Claude** running inside **Claude Code** → you are Claude Code
   - If you are **Cascade** running inside **Windsurf** → you are Windsurf
   - If you are running inside **Cursor** → you are Cursor
   - If you are **Codex** (OpenAI) → you are Codex
   - If you are **Gemini** running inside **Antigravity** → you are Antigravity
   - If you are running inside **OpenCode** → you are OpenCode

2. **Fallback — folder detection (only if self-identification is uncertain):**
   Check the project root for agent configuration directories:
   - `.claude/` → Claude Code
   - `.windsurf/` → Windsurf
   - `.cursor/` → Cursor
   - `.codex/` → Codex
   - `.antigravity/` → Antigravity
   - `.opencode/` → OpenCode

**Then always ask the developer to confirm:**

> "I'm running as **[Agent Name]**. Would you like me to set up Design Source prompts as native [Agent] commands? This lets you invoke them directly (e.g., `/resume-session`) instead of referencing files manually."
>
> 1. **Yes, use [Agent Name]**
> 2. **Use a different agent** — I'll show you the supported list
> 3. **Skip** — use `design-source/operations/` instead

**If the developer picks "Use a different agent"**, show:
> - Claude Code
> - Windsurf
> - Cursor
> - Codex
> - Antigravity
> - OpenCode

Record the developer's choice — it determines how files are placed in the next step.

### STEP 8.6 — Create Memory Structure

Create the `design-source/` directory structure at the project root. The operation prompts placement depends on the agent choice from STEP 8.5:

**Always create:**

1. **`design-source/memory.md`** with:
   - Project name and description from Step 1
   - Current status: "Active development"
   - Empty Active Work section
   - Constraints from Step 7
   - Empty History Index

2. **`design-source/project-definition.md`** — Save the approved Project Definition

3. **`design-source/methodology/`** — Copy the entire `methodology/` folder from Design Source, including all subfolders and files.

4. **`design-source/history/`** — Create empty directory for iteration tracking

**If the developer accepted native commands (STEP 8.5):**

Create agent-native commands for each operation prompt from the Design Source `prompts/operations/` folder. Adapt the format to match the agent's conventions:

**Claude Code** — For each prompt, create `.claude/commands/<name>.md`:
```markdown
<prompt content>
```
Simple markdown files. Each filename becomes a `/project:<name>` slash command.

**Windsurf** — For each prompt, create `.windsurf/workflows/<name>.md`:
```yaml
---
description: <one-line description from the prompt>
---
<prompt content>
```

**Cursor** — Create `.cursor/rules/design-source.mdc` with:
```yaml
---
description: "Design Source methodology — invoke with @design-source"
alwaysApply: false
---
<reference to design-source/methodology/>
```
Also create an `AGENTS.md` at project root referencing the Design Source methodology and operations.

**Codex** — Create `AGENTS.md` at project root with Design Source methodology instructions. Codex discovers `AGENTS.md` files automatically from the project root down to the current directory. Optionally configure `.codex/config.toml` for model preferences.

**Antigravity** — Create `.antigravity/rules.md` with Design Source conventions and methodology references. Create workflow files in `.antigravity/workflows/` for each operation prompt.

**OpenCode** — For each prompt, create `.opencode/commands/<name>.md`:
```yaml
---
description: <one-line description from the prompt>
---
<prompt content>
```

Do **NOT** create `design-source/operations/` — the prompts already live in the agent's native location.

Tell the developer:

> "I've created the `design-source/` folder with project memory and methodology, and placed operation prompts as native [Agent] commands. You can now use `/resume-session`, `/save-session`, and other prompts directly."

**If the developer skipped native commands (STEP 8.5):**

Also create:

5. **`design-source/operations/`** — Copy the entire `prompts/operations/` folder from Design Source. This includes resume-session, save-session, and all other operation prompts.

Tell the developer:

> "I've created the `design-source/` folder with project memory, operation prompts, and the Design Source methodology. This will be versioned with your code so any developer or AI assistant can pick up where you left off without needing access to the Design Source repository."

### STEP 9 — First Feature

Ask: *"What do you want to build first?"*

Once the developer describes their first feature:

1. **Create iteration file:** Create `design-source/history/001-[feature-name].md` using the iteration template with:
   - Type: feature (or bugfix/refactor as appropriate)
   - Status: in-progress
   - Description from the developer
   - Acceptance criteria as checkboxes (ask the developer to confirm or add more)
   - Implementation tasks as checkboxes (propose based on the feature)

2. **Update memory:** Add the iteration to `design-source/memory.md`:
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

1. **Project scaffolding** (Step 8.75) — project structure, dependencies, standard files (as approved by developer)
2. **Agent detection** (Step 8.5) — Code Agent identified and confirmed by developer
3. **Project memory structure** (`design-source/` directory) — created at the project root
4. **Project Definition** (`design-source/project-definition.md`) — generated from the conversation and approved
5. **Memory file** (`design-source/memory.md`) — initialized with project context
6. **Operation prompts** — placed as native agent commands (if accepted) or in `design-source/operations/` (if skipped)
7. **First iteration file** (`design-source/history/001-[name].md`) — with acceptance criteria and task checklist
8. **Analysis of the first feature** — requirements, complexity, dependencies, open questions

## Next Steps

After completing the wizard:
- The `design-source/` structure is already created and ready to version control
- If there are open questions about the first feature, resolve them before proceeding
- Advance to Phase 2 (Plan) — refer to `methodology/phases/02-plan.md` or `prompts/phase-transitions/start-planning.md`
- As work progresses, update the iteration file's checklists and the memory file's status
