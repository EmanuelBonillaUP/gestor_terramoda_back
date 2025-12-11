use crate::{
    application::dtos::SaleDTO,
    shared::{
        Pagination, PaginationResult, SharedError,
        input_handler::{Input, InputHandler},
    },
};
use std::sync::Arc;

use crate::domain::{entities::Sale, repositories::SaleRepository};

pub struct GetSalesQuery {
    pub pagination: Pagination,
}
pub struct GetSalesOutput {
    pub pagination_result: PaginationResult<SaleDTO>,
}

impl Input for GetSalesQuery {
    type Output = GetSalesOutput;
}
pub struct GetSalesQueryHandler {
    pub sale_repository: Arc<dyn SaleRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<GetSalesQuery> for GetSalesQueryHandler {
    async fn handle(
        &self,
        input: Arc<GetSalesQuery>,
    ) -> Result<<GetSalesQuery as Input>::Output, SharedError> {
        let sales = self.sale_repository.get_paginated(&input.pagination).await;
        Ok(GetSalesOutput {
            pagination_result: PaginationResult::from_other::<SaleDTO, Sale>(&sales),
        })
    }
}
