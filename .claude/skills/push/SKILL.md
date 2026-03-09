---
name: push
description:
  Push current branch changes to origin; use when asked to push or publish
  updates.
---

# Push

## Steps

1. Identify current branch and confirm remote state.
2. Run local validation (`cargo check`, `cargo test`) before pushing.
3. Push branch to `origin` with upstream tracking if needed.
4. If push is rejected:
   - If non-fast-forward, run the `pull` skill first.
   - Push again; use `--force-with-lease` only when history was rewritten.

## Notes

- Do not use `--force`; only use `--force-with-lease` as last resort.
