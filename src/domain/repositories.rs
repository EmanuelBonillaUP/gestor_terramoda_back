use super::entities::{Customer, Product, Sale};
use super::value_objects::{CC, Email, Phone, Url};
use crate::shared::{Pagination, PaginationResult, SharedError};

#[async_trait::async_trait]
pub trait SaleRepository {
    async fn get_by_id(&self, id: &u32) -> Option<Sale>;
    async fn create(
        &self,
        customer: &Customer,
        products_sale: Vec<(&Product, u32)>,
    ) -> Result<Sale, SharedError>;
    async fn get_paginated(&self, pagination: &Pagination) -> PaginationResult<Sale>;
    async fn get_all_by_customer_cc(&self, cc: &CC) -> Vec<Sale>;
    async fn get_all(&self) -> Vec<Sale>;
}
#[async_trait::async_trait]
pub trait ProductRepository {
    async fn get_by_id(&self, id: u32) -> Option<Product>;
    async fn get_by_sku(&self, sku: &str) -> Option<Product>;
    async fn get_many_by_skus(&self, skus: Vec<&str>) -> Vec<Product>;
    async fn create(
        &self,
        sku: &str,
        name: &str,
        price: i64,
        stock: u32,
        flags: &Vec<String>,
        img_url: Option<&Url>,
        description: Option<&String>,
    ) -> Result<Product, SharedError>;
    async fn save(&self, product: &Product) -> Result<(), SharedError>;
    async fn get_paginated(&self, pagination: &Pagination) -> PaginationResult<Product>;
}

#[async_trait::async_trait]
pub trait CustomerRepository {
    async fn get_by_id(&self, id: u32) -> Option<Customer>;
    async fn get_by_cc(&self, cc: &CC) -> Option<Customer>;
    async fn create(
        &self,
        cc: &CC,
        name: &str,
        email: &Email,
        phone: Option<&Phone>,
        direction: Option<&str>,
    ) -> Result<Customer, SharedError>;
    async fn save(&self, customer: &Customer) -> Result<(), SharedError>;
    async fn get_many_by_cc(&self, many_cc: Vec<&CC>) -> Vec<Customer>;
    async fn get_paginated(&self, pagination: &Pagination) -> PaginationResult<Customer>;
    async fn get_all(&self) -> Vec<Customer>;
}
