use crate::{
    application::dtos::ProductDTO,
    shared::{
        Pagination, PaginationResult, SharedError,
        input_handler::{Input, InputHandler},
    },
};
use std::sync::Arc;

use crate::domain::{entities::Product, repositories::ProductRepository};

pub struct GetProductsQuery {
    pub pagination: Pagination,
}
pub struct GetProductsOutput {
    pub pagination_result: PaginationResult<ProductDTO>,
}

impl Input for GetProductsQuery {
    type Output = GetProductsOutput;
}
pub struct GetProductsQueryHandler {
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<GetProductsQuery> for GetProductsQueryHandler {
    async fn handle(
        &self,
        input: Arc<GetProductsQuery>,
    ) -> Result<<GetProductsQuery as Input>::Output, SharedError> {
        let products = self
            .product_repository
            .get_paginated(&input.pagination)
            .await;
        Ok(GetProductsOutput {
            pagination_result: PaginationResult::from_other::<ProductDTO, Product>(&products),
        })
    }
}
