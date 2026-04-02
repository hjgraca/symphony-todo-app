use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
struct Todo {
    #[schema(example = "0f6c15ad-e8c6-4478-8f62-2606e2829654")]
    id: String,
    #[schema(example = "Buy milk")]
    title: String,
    #[schema(example = false)]
    completed: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
struct CreateTodo {
    #[schema(example = "Buy milk")]
    title: String,
}

#[derive(Debug, Deserialize, ToSchema)]
struct UpdateTodo {
    #[schema(example = "Buy eggs")]
    title: Option<String>,
    #[schema(example = true)]
    completed: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
struct ErrorResponse {
    #[schema(example = "Todo not found")]
    error: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct HealthResponse {
    #[schema(example = "ok")]
    status: String,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List all todos", body = [Todo])
    )
)]
async fn list_todos(data: web::Data<AppState>) -> HttpResponse {
    let todos = data.todos.lock().unwrap();
    HttpResponse::Ok().json(todos.clone())
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(
        ("id" = String, Path, description = "Todo ID")
    ),
    responses(
        (status = 200, description = "Todo found", body = Todo),
        (status = 404, description = "Todo not found", body = ErrorResponse)
    )
)]
async fn get_todo(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let todos = data.todos.lock().unwrap();
    match todos.iter().find(|t| t.id == id) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Todo not found".to_string(),
        }),
    }
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo created", body = Todo),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    )
)]
async fn create_todo(data: web::Data<AppState>, body: web::Json<CreateTodo>) -> HttpResponse {
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: body.title.clone(),
        completed: false,
    };
    let mut todos = data.todos.lock().unwrap();
    todos.push(todo.clone());
    HttpResponse::Created().json(todo)
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    params(
        ("id" = String, Path, description = "Todo ID")
    ),
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Todo updated", body = Todo),
        (status = 404, description = "Todo not found", body = ErrorResponse)
    )
)]
async fn update_todo(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateTodo>,
) -> HttpResponse {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            if let Some(title) = &body.title {
                todo.title = title.clone();
            }
            if let Some(completed) = body.completed {
                todo.completed = completed;
            }
            HttpResponse::Ok().json(todo.clone())
        }
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Todo not found".to_string(),
        }),
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(
        ("id" = String, Path, description = "Todo ID")
    ),
    responses(
        (status = 204, description = "Todo deleted"),
        (status = 404, description = "Todo not found", body = ErrorResponse)
    )
)]
async fn delete_todo(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    let len_before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < len_before {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: "Todo not found".to_string(),
        })
    }
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service health", body = HealthResponse)
    )
)]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn index_html() -> HttpResponse {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>my todo list</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 600px;
            margin: 50px auto;
            padding: 20px;
            background: #f5f5f5;
        }
        h1 {
            color: #333;
            text-align: center;
        }
        .add-form {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
        }
        .add-form input {
            flex: 1;
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-size: 16px;
        }
        .add-form button {
            padding: 10px 20px;
            background-color: #ffc107;
            color: #212529;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 16px;
            font-weight: bold;
        }
        .add-form button:hover {
            background-color: #e0a800;
        }
        .todo-list {
            list-style: none;
            padding: 0;
        }
        .todo-item {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 15px;
            background: white;
            margin-bottom: 10px;
            border-radius: 4px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }
        .todo-item.completed span {
            text-decoration: line-through;
            color: #888;
        }
        .todo-item span {
            flex: 1;
        }
        .todo-item button {
            padding: 5px 10px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }
        .delete-btn {
            background-color: #6c757d;
            color: white;
        }
        .toggle-btn {
            background-color: #28a745;
            color: white;
        }
    </style>
</head>
<body>
    <h1>my todo list</h1>
    <div class="add-form">
        <input type="text" id="todoInput" placeholder="Enter a new todo...">
        <button onclick="addTodo()">Add Todo</button>
    </div>
    <ul class="todo-list" id="todoList"></ul>

    <script>
        async function loadTodos() {
            const list = document.getElementById('todoList');
            list.innerHTML = '';
            try {
                const response = await fetch('/todos');
                if (!response.ok) {
                    throw new Error('Failed to load todos: ' + response.status);
                }
                const todos = await response.json();
                todos.forEach(todo => {
                    const li = document.createElement('li');
                    li.className = 'todo-item' + (todo.completed ? ' completed' : '');

                    const span = document.createElement('span');
                    span.textContent = todo.title;
                    li.appendChild(span);

                    const toggleBtn = document.createElement('button');
                    toggleBtn.className = 'toggle-btn';
                    toggleBtn.textContent = todo.completed ? 'Undo' : 'Done';
                    toggleBtn.addEventListener('click', () => toggleTodo(todo.id, !todo.completed));
                    li.appendChild(toggleBtn);

                    const deleteBtn = document.createElement('button');
                    deleteBtn.className = 'delete-btn';
                    deleteBtn.textContent = 'Delete';
                    deleteBtn.addEventListener('click', () => deleteTodo(todo.id));
                    li.appendChild(deleteBtn);

                    list.appendChild(li);
                });
            } catch (error) {
                console.error(error);
                const errorLi = document.createElement('li');
                errorLi.className = 'todo-item';
                errorLi.textContent = 'Failed to load todos. Please refresh.';
                list.appendChild(errorLi);
            }
        }

        async function addTodo() {
            const input = document.getElementById('todoInput');
            const title = input.value.trim();
            if (!title) return;
            try {
                const response = await fetch('/todos', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ title })
                });
                if (!response.ok) {
                    throw new Error('Failed to add todo: ' + response.status);
                }
                input.value = '';
                await loadTodos();
            } catch (error) {
                console.error(error);
                alert('Failed to add todo. Please try again.');
            }
        }

        async function toggleTodo(id, completed) {
            try {
                const response = await fetch('/todos/' + id, {
                    method: 'PUT',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ completed })
                });
                if (!response.ok) {
                    throw new Error('Failed to update todo: ' + response.status);
                }
                await loadTodos();
            } catch (error) {
                console.error(error);
                alert('Failed to update todo. Please try again.');
            }
        }

        async function deleteTodo(id) {
            try {
                const response = await fetch('/todos/' + id, { method: 'DELETE' });
                if (!response.ok) {
                    throw new Error('Failed to delete todo: ' + response.status);
                }
                await loadTodos();
            } catch (error) {
                console.error(error);
                alert('Failed to delete todo. Please try again.');
            }
        }

        document.getElementById('todoInput').addEventListener('keydown', (e) => {
            if (e.key === 'Enter') addTodo();
        });

        loadTodos();
    </script>
</body>
</html>"#;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[derive(OpenApi)]
#[openapi(
    paths(health, list_todos, get_todo, create_todo, update_todo, delete_todo),
    components(schemas(Todo, CreateTodo, UpdateTodo, ErrorResponse, HealthResponse)),
    tags(
        (name = "todo-app", description = "Todo API")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    println!("Starting todo server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", ApiDoc::openapi()))
            .route("/health", web::get().to(health))
            .route("/todos", web::get().to(list_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/", web::get().to(index_html))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
