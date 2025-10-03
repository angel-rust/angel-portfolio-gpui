//! Error types for the TREZZA TERMINAL application

use thiserror::Error;
use uuid::Uuid;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Product not found: {id}")]
    ProductNotFound { id: Uuid },

    #[error("Invalid quantity: {quantity}")]
    InvalidQuantity { quantity: u32 },

    #[error("Cart is empty")]
    EmptyCart,

    #[error("Order not found: {id}")]
    OrderNotFound { id: Uuid },

    #[error("Payment failed: {reason}")]
    PaymentFailed { reason: String },

    #[error("Insufficient inventory for product {product_id}: requested {requested}, available {available}")]
    InsufficientInventory {
        product_id: Uuid,
        requested: u32,
        available: u32,
    },

    #[error("Database error: {0}")]
    Database(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for convenience
pub type AppResult<T> = Result<T, AppError>;