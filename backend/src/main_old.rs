//! TREZZA TERMINAL Backend Server
//!
//! Axum-based API server for the TREZZA TERMINAL application

use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use shared::{APP_NAME, APP_VERSION};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .init();

    info!("Starting {} Backend v{}", APP_NAME, APP_VERSION);

    // Build the router
    let app = create_app();

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    info!("Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

/// Create the Axum application with all routes and middleware
fn create_app() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/api/health", get(health_check))
        .route("/api/products", get(get_products))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}

/// Health check endpoint
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": APP_NAME,
        "version": APP_VERSION
    }))
}

/// Get all products endpoint (placeholder)
async fn get_products() -> Result<Json<Value>, StatusCode> {
    // TODO: Implement actual product retrieval from database
    let products = json!([
        {
            "id": "550e8400-e29b-41d4-a716-446655440001",
            "name": "Espresso",
            "price": 300,
            "category": "Coffee"
        },
        {
            "id": "550e8400-e29b-41d4-a716-446655440002",
            "name": "Latte",
            "price": 450,
            "category": "Coffee"
        }
    ]);

    Ok(Json(products))
}