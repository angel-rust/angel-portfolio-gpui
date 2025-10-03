//! Unit tests for shared types

#[cfg(test)]
mod tests {
    use shared::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_money_from_dollars() {
        let money = Money::from_dollars(Decimal::new(1050, 2)); // $10.50
        assert_eq!(money.amount, 1050);
    }

    #[test]
    fn test_money_addition() {
        let m1 = Money::from_cents(500);
        let m2 = Money::from_cents(300);
        let result = m1.add(m2);
        assert_eq!(result.amount, 800);
    }

    #[test]
    fn test_money_multiply() {
        let money = Money::from_cents(350);
        let result = money.multiply(3);
        assert_eq!(result.amount, 1050);
    }

    #[test]
    fn test_cart_item_creation() {
        let product = Product {
            id: uuid::Uuid::new_v4(),
            name: "Test Product".to_string(),
            description: None,
            price: Money::from_cents(500),
            category_id: None,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let cart_item = CartItem::new(&product, 2);
        assert_eq!(cart_item.quantity, 2);
        assert_eq!(cart_item.total_price.amount, 1000);
    }
}
