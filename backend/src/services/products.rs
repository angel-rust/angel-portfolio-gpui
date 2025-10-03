//! Product service

use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{Category, Product};

pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE is_active = true ORDER BY name",
    )
    .fetch_all(pool)
    .await?;

    Ok(products)
}

pub async fn get_product_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Product>> {
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(product)
}

pub async fn get_products_by_category(pool: &PgPool, category_id: Uuid) -> Result<Vec<Product>> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE category_id = $1 AND is_active = true ORDER BY name",
    )
    .bind(category_id)
    .fetch_all(pool)
    .await?;

    Ok(products)
}

pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<Category>> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE is_active = true ORDER BY sort_order, name",
    )
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn search_products(pool: &PgPool, query: &str) -> Result<Vec<Product>> {
    let search_pattern = format!("%{}%", query);
    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products
         WHERE is_active = true
         AND (name ILIKE $1 OR description ILIKE $1 OR sku ILIKE $1 OR barcode = $2)
         ORDER BY name
         LIMIT 50",
    )
    .bind(&search_pattern)
    .bind(query)
    .fetch_all(pool)
    .await?;

    Ok(products)
}
