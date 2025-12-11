use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::sync::Arc;

use crate::{
    application::services::Logger,
    domain::{
        entities::Product,
        repositories::ProductRepository,
        value_objects::{Url, ValueObject},
    },
    shared::{Pagination, PaginationResult, SharedError},
};

#[derive(FromRow, Debug, Clone)]
struct ProductModel {
    pub id: u32,
    pub sku: String,
    pub name: String,
    pub price: u64,
    pub stock: u32,
    pub flags: String,
    pub img_url: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Into<Product> for ProductModel {
    fn into(self) -> Product {
        Product::new(
            self.id,
            self.sku,
            self.name,
            (self.price as f64) / 100.0,
            self.stock,
            self.flags.split(',').map(|s| s.to_string()).collect(),
            match self.img_url {
                Some(url) => Some(Url::new(url).unwrap()),
                None => None,
            },
            self.description,
            self.created_at,
            self.updated_at,
        )
    }
}
pub struct MySQLProductRepository {
    pub pool: Arc<sqlx::MySqlPool>,
    pub logger: Arc<dyn Logger + Send + Sync>,
}
impl MySQLProductRepository {
    pub fn new(pool: Arc<sqlx::MySqlPool>, logger: Arc<dyn Logger + Send + Sync>) -> Self {
        Self {
            pool: pool,
            logger: logger,
        }
    }
}

#[async_trait::async_trait]
impl ProductRepository for MySQLProductRepository {
    async fn get_by_id(&self, id: u32) -> Option<Product> {
        let result = sqlx::query_as::<_, ProductModel>("SELECT * FROM products WHERE id = ?")
            .bind(id)
            .fetch_optional(self.pool.as_ref())
            .await;
        match result {
            Ok(opt) => opt.map(|model| model.into()),
            Err(e) => {
                self.logger
                    .error(&format!("Error fetching product by id {}: {}", id, e));
                None
            }
        }
    }
    async fn get_by_sku(&self, sku: &str) -> Option<Product> {
        let result = sqlx::query_as::<_, ProductModel>("SELECT * FROM products WHERE sku = ?")
            .bind(sku)
            .fetch_optional(self.pool.as_ref())
            .await;
        match result {
            Ok(opt) => opt.map(|model| model.into()),
            Err(e) => {
                self.logger
                    .error(&format!("Error fetching product by sku {}: {}", sku, e));
                None
            }
        }
    }
    async fn get_many_by_skus(&self, skus: Vec<&str>) -> Vec<Product> {
        if skus.is_empty() {
            return Vec::new();
        }
        let skus_str: String = skus
            .iter()
            .map(|s| format!("'{}'", s))
            .collect::<Vec<_>>()
            .join(", ");
        let result = sqlx::query_as::<_, ProductModel>(&format!(
            "SELECT * FROM products WHERE sku IN ({})",
            skus_str
        ))
        .fetch_all(self.pool.as_ref())
        .await;
        self.logger.debug(&format!("resutl: {}", result.is_ok()));
        match result {
            Ok(models) => models.iter().map(|model| model.clone().into()).collect(),
            Err(e) => {
                self.logger.error(&format!(
                    "Error fetching products by skus {:?}: {}",
                    skus, e
                ));
                Vec::new()
            }
        }
    }
    async fn create(
        &self,
        sku: &str,
        name: &str,
        price: i64,
        stock: u32,
        flags: &Vec<String>,
        img_url: Option<&Url>,
        description: Option<&String>,
    ) -> Result<Product, SharedError> {
        let flags_str = flags.join(",");
        let img_url_str = img_url.map(|u| u.value().clone());
        let result = sqlx::query(
            "INSERT INTO products (sku, name, price, stock, flags, img_url, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, NOW(), NOW())"
        )
        .bind(sku)
        .bind(name)
        .bind(price)
        .bind(stock)
        .bind(flags_str)
        .bind(img_url_str)
        .bind(description)
        .execute(self.pool.as_ref())
        .await;
        match result {
            Ok(res) => {
                let id = res.last_insert_id() as u32;
                match self.get_by_id(id).await {
                    Some(product) => Ok(product),
                    None => Err(SharedError::new("Failed to retrieve created product", 500)),
                }
            }
            Err(e) => {
                self.logger
                    .error(&format!("Error creating product with sku {}: {}", sku, e));
                Err(SharedError::new("Failed to create product", 500))
            }
        }
    }
    async fn save(&self, product: &Product) -> Result<(), SharedError> {
        let flags = product.flags().join(",");
        let price = (product.price() * 100.0) as i64;
        let img_url = match product.img_url() {
            Some(url) => Some(url.value().clone()),
            None => None,
        };
        let result = sqlx::query(
            "UPDATE products SET name = ?, price = ?, stock = ?, flags = ?, img_url = ?, description = ?, updated_at = NOW() WHERE id = ?"
        ).bind(product.name())
            .bind(price)
            .bind(product.stock())
            .bind(flags)
            .bind(img_url)
            .bind(product.description())
            .bind(product.id())
            .execute(self.pool.as_ref())
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                self.logger
                    .error(&format!("Error saving product id {}: {}", product.id(), e));
                Err(SharedError::new("Failed to save product", 500))
            }
        }
    }
    async fn get_paginated(&self, pagination: &Pagination) -> PaginationResult<Product> {
        let total_count_result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM products")
            .fetch_one(self.pool.as_ref())
            .await;
        if let Err(e) = total_count_result {
            self.logger
                .error(&format!("Error counting products: {}", e));
            return PaginationResult::from((pagination, 0));
        }
        let total_count = total_count_result.unwrap() as u32;
        if total_count == 0 {
            return PaginationResult::from((pagination, 0));
        }
        let offset = pagination.per_page * (pagination.page - 1);
        let items_result =
            sqlx::query_as::<_, ProductModel>("SELECT * FROM products LIMIT ? OFFSET ?")
                .bind(pagination.per_page as i64)
                .bind(offset as i64)
                .fetch_all(self.pool.as_ref())
                .await;
        match items_result {
            Ok(models) => PaginationResult::from((pagination, total_count))
                .with_data(models.into_iter().map(|model| model.into()).collect()),
            Err(e) => {
                self.logger
                    .error(&format!("Error fetching paginated products: {}", e));
                PaginationResult::from((pagination, 0))
            }
        }
    }
}
