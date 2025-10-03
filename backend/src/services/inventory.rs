//! Inventory management service

use anyhow::Result;
use chrono::Utc;
use shared::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::Inventory;

pub async fn get_inventory(pool: &PgPool, product_id: Uuid) -> Result<Option<Inventory>> {
    let inventory = sqlx::query_as::<_, Inventory>(
        "SELECT * FROM inventory WHERE product_id = $1",
    )
    .bind(product_id)
    .fetch_optional(pool)
    .await?;

    Ok(inventory)
}

pub async fn check_availability(pool: &PgPool, product_id: Uuid, quantity: i32) -> Result<bool> {
    let inventory = get_inventory(pool, product_id).await?;

    match inventory {
        Some(inv) => Ok(inv.quantity >= quantity),
        None => Ok(false),
    }
}

pub async fn reserve_inventory(
    pool: &PgPool,
    product_id: Uuid,
    quantity: i32,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(|e| AppError::Database(e.to_string()))?;

    let inventory = sqlx::query_as::<_, Inventory>(
        "SELECT * FROM inventory WHERE product_id = $1 FOR UPDATE",
    )
    .bind(product_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    match inventory {
        Some(inv) => {
            if inv.quantity < quantity {
                return Err(AppError::InsufficientInventory {
                    product_id,
                    requested: quantity as u32,
                    available: inv.quantity as u32,
                });
            }

            sqlx::query("UPDATE inventory SET quantity = quantity - $1 WHERE product_id = $2")
                .bind(quantity)
                .bind(product_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }
        None => {
            return Err(AppError::ProductNotFound { id: product_id });
        }
    }

    tx.commit().await.map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

pub async fn restock_inventory(
    pool: &PgPool,
    product_id: Uuid,
    quantity: i32,
) -> Result<()> {
    sqlx::query(
        "UPDATE inventory
         SET quantity = quantity + $1, last_restocked_at = $2
         WHERE product_id = $3",
    )
    .bind(quantity)
    .bind(Utc::now())
    .bind(product_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_low_stock_items(pool: &PgPool) -> Result<Vec<Inventory>> {
    let items = sqlx::query_as::<_, Inventory>(
        "SELECT * FROM inventory WHERE quantity <= reorder_level ORDER BY quantity",
    )
    .fetch_all(pool)
    .await?;

    Ok(items)
}
