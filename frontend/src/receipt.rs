//! Receipt generation and printing

use chrono::Utc;
use uuid::Uuid;

use crate::state::{format_cents, CartItem};

pub struct Receipt {
    pub order_number: String,
    pub items: Vec<ReceiptItem>,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub payment_method: String,
    pub timestamp: String,
}

pub struct ReceiptItem {
    pub name: String,
    pub quantity: u32,
    pub unit_price_cents: i64,
    pub total_cents: i64,
}

impl Receipt {
    pub fn from_cart(cart_items: Vec<&CartItem>, payment_method: &str) -> Self {
        let items: Vec<ReceiptItem> = cart_items
            .iter()
            .map(|ci| ReceiptItem {
                name: ci.product.name.clone(),
                quantity: ci.quantity,
                unit_price_cents: ci.product.price_cents,
                total_cents: ci.total_cents(),
            })
            .collect();

        let subtotal_cents: i64 = items.iter().map(|i| i.total_cents).sum();
        let tax_cents = (subtotal_cents as f64 * shared::DEFAULT_TAX_RATE) as i64;
        let total_cents = subtotal_cents + tax_cents;

        Self {
            order_number: format!("ORD-{}", Utc::now().timestamp()),
            items,
            subtotal_cents,
            tax_cents,
            total_cents,
            payment_method: payment_method.to_string(),
            timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn to_text(&self) -> String {
        let mut output = String::new();

        output.push_str("=====================================\n");
        output.push_str("        TREZZA TERMINAL\n");
        output.push_str("         Point of Sale\n");
        output.push_str("=====================================\n\n");
        output.push_str(&format!("Order: {}\n", self.order_number));
        output.push_str(&format!("Date:  {}\n\n", self.timestamp));
        output.push_str("-------------------------------------\n");
        output.push_str("ITEM                  QTY      TOTAL\n");
        output.push_str("-------------------------------------\n");

        for item in &self.items {
            output.push_str(&format!(
                "{:<20} x{:<3}  {}\n",
                truncate(&item.name, 20),
                item.quantity,
                format_cents(item.total_cents)
            ));
        }

        output.push_str("-------------------------------------\n");
        output.push_str(&format!(
            "Subtotal:             {}\n",
            format_cents(self.subtotal_cents)
        ));
        output.push_str(&format!(
            "Tax (8.25%):          {}\n",
            format_cents(self.tax_cents)
        ));
        output.push_str("-------------------------------------\n");
        output.push_str(&format!(
            "TOTAL:                {}\n",
            format_cents(self.total_cents)
        ));
        output.push_str("-------------------------------------\n\n");
        output.push_str(&format!("Payment: {}\n\n", self.payment_method));
        output.push_str("Thank you for your business!\n");
        output.push_str("=====================================\n");

        output
    }

    pub fn print(&self) -> anyhow::Result<()> {
        // In a real implementation, this would send to a thermal printer
        // For now, we'll just write to a file
        let filename = format!("receipt_{}.txt", self.order_number);
        std::fs::write(&filename, self.to_text())?;
        log::info!("Receipt saved to {}", filename);
        Ok(())
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
