use crate::domain::value_objects::{CC, Email, Url, Phone};
use chrono::{DateTime, Utc};


#[derive(Clone)]
pub struct Customer {
    id: u32,
    cc: CC,
    name: String,
    email: Email,
    phone: Option<Phone>,
    direction: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
impl Customer {
    pub fn new(
        id: u32,
        cc: CC,
        name: String,
        email: Email,
        phone: Option<Phone>,
        direction: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Customer {
            id,
            cc,
            name,
            email,
            phone,
            direction,
            created_at,
            updated_at,
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn cc(&self) -> &CC {
        &self.cc
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn phone(&self) -> &Option<Phone> {
        &self.phone
    }
    pub fn direction(&self) -> &Option<String> {
        &self.direction
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_email(&mut self, email: Email) {
        self.email = email;
    }
    pub fn set_phone(&mut self, phone: Option<Phone>) {
        self.phone = phone;
    }
    pub fn set_direction(&mut self, direction: Option<String>) {
        self.direction = direction;
    }
}

#[derive(Clone)]
pub struct Product {
    id: u32,
    sku: String,
    name: String,
    price: f64,
    stock: u32,
    flags: Vec<String>,
    img_url: Option<Url>,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
impl Product {
    pub fn new(
        id: u32,
        sku: String,
        name: String,
        price: f64,
        stock: u32,
        flags: Vec<String>,
        img_url: Option<Url>,
        description: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Product {
            id,
            sku,
            name,
            price,
            stock,
            flags,
            img_url,
            description,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn sku(&self) -> &String {
        &self.sku
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn stock(&self) -> u32 {
        self.stock
    }
    pub fn flags(&self) -> Vec<String> {
        self.flags.clone()
    }
    pub fn img_url(&self) -> &Option<Url> {
        &self.img_url
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn set_stock(&mut self, stock: u32) {
        self.stock = stock;
    }
    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
    pub fn set_img_url(&mut self, img_url: Option<Url>) {
        self.img_url = img_url;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_flags(&mut self, flags: Vec<String>) {
        self.flags = flags;
    }
}

#[derive(Clone)]
pub struct Sale {
    id: u32,
    /// Vector of (Product, quantity)
    products_sale: Vec<(Product, u32)>,
    customer: Customer,
    generated_at: DateTime<Utc>,
}
impl Sale {
    pub fn new(
        id: u32,
        products: Vec<(Product, u32)>,
        customer: Customer,
        generated_at: DateTime<Utc>,
    ) -> Self {
        Sale {
            id,
            products_sale: products,
            customer,
            generated_at: generated_at,
        }
    }

    pub fn total_amount(&self) -> f64 {
        self.products_sale
            .iter()
            .map(|(product, quantity)| product.price * (*quantity as f64))
            .sum()
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn products_sale(&self) -> &Vec<(Product, u32)> {
        &self.products_sale
    }
    pub fn customer(&self) -> &Customer {
        &self.customer
    }
    pub fn generated_at(&self) -> &DateTime<Utc> {
        &self.generated_at
    }
}
