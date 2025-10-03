//! Application constants

/// Default tax rate (8.25%)
pub const DEFAULT_TAX_RATE: f64 = 0.0825;

/// Maximum cart items
pub const MAX_CART_ITEMS: usize = 100;

/// Maximum quantity per item
pub const MAX_ITEM_QUANTITY: u32 = 999;

/// Application name
pub const APP_NAME: &str = "TREZZA TERMINAL";

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");