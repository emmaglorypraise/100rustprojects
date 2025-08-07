use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
 
const SECRET_KEY: &[u8] = b"supersecretkeyyoushouldchange";
 
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
 
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
 
async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    if req.username == "admin" && req.password == "password" {
        let claims = Claims {
            sub: req.username.clone(),
            exp: chrono::Utc::now().timestamp() as usize + 3600,
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
            .expect("Token creation failed");
        HttpResponse::Ok().json(serde_json::json!({ "token": token }))
    } else {
        HttpResponse::Unauthorized().body("❌ Invalid credentials")
    }
}
 
async fn protected(req: HttpRequest) -> impl Responder {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
 
    if let Some(header_value) = auth_header {
        if let Some(token) = header_value.strip_prefix("Bearer ") {
            let validation = Validation::new(Algorithm::HS256);
            let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &validation);
            return match token_data {
                Ok(data) => HttpResponse::Ok().body(format!("🔓 Welcome, {}!", data.claims.sub)),
                Err(_) => HttpResponse::Unauthorized().body("❌ Invalid token"),
            };
        }
    }
 
    HttpResponse::Unauthorized().body("❌ Authorization header missing or malformed")
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🔐 JWT Auth API running at http://127.0.0.1:8080");
 
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login))
            .route("/protected", web::get().to(protected))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
