//! Order management service

use anyhow::Result;
use chrono::Utc;
use shared::{AppError, DEFAULT_TAX_RATE};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{Order, OrderItem, Product};
use crate::services::inventory;

#[derive(Debug, serde::Deserialize)]
pub struct CreateOrderRequest {
    pub items: Vec<CreateOrderItem>,
    pub customer_name: Option<String>,
    pub customer_email: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateOrderItem {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, serde::Serialize)]
pub struct OrderWithItems {
    pub order: Order,
    pub items: Vec<OrderItem>,
}

pub async fn create_order(
    pool: &PgPool,
    user_id: Uuid,
    request: CreateOrderRequest,
) -> Result<OrderWithItems, AppError> {
    if request.items.is_empty() {
        return Err(AppError::EmptyCart);
    }

    let mut tx = pool.begin().await.map_err(|e| AppError::Database(e.to_string()))?;

    // Calculate order totals
    let mut subtotal_cents: i64 = 0;
    let mut order_items = Vec::new();

    for item in &request.items {
        // Get product
        let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
            .bind(item.product_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or(AppError::ProductNotFound { id: item.product_id })?;

        // Check inventory
        if !inventory::check_availability(&pool, item.product_id, item.quantity).await.unwrap_or(false) {
            return Err(AppError::InsufficientInventory {
                product_id: item.product_id,
                requested: item.quantity as u32,
                available: 0,
            });
        }

        let item_total = product.price_cents * item.quantity as i64;
        subtotal_cents += item_total;

        order_items.push((product, item.quantity, item_total));
    }

    // Calculate tax and total
    let tax_cents = (subtotal_cents as f64 * DEFAULT_TAX_RATE) as i64;
    let total_cents = subtotal_cents + tax_cents;

    // Generate order number
    let order_number = format!("ORD-{}", Utc::now().timestamp());

    // Create order
    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (order_number, user_id, customer_name, customer_email,
         subtotal_cents, tax_cents, total_cents, status, notes)
         VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending', $8)
         RETURNING *",
    )
    .bind(&order_number)
    .bind(user_id)
    .bind(&request.customer_name)
    .bind(&request.customer_email)
    .bind(subtotal_cents)
    .bind(tax_cents)
    .bind(total_cents)
    .bind(&request.notes)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Create order items and reserve inventory
    let mut items = Vec::new();
    for (product, quantity, total_price) in order_items {
        let order_item = sqlx::query_as::<_, OrderItem>(
            "INSERT INTO order_items (order_id, product_id, product_name, quantity,
             unit_price_cents, total_price_cents)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(order.id)
        .bind(product.id)
        .bind(&product.name)
        .bind(quantity)
        .bind(product.price_cents)
        .bind(total_price)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        items.push(order_item);
    }

    tx.commit().await.map_err(|e| AppError::Database(e.to_string()))?;

    // Reserve inventory after commit
    for item in &request.items {
        inventory::reserve_inventory(pool, item.product_id, item.quantity).await?;
    }

    Ok(OrderWithItems { order, items })
}

pub async fn get_order(pool: &PgPool, order_id: Uuid) -> Result<Option<OrderWithItems>> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(order_id)
        .fetch_optional(pool)
        .await?;

    match order {
        Some(o) => {
            let items = sqlx::query_as::<_, OrderItem>(
                "SELECT * FROM order_items WHERE order_id = $1 ORDER BY created_at",
            )
            .bind(order_id)
            .fetch_all(pool)
            .await?;

            Ok(Some(OrderWithItems { order: o, items }))
        }
        None => Ok(None),
    }
}

pub async fn complete_order(
    pool: &PgPool,
    order_id: Uuid,
    payment_method: &str,
    payment_reference: Option<String>,
) -> Result<Order, AppError> {
    let order = sqlx::query_as::<_, Order>(
        "UPDATE orders
         SET status = 'completed', payment_method = $1, payment_reference = $2, completed_at = $3
         WHERE id = $4
         RETURNING *",
    )
    .bind(payment_method)
    .bind(payment_reference)
    .bind(Utc::now())
    .bind(order_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(order)
}

pub async fn cancel_order(pool: &PgPool, order_id: Uuid) -> Result<Order, AppError> {
    // Get order items to restore inventory
    let items = sqlx::query_as::<_, OrderItem>(
        "SELECT * FROM order_items WHERE order_id = $1",
    )
    .bind(order_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Restore inventory
    for item in items {
        if let Some(product_id) = item.product_id {
            inventory::restock_inventory(pool, product_id, item.quantity).await.ok();
        }
    }

    // Update order status
    let order = sqlx::query_as::<_, Order>(
        "UPDATE orders SET status = 'cancelled' WHERE id = $1 RETURNING *",
    )
    .bind(order_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(order)
}
