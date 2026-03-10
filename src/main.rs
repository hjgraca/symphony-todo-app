use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

async fn list_todos(data: web::Data<AppState>) -> HttpResponse {
    let todos = data.todos.lock().unwrap();
    HttpResponse::Ok().json(todos.clone())
}

async fn get_todo(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let todos = data.todos.lock().unwrap();
    match todos.iter().find(|t| t.id == id) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Todo not found"})),
    }
}

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
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Todo not found"})),
    }
}

async fn delete_todo(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    let len_before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < len_before {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(serde_json::json!({"error": "Todo not found"}))
    }
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

async fn index_html() -> HttpResponse {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Todo App</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 600px;
            margin: 50px auto;
            padding: 20px;
            background: #f5f5f5;
        }
        h1 { color: #333; }
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
            background-color: #007bff;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 16px;
            font-weight: 500;
        }
        .add-form button:hover {
            background-color: #0056b3;
        }
        .todo-list {
            list-style: none;
            padding: 0;
        }
        .todo-item {
            background: white;
            padding: 15px;
            margin-bottom: 10px;
            border-radius: 4px;
            display: flex;
            align-items: center;
            gap: 10px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }
        .todo-item.completed span {
            text-decoration: line-through;
            color: #888;
        }
        .todo-item input[type="checkbox"] {
            width: 20px;
            height: 20px;
        }
        .todo-item span { flex: 1; }
        .delete-btn {
            background: #dc3545;
            color: white;
            border: none;
            padding: 5px 10px;
            border-radius: 4px;
            cursor: pointer;
        }
    </style>
</head>
<body>
    <h1>Todo App</h1>
    <div class="add-form">
        <input type="text" id="todo-input" placeholder="Enter a new todo...">
        <button id="add-todo-btn">Add Todo</button>
    </div>
    <ul class="todo-list" id="todo-list"></ul>

    <script>
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }

        async function loadTodos() {
            const res = await fetch('/todos');
            const todos = await res.json();
            const list = document.getElementById('todo-list');
            list.innerHTML = todos.map(todo => `
                <li class="todo-item ${todo.completed ? 'completed' : ''}" data-id="${escapeHtml(todo.id)}">
                    <input type="checkbox" ${todo.completed ? 'checked' : ''} onchange="toggleTodo('${escapeHtml(todo.id)}', this.checked)">
                    <span>${escapeHtml(todo.title)}</span>
                    <button class="delete-btn" onclick="deleteTodo('${escapeHtml(todo.id)}')">Delete</button>
                </li>
            `).join('');
        }

        async function addTodo() {
            const input = document.getElementById('todo-input');
            const title = input.value.trim();
            if (!title) return;
            await fetch('/todos', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ title })
            });
            input.value = '';
            loadTodos();
        }

        async function toggleTodo(id, completed) {
            await fetch(`/todos/${id}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ completed })
            });
            loadTodos();
        }

        async function deleteTodo(id) {
            await fetch(`/todos/${id}`, { method: 'DELETE' });
            loadTodos();
        }

        document.getElementById('add-todo-btn').addEventListener('click', addTodo);
        document.getElementById('todo-input').addEventListener('keydown', (e) => {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    println!("Starting todo server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index_html))
            .route("/health", web::get().to(health))
            .route("/todos", web::get().to(list_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
