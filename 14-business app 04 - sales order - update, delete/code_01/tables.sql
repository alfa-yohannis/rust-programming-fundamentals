-- Drop existing tables if they exist
DROP TABLE IF EXISTS sales_order_details;
DROP TABLE IF EXISTS sales_orders;
DROP TABLE IF EXISTS item;

-- Create item table with code as the unique identifier
CREATE TABLE item (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) DEFAULT LPAD(CAST(FLOOR(1 + random() * 9)::INT * 1000000000 + FLOOR(random() * 1000000000)::INT AS VARCHAR), 10, '0') NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    currency VARCHAR(3) NOT NULL,
    price FLOAT4 NOT NULL,
    quantity FLOAT4 NOT NULL DEFAULT 0,
    unit VARCHAR(50) NULL
);

-- Insert items into item table
INSERT INTO item (code, name, currency, price, quantity, unit)
VALUES
    ('0000000010', 'Organic Almonds', 'USD', 15.49, 50.00, 'kg'),
    ('0000000020', 'Stainless Steel Water Bottle', 'USD', 23.99, 150.00, 'pcs'),
    ('0000000030', 'Bluetooth Headphones', 'USD', 89.99, 75.00, 'pcs');

-- Create sales_orders table
CREATE TABLE sales_orders (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) NOT NULL UNIQUE,
    order_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    note VARCHAR(256) NULL
);

-- Create sales_order_details table using item_code instead of item_id
CREATE TABLE sales_order_details (
    id SERIAL PRIMARY KEY,
    order_code VARCHAR(10) NOT NULL REFERENCES sales_orders(code),
    line_num INTEGER NOT NULL, 
    item_code VARCHAR(10) NOT NULL REFERENCES item(code),
    item_name VARCHAR NOT NULL,
    quantity REAL NOT NULL DEFAULT 0,
    unit VARCHAR(50) NULL,
    unit_price REAL NOT NULL,
    currency VARCHAR(3) NOT NULL
);

-- Insert Sales Orders
INSERT INTO sales_orders (code, note)
VALUES 
    ('SO00001', 'Order for various items'),
    ('SO00002', 'Second order for selected items'),
    ('SO00003', 'Third order including electronics');

-- Insert Sales Order Details for all orders using item_code and item_name
INSERT INTO sales_order_details (order_code, line_num, item_code, item_name, quantity, unit, unit_price, currency)
VALUES
    -- Details for Sales Order SO00001
    ('SO00001', 1, '0000000010', 'Organic Almonds', 5.00, 'kg', 15.49, 'USD'),
    ('SO00001', 2, '0000000020', 'Stainless Steel Water Bottle', 10.00, 'pcs', 23.99, 'USD'),

    -- Details for Sales Order SO00002
    ('SO00002', 1, '0000000020', 'Stainless Steel Water Bottle', 20.00, 'pcs', 23.99, 'USD'),
    ('SO00002', 2, '0000000030', 'Bluetooth Headphones', 5.00, 'pcs', 89.99, 'USD'),

    -- Details for Sales Order SO00003
    ('SO00003', 1, '0000000010', 'Organic Almonds', 10.00, 'kg', 15.49, 'USD'),
    ('SO00003', 2, '0000000030', 'Bluetooth Headphones', 3.00, 'pcs', 89.99, 'USD');
