# Todo App

Simple REST API for testing symphony workflows.

## Run

```bash
cd todo-app
cargo run
```

Server starts on `http://127.0.0.1:8080`.

## API

| Method | Endpoint       | Body                              | Description       |
|--------|----------------|-----------------------------------|--------------------|
| GET    | /health        | —                                 | Health check       |
| GET    | /todos         | —                                 | List all todos     |
| POST   | /todos         | `{"title": "Buy milk"}`           | Create a todo      |
| GET    | /todos/{id}    | —                                 | Get a todo by ID   |
| PUT    | /todos/{id}    | `{"title": "...", "completed": true}` | Update a todo |
| DELETE | /todos/{id}    | —                                 | Delete a todo      |

### OpenAPI docs

- OpenAPI spec (JSON): `http://127.0.0.1:8080/openapi.json`
- Interactive Swagger UI: `http://127.0.0.1:8080/docs`

Use Swagger UI to inspect schemas and execute requests directly against the running app.

## Example workflow test

```bash
# Create
curl -X POST http://127.0.0.1:8080/todos -H "Content-Type: application/json" -d '{"title":"Write tests"}'

# List
curl http://127.0.0.1:8080/todos

# Update (replace ID)
curl -X PUT http://127.0.0.1:8080/todos/<id> -H "Content-Type: application/json" -d '{"completed":true}'

# Delete
curl -X DELETE http://127.0.0.1:8080/todos/<id>
```
