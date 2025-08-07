use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
 
#[derive(Serialize, Deserialize, Clone)]
struct Post {
    id: usize,
    title: String,
    content: String,
}
 
struct AppState {
    posts: Mutex<HashMap<usize, Post>>,
}
 
async fn list_posts(data: web::Data<AppState>) -> impl Responder {
    let posts = data.posts.lock().unwrap();
    let result: Vec<_> = posts.values().cloned().collect();
    HttpResponse::Ok().json(result)
}
 
async fn get_post(id: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let posts = data.posts.lock().unwrap();
    if let Some(post) = posts.get(&id.into_inner()) {
        HttpResponse::Ok().json(post)
    } else {
        HttpResponse::NotFound().body("❌ Post not found")
    }
}
 
async fn create_post(post: web::Json<Post>, data: web::Data<AppState>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    let post = post.into_inner();
    posts.insert(post.id, post);
    HttpResponse::Created().body("✅ Post created")
}
 
async fn update_post(id: web::Path<usize>, new_post: web::Json<Post>, data: web::Data<AppState>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    let id = id.into_inner();
 
    if posts.contains_key(&id) {
        posts.insert(id, new_post.into_inner());
        HttpResponse::Ok().body("🔄 Post updated")
    } else {
        HttpResponse::NotFound().body("❌ Post not found")
    }
}
 
async fn delete_post(id: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    if posts.remove(&id.into_inner()).is_some() {
        HttpResponse::Ok().body("🗑️ Post deleted")
    } else {
        HttpResponse::NotFound().body("❌ Post not found")
    }
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("📝 CRUD API running at http://127.0.0.1:8080");
 
    let data = web::Data::new(AppState {
        posts: Mutex::new(HashMap::new()),
    });
 
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/posts", web::get().to(list_posts))
            .route("/posts/{id}", web::get().to(get_post))
            .route("/posts", web::post().to(create_post))
            .route("/posts/{id}", web::put().to(update_post))
            .route("/posts/{id}", web::delete().to(delete_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}