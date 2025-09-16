use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool, Row}; // Added Row import
use sqlx::postgres::PgRow; // Import PgRow if needed for type annotations

#[derive(Serialize, FromRow)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodo {
    completed: bool,
}

async fn list_todos(db: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
        .fetch_all(db.get_ref())
        .await
    {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            eprintln!("❌ Database error: {}", e);
            HttpResponse::InternalServerError().body(format!("❌ Failed to fetch todos: {}", e))
        }
    }
}

async fn add_todo(db: web::Data<PgPool>, json: web::Json<NewTodo>) -> impl Responder {
    // Method 1: Using query_as with returning
    match sqlx::query_as::<_, Todo>("INSERT INTO todos (title) VALUES ($1) RETURNING *")
        .bind(&json.title)
        .fetch_one(db.get_ref())
        .await
    {
        Ok(todo) => HttpResponse::Created().json(serde_json::json!({
            "message": "✅ Todo added",
            "id": todo.id,
            "todo": todo
        })),
        Err(e) => {
            eprintln!("❌ Insert error: {}", e);
            HttpResponse::InternalServerError().body(format!("❌ Failed to insert: {}", e))
        }
    }
}

// Alternative add_todo using query and get()
async fn add_todo_alternative(db: web::Data<PgPool>, json: web::Json<NewTodo>) -> impl Responder {
    match sqlx::query("INSERT INTO todos (title) VALUES ($1) RETURNING id")
        .bind(&json.title)
        .fetch_one(db.get_ref())
        .await
    {
        Ok(row) => {
            let id: i32 = row.get("id");
            HttpResponse::Created().json(serde_json::json!({
                "message": "✅ Todo added",
                "id": id
            }))
        },
        Err(e) => {
            eprintln!("❌ Insert error: {}", e);
            HttpResponse::InternalServerError().body(format!("❌ Failed to insert: {}", e))
        }
    }
}

async fn update_todo(
    db: web::Data<PgPool>,
    path: web::Path<i32>,
    json: web::Json<UpdateTodo>,
) -> impl Responder {
    match sqlx::query("UPDATE todos SET completed = $1 WHERE id = $2")
        .bind(json.completed)
        .bind(*path)
        .execute(db.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("🔄 Todo updated")
            } else {
                HttpResponse::NotFound().body("❌ Todo not found")
            }
        },
        Err(e) => {
            eprintln!("❌ Update error: {}", e);
            HttpResponse::InternalServerError().body(format!("❌ Update failed: {}", e))
        }
    }
}

async fn delete_todo(db: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    match sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(*path)
        .execute(db.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("🗑️ Todo deleted")
            } else {
                HttpResponse::NotFound().body("❌ Todo not found")
            }
        },
        Err(e) => {
            eprintln!("❌ Delete error: {}", e);
            HttpResponse::InternalServerError().body(format!("❌ Delete failed: {}", e))
        }
    }
}

async fn health_check(db: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1").execute(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("✅ Database connection OK"),
        Err(e) => {
            eprintln!("❌ Health check failed: {}", e);
            HttpResponse::InternalServerError().body(format!("❌ Database connection failed: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not set");
    
    // Use PgPoolOptions from the postgres module
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&db_url)
        .expect("❌ Failed to create connection pool");
    
    println!("✅ Todo API running at http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/health", web::get().to(health_check))
            .route("/todos", web::get().to(list_todos))
            .route("/todos", web::post().to(add_todo)) // Use add_todo (not add_todo_alternative)
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}