use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
 
#[derive(Serialize, FromRow)]
struct Post {
    id: i32,
    title: String,
    body: String,
    category_id: i32,
}
 
#[derive(Serialize, FromRow)]
struct Comment {
    id: i32,
    post_id: i32,
    author: String,
    content: String,
}
 
#[derive(Serialize, FromRow)]
struct Category {
    id: i32,
    name: String,
}
 
#[derive(Deserialize)]
struct NewPost {
    title: String,
    body: String,
    category_id: i32,
}
 
#[derive(Deserialize)]
struct NewComment {
    post_id: i32,
    author: String,
    content: String,
}
 
#[derive(Deserialize)]
struct NewCategory {
    name: String,
}
 
// 🧱 Routes
 
async fn list_posts(pool: web::Data<PgPool>) -> impl Responder {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY id")
        .fetch_all(pool.get_ref())
        .await;
    HttpResponse::Ok().json(posts.unwrap())
}
 
async fn create_post(pool: web::Data<PgPool>, data: web::Json<NewPost>) -> impl Responder {
    let result = sqlx::query("INSERT INTO posts (title, body, category_id) VALUES ($1, $2, $3)")
        .bind(&data.title)
        .bind(&data.body)
        .bind(data.category_id)
        .execute(pool.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Created().body("✅ Post created"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to create post"),
    }
}
 
async fn list_comments(pool: web::Data<PgPool>) -> impl Responder {
    let comments = sqlx::query_as::<_, Comment>("SELECT * FROM comments ORDER BY id")
        .fetch_all(pool.get_ref())
        .await;
    HttpResponse::Ok().json(comments.unwrap())
}
 
async fn add_comment(pool: web::Data<PgPool>, data: web::Json<NewComment>) -> impl Responder {
    let result = sqlx::query("INSERT INTO comments (post_id, author, content) VALUES ($1, $2, $3)")
        .bind(data.post_id)
        .bind(&data.author)
        .bind(&data.content)
        .execute(pool.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Created().body("✅ Comment added"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to add comment"),
    }
}
 
async fn list_categories(pool: web::Data<PgPool>) -> impl Responder {
    let cats = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(pool.get_ref())
        .await;
    HttpResponse::Ok().json(cats.unwrap())
}
 
async fn add_category(pool: web::Data<PgPool>, data: web::Json<NewCategory>) -> impl Responder {
    let result = sqlx::query("INSERT INTO categories (name) VALUES ($1)")
        .bind(&data.name)
        .execute(pool.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Created().body("✅ Category added"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to create category"),
    }
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not set");
    let db = PgPool::connect(&db_url).await.expect("❌ Could not connect to DB");
 
    println!("📝 Blog API running at http://localhost:8080");
 
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/posts", web::get().to(list_posts))
            .route("/posts", web::post().to(create_post))
            .route("/comments", web::get().to(list_comments))
            .route("/comments", web::post().to(add_comment))
            .route("/categories", web::get().to(list_categories))
            .route("/categories", web::post().to(add_category))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
