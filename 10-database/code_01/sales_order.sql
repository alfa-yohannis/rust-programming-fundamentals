-- select * from sales_orders;

DROP TABLE IF EXISTS sales_order_details;
DROP TABLE IF EXISTS sales_orders;

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
