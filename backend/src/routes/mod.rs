//! API routes

pub mod auth;
pub mod products;
pub mod orders;
pub mod inventory;

pub use auth::auth_routes;
pub use products::product_routes;
pub use orders::order_routes;
pub use inventory::inventory_routes;
