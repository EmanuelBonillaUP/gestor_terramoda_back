-- Add up migration script here

-- cc is only digits and greater than 5 digits also unique
-- email is unique
-- phone is only digits and equal to 10 digits
INSERT INTO customers (cc, name, email, phone) VALUES
('1234567890', 'John Doe', 'johndoe@email.com', '1234567890'),
('0987654321', 'Jane Smith', 'janesmith@email.com', '0987654321'),
('1122334455', 'Alice Johnson', 'alicejohnson@email.com', '1122334455'),
('2233445566', 'Bob Martinez', 'bobmartinez@email.com', '2233445566'),
('3344556677', 'Carol Lopez', 'carollopez@email.com', '3344556677'),
('4455667788', 'David Garcia', 'davidgarcia@email.com', '4455667788'),
('5566778899', 'Eva Rodriguez', 'evarodriguez@email.com', '5566778899'),
('6677889900', 'Frank Wilson', 'frankwilson@email.com', '6677889900'),
('7788990011', 'Grace Lee', 'gracelee@email.com', '7788990011'),
('8899001122', 'Hector Gomez', 'hectorgomez@email.com', '8899001122'),
('9900112233', 'Irene Torres', 'irenetorres@email.com', '9900112233'),
('1011121314', 'Jose Perez', 'joseperez@email.com', '1011121314'),
('1213141516', 'Karen Diaz', 'karendiaz@email.com', '1213141516'),
('1314151617', 'Luis Suarez', 'luissuarez@email.com', '1314151617'),
('1415161718', 'Maria Fernandez', 'mariafernandez@email.com', '1415161718'),
('1516171819', 'Nora Alvarez', 'noraalvarez@email.com', '1516171819'),
('1617181920', 'Oscar Ramirez', 'oscarramirez@email.com', '1617181920'),
('1718192021', 'Paula Herrera', 'paulaherrera@email.com', '1718192021'),
('1819202122', 'Quinn Morales', 'quinnmorales@email.com', '1819202122'),
('1920212223', 'Ruben Castillo', 'rubencastillo@email.com', '1920212223');


-- sku is unique
-- price is in minor units
INSERT INTO products (sku, name, price, stock) VALUES
('TSHIRT-XD', 'Camiseta Amarilla', 420000, 989),
('TSHIRT-XD-2', 'Camiseta Azul', 420000, 975),
('HOODIE-001', 'Sudadera Negra', 650000, 120),
('HOODIE-002', 'Sudadera Gris', 650000, 85),
('CAP-001', 'Gorra Terramoda', 19900, 340),
('BAG-001', 'Mochila Urbana', 1200000, 45),
('MUG-001', 'Taza Terramoda', 25900, 210),
('NOTEBOOK-001', 'Cuaderno A5', 15900, 500),
('PEN-001', 'Bolígrafo Premium', 5900, 1200),
('STICKER-001', 'Sticker Logo', 900, 2500),
('KEYRING-001', 'Llavero Metálico', 12900, 760),
('JACKET-001', 'Chaqueta Invierno', 1850000, 30),
('DRESS-001', 'Vestido Veraniego', 780000, 60),
('SKIRT-001', 'Falda Casual', 320000, 95),
('JEANS-001', 'Jeans Clásico', 450000, 140),
('SOCKS-001', 'Calcetines Pack x3', 24900, 980),
('BELT-001', 'Cinturón Cuero', 59900, 210),
('SNEAKERS-001', 'Zapatillas Deportivas', 980000, 55),
('WATCH-001', 'Reloj Analógico', 2500000, 18),
('SUNGLASSES-001', 'Gafas de Sol', 89000, 130);


-- customer_cc must exist in customers table
INSERT INTO sales (customer_cc) VALUES
('1234567890'),
('0987654321'),
('1122334455'),
('2233445566'),
('3344556677');

-- sale_id and product_sku must exist in sales and products table respectively
INSERT INTO sale_product (sale_id, product_sku, quantity) VALUES
(1, 'TSHIRT-XD', 2),
(1, 'MUG-001', 1),
(2, 'HOODIE-001', 1),
(2, 'CAP-001', 3),
(3, 'BAG-001', 1),
(3, 'NOTEBOOK-001', 2),
(4, 'PEN-001', 5),
(4, 'STICKER-001', 10),
(5, 'JACKET-001', 1),
(5, 'SNEAKERS-001', 1);
