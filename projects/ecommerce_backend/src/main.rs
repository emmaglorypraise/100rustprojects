use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use dotenvy::dotenv;
 
#[derive(FromRow, Serialize)]
struct Product {
    id: i32,
    name: String,
    price: f64,
}
 
#[derive(FromRow, Serialize)]
struct Customer {
    id: i32,
    name: String,
    email: String,
}
 
#[derive(Deserialize)]
struct NewProduct {
    name: String,
    price: f64,
}
 
#[derive(Deserialize)]
struct NewCustomer {
    name: String,
    email: String,
}
 
// Products
async fn create_product(pool: web::Data<PgPool>, json: web::Json<NewProduct>) -> impl Responder {
    let res = sqlx::query("INSERT INTO products (name, price) VALUES ($1, $2)")
        .bind(&json.name)
        .bind(json.price)
        .execute(pool.get_ref())
        .await;
 
    match res {
        Ok(_) => HttpResponse::Created().body("✅ Product added"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to add product"),
    }
}
 
async fn list_products(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY id")
        .fetch_all(pool.get_ref())
        .await;
 
    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to fetch products"),
    }
}
 
// Customers
async fn create_customer(pool: web::Data<PgPool>, json: web::Json<NewCustomer>) -> impl Responder {
    let res = sqlx::query("INSERT INTO customers (name, email) VALUES ($1, $2)")
        .bind(&json.name)
        .bind(&json.email)
        .execute(pool.get_ref())
        .await;
 
    match res {
        Ok(_) => HttpResponse::Created().body("✅ Customer added"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to add customer"),
    }
}
 
async fn list_customers(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY id")
        .fetch_all(pool.get_ref())
        .await;
 
    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to fetch customers"),
    }
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not set");
    let db = PgPool::connect(&db_url).await.expect("❌ DB connection failed");
 
    println!("🛍️ E-Commerce API running at http://localhost:8080");
 
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/products", web::post().to(create_product))
            .route("/products", web::get().to(list_products))
            .route("/customers", web::post().to(create_customer))
            .route("/customers", web::get().to(list_customers))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}