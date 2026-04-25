@AGENTS.md

## Claude Code / OpenClaude

Operation prompts are available as Claude Code commands. Invoke them with `/command-name`:

| Command | Purpose |
|---------|---------|
| `/resume-session` | Resume work — loads project context and picks up where you left off |
| `/save-session` | Save session progress — updates memory.md and iteration files |
| `/create-technical-spec` | Write a technical specification for new work |
| `/write-tests` | Generate tests for existing or new code |
| `/debug-issue` | Diagnose and fix a bug with structured root cause analysis |
| `/refactor-code` | Safely restructure code without changing behavior |
| `/review-changes` | Self-review code changes before committing |
| `/write-commit-message` | Generate a well-structured commit message |
| `/update-documentation` | Update docs after implementing a change |

These commands live in `.claude/commands/` and mirror the source prompts in `prompts/operations/`.
