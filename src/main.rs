use std::sync::Arc;

use axum::{http::StatusCode, routing::{get, post}, Json, Router};
use database::DB;
use handler::{create_table, signup, signin};
use tower_http::cors::{Any, CorsLayer};

mod database;
mod handler;

struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_state = Arc::new(AppState { db: DB::init().await.unwrap() });

    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route("/create_table", get(create_table))
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .layer(cors)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    println!("server running on port 8081");
    axum::serve(listener, app).await.unwrap();

    Ok(())
    
}

pub async fn health_check_handler() -> (StatusCode, Json<String>) {
    const MESSAGE: &str = "API Services";

    (StatusCode::OK, Json(MESSAGE.to_string()))
}
