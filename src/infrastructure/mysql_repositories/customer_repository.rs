use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::sync::Arc;

use crate::{
    application::services::Logger,
    domain::{
        entities::Customer as CustomerDomain,
        repositories::CustomerRepository,
        value_objects::{CC, Email, Phone, ValueObject},
    },
    shared::{Pagination, PaginationResult, SharedError},
};

#[derive(FromRow, Debug, Clone)]
struct CustomerModel {
    pub id: i32,
    pub cc: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub direction: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Into<CustomerDomain> for CustomerModel {
    fn into(self) -> CustomerDomain {
        CustomerDomain::new(
            self.id as u32,
            CC::new(self.cc).unwrap(),
            self.name,
            Email::new(self.email).unwrap(),
            self.phone.map(|p| Phone::new(p).unwrap()),
            self.direction,
            self.created_at,
            self.updated_at,
        )
    }
}

pub struct MysqlCustomerRepository {
    pub pool: Arc<sqlx::MySqlPool>,
    pub logger: Arc<dyn Logger + Send + Sync>,
}
impl MysqlCustomerRepository {
    pub fn new(pool: Arc<sqlx::MySqlPool>, logger: Arc<dyn Logger + Send + Sync>) -> Self {
        Self {
            pool: pool,
            logger: logger,
        }
    }
}

#[async_trait::async_trait]
impl CustomerRepository for MysqlCustomerRepository {
    async fn get_by_id(&self, id: u32) -> Option<CustomerDomain> {
        let result = sqlx::query_as::<_, CustomerModel>("SELECT * FROM customers WHERE id = ?")
            .bind(id)
            .fetch_optional(self.pool.as_ref())
            .await;
        match result {
            Ok(customer) => customer.map(|c| c.into()),
            Err(e) => {
                self.logger
                    .error(format!("Failed to fetch customer by ID from database {}", e).as_str());
                None
            }
        }
    }
    async fn get_by_cc(&self, cc: &CC) -> Option<CustomerDomain> {
        let cc = cc.value();
        let result = sqlx::query_as::<_, CustomerModel>("SELECT * FROM customers WHERE cc = ?")
            .bind(cc)
            .fetch_optional(self.pool.as_ref())
            .await;
        match result {
            Ok(customer) => customer.map(|c| c.into()),
            Err(e) => {
                self.logger
                    .error(format!("Failed to fetch customer by CC from database {}", e).as_str());
                None
            }
        }
    }

    async fn create(
        &self,
        cc: &CC,
        name: &str,
        email: &Email,
        phone: Option<&Phone>,
        direction: Option<&str>,
    ) -> Result<CustomerDomain, SharedError> {
        let cc_value = cc.value();
        let email_value = email.value();
        let result = sqlx::query(
            "INSERT INTO customers (cc, name, email, phone, direction) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(cc_value)
        .bind(name)
        .bind(email_value)
        .bind(phone.map(|p| p.value()))
        .bind(direction)
        .execute(self.pool.as_ref())
        .await;
        match result {
            Ok(res) => {
                let id = res.last_insert_id() as u32;
                let now = Utc::now();
                Ok(CustomerDomain::new(
                    id,
                    cc.clone(),
                    name.to_string(),
                    email.clone(),
                    phone.cloned(),
                    direction.map(|d| d.to_string()),
                    now,
                    now,
                ))
            }
            Err(e) => Err(SharedError::new(
                format!("Failed to create customer: {}", e).as_str(),
                500,
            )),
        }
    }

    async fn save(&self, customer: &CustomerDomain) -> Result<(), SharedError> {
        let result = sqlx::query(
            "UPDATE customers SET name = ?, email = ?, phone = ?, direction = ?, updated_at = ? WHERE cc = ?",
        )
        .bind(customer.name())
        .bind(customer.email().value())
        .bind(customer.phone().as_ref().map(|p| p.value()))
        .bind(customer.direction().as_deref())
        .bind(Utc::now())
        .bind(customer.cc().value())
        .execute(self.pool.as_ref())
        .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(SharedError::new(
                format!("Failed to update customer: {}", e).as_str(),
                500,
            )),
        }
    }

    async fn get_paginated(
        &self,
        pagination: &Pagination,
    ) -> crate::shared::PaginationResult<CustomerDomain> {
        let total_elements_result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM customers")
            .fetch_one(self.pool.as_ref())
            .await;

        let total = match total_elements_result {
            Ok(count) => count as u32,
            Err(e) => {
                self.logger.error(
                    format!("Failed to fetch total customers count from database {}", e).as_str(),
                );
                return PaginationResult::from((pagination, 0));
            }
        };

        if total == 0 {
            self.logger
                .info("No customers found in the database.".to_string().as_str());
            return PaginationResult::from((pagination, 0));
        }

        let offset = (pagination.page - 1) * pagination.per_page;

        let result = sqlx::query_as::<_, CustomerModel>(
            "SELECT * FROM customers ORDER BY created_at desc LIMIT ? OFFSET ?",
        )
        .bind(pagination.per_page as u64)
        .bind(offset as u64)
        .fetch_all(&*self.pool)
        .await;

        match result {
            Ok(customers) => PaginationResult::from((pagination, total))
                .with_data(customers.iter().map(|c| c.to_owned().into()).collect()),
            Err(e) => {
                self.logger.error(
                    format!("Failed to fetch paginated customers from database {}", e).as_str(),
                );
                PaginationResult::from((pagination, total))
            }
        }
    }
    async fn get_all(&self) -> Vec<CustomerDomain> {
        let result = sqlx::query_as::<_, CustomerModel>("SELECT * FROM customers")
            .fetch_all(self.pool.as_ref())
            .await;
        match result {
            Ok(customers) => customers.into_iter().map(|c| c.into()).collect(),
            Err(e) => {
                self.logger
                    .error(format!("Failed to fetch all customers from database {}", e).as_str());
                vec![]
            }
        }
    }
    async fn get_many_by_cc(&self, many_cc: Vec<&CC>) -> Vec<CustomerDomain> {
        let cc_values: Vec<String> = many_cc.iter().map(|cc| cc.value().to_string()).collect();
        let result = sqlx::query_as::<_, CustomerModel>(&format!(
            "SELECT * FROM customers WHERE cc IN ({})",
            cc_values.join(",")
        ))
        .fetch_all(self.pool.as_ref())
        .await;
        match result {
            Ok(customers) => customers.into_iter().map(|c| c.into()).collect(),
            Err(e) => {
                self.logger.error(
                    format!("Failed to fetch customers by CCs from database {}", e).as_str(),
                );
                vec![]
            }
        }
    }
}
