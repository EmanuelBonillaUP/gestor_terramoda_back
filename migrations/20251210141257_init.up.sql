-- Add up migration script here
-- Migration Script: Initialize Database Schema

-- Create Customers Table
CREATE TABLE customers (
  id INT AUTO_INCREMENT PRIMARY KEY,
  cc VARCHAR(20) UNIQUE NOT NULL,
  name VARCHAR(100) NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL,
  direction VARCHAR(255),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Create Products Table
CREATE TABLE products (
  id  INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
  sku VARCHAR(50) UNIQUE NOT NULL,
  name VARCHAR(100) NOT NULL,
  price DECIMAL(10, 2) NOT NULL,
  stock INT UNSIGNED DEFAULT 0,
  flags VARCHAR(255),
  img_url VARCHAR(255),
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Create Sales Table
CREATE TABLE sales (
  id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
  customer_cc VARCHAR(20) NOT NULL,
  generated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (customer_cc) REFERENCES customers(cc)
);


-- Create sales_product Table
CREATE TABLE sale_product (
  sale_id INT UNSIGNED NOT NULL,
  product_sku VARCHAR(50) NOT NULL,
  quantity INT UNSIGNED NOT NULL,
  FOREIGN KEY (sale_id) REFERENCES sales(id),
  FOREIGN KEY (product_sku) REFERENCES products(sku)
);

-- Create Index on sale_products Table
CREATE INDEX idx_sale_id_product_sku ON sale_product(sale_id, product_sku);

