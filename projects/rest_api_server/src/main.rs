use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
 
#[derive(Serialize, Deserialize, Clone)]
struct Book {
    id: usize,
    title: String,
    author: String,
}
 
struct AppState {
    books: Mutex<Vec<Book>>,
}
 
async fn get_books(data: web::Data<AppState>) -> impl Responder {
    let books = data.books.lock().unwrap();
    HttpResponse::Ok().json(&*books)
}
 
async fn add_book(book: web::Json<Book>, data: web::Data<AppState>) -> impl Responder {
    let mut books = data.books.lock().unwrap();
    books.push(book.into_inner());
    HttpResponse::Created().body("📚 Book added")
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("📘 REST API running at http://127.0.0.1:8080");
 
    let books = web::Data::new(AppState {
        books: Mutex::new(vec![]),
    });
 
    HttpServer::new(move || {
        App::new()
            .app_data(books.clone())
            .route("/books", web::get().to(get_books))
            .route("/books", web::post().to(add_book))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}