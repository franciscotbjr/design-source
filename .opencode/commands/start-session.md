---
description: Start a new session for tracking implementation work
---

# Start Session

> Use at the beginning of an implementation cycle. Creates an iteration file and marks it as the open session so all subsequent operations register their contributions under it.

## Instructions

You are helping the developer start a new work session. Your job is to create the tracking files and mark the session as open so that other agent instances — running different operations like spec writing, code review, or documentation — all register their work under the same iteration.

**Your role:**
- Check for any existing open session
- Ask the developer what they're working on
- Create an iteration file and mark the session as open in `memory.md`

---

### STEP 1 — Check for Existing Open Session

Read `.stateful-spec/memory.md`.

If the file has an **Open Session** section that references an active iteration file (not `_(none)_`):

> "There is already an open session: `[iteration-file]`. Would you like me to close it and start a new one, or should I cancel?"

- If the developer says **close it**, switch to the `end-session.md` workflow first, then return to this step.
- If the developer says **cancel**, stop here.

### STEP 2 — Ask What's Being Worked On

Ask:

> "What are you planning to work on? Give me a brief description so I can create the session."

The description should be concise — one or two sentences. Examples: "Add user authentication with JWT", "Fix the race condition in the order processing pipeline", "Refactor the payment module to use a strategy pattern".

### STEP 3 — Determine Iteration Number

Look at `.stateful-spec/history/` to find the highest existing number (`NNN`). The new iteration will be `NNN + 1`.

If no iteration files exist, start from `001`.

### STEP 4 — Create Iteration File

Create `.stateful-spec/history/NNN-[kebab-name].md` using `templates/implementation/iteration.md`:

- **Type:** Ask the developer or infer from the description (feature / bugfix / refactor / chore)
- **Status:** `in-progress`
- **Created:** Today's date
- **Description:** The description from STEP 2
- **Acceptance Criteria:** Leave for the developer to fill (or create placeholder)

### STEP 5 — Update memory.md

Update `.stateful-spec/memory.md`:

1. **Open Session section** — Set it to point to the new iteration file:
   ```
   - Currently open: [NNN] [brief description] → `history/NNN-name.md`
   ```

2. **Active Work section** — Add the new work:
   ```
   - [ ] [Brief description] → `history/NNN-name.md`
   ```

3. **History Index** — Add a row for the new iteration with status `in-progress`.

### STEP 6 — Confirm

Tell the developer:

> "Session started: `history/NNN-name.md`"
> "All operations you run in this session (spec writing, code review, documentation, etc.) will register their contributions under this iteration. Use `end-session` when the implementation cycle is complete."

## Output

After completing the start flow:

1. **Iteration file created** — Ready to track work
2. **Open Session marked** — memory.md points to the active iteration
3. **All future operations aware** — Agent instances will check for the open session and register contributions

## Next Steps

- Proceed with the appropriate methodology phase (Analyze → Plan → Specify → Implement → Verify)
- When the cycle is complete, use `end-session` to close the session
