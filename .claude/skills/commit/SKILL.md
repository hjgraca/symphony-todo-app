---
name: commit
description:
  Create a well-formed git commit from current changes using session history for
  rationale and summary; use when asked to commit, prepare a commit message, or
  finalize staged work.
---

# Commit

## Goals

- Produce a commit that reflects the actual code changes and the session context.
- Follow common git conventions (type prefix, short subject, wrapped body).
- Include both summary and rationale in the body.

## Steps

1. Read session history to identify scope, intent, and rationale.
2. Inspect the working tree and staged changes (`git status`, `git diff`,
   `git diff --staged`).
3. Stage intended changes (`git add -A`) after confirming scope.
4. Sanity-check newly added files; flag build artifacts, logs, or temp files.
5. Choose a conventional type and optional scope (e.g., `feat(scope): ...`,
   `fix(scope): ...`, `refactor(scope): ...`).
6. Write a subject line in imperative mood, <= 72 characters, no trailing period.
7. Write a body that includes:
   - Summary of key changes (what changed).
   - Rationale and trade-offs (why it changed).
   - Tests or validation run.
8. Wrap body lines at 72 characters.
9. Create the commit message with a temp file and use `git commit -F <file>`.
10. Commit only when the message matches the staged changes.

## Template

```
<type>(<scope>): <short summary>

Summary:
- <what changed>

Rationale:
- <why>

Tests:
- <command or "not run (reason)">
```
