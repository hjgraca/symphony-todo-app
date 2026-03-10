---
name: gh
description: |
  Interact with GitHub Issues and PRs using the gh CLI. Use for issue queries,
  label-based state transitions, comment management, and PR operations.
  Environment variables $GITHUB_TOKEN and $GH_REPO are set by Symphony.
---

# GitHub CLI (gh)

Use this skill for all GitHub interactions during Symphony worker sessions.
The `gh` CLI is pre-authenticated via `$GITHUB_TOKEN` and `$GH_REPO` env vars.

## Label-based state management

Symphony uses `status:` prefixed labels to track workflow state on GitHub Issues.
The agent transitions states by swapping labels.

Available states:
- `status:todo` — queued for work
- `status:in-progress` — actively being worked on
- `status:in-review` — PR submitted, waiting for human review
- `status:done` — completed

### Transition state

Remove the old status label and add the new one:

```bash
gh issue edit <number> --remove-label "status:todo" --add-label "status:in-progress"
```

## Issue operations

### View an issue

```bash
gh issue view <number>
gh issue view <number> --json title,body,state,labels,comments
```

### List issue comments

```bash
gh issue view <number> --json comments --jq '.comments'
```

### Create a comment

```bash
gh issue comment <number> --body "## Workpad
..."
```

### Edit an existing comment

Use the GitHub API via `gh api` to update a comment by ID:

```bash
# List comments to find the workpad comment ID
COMMENTS=$(gh api repos/{owner}/{repo}/issues/<number>/comments)

# Update a specific comment
gh api repos/{owner}/{repo}/issues/comments/<comment_id> \
  -X PATCH -f body="## Workpad
<updated content>"
```

### Create a new issue (for follow-ups)

```bash
gh issue create --title "..." --body "..." --label "status:todo"
```

## PR operations

### Create a PR

```bash
gh pr create --title "..." --body "..." --label "symphony" --head <branch>
```

### View PR details

```bash
gh pr view --json number,title,state,reviews,statusCheckRollup,comments
```

### List PR review comments

```bash
gh api repos/{owner}/{repo}/pulls/<pr_number>/comments
```

### Check PR status

```bash
gh pr checks
```

### Link PR to issue

Include `Closes #<number>` or `Fixes #<number>` in the PR body to auto-link.

## Screenshot upload to GitHub

To attach a screenshot to an issue comment, upload it as a release asset or
embed it directly. The simplest approach for issue comments:

```bash
# Upload screenshot and get the URL from the response
# GitHub auto-hosts images pasted into comments via the API
# Use the gh api to create a comment with the image embedded
gh issue comment <number> --body "### UI Validation
![screenshot](https://user-images.githubusercontent.com/...)"
```

For file-based uploads, commit the screenshot to the branch and reference it:

```bash
git add validation_screenshot.png
git commit -m "add validation screenshot"
git push
# Then reference in comment:
gh issue comment <number> --body "### UI Validation
![screenshot](https://raw.githubusercontent.com/{owner}/{repo}/<branch>/validation_screenshot.png)"
```

## Usage rules

- Always use `gh issue edit` with `--remove-label` and `--add-label` for state transitions.
- Never remove labels that don't have the `status:` prefix unless explicitly instructed.
- Use `--json` and `--jq` flags for structured data extraction.
- Prefer `gh` CLI over raw `curl` for GitHub API calls.
- Use `gh api` for operations not directly supported by `gh` subcommands.
