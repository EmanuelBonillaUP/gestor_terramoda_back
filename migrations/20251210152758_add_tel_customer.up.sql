-- Add up migration script here

ALTER TABLE customers ADD COLUMN phone CHAR(10);
