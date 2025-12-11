-- Add down migration script here


ALTER TABLE products MODIFY COLUMN price DECIMAL(10,2) NOT NULL;
