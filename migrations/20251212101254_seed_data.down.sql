-- Add down migration script here
-- DELETE seeded data from sale_product
DELETE FROM sale_product WHERE product_sku IN (
  'TSHIRT-XD', 
  'MUG-001',
  'HOODIE-001',
  'CAP-001',
  'BAG-001',
  'NOTEBOOK-001',
  'PEN-001',
  'STICKER-001',
  'JACKET-001',
  'SNEAKERS-001'
);


-- DELETE seeded data from sales
DELETE FROM sales WHERE customer_cc IN (
  '1234567890',
  '0987654321',
  '1122334455',
  '2233445566',
  '3344556677'
);

-- DELETE seeded data from products
DELETE FROM products WHERE sku IN (
  'TSHIRT-XD',
  'TSHIRT-XD-2',
  'HOODIE-001',
  'HOODIE-002',
  'CAP-001',
  'BAG-001',
  'MUG-001',
  'NOTEBOOK-001',
  'PEN-001',
  'STICKER-001',
  'KEYRING-001',
  'JACKET-001',
  'DRESS-001',
  'SKIRT-001',
  'JEANS-001',
  'SOCKS-001',
  'BELT-001',
  'SNEAKERS-001',
  'WATCH-001',
  'SUNGLASSES-001'
);

-- DELETE seeded data from customers
DELETE FROM customers WHERE cc IN (
  '1234567890',
  '0987654321',
  '1122334455',
  '2233445566',
  '3344556677',
  '4455667788',
  '5566778899',
  '6677889900',
  '7788990011',
  '8899001122',
  '9900112233',
  '1011121314',
  '1213141516',
  '1314151617',
  '1415161718',
  '1516171819',
  '1617181920',
  '1718192021',
  '1819202122',
  '1920212223'
);
