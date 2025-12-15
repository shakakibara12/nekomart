use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::sync::Mutex;

// In-memory "Database" for orders (use Redis/Postgres in production)
struct AppState {
    orders: Mutex<Vec<String>>,
}

// Health Check
async fn health() -> impl Responder {
    HttpResponse::Ok().json("{\"service\": \"checkout\", \"status\": \"healthy\"}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        orders: Mutex::new(vec![]),
    });

    println!("Rust Checkout Service starting on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(health))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
