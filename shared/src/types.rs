//! Common data types shared across the application

use chrono::{DateTime, Utc};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents monetary amounts with precision
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money {
    /// Amount in the smallest currency unit (e.g., cents for USD)
    pub amount: i64,
    /// Currency code (ISO 4217)
    pub currency: CurrencyCode,
}

impl Money {
    /// Create a new Money instance from cents
    pub fn from_cents(cents: i64) -> Self {
        Self {
            amount: cents,
            currency: CurrencyCode::USD,
        }
    }

    /// Create a new Money instance from dollars
    pub fn from_dollars(dollars: Decimal) -> Self {
        let cents = (dollars * Decimal::from(100)).to_i64().unwrap_or(0);
        Self::from_cents(cents)
    }

    /// Get the amount as a decimal (dollars)
    pub fn as_decimal(&self) -> Decimal {
        Decimal::from(self.amount) / Decimal::from(100)
    }

    /// Add two Money amounts
    pub fn add(&self, other: Money) -> Money {
        Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }

    /// Multiply by quantity
    pub fn multiply(&self, quantity: u32) -> Money {
        Money {
            amount: self.amount * quantity as i64,
            currency: self.currency,
        }
    }
}

/// Supported currency codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CurrencyCode {
    USD,
    EUR,
    GBP,
    CAD,
}

/// Product information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Money,
    pub category_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Product category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
}

/// Shopping cart item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: Money,
    pub total_price: Money,
    pub created_at: DateTime<Utc>,
}

impl CartItem {
    /// Create a new cart item
    pub fn new(product: &Product, quantity: u32) -> Self {
        let total_price = product.price.multiply(quantity);

        Self {
            id: Uuid::new_v4(),
            product_id: product.id,
            product_name: product.name.clone(),
            quantity,
            unit_price: product.price,
            total_price,
            created_at: Utc::now(),
        }
    }
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Draft,
    Pending,
    Processing,
    Completed,
    Cancelled,
    Refunded,
}

/// Payment method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    Cash,
    CreditCard,
    DebitCard,
    GiftCard,
    Crypto,
}

/// Complete order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,
    pub items: Vec<CartItem>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total_amount: Money,
    pub status: OrderStatus,
    pub payment_method: Option<PaymentMethod>,
    pub customer_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}