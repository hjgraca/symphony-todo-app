---
name: linear
description: |
  Interact with Linear's GraphQL API using curl and environment variables
  ($LINEAR_API_KEY, $LINEAR_ENDPOINT). Use for issue queries, comment
  management, state transitions, and attachment operations.
---

# Linear GraphQL

Use this skill for all Linear API interactions during Symphony worker sessions.

## Access method

Use `curl` with the environment variables set by Symphony:

```bash
curl -s -X POST "$LINEAR_ENDPOINT" \
  -H "Authorization: $LINEAR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"query": "<GRAPHQL>", "variables": {}}'
```

- Send one GraphQL operation per call.
- Treat a top-level `errors` array as a failed operation.
- Keep queries narrowly scoped; ask only for the fields you need.

## Discovering unfamiliar operations

When you need an unfamiliar mutation, input type, or object field, use targeted
introspection.

List mutation names:

```graphql
query ListMutations {
  __type(name: "Mutation") {
    fields { name }
  }
}
```

Inspect a specific input object:

```graphql
query CommentCreateInputShape {
  __type(name: "CommentCreateInput") {
    inputFields {
      name
      type { kind name ofType { kind name } }
    }
  }
}
```

## Common workflows

### Query an issue by key, identifier, or id

Use these progressively:

- Start with `issue(id: $key)` when you have a ticket key such as `HJG-10`.
- Fall back to `issues(filter: ...)` when you need identifier search.
- Once you have the internal issue id, prefer `issue(id: $id)` for narrower reads.

### Query team workflow states for an issue

Use this before changing issue state when you need the exact `stateId`:

```graphql
query IssueTeamStates($id: String!) {
  issue(id: $id) {
    id
    team {
      id
      key
      states { nodes { id name type } }
    }
  }
}
```

### Create a comment

```graphql
mutation CreateComment($issueId: String!, $body: String!) {
  commentCreate(input: { issueId: $issueId, body: $body }) {
    success
    comment { id url }
  }
}
```

### Edit an existing comment

```graphql
mutation UpdateComment($id: String!, $body: String!) {
  commentUpdate(id: $id, input: { body: $body }) {
    success
    comment { id body }
  }
}
```

### Move an issue to a different state

```graphql
mutation MoveIssueToState($id: String!, $stateId: String!) {
  issueUpdate(id: $id, input: { stateId: $stateId }) {
    success
    issue { id identifier state { id name } }
  }
}
```

### Read issue comments

```graphql
query IssueComments($id: String!) {
  issue(id: $id) {
    comments { nodes { id body createdAt } }
  }
}
```

## Usage rules

- Prefer the narrowest issue lookup that matches what you already know.
- For state transitions, fetch team states first and use the exact `stateId`.
- Do not hardcode state names inside mutations.
