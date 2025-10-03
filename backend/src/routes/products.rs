//! Product routes

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::services::products;
use crate::AppState;

pub fn product_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_products))
        .route("/:id", get(get_product))
        .route("/search", get(search_products))
}

#[derive(Debug, Deserialize)]
struct ProductQuery {
    category_id: Option<Uuid>,
}

async fn get_products(
    State(state): State<AppState>,
    Query(params): Query<ProductQuery>,
) -> Result<Json<Value>, StatusCode> {
    let products = if let Some(category_id) = params.category_id {
        products::get_products_by_category(&state.db, category_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        products::get_all_products(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    Ok(Json(json!(products)))
}

async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let product = products::get_product_by_id(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(json!(product)))
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String,
}

async fn search_products(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Value>, StatusCode> {
    let products = products::search_products(&state.db, &params.q)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!(products)))
}
