//! API client for TREZZA TERMINAL backend

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const API_BASE_URL: &str = "http://127.0.0.1:3000/api";

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    token: Option<String>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: None,
        }
    }

    pub fn with_token(token: String) -> Self {
        Self {
            client: Client::new(),
            token: Some(token),
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    // Auth endpoints
    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse> {
        let response = self
            .client
            .post(format!("{}/auth/login", API_BASE_URL))
            .json(&serde_json::json!({
                "username": username,
                "password": password
            }))
            .send()
            .await?
            .json::<LoginResponse>()
            .await?;

        Ok(response)
    }

    // Product endpoints
    pub async fn get_products(&self) -> Result<Vec<ProductResponse>> {
        let response = self
            .client
            .get(format!("{}/products", API_BASE_URL))
            .send()
            .await?
            .json::<Vec<ProductResponse>>()
            .await?;

        Ok(response)
    }

    pub async fn get_product(&self, id: Uuid) -> Result<ProductResponse> {
        let response = self
            .client
            .get(format!("{}/products/{}", API_BASE_URL, id))
            .send()
            .await?
            .json::<ProductResponse>()
            .await?;

        Ok(response)
    }

    pub async fn search_products(&self, query: &str) -> Result<Vec<ProductResponse>> {
        let response = self
            .client
            .get(format!("{}/products/search", API_BASE_URL))
            .query(&[("q", query)])
            .send()
            .await?
            .json::<Vec<ProductResponse>>()
            .await?;

        Ok(response)
    }

    // Order endpoints
    pub async fn create_order(&self, items: Vec<OrderItemRequest>) -> Result<OrderResponse> {
        let mut request = self
            .client
            .post(format!("{}/orders", API_BASE_URL))
            .json(&serde_json::json!({
                "items": items
            }));

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?.json::<OrderResponse>().await?;

        Ok(response)
    }

    pub async fn complete_order(
        &self,
        order_id: Uuid,
        payment_method: &str,
    ) -> Result<OrderSummary> {
        let mut request = self
            .client
            .post(format!("{}/orders/{}/complete", API_BASE_URL, order_id))
            .json(&serde_json::json!({
                "payment_method": payment_method
            }));

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?.json::<OrderSummary>().await?;

        Ok(response)
    }
}

// API Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub currency: String,
    pub category_id: Option<Uuid>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: OrderSummary,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    pub id: Uuid,
    pub order_number: String,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
    pub total_price_cents: i64,
}
