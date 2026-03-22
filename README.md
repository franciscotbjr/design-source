# Design Source

## Get Started

Pick one file below and give it to your AI assistant. You can do this by:
- **Copy & paste** the file content into your AI chat, or
- **Point your AI** to the file URL on GitHub, or
- **Reference the local file** if you cloned this repository

| Your situation | Give your AI this file |
|---|---|
| **Starting a new project** | [`prompts/initialization/new-project.md`](prompts/initialization/new-project.md) |
| **Adding AI to an existing project** | [`prompts/initialization/onboard-existing.md`](prompts/initialization/onboard-existing.md) |
| **Resuming previous work** | [`prompts/operations/resume-session.md`](prompts/operations/resume-session.md) |

**That's it.** The AI reads the instructions and walks you through everything else — project setup, conventions, and your first task.

---

## Example

Here's what it looks like in practice. You point your AI to an operation prompt and a business document:

> **You:** Read `create-technical-spec.md` and use it to write a spec based on `requirements/user-notifications.md`.

> **AI:** I've analyzed the requirements. Here's the technical specification:
>
> **Feature:** User Notification System
> **Complexity:** Medium — requires a new service, database table, and WebSocket integration
>
> **Acceptance Criteria:**
> 1. Users receive real-time notifications for mentions and replies
> 2. Notifications persist and can be marked as read
> 3. API supports pagination with a maximum of 50 notifications per page
>
> *[... full spec follows the project's template and conventions]*

That's the pattern — **you provide context, the AI follows the methodology.**

---

## How It Works

Design Source gives your AI assistant a structured process to follow:

```
Analyze → Plan → Specify → Implement → Verify
```

When you initialize a project, the AI creates a `design-source/` folder in your project with:
- **Project Definition** — Your tech stack, conventions, and quality gates
- **Memory** — What's in progress, what was decided, what's next
- **Methodology** — The full process the AI follows
- **Operation prompts** — Ready-made prompts for common tasks (debug, refactor, write tests, etc.)

This makes your project **self-contained** — any AI assistant can pick up where you (or another AI) left off.

## Philosophy

**If you give an AI structured context and memory, it becomes a reliable collaborator instead of a stateless tool.**

## Operation Prompts

Once set up, these prompts are available in your project — either as native agent commands (e.g., `/resume-session` in Windsurf, Claude Code, etc.) or in `design-source/operations/`:

| Prompt | Purpose |
|--------|---------|
| `resume-session.md` | Pick up where you left off |
| `save-session.md` | Save progress before ending a session |
| `create-technical-spec.md` | Write a spec for new work |
| `write-tests.md` | Generate tests for existing code |
| `debug-issue.md` | Diagnose and fix a bug |
| `refactor-code.md` | Safely restructure code |
| `review-changes.md` | Self-review code before committing |
| `write-commit-message.md` | Generate a structured commit message |
| `update-documentation.md` | Update docs after a change |

## Available Presets

Pre-filled Project Definitions for common stacks — the AI will suggest one if it matches your tech:

| Preset | Stack |
|--------|-------|
| [`rust-library.md`](presets/rust-library.md) | Rust + Cargo + clippy/fmt |
| [`node-express-api.md`](presets/node-express-api.md) | Node.js + Express + Jest + ESLint |
| [`python-fastapi.md`](presets/python-fastapi.md) | Python + FastAPI + pytest + ruff |
| [`react-webapp.md`](presets/react-webapp.md) | React + TypeScript + Vite + Vitest |
| [`go-service.md`](presets/go-service.md) | Go + stdlib + go vet/test |

## Learn More

- [`methodology/overview.md`](methodology/overview.md) — Philosophy, principles, and the iteration cycle
- [`methodology/phases/`](methodology/phases/) — Detailed guide for each of the 5 phases
- [`methodology/roles.md`](methodology/roles.md) — Human and AI responsibilities
- [`methodology/decision-framework.md`](methodology/decision-framework.md) — How to make and record technical decisions
- [`templates/`](templates/) — All templates (project setup, specs, implementation plans)
- [`presets/`](presets/) — Pre-filled Project Definitions

## License

MIT
