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
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};
use tracing_subscriber;

mod auth;
mod config;
mod db;
mod routes;
mod services;

use config::Config;
use db::pool::{create_pool, run_migrations};
use routes::{auth_routes, inventory_routes, order_routes, product_routes};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .init();

    info!("Starting {} Backend v{}", APP_NAME, APP_VERSION);

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    info!("Configuration loaded");

    // Create database pool
    let db = create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");
    info!("Database pool created");

    // Run migrations
    run_migrations(&db)
        .await
        .expect("Failed to run migrations");
    info!("Database migrations completed");

    // Build the app state
    let state = AppState { db, config: config.clone() };

    // Build the router
    let app = create_app(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(config.server_address())
        .await
        .expect("Failed to bind to address");

    info!("Server listening on http://{}", config.server_address());

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

/// Create the Axum application with all routes and middleware
fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/api/health", get(health_check))
        .nest("/api/auth", auth_routes())
        .nest("/api/products", product_routes())
        .nest("/api/orders", order_routes())
        .nest("/api/inventory", inventory_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": APP_NAME,
        "version": APP_VERSION
    }))
}
