//! Application state management

use gpui::{Model, ModelContext};
use std::collections::HashMap;
use uuid::Uuid;

use crate::api::{ApiClient, ProductResponse};

#[derive(Clone, Debug)]
pub struct CartItem {
    pub product: ProductResponse,
    pub quantity: u32,
}

impl CartItem {
    pub fn total_cents(&self) -> i64 {
        self.product.price_cents * self.quantity as i64
    }
}

pub struct AppState {
    pub api: ApiClient,
    pub cart: HashMap<Uuid, CartItem>,
    pub products: Vec<ProductResponse>,
    pub current_user: Option<String>,
    pub is_loading: bool,
    pub error_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            api: ApiClient::new(),
            cart: HashMap::new(),
            products: Vec::new(),
            current_user: None,
            is_loading: false,
            error_message: None,
        }
    }

    pub fn login(&mut self, token: String, username: String) {
        self.api.set_token(token);
        self.current_user = Some(username);
    }

    pub fn add_to_cart(&mut self, product: ProductResponse, cx: &mut ModelContext<Self>) {
        let product_id = product.id;

        if let Some(item) = self.cart.get_mut(&product_id) {
            item.quantity += 1;
        } else {
            self.cart.insert(
                product_id,
                CartItem {
                    product,
                    quantity: 1,
                },
            );
        }
        cx.notify();
    }

    pub fn remove_from_cart(&mut self, product_id: Uuid, cx: &mut ModelContext<Self>) {
        if let Some(item) = self.cart.get_mut(&product_id) {
            if item.quantity > 1 {
                item.quantity -= 1;
            } else {
                self.cart.remove(&product_id);
            }
        }
        cx.notify();
    }

    pub fn clear_cart(&mut self, cx: &mut ModelContext<Self>) {
        self.cart.clear();
        cx.notify();
    }

    pub fn cart_items(&self) -> Vec<&CartItem> {
        self.cart.values().collect()
    }

    pub fn cart_subtotal(&self) -> i64 {
        self.cart.values().map(|item| item.total_cents()).sum()
    }

    pub fn cart_tax(&self) -> i64 {
        (self.cart_subtotal() as f64 * shared::DEFAULT_TAX_RATE) as i64
    }

    pub fn cart_total(&self) -> i64 {
        self.cart_subtotal() + self.cart_tax()
    }

    pub fn set_loading(&mut self, loading: bool, cx: &mut ModelContext<Self>) {
        self.is_loading = loading;
        cx.notify();
    }

    pub fn set_error(&mut self, error: Option<String>, cx: &mut ModelContext<Self>) {
        self.error_message = error;
        cx.notify();
    }

    pub fn set_products(&mut self, products: Vec<ProductResponse>, cx: &mut ModelContext<Self>) {
        self.products = products;
        cx.notify();
    }
}

// Helper functions to format currency
pub fn format_cents(cents: i64) -> String {
    format!("${:.2}", cents as f64 / 100.0)
}
