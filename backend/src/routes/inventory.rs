//! Inventory routes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::services::inventory;
use crate::AppState;

pub fn inventory_routes() -> Router<AppState> {
    Router::new()
        .route("/low-stock", get(get_low_stock))
        .route("/:product_id", get(get_inventory))
        .route("/:product_id/restock", post(restock))
}

async fn get_inventory(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let inv = inventory::get_inventory(&state.db, product_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(json!(inv)))
}

async fn get_low_stock(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let items = inventory::get_low_stock_items(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!(items)))
}

#[derive(Debug, Deserialize)]
struct RestockRequest {
    quantity: i32,
}

async fn restock(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
    Json(payload): Json<RestockRequest>,
) -> Result<Json<Value>, StatusCode> {
    inventory::restock_inventory(&state.db, product_id, payload.quantity)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"success": true})))
}
