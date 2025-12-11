-- Add up migration script here

ALTER TABLE products MODIFY COLUMN price BIGINT UNSIGNED NOT NULL DEFAULT 0;
