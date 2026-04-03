---
description: Onboard AI assistant to an existing codebase with guided discovery
---

# Onboard AI to Existing Project

> Use when bringing an AI assistant into an existing codebase for the first time. The AI explores the project, generates a Project Definition, sets up Stateful Spec, and helps with the first task.

## Methodology Source

The Stateful Spec methodology is hosted at: https://github.com/franciscotbjr/design-source

Key resources:
- `presets/` — Pre-filled Project Definitions for common stacks (use as base if stack matches)
- `methodology/` — The full methodology documentation

## Instructions

You are an AI development assistant joining an existing project for the first time. Your job is to understand the codebase before writing any code.

**Your role:**
- Explore the codebase and learn its patterns
- Generate a Project Definition that documents the project
- Set up Stateful Spec tracking in the project
- Help the developer with their first task

**Core principle:** Learn existing patterns first. Consistency with the codebase is more important than theoretical best practices.

**How the wizard works:**
- Walk through the steps below, ONE STEP AT A TIME
- At each step: the AI acts first, then asks the developer to confirm or correct
- When asking questions, **present selectable options** whenever possible — free-text input only when the answer can't be anticipated
- Keep the conversation concise — don't overwhelm with too many questions at once

---

### STEP 0 — Load Methodology

Before starting, try to access the Stateful Spec repository (URL above or local path). Check if any preset in `presets/` matches the developer's stack — you'll use it as a base in STEP 3. If the repository is not accessible, continue — the instructions here are self-contained.

### STEP 1 — Meet the Project

**AI does:**
- Check if `.stateful-spec/` directory exists (project may already use Stateful Spec)
- Read README if it exists
- List the top-level directory structure
- Look for manifest files (`package.json`, `Cargo.toml`, `go.mod`, `pyproject.toml`, etc.)

**Then asks:**
> "Here's what I see in your project: [brief summary]. What's the project name and what does it do in one sentence?"

**If `.stateful-spec/` already exists:**
- Read `.stateful-spec/memory.md` and `.stateful-spec/project-definition.md`
- Say: *"This project already uses Stateful Spec. Let me review the existing context..."*
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

### STEP 4 — Detect Code Agent

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

> "I'm running as **[Agent Name]**. Would you like me to set up Stateful Spec prompts as native [Agent] commands? This lets you invoke them directly instead of opening files manually — for example: **Cursor** uses `@rule-name` for rules under `.cursor/rules/`; **Claude Code / Windsurf** often use slash commands such as `/resume-session`."
>
> 1. **Yes, use [Agent Name]**
> 2. **Use a different agent** — I'll show you the supported list
> 3. **Skip** — use `.stateful-spec/operations/` instead

**If the developer picks "Use a different agent"**, show:
> - Claude Code
> - Windsurf
> - Cursor
> - Codex
> - Antigravity
> - OpenCode

Record the developer's choice — it determines how files are placed in the next step.

### STEP 4.5 — Set Up Stateful Spec

**AI does:**

Create the `.stateful-spec/` directory structure. The operation prompts placement depends on the agent choice from STEP 4:

**Always create:**
1. **`.stateful-spec/memory.md`** — Initialize with project name, description, "Active development" status, constraints, empty history index
2. **`.stateful-spec/project-definition.md`** — Save the approved Project Definition
3. **`.stateful-spec/methodology/`** — Copy the entire `methodology/` folder from Stateful Spec, including all subfolders
4. **`.stateful-spec/history/`** — Create empty directory

**If `.stateful-spec/` already exists** (partial setup): check for missing pieces and create only what's missing.

**If the developer accepted native commands (STEP 4):**

Create agent-native commands for each operation prompt from the Stateful Spec `prompts/operations/` folder. Adapt the format to match the agent's conventions:

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

**Cursor** — For **each** file in `prompts/operations/`, create `.cursor/rules/<name>.mdc` where `<name>` matches the source filename without `.md` (e.g. `resume-session.md` → `resume-session.mdc`). **Do not** create only a single umbrella rule — you must create **one `.mdc` per operation**, same as other agents get one file per prompt.

Each `.mdc` file must contain Cursor rule frontmatter plus the full prompt body from the matching source file:

```yaml
---
description: "<one-line description: use YAML `description` from the source prompt if present, otherwise derive from the first `#` heading>"
alwaysApply: false
---

<full content of prompts/operations/<name>.md, unchanged>
```

**Verify these nine files exist** (basenames must match exactly):

| File | Invoked in chat as |
|------|-------------------|
| `.cursor/rules/resume-session.mdc` | `@resume-session` |
| `.cursor/rules/save-session.mdc` | `@save-session` |
| `.cursor/rules/create-technical-spec.mdc` | `@create-technical-spec` |
| `.cursor/rules/write-tests.mdc` | `@write-tests` |
| `.cursor/rules/debug-issue.mdc` | `@debug-issue` |
| `.cursor/rules/refactor-code.mdc` | `@refactor-code` |
| `.cursor/rules/review-changes.mdc` | `@review-changes` |
| `.cursor/rules/write-commit-message.mdc` | `@write-commit-message` |
| `.cursor/rules/update-documentation.mdc` | `@update-documentation` |

Also create an **`AGENTS.md`** at project root referencing the Stateful Spec methodology, `.stateful-spec/project-definition.md`, and listing each operation with its `@name` (so discoverability matches the rules).

**Codex** — Create `AGENTS.md` at project root with Stateful Spec methodology instructions. Codex discovers `AGENTS.md` files automatically from the project root down to the current directory. Optionally configure `.codex/config.toml` for model preferences.

**Antigravity** — Create `.antigravity/rules.md` with Stateful Spec conventions and methodology references. Create workflow files in `.antigravity/workflows/` for each operation prompt.

**OpenCode** — For each prompt, create `.opencode/commands/<name>.md`:
```yaml
---
description: <one-line description from the prompt>
---
<prompt content>
```

Do **NOT** create `.stateful-spec/operations/` — the prompts already live in the agent's native location.

**Then tells the developer** (adapt the invocation examples to the agent — **Cursor:** emphasize `@resume-session`, `@save-session`, and the other seven rules under `.cursor/rules/`; **Claude Code / Windsurf / OpenCode:** mention slash commands or workflows as appropriate):
> "Stateful Spec is set up. I've created the `.stateful-spec/` folder with project memory and methodology, and placed operation prompts as native [Agent] commands."

**If the developer skipped native commands (STEP 4):**

Create `.stateful-spec/operations/` — Copy the entire `prompts/operations/` folder from Stateful Spec.

**Then tells the developer:**
> "Stateful Spec is set up. The `.stateful-spec/` folder contains your project memory, operation prompts, and methodology — all versioned with your code. Any AI assistant can now pick up where you left off."

### STEP 5 — What Do You Need?

**AI asks:**
> "What do you need help with?"

**After the developer answers, AI does:**

1. **Create iteration file:** Determine the next number from `.stateful-spec/history/`, then create `.stateful-spec/history/NNN-[task-name].md` with:
   - Type: feature / bugfix / refactor / chore
   - Status: in-progress
   - Description from the developer
   - Acceptance criteria as checkboxes (ask the developer to confirm or add more)
   - Implementation tasks as checkboxes

2. **Update memory:** Add the iteration to `.stateful-spec/memory.md` (Active Work + History Index)

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
2. **Project Definition** (`.stateful-spec/project-definition.md`) — generated and approved
3. **Agent detection** (Step 4) — Code Agent identified and confirmed by developer
4. **Stateful Spec structure** (`.stateful-spec/`) — created or verified
5. **Operation prompts** — placed as native agent commands (if accepted) or in `.stateful-spec/operations/` (if skipped). **Cursor:** confirm nine `.mdc` files exist under `.cursor/rules/` (one per operation in `prompts/operations/`).
6. **Iteration file** (`.stateful-spec/history/NNN-[name].md`) — with tasks and acceptance criteria
7. **Task assessment** — recommended phase and initial analysis

## Next Steps

- Proceed with the recommended methodology phase
- Update iteration checklists and memory as work progresses
- Use `@save-session` (Cursor), `/save-session` or the agent’s native equivalent, or `.stateful-spec/operations/save-session.md` if native commands were skipped, to save progress before ending a session
