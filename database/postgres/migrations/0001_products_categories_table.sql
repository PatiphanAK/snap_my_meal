CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table for Products
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- ใช้ UUID และสร้างแบบสุ่ม
    name VARCHAR(255) NOT NULL,
    brand VARCHAR(255),
    image_url VARCHAR(255),
    serving_size_grams REAL,
    calories INTEGER NOT NULL,
    fat REAL NOT NULL,
    sugar REAL NOT NULL,
    sodium REAL NOT NULL,
    protein REAL NOT NULL,
    carbs REAL NOT NULL,
    saturated_fat REAL NOT NULL,
    cholesterol REAL NOT NULL,
    vitamin_c REAL,
    calcium REAL,
    vitamin_b1 REAL,
    vitamin_a REAL,
    price REAL,
    is_upf BOOLEAN NOT NULL,
    is_healthier BOOLEAN NOT NULL
);

-- Table for Categories
CREATE TABLE categories (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

-- Junction table for the many-to-many relationship between Products and Categories
CREATE TABLE product_category (
    product_id UUID REFERENCES products(id),
    category_id INT REFERENCES categories(id),
    PRIMARY KEY (product_id, category_id)
);
