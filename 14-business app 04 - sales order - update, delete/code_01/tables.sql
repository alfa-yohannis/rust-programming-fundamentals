DROP TABLE IF EXISTS item;
DROP TABLE IF EXISTS sales_order_details;
DROP TABLE IF EXISTS sales_orders;

CREATE TABLE item (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) DEFAULT LPAD(CAST(FLOOR(1 + random() * 9)::INT * 1000000000 + FLOOR(random() * 1000000000)::INT AS VARCHAR), 10, '0') NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    currency VARCHAR(3) NOT NULL,
    price FLOAT4 NOT NULL,
    quantity FLOAT4 NOT NULL DEFAULT 0,
    unit VARCHAR(50) NULL
);

INSERT INTO item (code, name, currency, price, quantity, unit)
VALUES
    ('0000000010', 'Organic Almonds', 'USD', 15.49, 50.00, 'kg'),
    ('0000000020', 'Stainless Steel Water Bottle', 'USD', 23.99, 150.00, 'pcs'),
    ('0000000030', 'Bluetooth Headphones', 'USD', 89.99, 75.00, 'pcs');

CREATE TABLE sales_orders (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) NOT NULL UNIQUE,
    order_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
	note VARCHAR (256) NULL
);

CREATE TABLE sales_order_details (
    id SERIAL PRIMARY KEY,
    order_code VARCHAR(10) NOT NULL REFERENCES sales_orders(code),
    line_num INTEGER NOT NULL, 
    item_id INTEGER NOT NULL,
    quantity REAL NOT NULL DEFAULT 0,
    unit VARCHAR(50) NULL,
    unit_price REAL NOT NULL);
