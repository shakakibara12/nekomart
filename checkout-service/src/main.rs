use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

// Data Structures
#[derive(Serialize, Deserialize, Clone, Debug)]
struct OrderItem {
    product_id: u32,
    quantity: u32,
}

#[derive(Serialize, Deserialize)]
struct OrderRequest {
    user_id: String,
    items: Vec<OrderItem>,
}

#[derive(Serialize)]
struct OrderResponse {
    order_id: String,
    status: String,
    total_cost: f32,
}

// In-memory "Database" for orders (use Redis/Postgres in production)
struct AppState {
    orders: Mutex<Vec<String>>,
}

// Health Check
async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({
    "service": "checkout",
    "status": "healthy"
    }))
}

// Create Order Endpoint
async fn create_order(order: web::Json<OrderRequest>, data: web::Data<AppState>) -> impl Responder {
    // Generate a fake Order ID
    let order_id = format!("ORD-{}", uuid::Uuid::new_v4());

    // Calculate Cost (Mock Logic)
    let total_cost: f32 = order.items.iter().map(|i| i.quantity as f32 * 10.0).sum();

    // Store in "DB"
    data.orders.lock().unwrap().push(order_id.clone());

    println!(
        "Order Received from User {}: {:?}",
        order.user_id, order.items
    );

    HttpResponse::Ok().json(OrderResponse {
        order_id,
        status: "Processing".to_string(),
        total_cost,
    })
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
            .route("/order", web::post().to(create_order))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
