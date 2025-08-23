use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
 
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
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
        .fetch_all(db.get_ref())
        .await;
 
    match todos {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to fetch todos"),
    }
}
 
async fn add_todo(db: web::Data<PgPool>, json: web::Json<NewTodo>) -> impl Responder {
    let result = sqlx::query("INSERT INTO todos (title) VALUES ($1)")
        .bind(&json.title)
        .execute(db.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Created().body("✅ Todo added"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to insert"),
    }
}
 
async fn update_todo(
    db: web::Data<PgPool>,
    path: web::Path<i32>,
    json: web::Json<UpdateTodo>,
) -> impl Responder {
    let result = sqlx::query("UPDATE todos SET completed = $1 WHERE id = $2")
        .bind(json.completed)
        .bind(*path)
        .execute(db.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Ok().body("🔄 Todo updated"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Update failed"),
    }
}
 
async fn delete_todo(db: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(*path)
        .execute(db.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Ok().body("🗑️ Todo deleted"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Delete failed"),
    }
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not set");
    let db = PgPool::connect(&db_url).await.expect("❌ Failed to connect to DB");
 
    println!("✅ Todo API running at http://127.0.0.1:8080");
 
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/todos", web::get().to(list_todos))
            .route("/todos", web::post().to(add_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}