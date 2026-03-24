# Capstone Project: REST API Server

## Overview

In this capstone, you'll build a REST API server in Rust — the same kind of thing you'd build
with Express, Fastify, or Hono in TypeScript. This ties together everything from the book:
structs, traits, error handling, async, modules, and testing.

## The Stack

| TypeScript | Rust | Role |
|-----------|------|------|
| Express / Fastify | `axum` | HTTP framework |
| Prisma / Drizzle | `sqlx` | Database access |
| Zod | `serde` + `validator` | Validation & serialization |
| dotenv | `dotenvy` | Environment config |
| Jest / Vitest | `cargo test` + `reqwest` | Testing |

## Project Structure

```
todo-api/
├── Cargo.toml
├── .env
├── migrations/
│   └── 001_create_todos.sql
├── src/
│   ├── main.rs          # entry point, server startup
│   ├── config.rs        # configuration from env
│   ├── routes/
│   │   ├── mod.rs
│   │   └── todos.rs     # route handlers
│   ├── models/
│   │   ├── mod.rs
│   │   └── todo.rs      # data structures
│   ├── db.rs            # database connection pool
│   └── error.rs         # error types
└── tests/
    └── api_tests.rs     # integration tests
```

## Step 1: Dependencies

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
tower-http = { version = "0.5", features = ["cors", "trace"] }
thiserror = "2"
uuid = { version = "1", features = ["v4", "serde"] }

[dev-dependencies]
reqwest = { version = "0.12", features = ["json"] }
```

## Step 2: Models

```rust
// src/models/todo.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            completed: false,
        }
    }
}
```

## Step 3: Error Handling

```rust
// src/error.rs
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

## Step 4: Route Handlers

```rust
// src/routes/todos.rs
use axum::extract::{Path, State};
use axum::Json;
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::todo::{CreateTodo, Todo, UpdateTodo};

pub async fn list_todos(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

pub async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    if input.title.trim().is_empty() {
        return Err(AppError::BadRequest("title cannot be empty".into()));
    }

    let todo = Todo::new(input.title);
    sqlx::query("INSERT INTO todos (id, title, completed) VALUES (?, ?, ?)")
        .bind(&todo.id)
        .bind(&todo.title)
        .bind(todo.completed)
        .execute(&pool)
        .await?;

    Ok(Json(todo))
}

pub async fn get_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Todo>, AppError> {
    let todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("todo {id} not found")))?;

    Ok(Json(todo))
}

pub async fn update_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let existing = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("todo {id} not found")))?;

    let title = input.title.unwrap_or(existing.title);
    let completed = input.completed.unwrap_or(existing.completed);

    sqlx::query("UPDATE todos SET title = ?, completed = ? WHERE id = ?")
        .bind(&title)
        .bind(completed)
        .bind(&id)
        .execute(&pool)
        .await?;

    Ok(Json(Todo { id, title, completed }))
}

pub async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("todo {id} not found")));
    }

    Ok(StatusCode::NO_CONTENT)
}
```

## Step 5: Main Entry Point

```rust
// src/main.rs
mod config;
mod db;
mod error;
mod models;
mod routes;

use axum::{routing::{get, post, put, delete}, Router};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = db::create_pool().await?;

    let app = Router::new()
        .route("/todos", get(routes::todos::list_todos))
        .route("/todos", post(routes::todos::create_todo))
        .route("/todos/:id", get(routes::todos::get_todo))
        .route("/todos/:id", put(routes::todos::update_todo))
        .route("/todos/:id", delete(routes::todos::delete_todo))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let addr = "0.0.0.0:3000";
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

## Step 6: Integration Tests

```rust
// tests/api_tests.rs
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_create_and_get_todo() {
    let client = Client::new();
    let base = "http://localhost:3000";

    // Create
    let res = client.post(format!("{base}/todos"))
        .json(&json!({ "title": "Learn Rust" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let todo: serde_json::Value = res.json().await.unwrap();
    let id = todo["id"].as_str().unwrap();

    // Get
    let res = client.get(format!("{base}/todos/{id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let fetched: serde_json::Value = res.json().await.unwrap();
    assert_eq!(fetched["title"], "Learn Rust");
    assert_eq!(fetched["completed"], false);
}
```

## What You've Learned

By completing this capstone, you've used:
- **Structs and enums** for data modeling.
- **Traits** (`FromRow`, `IntoResponse`, `Serialize`, `Deserialize`).
- **Error handling** with `thiserror` and `?`.
- **Async/await** with `tokio` and `axum`.
- **Modules** for project organization.
- **Testing** with `cargo test`.
- **Logging** with `tracing`.

Congratulations — you're now a Rust developer. 🦀
