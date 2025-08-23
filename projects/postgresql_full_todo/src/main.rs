use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow, Error as SqlxError};
use dotenvy::dotenv;
use std::fmt;

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Deserialize, Debug)]
struct CreateTodo {
    title: String,
}

#[derive(Deserialize, Debug)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    fn success(message: &str, data: Option<T>) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data,
        }
    }

    fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

// Custom error type for better error handling
#[derive(Debug)]
enum AppError {
    Database(SqlxError),
    NotFound,
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(e) => {
                eprintln!("Database error: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Database operation failed: {}", e)))
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(ApiResponse::<()>::error("Todo not found"))
            }
            AppError::Validation(msg) => {
                HttpResponse::BadRequest().json(ApiResponse::<()>::error(msg))
            }
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> Self {
        AppError::Database(error)
    }
}

async fn get_all(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    println!("Fetching all todos");
    
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
        .fetch_all(pool.get_ref())
        .await?;

    println!("Successfully fetched {} todos", todos.len());
    Ok(HttpResponse::Ok().json(ApiResponse::success("Todos retrieved successfully", Some(todos))))
}

async fn get_one(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    println!("Fetching todo with ID: {}", id);
    
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await?;

    match todo {
        Some(todo) => {
            println!("Found todo with ID: {}", id);
            Ok(HttpResponse::Ok().json(ApiResponse::success("Todo retrieved successfully", Some(todo))))
        }
        None => {
            println!("Todo with ID {} not found", id);
            Err(AppError::NotFound)
        }
    }
}

async fn create_todo(data: web::Json<CreateTodo>, pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    println!("Creating new todo: {:?}", data);
    
    // Validation
    if data.title.trim().is_empty() {
        return Err(AppError::Validation("Todo title cannot be empty".to_string()));
    }

    let result = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed"
    )
    .bind(data.title.trim())
    .fetch_one(pool.get_ref())
    .await?;

    println!("Todo created successfully with ID: {}", result.id);
    Ok(HttpResponse::Created().json(ApiResponse::success("Todo created successfully", Some(result))))
}

async fn update_todo(
    id: web::Path<i32>,
    data: web::Json<UpdateTodo>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    println!("Updating todo ID {} with data: {:?}", id, data);
    
    // Check if todo exists first
    let existing_todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await?;

    let todo = match existing_todo {
        Some(todo) => todo,
        None => {
            println!("Todo with ID {} not found for update", id);
            return Err(AppError::NotFound);
        }
    };

    // Prepare update values
    let new_title = match &data.title {
        Some(title) if !title.trim().is_empty() => title.trim(),
        Some(_) => return Err(AppError::Validation("Todo title cannot be empty".to_string())),
        None => &todo.title,
    };

    let new_completed = data.completed.unwrap_or(todo.completed);

    let updated_todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING id, title, completed"
    )
    .bind(new_title)
    .bind(new_completed)
    .bind(*id)
    .fetch_one(pool.get_ref())
    .await?;

    println!("Todo ID {} updated successfully", id);
    Ok(HttpResponse::Ok().json(ApiResponse::success("Todo updated successfully", Some(updated_todo))))
}

async fn delete_todo(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    println!("Deleting todo with ID: {}", id);
    
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(*id)
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        println!("Todo with ID {} not found for deletion", id);
        return Err(AppError::NotFound);
    }

    println!("Todo with ID {} deleted successfully", id);
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success("Todo deleted successfully", None)))
}

async fn health_check(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success("Database connection healthy", None)),
        Err(e) => {
            eprintln!("Health check failed: {}", e);
            HttpResponse::ServiceUnavailable().json(ApiResponse::<()>::error(&format!("Database connection failed: {}", e)))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let db_url = std::env::var("DATABASE_URL")
        .expect("❌ DATABASE_URL not set in environment variables");
    
    println!("Connecting to database...");
    
    let db = PgPool::connect(&db_url)
        .await
        .expect("❌ Failed to connect to database");

    println!("✅ Successfully connected to database");
    
    // Create table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
        )
        "#
    )
    .execute(&db)
    .await
        .expect("❌ Failed to create todos table");

    println!("✅ Todo App running at http://localhost:8080");
    println!("Available endpoints:");
    println!("  GET    /health          - Health check");
    println!("  GET    /todos           - Get all todos");
    println!("  GET    /todos/{{id}}      - Get specific todo");
    println!("  POST   /todos           - Create new todo");
    println!("  PUT    /todos/{{id}}      - Update todo");
    println!("  DELETE /todos/{{id}}      - Delete todo");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/health", web::get().to(health_check))
            .route("/todos", web::get().to(get_all))
            .route("/todos/{id}", web::get().to(get_one))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}