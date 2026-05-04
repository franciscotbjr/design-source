# [Project Name] — AI Agent Instructions

This project uses the **Stateful Spec** methodology for AI-assisted development.

## Getting Started

1. **Read `.stateful-spec/memory.md`** — your entry point for current project state
2. **Read `.stateful-spec/project-definition.md`** — technology stack, conventions, quality gates
3. **Read `.stateful-spec/methodology/`** — the full methodology (phases, roles, decision framework)

To **refresh** methodology, prompts, or agent rules from upstream in an already configured repo, use [`prompts/initialization/update-project.md`](prompts/initialization/update-project.md).

## Methodology

Follow the 5-phase iteration cycle: **Analyze → Plan → Specify → Implement → Verify**

- Phase guides: `.stateful-spec/methodology/phases/`
- Roles and boundaries: `.stateful-spec/methodology/roles.md`
- Decision framework: `.stateful-spec/methodology/decision-framework.md`

## Operation Prompts

The following operations are available as native commands in supported tools:

- **Cursor:** invoke with `@name` (rules in `.cursor/rules/`)
- **Claude Code / OpenClaude:** invoke with `/name` (commands in `.claude/commands/`)
- **OpenCode:** invoke with `/name` (commands in `.opencode/commands/`)

| Operation | Purpose |
|-----------|---------|
| `start-session` | Start a new session — creates iteration file and marks it as open |
| `end-session` | End the current session — summarizes work and closes the iteration |
| `resume-session` | Resume work — loads project context and picks up where you left off |
| `save-session` | Save session progress — updates memory.md and iteration files |
| `create-technical-spec` | Write a technical specification for new work |
| `write-tests` | Generate tests for existing or new code |
| `debug-issue` | Diagnose and fix a bug with structured root cause analysis |
| `refactor-code` | Safely restructure code without changing behavior |
| `review-changes` | Self-review code changes before committing |
| `write-commit-message` | Generate a well-structured commit message |
| `update-documentation` | Update docs after implementing a change |

Source prompts live in `prompts/operations/`. The tool-specific files (`.cursor/rules/`, `.claude/commands/`, `.opencode/commands/`) mirror these sources.

## Rules for All Work

1. All content must follow the conventions in the Project Definition
2. All quality gates must pass before work is considered complete
3. Do not introduce patterns or structures not in the Project Definition without discussing first
4. When modifying existing content, minimize the diff — prefer targeted changes over rewrites
5. Make small, logical commits that leave the repository in a working state
6. Track iterations in `.stateful-spec/history/` and keep `.stateful-spec/memory.md` current

### Iteration tracking

For every **non-trivial** work unit (feature, bugfix, refactor, methodology or documentation change driven by a plan or spec), **before** substantive implementation:

- Create `.stateful-spec/history/NNN-[kebab-name].md` using `templates/implementation/iteration.md`, where `NNN` is the next number after existing files in `history/`.
- Update **Active Work** and **History Index** in `.stateful-spec/memory.md` when starting; move completed work to **Recent Completions** when done.

**Trivial** edits may skip a new iteration file (e.g. typo fix, obvious one-line correction with no acceptance criteria). When in doubt, create an iteration file — it keeps audits straightforward.

If the session **starts with a direct task** (e.g. "implement this plan") instead of a full session-resume dialog, still create or attach to an iteration file **before** implementing.

### Session lifecycle

Use `start-session` at the beginning of an implementation cycle to create an iteration file and mark it as the **Open Session** in `memory.md`. While a session is open, every operation prompt (spec writing, code review, documentation, debugging, etc.) registers its contributions to the Session Log of the open iteration file. Use `end-session` to summarize all work, close the iteration, and clear the Open Session flag.

If an agent instance detects an Open Session in `memory.md`, it should maintain that session — appending entries and not creating competing iterations. If asked to start a new session while one is open, the agent must ask for approval to close the existing one first.
