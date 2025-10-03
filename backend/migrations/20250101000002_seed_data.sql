-- TREZZA TERMINAL Seed Data
-- Sample data for development and testing

-- Insert default admin user (password: admin123)
-- In production, this should be changed immediately
INSERT INTO users (id, username, email, password_hash, first_name, last_name, role)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'admin',
    'admin@trezzaterminal.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYzpLHJ8n5i', -- admin123
    'System',
    'Administrator',
    'admin'
);

-- Insert sample categories
INSERT INTO categories (id, name, description, sort_order) VALUES
    ('10000000-0000-0000-0000-000000000001', 'Coffee', 'Hot and cold coffee beverages', 1),
    ('10000000-0000-0000-0000-000000000002', 'Tea', 'Various tea selections', 2),
    ('10000000-0000-0000-0000-000000000003', 'Pastries', 'Fresh baked goods', 3),
    ('10000000-0000-0000-0000-000000000004', 'Bottled Drinks', 'Bottled beverages', 4),
    ('10000000-0000-0000-0000-000000000005', 'Snacks', 'Quick snacks and treats', 5);

-- Insert sample products
INSERT INTO products (id, name, description, price_cents, category_id, sku, barcode) VALUES
    -- Coffee
    ('20000000-0000-0000-0000-000000000001', 'Espresso', 'Classic Italian espresso shot', 300, '10000000-0000-0000-0000-000000000001', 'COFFEE-ESP-001', '1234567890001'),
    ('20000000-0000-0000-0000-000000000002', 'Americano', 'Espresso with hot water', 350, '10000000-0000-0000-0000-000000000001', 'COFFEE-AME-001', '1234567890002'),
    ('20000000-0000-0000-0000-000000000003', 'Latte', 'Espresso with steamed milk', 450, '10000000-0000-0000-0000-000000000001', 'COFFEE-LAT-001', '1234567890003'),
    ('20000000-0000-0000-0000-000000000004', 'Cappuccino', 'Espresso with foamed milk', 425, '10000000-0000-0000-0000-000000000001', 'COFFEE-CAP-001', '1234567890004'),
    ('20000000-0000-0000-0000-000000000005', 'Mocha', 'Espresso with chocolate and milk', 475, '10000000-0000-0000-0000-000000000001', 'COFFEE-MOC-001', '1234567890005'),
    ('20000000-0000-0000-0000-000000000006', 'Cold Brew', 'Smooth cold-brewed coffee', 400, '10000000-0000-0000-0000-000000000001', 'COFFEE-CB-001', '1234567890006'),

    -- Tea
    ('20000000-0000-0000-0000-000000000007', 'Green Tea', 'Traditional green tea', 275, '10000000-0000-0000-0000-000000000002', 'TEA-GRN-001', '1234567890007'),
    ('20000000-0000-0000-0000-000000000008', 'Black Tea', 'Classic black tea', 275, '10000000-0000-0000-0000-000000000002', 'TEA-BLK-001', '1234567890008'),
    ('20000000-0000-0000-0000-000000000009', 'Chamomile Tea', 'Relaxing herbal tea', 300, '10000000-0000-0000-0000-000000000002', 'TEA-CHM-001', '1234567890009'),

    -- Pastries
    ('20000000-0000-0000-0000-000000000010', 'Croissant', 'Buttery French croissant', 325, '10000000-0000-0000-0000-000000000003', 'PAST-CRO-001', '1234567890010'),
    ('20000000-0000-0000-0000-000000000011', 'Muffin', 'Fresh-baked muffin', 295, '10000000-0000-0000-0000-000000000003', 'PAST-MUF-001', '1234567890011'),
    ('20000000-0000-0000-0000-000000000012', 'Chocolate Chip Cookie', 'Large chocolate chip cookie', 250, '10000000-0000-0000-0000-000000000003', 'PAST-CCC-001', '1234567890012'),

    -- Bottled Drinks
    ('20000000-0000-0000-0000-000000000013', 'Orange Juice', 'Fresh orange juice', 350, '10000000-0000-0000-0000-000000000004', 'BTL-OJ-001', '1234567890013'),
    ('20000000-0000-0000-0000-000000000014', 'Sparkling Water', 'Carbonated water', 200, '10000000-0000-0000-0000-000000000004', 'BTL-SPRK-001', '1234567890014');

-- Insert inventory for all products
INSERT INTO inventory (product_id, quantity, reorder_level, reorder_quantity)
SELECT id, 100, 20, 50 FROM products;
