//! Order routes

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

use crate::auth::AuthContext;
use crate::services::orders::{self, CreateOrderRequest};
use crate::AppState;

pub fn order_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_order))
        .route("/:id", get(get_order))
        .route("/:id/complete", post(complete_order))
        .route("/:id/cancel", post(cancel_order))
}

async fn create_order(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<Value>, StatusCode> {
    let order_with_items = orders::create_order(&state.db, auth.user_id, payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(json!(order_with_items)))
}

async fn get_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let order = orders::get_order(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(json!(order)))
}

#[derive(Debug, Deserialize)]
struct CompleteOrderRequest {
    payment_method: String,
    payment_reference: Option<String>,
}

async fn complete_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CompleteOrderRequest>,
) -> Result<Json<Value>, StatusCode> {
    let order = orders::complete_order(&state.db, id, &payload.payment_method, payload.payment_reference)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(json!(order)))
}

async fn cancel_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let order = orders::cancel_order(&state.db, id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(json!(order)))
}
