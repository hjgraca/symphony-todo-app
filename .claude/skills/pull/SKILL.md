---
name: pull
description:
  Pull latest origin/main into the current local branch and resolve merge
  conflicts. Use when needing to sync a feature branch with origin or resolve
  conflicts.
---

# Pull

## Workflow

1. Verify git status is clean or commit/stash changes before merging.
2. Confirm remotes and branches.
3. Fetch latest refs: `git fetch origin`
4. Merge: `git merge origin/main`
5. If conflicts appear, resolve them, then `git add` and `git merge --continue`.
6. Verify with `cargo check` and `cargo test`.
7. Summarize the merge: call out conflicts and how they were resolved.

## Conflict Resolution

- Inspect context before editing with `git status` and `git diff`.
- Prefer minimal, intention-preserving edits.
- Resolve one file at a time and rerun checks after each batch.
- After resolving, ensure no conflict markers remain: `git diff --check`.
