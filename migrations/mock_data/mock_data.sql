INSERT INTO products (
    id, name, brand, image_url, serving_size_grams,
    calories, fat, sugar, sodium, protein, carbs,
    saturated_fat, cholesterol, vitamin_c, calcium,
    vitamin_b1, vitamin_a, is_upf, is_healthier
)
VALUES
('p001', 'Whole Grain Bread', 'HealthyLife', 'https://example.com/images/bread.jpg', 50.0,
 120, 1.5, 2.0, 240.0, 5.0, 22.0,
 0.5, 0.0, 2.5, 30.0,
 0.8, 500.0, false, true),

('p002', 'Chocolate Bar', 'SweetStuff', 'https://example.com/images/choco.jpg', 45.0,
 220, 12.0, 24.0, 150.0, 2.0, 25.0,
 8.0, 5.0, NULL, 20.0,
 NULL, 200.0, true, false),

('p003', 'Low-fat Milk', 'DairyBest', 'https://example.com/images/milk.jpg', 250.0,
 100, 2.5, 12.0, 125.0, 8.0, 10.0,
 1.0, 10.0, 5.0, 300.0,
 1.2, 600.0, false, true);

INSERT INTO categories (id, name)
VALUES
('c001', 'Beverage'),
('c002', 'Snacks'),
('c003', 'Bakery'),
('c004', 'Dairy'),
('c005', 'Breakfast');

INSERT INTO product_category (product_id, category_id)
VALUES
-- Bread → Bakery, Breakfast
('p001', 'c003'),
('p001', 'c005'),

-- Chocolate → Snacks
('p002', 'c002'),

-- Milk → Dairy, Beverage
('p003', 'c004'),
('p003', 'c001');
