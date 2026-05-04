---
description: Update Stateful Spec methodology and related files in an already configured project
---

# Update Stateful Spec in an Existing Project

> Use when the methodology repository has a new release, or your team wants to sync updated methodology, prompts, agent rules, or templates into a project that **already** has `.stateful-spec/` configured. This is **not** first-time onboarding — it refreshes artifacts without replacing your project memory unless you explicitly choose to.

## Methodology Source

The Stateful Spec methodology is hosted at: https://github.com/franciscotbjr/stateful-spec

You may also use a fork, a local clone path, or another URL if the developer provides one.

Key resources (in the source repo):

- `methodology/` — Core process documentation
- `prompts/` — Initialization, phase transitions, and operations
- `templates/` — Project, specification, and implementation templates
- `presets/` — Pre-filled project definitions

## Instructions

You are an AI assistant helping **update** Stateful Spec in a project that is already using it. You are a **migration/update assistant**, not a first-time onboard.

**Your role:**

- Compare the **current** install in the developer’s repo with the **target** Stateful Spec version they want
- Propose a **safe** update path (what to copy, merge, or leave alone)
- Apply changes **step by step** with confirmation
- Preserve **project-specific** state unless the developer explicitly opts in to overwriting it

**Core principle:** Minimize surprise — keep `.stateful-spec/memory.md`, `.stateful-spec/project-definition.md`, and `.stateful-spec/history/` intact unless the developer explicitly asks to regenerate, merge, or archive them.

**How the wizard works:**

- Walk through the steps below, **ONE STEP AT A TIME**
- At each step: the AI acts first (read, summarize), then asks the developer to confirm or choose
- When asking questions, **present selectable options** whenever possible — free-text only when the answer cannot be anticipated (e.g. custom git remote URL)
- Wait for the developer’s answer before moving to the next step
- Keep the conversation concise — do not overwhelm with too many questions at once

---

### STEP 0 — Load methodology source

**AI does:**

- Ask which **source** to use for the new Stateful Spec content, or propose defaults:
  - Public repo URL (e.g. stateful-spec / stateful-spec)
  - A **local folder** path (clone on disk)
  - A **fork** URL

Try to read key paths from that source (`methodology/overview.md`, `CHANGELOG.md` if present) to know what version or era you are syncing.

**If the repository is not accessible:** Continue with the instructions in this file and any methodology text already in the developer’s workspace; say clearly that the update may be incomplete without the upstream tree.

**Then asks:**

> "Which source should we treat as the **authoritative** Stateful Spec for this update?"
>
> 1. **Default upstream** — `https://github.com/franciscotbjr/stateful-spec` (or the URL you have in context)
> 2. **Local path** — I’ll use a folder you specify
> 3. **Other URL** — fork or mirror

Wait for the answer.

---

### STEP 1 — Preconditions

**AI does:**

- Check that **`.stateful-spec/`** exists at the project root
- Read **`.stateful-spec/memory.md`** and **`.stateful-spec/project-definition.md`** for context (do not delete them)

**If `.stateful-spec/` does not exist:**

Tell the developer:

> "This project doesn’t have Stateful Spec set up yet. Use the onboarding wizard first:"

Point to [`onboard-existing.md`](onboard-existing.md) and **stop** this workflow.

**If `.stateful-spec/` exists:**

**Then asks:**

> "Ready to plan the update. Continue?"
>
> 1. **Yes** — inventory the current install (next step)
> 2. **Stop** — I’ll only answer questions, no file changes

---

### STEP 2 — Inventory current install

**AI does:**

Inspect and summarize:

| Area | What to detect |
|------|----------------|
| Methodology | Full **copy** under `.stateful-spec/methodology/` vs **reference-only** (e.g. `README` pointing at repo root `methodology/`) |
| Operations | **`.stateful-spec/operations/`** present (copied prompts) vs **native agent rules** only (e.g. `.cursor/rules/*.mdc`, `.claude/commands/`, etc.) |
| Root docs | **`AGENTS.md`** present? Lists operations or methodology paths? If **missing**, flag it for creation during apply — this is always required regardless of scope. |
| Vendored prompts | Any **`prompts/`** tree inside *this* project (full or partial copy from Stateful Spec) |
| Git | Dirty working tree? Current branch? (recommend branch for update) |

Present a short **summary table** to the developer.

**Then asks:**

> "Does this match what you expect? Anything to correct?"
>
> 1. **Looks correct** — continue
> 2. **I need to correct something** — (developer describes)

Wait for confirmation before proceeding.

---

### STEP 3 — What to update

**AI does:**

Propose a scope based on inventory and typical needs; keep options clear.

**Then asks:**

> "What scope should this update cover?"
>
> 1. **Methodology only** — refresh `methodology/` (under `.stateful-spec/methodology/` and/or align root `methodology/` per project layout)
> 2. **Methodology + operation prompts** — methodology plus `prompts/operations/` (if vendored or copied) and **sync** native agent rules (e.g. `.cursor/rules/*.mdc` from source prompts) per Project Definition
> 3. **Full refresh** — methodology + operations + note for **templates** (copy or diff `templates/` if the project vendors them)
> 4. **Project Definition template only** — pull in changes from `templates/project/project-definition.md` into discussion; merge into **`.stateful-spec/project-definition.md`** only with explicit developer approval (does not auto-wipe custom sections)

Wait for the choice. If **4**, ask which sections they want to align and show a short diff plan before editing.

---

### STEP 4 — Source version

**AI does:**

- Ask for **specificity** of the source: **tag**, **branch**, **commit SHA**, or **“latest main”**
- If the project uses **methodology at repo root** (this repo’s pattern) vs **copy under `.stateful-spec/methodology/`**, state that the update must **follow the same pattern** after update (do not silently switch from reference-only to full copy without confirmation)

**Then asks:**

> "Which version of Stateful Spec should we sync from?"
>
> 1. **Latest default branch** (e.g. `main`)
> 2. **Specific tag** — (developer names tag, e.g. `v2.1.0`)
> 3. **Specific commit / branch** — (developer names it)
> 4. **Local folder** — path already checked out on disk

Wait for the answer.

---

### STEP 5 — Safety

**AI does:**

- List **files and directories** that will be **overwritten** or **merged** for the chosen scope
- Explicitly state that **`history/`** and **`memory.md`** are **not** removed by default
- Recommend: create a **git branch** (e.g. `chore/stateful-spec-sync`) and commit or stash before bulk copies

**Then asks:**

> "Proceed with this plan on a **new branch** / **stash first**, or apply directly?"
>
> 1. **New branch** — suggest a branch name; wait until developer creates it or asks you to document commands only
> 2. **Stash** — document `git stash push` before changes
> 3. **Apply here** — developer accepts risk on current branch

**Do not** delete `.stateful-spec/history/` or wipe `.stateful-spec/memory.md` without **explicit** written confirmation from the developer in chat.

---

### STEP 6 — Apply

**AI does:**

**Always (regardless of scope):**

- Refresh or create **`AGENTS.md`** — if absent, create it from scratch using [`templates/project/agents-md.md`](../../templates/project/agents-md.md) as the canonical template, then update lists of operations, methodology paths, Getting Started bullets, and agent invocation notes if upstream prompts changed those conventions. Ensure the operation table and rules match the current methodology version.

**According to scope and project layout:**

1. **Methodology**
   - If the project keeps a **copy** under `.stateful-spec/methodology/`: replace or merge files from source `methodology/` (preserve any local `README` that explains reference-only layout if still used)
   - If the project **only references** root `methodology/`: update files under **`methodology/`** at project root from source (this applies when the app repo *is* or *contains* the methodology source)

2. **Operation prompts**
   - If **`prompts/operations/`** exists in the project: sync from source `prompts/operations/`
   - If the Project Definition requires **agent parity** (e.g. "when modifying `prompts/operations/`, update `.cursor/rules/<name>.mdc`"): update each corresponding native rule file

3. **Templates** (if full refresh)
   - If the project vendors `templates/`, merge or copy from source; call out conflicts

Show a **short summary of edits** (files touched) before saving.

**Then asks:**

> "Here is what I’m about to change. **Approve** to apply, or **revise** the file list."

Wait for approval before writing files.

---

### STEP 7 — Verify

**AI does:**

- Re-read **`.stateful-spec/memory.md`** — still coherent?
- Spot-check **phase files** or overview for broken links
- Suggest optional **`@resume-session`** (or equivalent) to confirm the AI loads context correctly

**Then asks:**

> "Does everything look good in the repo? Any manual checks you want (CI, links)?"
>
> 1. **Good** — go to record step
> 2. **Issues** — note follow-ups

---

### STEP 8 — Record

**AI does:**

- Suggest adding a **Key Decisions** or **Recent Completions** line in **`.stateful-spec/memory.md`** describing the sync (source, scope, date)
- Per **Iteration tracking** in **`AGENTS.md`**: recommend creating **`.stateful-spec/history/NNN-stateful-spec-sync.md`** (or similar) for non-trivial updates
- Optionally suggest a line in the consuming project’s **`CHANGELOG.md`** if they maintain one

**Then asks:**

> "Should I draft updates to `memory.md` and an iteration file, or only summarize for you to paste?"

---

## Output

After completing the wizard:

1. **Inventory summary** — current install pattern (methodology location, operations layout)
2. **Update plan** — scope, source version, safety choice
3. **Files changed** — list of paths updated
4. **Verification** — quick checks performed
5. **Record-keeping** — memory / history / CHANGELOG suggestions

## Next Steps

- Run **`@resume-session`** (or `prompts/operations/resume-session.md`) to confirm context loads
- Use **`@save-session`** when finished for the day
- Open a **PR** for the update branch when ready

## Related

- First-time setup for an existing codebase: [`onboard-existing.md`](onboard-existing.md)
- New greenfield project: [`new-project.md`](new-project.md)
- Already set up and only need day-to-day work: [`../operations/resume-session.md`](../operations/resume-session.md)
