use actix_web::{
    get, post, web, App, HttpResponse, HttpServer, Responder, 
    middleware::Logger, error, Result
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use env_logger::Env;

// Custom error type
#[derive(Debug)]
struct AppError {
    msg: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::ResponseError for AppError {}

// Application state
struct AppState {
    visitor_count: Mutex<u32>,
}

// Request/Response models
#[derive(Serialize, Deserialize)]
struct GreetingRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct GreetingResponse {
    greeting: String,
    visitor_count: u32,
}

#[derive(Serialize, Deserialize)]
struct HealthCheckResponse {
    status: String,
    version: String,
}

// Route handlers
#[get("/")]
async fn hello(data: web::Data<AppState>) -> Result<impl Responder> {
    let mut count = data.visitor_count.lock().unwrap();
    *count += 1;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"
            <h1>👋 Hello, Rust Web!</h1>
            <p>Try these endpoints:</p>
            <ul>
                <li><a href="/health">/health</a> - Health check</li>
                <li><a href="/greet">/greet</a> - Greeting form</li>
                <li><a href="/api/greet?name=Visitor">/api/greet?name=Visitor</a> - JSON API</li>
                <li><a href="/metrics">/metrics</a> - Visitor count</li>
            </ul>
        "#))
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "OK".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[post("/api/greet")]
async fn greet_api(
    data: web::Data<AppState>,
    query: web::Query<GreetingRequest>,
) -> Result<impl Responder> {
    let mut count = data.visitor_count.lock().unwrap();
    *count += 1;
    
    Ok(HttpResponse::Ok().json(GreetingResponse {
        greeting: format!("Hello, {}! 👋", query.name),
        visitor_count: *count,
    }))
}

#[get("/greet")]
async fn greet_form() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"
            <form action="/api/greet" method="get">
                <label for="name">Your name:</label>
                <input type="text" id="name" name="name" required>
                <button type="submit">Greet me!</button>
            </form>
        "#)
}

#[get("/metrics")]
async fn metrics(data: web::Data<AppState>) -> impl Responder {
    let count = data.visitor_count.lock().unwrap();
    HttpResponse::Ok().body(format!("Total visitors: {}", *count))
}

// Error handler
async fn not_found() -> Result<HttpResponse, AppError> {
    Err(AppError {
        msg: "Resource not found".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("🚀 Starting server at http://127.0.0.1:8080");
    println!("📦 Version: {}", env!("CARGO_PKG_VERSION"));

    // Shared application state
    let app_data = web::Data::new(AppState {
        visitor_count: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .service(hello)
            .service(health_check)
            .service(greet_api)
            .service(greet_form)
            .service(metrics)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// use actix_web::{get, App, HttpServer, Responder, HttpResponse};
 
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("👋 Hello, Rust Web!")
// }
 
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     println!("🚀 Starting server at http://127.0.0.1:8080");
 
//     HttpServer::new(|| {
//         App::new()
//             .service(hello) // Register the route handler
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }