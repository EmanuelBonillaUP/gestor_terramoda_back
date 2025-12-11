use crate::{
    application::dtos::CustomerDTO,
    shared::{
        Pagination, PaginationResult, SharedError,
        input_handler::{Input, InputHandler},
    },
};
use std::sync::Arc;

use crate::domain::{entities::Customer, repositories::CustomerRepository};

pub struct GetCustomersQuery {
    pub pagination: Pagination,
}
pub struct GetCustomersOutput {
    pub pagination_result: PaginationResult<CustomerDTO>,
}

impl Input for GetCustomersQuery {
    type Output = GetCustomersOutput;
}

pub struct GetCustomersQueryHandler {
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<GetCustomersQuery> for GetCustomersQueryHandler {
    async fn handle(
        &self,
        input: Arc<GetCustomersQuery>,
    ) -> Result<<GetCustomersQuery as Input>::Output, SharedError> {
        let customers = self
            .customer_repository
            .get_paginated(&input.pagination)
            .await;
        Ok(GetCustomersOutput {
            pagination_result: PaginationResult::from_other::<CustomerDTO, Customer>(&customers),
        })
    }
}
