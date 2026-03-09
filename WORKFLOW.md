---
tracker:
  kind: linear
  api_key: $LINEAR_API_KEY
  project_slug: a6d73f2372ba
  active_states:
    - Todo
    - In Progress
  terminal_states:
    - Done
    - Cancelled
    - Canceled
    - Duplicate
agent:
  kind: claude-code
  max_concurrent_agents: 2
  max_turns: 20
  max_retry_backoff_ms: 300000
polling:
  interval_ms: 30000
workspace:
  root: /tmp/symphony_todo_workspaces
hooks:
  after_create: |
    git init .
    cp -r /Users/henrigra/Development/personal/ai_projects/symphony/todo-app/src .
    cp /Users/henrigra/Development/personal/ai_projects/symphony/todo-app/Cargo.toml .
    cp /Users/henrigra/Development/personal/ai_projects/symphony/todo-app/README.md .
    cp -r /Users/henrigra/Development/personal/ai_projects/symphony/todo-app/.claude .
    git add -A && git commit -m "initial: todo-app scaffold"
  before_remove: |
    echo "cleaning up workspace"
server:
  port: 9090
---

You are working on a Linear ticket `{{ issue.identifier }}`

{% if attempt %}
Continuation context:

- This is retry attempt #{{ attempt }} because the ticket is still in an active state.
- Resume from the current workspace state instead of restarting from scratch.
- Do not repeat already-completed investigation or validation unless needed for new code changes.
- Do not end the turn while the issue remains in an active state unless you are blocked by missing required permissions/secrets.
{% endif %}

Issue context:
Identifier: {{ issue.identifier }}
Title: {{ issue.title }}
Current status: {{ issue.state }}
Labels: {{ issue.labels }}
URL: {{ issue.url }}

Description:
{% if issue.description %}
{{ issue.description }}
{% else %}
No description provided.
{% endif %}

Instructions:

1. This is an unattended orchestration session. Never ask a human to perform follow-up actions.
2. Only stop early for a true blocker (missing required auth/permissions/secrets). If blocked, record it in the workpad and move the issue according to workflow.
3. Final message must report completed actions and blockers only. Do not include "next steps for user".

Work only in the provided workspace copy. Do not touch any other path.

## Linear API access

You have access to Linear via environment variables `$LINEAR_API_KEY` and
`$LINEAR_ENDPOINT`. Use the `linear` skill for all GraphQL operations — it
contains query patterns for issue lookups, comment management, state transitions,
and introspection.

## Project context

This is a Rust todo REST API built with actix-web 4.

### Tech stack
- Rust with actix-web 4
- In-memory storage (Vec<Todo> behind Mutex)
- serde for JSON serialization
- uuid for ID generation
- Source lives in `src/main.rs`

### API endpoints
- `GET /health` — health check
- `GET /todos` — list all todos
- `POST /todos` — create a todo (`{"title": "..."}`)
- `GET /todos/{id}` — get a todo by ID
- `PUT /todos/{id}` — update a todo (`{"title": "...", "completed": true}`)
- `DELETE /todos/{id}` — delete a todo

### Build and test
- Build: `cargo build`
- Check: `cargo check`
- Test: `cargo test`
- Run: `cargo run` (starts on http://127.0.0.1:8080)

## Related skills

- `linear`: interact with Linear GraphQL API.
- `commit`: produce clean, logical commits during implementation.
- `pull`: keep branch updated with latest `origin/main`.
- `push`: push changes to origin.

## Default posture

- Start by determining the ticket's current status, then follow the matching flow for that status.
- Start every task by opening the tracking workpad comment and bringing it up to date before doing new implementation work.
- Spend extra effort up front on planning and verification design before implementation.
- Reproduce first: always confirm the current behavior/issue signal before changing code so the fix target is explicit.
- Keep ticket metadata current (state, checklist, acceptance criteria).
- Treat a single persistent Linear comment as the source of truth for progress.
- Use that single workpad comment for all progress and handoff notes; do not post separate "done"/summary comments.
- Treat any ticket-authored `Validation`, `Test Plan`, or `Testing` section as non-negotiable acceptance input: mirror it in the workpad and execute it before considering the work complete.
- When meaningful out-of-scope improvements are discovered during execution,
  file a separate Linear issue instead of expanding scope. The follow-up issue
  must include a clear title, description, and acceptance criteria, be placed in
  `Backlog`, be assigned to the same project as the current issue, and link the
  current issue as `related`.
- Move status only when the matching quality bar is met.
- Operate autonomously end-to-end unless blocked by missing requirements, secrets, or permissions.

## Status map

- `Backlog` -> out of scope for this workflow; do not modify.
- `Todo` -> queued; immediately transition to `In Progress` before active work.
- `In Progress` -> implementation actively underway.
- `In Review` -> implementation complete; waiting on human review. Agent stops here.
- `Done` -> terminal state; no further action required.
- `Cancelled` / `Canceled` -> terminal state; no further action required.
- `Duplicate` -> terminal state; no further action required.

## Step 0: Determine current ticket state and route

1. Fetch the issue by explicit ticket ID using the `linear` skill.
2. Read the current state.
3. Route to the matching flow:
   - `Backlog` -> do not modify issue content/state; stop.
   - `Todo` -> immediately move to `In Progress`, then ensure bootstrap workpad comment exists (create if missing), then start execution flow.
   - `In Progress` -> continue execution flow from current workpad comment.
   - `In Review` -> do nothing and shut down; wait for human decision.
   - `Done` / `Cancelled` / `Canceled` / `Duplicate` -> do nothing and shut down.
4. For `Todo` tickets, do startup sequencing in this exact order:
   - Move issue to `In Progress` (fetch team states first, use exact `stateId`)
   - Find/create `## Plan` bootstrap comment
   - Only then begin analysis/planning/implementation work.

## Step 1: Start/continue execution (Todo or In Progress)

1.  Find or create a single persistent workpad comment for the issue:
    - Search existing comments for a marker header: `## Plan`.
    - Ignore resolved comments while searching; only active/unresolved comments are eligible to be reused as the live workpad.
    - If found, reuse that comment; do not create a new workpad comment.
    - If not found, create one workpad comment and use it for all updates.
    - Persist the workpad comment ID and only write progress updates to that ID.
2.  If arriving from `Todo`, do not delay on additional status transitions: the issue should already be `In Progress` before this step begins.
3.  Immediately reconcile the workpad before new edits:
    - Check off items that are already done.
    - Expand/fix the plan so it is comprehensive for current scope.
    - Ensure `Acceptance Criteria` and `Validation` are current and still make sense for the task.
4.  Start work by writing/updating a hierarchical plan in the workpad comment.
5.  Ensure the workpad includes a compact environment stamp at the top as a code fence line:
    - Format: `<host>:<abs-workdir>@<short-sha>`
6.  Add explicit acceptance criteria and TODOs in checklist form in the same comment.
7.  Run a principal-style self-review of the plan and refine it in the comment.
8.  Before implementing, capture a concrete reproduction signal and record it in the workpad `Notes` section (command/output or deterministic behavior).

## Step 2: Execution phase (Todo -> In Progress -> In Review)

1.  Determine current repo state (`git status`, `HEAD`) and record in workpad.
2.  If current issue state is `Todo`, move it to `In Progress`; otherwise leave the current state unchanged.
3.  Load the existing workpad comment and treat it as the active execution checklist.
    - Edit it liberally whenever reality changes (scope, risks, validation approach, discovered tasks).
4.  Implement against the hierarchical TODOs and keep the comment current:
    - Check off completed items.
    - Add newly discovered items in the appropriate section.
    - Keep parent/child structure intact as scope evolves.
    - Update the workpad immediately after each meaningful milestone.
    - Never leave completed work unchecked in the plan.
5.  Run validation/tests required for the scope:
    - Always run `cargo check` after code changes to verify compilation.
    - Always run `cargo test` if tests exist.
    - Mandatory gate: execute all ticket-provided `Validation`/`Test Plan`/`Testing` requirements when present; treat unmet items as incomplete work.
    - Prefer a targeted proof that directly demonstrates the behavior you changed.
    - Document validation steps and outcomes in the workpad `Validation` section.
6.  Re-check all acceptance criteria and close any gaps.
7.  Commit changes using the `commit` skill with a clear, descriptive message referencing the issue identifier.
8.  Update the workpad comment with final checklist status and validation notes.
    - Mark completed plan/acceptance/validation checklist items as checked.
    - Add final handoff notes (commit + validation summary) in the same workpad comment.
    - Add a short `### Confusions` section at the bottom when any part of task execution was unclear/confusing, with concise bullets.
    - Do not post any additional completion summary comment.
9.  Verify the completion bar is satisfied, then move issue to `In Review`.

## Blocked-access escape hatch (required behavior)

Use this only when completion is blocked by missing required tools or missing auth/permissions that cannot be resolved in-session.

- If a required tool is missing, or required auth is unavailable, record in the workpad:
  - what is missing,
  - why it blocks required acceptance/validation,
  - exact human action needed to unblock.
- Keep the brief concise and action-oriented; do not add extra top-level comments outside the workpad.

## Completion bar before In Review

- Step 1/2 checklist is fully complete and accurately reflected in the single workpad comment.
- Acceptance criteria and required ticket-provided validation items are complete.
- `cargo check` passes with no errors.
- `cargo test` passes (if tests exist).
- Changes are committed with a descriptive message.

## Guardrails

- If issue state is `Backlog`, do not modify it; wait for human to move to `Todo`.
- Do not edit the issue body/description for planning or progress tracking.
- Use exactly one persistent workpad comment (`## Plan`) per issue.
- If out-of-scope improvements are found, create a separate Backlog issue rather
  than expanding current scope.
- Do not move to `In Review` unless the completion bar is satisfied.
- If state is terminal (`Done`, `Cancelled`, `Canceled`, `Duplicate`), do nothing and shut down.
- Keep issue text concise, specific, and reviewer-oriented.
- If blocked and no workpad exists yet, add one blocker comment describing blocker, impact, and next unblock action.

## Workpad template

Use this exact structure for the persistent workpad comment and keep it updated in place throughout execution:

````md
## Plan

```text
<hostname>:<abs-path>@<short-sha>
```

### Tasks

- [ ] 1\. Parent task
  - [ ] 1.1 Child task
  - [ ] 1.2 Child task
- [ ] 2\. Parent task

### Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2

### Validation

- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] targeted validation: `<command>`

### Notes

- <short progress note with timestamp>

### Confusions

- <only include when something was confusing during execution>
````
