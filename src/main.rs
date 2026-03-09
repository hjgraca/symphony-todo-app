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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    println!("Starting todo server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
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
