use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::sync::Arc;

use crate::{
    application::services::Logger,
    domain::{
        entities::{Customer, Product, Sale},
        repositories::{CustomerRepository, ProductRepository, SaleRepository},
        value_objects::{CC, ValueObject},
    },
    shared::{Pagination, PaginationResult, SharedError},
};

#[derive(FromRow, Debug, Clone)]
struct SaleModel {
    id: u32,
    customer_cc: String,
    generated_at: DateTime<Utc>,
}
impl Into<Sale> for (SaleModel, Customer, Vec<(&Product, u32)>) {
    fn into(self) -> Sale {
        let (sale_model, customer, products_sale) = self;
        Sale::new(
            sale_model.id,
            products_sale
                .into_iter()
                .map(|(p, q)| (p.clone(), q))
                .collect(),
            customer,
            sale_model.generated_at,
        )
    }
}
#[derive(FromRow, Debug, Clone)]
struct SaleProductModel {
    sale_id: u32,
    product_sku: String,
    quantity: u32,
}

pub struct MySqlSaleRepository {
    pub pool: Arc<sqlx::MySqlPool>,
    pub logger: Arc<dyn Logger + Send + Sync>,
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
}
impl MySqlSaleRepository {
    pub fn new(
        pool: Arc<sqlx::MySqlPool>,
        logger: Arc<dyn Logger + Send + Sync>,
        customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
        product_repository: Arc<dyn ProductRepository + Send + Sync>,
    ) -> Self {
        Self {
            pool,
            logger,
            customer_repository,
            product_repository,
        }
    }
}

#[async_trait::async_trait]
impl SaleRepository for MySqlSaleRepository {
    async fn get_by_id(&self, id: &u32) -> Option<Sale> {
        let sale_model =
            match sqlx::query_as::<_, SaleModel>("SELECT * FROM sales WHERE id = ? LIMIT 1")
                .bind(id)
                .fetch_one(self.pool.as_ref())
                .await
            {
                Ok(sale) => sale,
                Err(e) => {
                    self.logger
                        .error(format!("Failed to fetch sale by id {}: {}", id, e).as_str());
                    return None;
                }
            };
        let customer = match self
            .customer_repository
            .get_by_cc(&CC::new(sale_model.customer_cc.clone()).unwrap())
            .await
        {
            Some(cust) => cust,
            None => {
                self.logger.error(
                    format!(
                        "Customer with CC {} not found for sale {}",
                        sale_model.customer_cc, sale_model.id
                    )
                    .as_str(),
                );
                return None;
            }
        };
        let sales_products = match sqlx::query_as::<_, SaleProductModel>(
            "SELECT * FROM sale_product WHERE sale_id = ?",
        )
        .bind(sale_model.id)
        .fetch_all(self.pool.as_ref())
        .await
        {
            Ok(sales_products) => sales_products,
            Err(e) => {
                self.logger.error(
                    format!(
                        "Failed to fetch sale products for sale {}: {}",
                        sale_model.id, e
                    )
                    .as_str(),
                );
                return None;
            }
        };
        let mut products_sale: Vec<(Product, u32)> = Vec::new();
        for sp in sales_products.iter() {
            match self.product_repository.get_by_sku(&sp.product_sku).await {
                Some(prod) => {
                    products_sale.push((prod, sp.quantity));
                }
                None => {
                    self.logger.error(
                        format!(
                            "Product with SKU {} not found for sale {}",
                            sp.product_sku, sale_model.id
                        )
                        .as_str(),
                    );
                    continue;
                }
            }
        }
        Some(
            (
                sale_model,
                customer,
                products_sale.iter().map(|(p, q)| (p, *q)).collect(),
            )
                .into(),
        )
    }
    async fn create(
        &self,
        customer: &Customer,
        products_sale: Vec<(&Product, u32)>,
    ) -> Result<Sale, SharedError> {
        let customer_cc = customer.cc().value().to_string();
        let result_sale = sqlx::query("INSERT INTO sales (customer_cc) VALUES(?)")
            .bind(&customer_cc)
            .execute(self.pool.as_ref())
            .await;
        let sale_id = match result_sale {
            Ok(res) => res.last_insert_id() as u32,
            Err(e) => {
                self.logger
                    .error(format!("Failed to create sale: {}", e).as_str());
                return Err(SharedError::new("Failed to create sale", 500));
            }
        };
        let mut values = String::new();
        for (p, quantity) in products_sale.iter() {
            values.push_str(&format!("({}, '{}', {}),", sale_id, p.sku(), quantity));
        }
        values.pop();

        let result = sqlx::query(&format!(
            "INSERT INTO sale_product (sale_id, product_sku, quantity) VALUES {}",
            values
        ))
        .execute(self.pool.as_ref())
        .await;
        match result {
            Ok(_) => Ok((
                SaleModel {
                    id: sale_id,
                    customer_cc: customer_cc,
                    generated_at: Utc::now(),
                },
                customer.clone(),
                products_sale,
            )
                .into()),
            Err(e) => {
                self.logger
                    .error(format!("Failed to create sale products: {}", e).as_str());
                Err(SharedError::new("Failed to create sale products", 500))
            }
        }
    }
    async fn get_all(&self) -> Vec<Sale> {
        let sales = match sqlx::query_as::<_, SaleModel>("SELECT * FROM sales")
            .fetch_all(self.pool.as_ref())
            .await
        {
            Ok(sales) => sales,
            Err(e) => {
                self.logger
                    .error(format!("Failed to fetch sales: {}", e).as_str());
                return vec![];
            }
        };
        let customers = self.customer_repository.get_all().await;
        let sales_products =
            match sqlx::query_as::<_, SaleProductModel>("SELECT * FROM sale_product")
                .fetch_all(self.pool.as_ref())
                .await
            {
                Ok(sales_products) => sales_products,
                Err(e) => {
                    self.logger
                        .error(format!("Failed to fetch sales products: {}", e).as_str());
                    return vec![];
                }
            };
        let mut products = self
            .product_repository
            .get_many_by_skus(
                sales_products
                    .iter()
                    .map(|sp| sp.product_sku.as_str())
                    .collect(),
            )
            .await;

        let mut result_sales: Vec<Sale> = Vec::new();
        for sale in sales {
            let customer = match customers
                .iter()
                .find(|c| *c.cc().value() == sale.customer_cc)
            {
                Some(cust) => cust.clone(),
                None => {
                    self.logger.error(
                        format!(
                            "Customer with CC {} not found for sale {}",
                            sale.customer_cc, sale.id
                        )
                        .as_str(),
                    );
                    continue;
                }
            };
            let mut products_sale: Vec<(&Product, u32)> = Vec::new();
            for sp in sales_products.iter().filter(|sp| sp.sale_id == sale.id) {
                match products.iter().find(|p| p.sku().clone() == sp.product_sku) {
                    Some(prod) => {
                        products_sale.push((prod, sp.quantity));
                    }
                    None => {
                        self.logger.error(
                            format!(
                                "Product with SKU {} not found for sale {}",
                                sp.product_sku, sale.id
                            )
                            .as_str(),
                        );
                        continue;
                    }
                }
            }
            let sale = ((sale, customer.clone(), products_sale)).into();
            result_sales.push(sale);
        }
        result_sales
    }
    async fn get_all_by_customer_cc(&self, cc: &CC) -> Vec<Sale> {
        let customer = match self.customer_repository.get_by_cc(cc).await {
            Some(cust) => cust,
            None => {
                self.logger
                    .error(format!("Customer with CC {} not found", cc.value()).as_str());
                return vec![];
            }
        };
        let sales =
            match sqlx::query_as::<_, SaleModel>("SELECT * FROM sales WHERE customer_cc = ?")
                .bind(cc.value())
                .fetch_all(self.pool.as_ref())
                .await
            {
                Ok(sales) => sales,
                Err(e) => {
                    self.logger
                        .error(format!("Failed to fetch sales: {}", e).as_str());
                    return vec![];
                }
            };
        let mut sales_ids_str = String::new();
        for sale in sales.iter() {
            sales_ids_str.push_str(&format!("{},", sale.id));
        }
        sales_ids_str.pop();
        let sales_products = match sqlx::query_as::<_, SaleProductModel>(&format!(
            "SELECT * FROM sale_product WHERE sale_id IN ({})",
            sales_ids_str
        ))
        .fetch_all(self.pool.as_ref())
        .await
        {
            Ok(sales_products) => sales_products,
            Err(e) => {
                self.logger
                    .error(format!("Failed to fetch sales products: {}", e).as_str());
                return vec![];
            }
        };
        let mut result_sales: Vec<Sale> = Vec::new();
        for sale in sales {
            let mut products_sale: Vec<(Product, u32)> = Vec::new();
            for sp in sales_products.iter().filter(|sp| sp.sale_id == sale.id) {
                match self.product_repository.get_by_sku(&sp.product_sku).await {
                    Some(prod) => {
                        products_sale.push((prod.clone(), sp.quantity));
                    }
                    None => {
                        self.logger.error(
                            format!(
                                "Product with SKU {} not found for sale {}",
                                sp.product_sku, sale.id
                            )
                            .as_str(),
                        );
                        continue;
                    }
                }
            }
            let sale = ((
                sale,
                customer.clone(),
                products_sale.iter().map(|(p, q)| (p, *q)).collect(),
            ))
                .into();
            result_sales.push(sale);
        }
        result_sales
    }

    async fn get_paginated(&self, pagination: &Pagination) -> PaginationResult<Sale> {
        let offset = (pagination.page - 1) * pagination.per_page;
        let total_count = match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sales")
            .fetch_one(self.pool.as_ref())
            .await
        {
            Ok(count) => count as u32,
            Err(e) => {
                self.logger
                    .error(format!("Failed to fetch sales count: {}", e).as_str());
                return PaginationResult::from((pagination, 0));
            }
        };
        let sales = match sqlx::query_as::<_, SaleModel>(
            "SELECT * FROM sales ORDER BY generated_at DESC LIMIT ? OFFSET ?",
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(self.pool.as_ref())
        .await
        {
            Ok(sales) => sales,
            Err(e) => {
                self.logger
                    .error(format!("Failed to fetch paginated sales: {}", e).as_str());
                return PaginationResult::from((pagination, 0));
            }
        };
        let mut sales_id_str = String::new();
        for sale in sales.iter() {
            sales_id_str.push_str(&format!("{},", sale.id));
        }
        sales_id_str.pop();
        let sales_product = match sqlx::query_as::<_, SaleProductModel>(&format!(
            "SELECT * FROM sale_product WHERE sale_id IN ({})",
            sales_id_str
        ))
        .fetch_all(self.pool.as_ref())
        .await
        {
            Ok(sales_products) => sales_products,
            Err(e) => {
                self.logger
                    .error(format!("Failed to fetch sales products: {}", e).as_str());
                return PaginationResult::from((pagination, 0));
            }
        };
        let mut customers_cc = Vec::new();
        for sale in sales.iter() {
            match CC::new(sale.customer_cc.clone()) {
                Ok(cc) => {
                    if !customers_cc.iter().any(|c| c == &cc) {
                        customers_cc.push(cc);
                    }
                }
                Err(e) => {
                    self.logger.error(
                        format!(
                            "Invalid CC {} for sale {}: {}",
                            sale.customer_cc, sale.id, e
                        )
                        .as_str(),
                    );
                    continue;
                }
            }
        }
        let products_sku = sales_product
            .iter()
            .map(|sp| sp.product_sku.as_str())
            .collect::<Vec<&str>>();
        let customers = self
            .customer_repository
            .get_many_by_cc(customers_cc.iter().map(|c| c).collect())
            .await;
        let products = self.product_repository.get_many_by_skus(products_sku).await;
        let mut result_sales: Vec<Sale> = Vec::new();
        for sale in sales {
            let customer = match customers
                .iter()
                .find(|c| *c.cc().value() == sale.customer_cc)
            {
                Some(cust) => cust.clone(),
                None => {
                    self.logger.error(
                        format!(
                            "Customer with CC {} not found for sale {}",
                            sale.customer_cc, sale.id
                        )
                        .as_str(),
                    );
                    continue;
                }
            };
            let mut products_sale: Vec<(&Product, u32)> = Vec::new();
            for sp in sales_product.iter().filter(|sp| sp.sale_id == sale.id) {
                match products.iter().find(|p| p.sku().clone() == sp.product_sku) {
                    Some(prod) => {
                        products_sale.push((prod, sp.quantity));
                    }
                    None => {
                        self.logger.error(
                            format!(
                                "Product with SKU {} not found for sale {}",
                                sp.product_sku, sale.id
                            )
                            .as_str(),
                        );
                        continue;
                    }
                }
            }
            let sale = ((sale, customer.clone(), products_sale)).into();
            result_sales.push(sale);
        }
        PaginationResult::from((pagination, total_count)).with_data(result_sales)
    }
}
