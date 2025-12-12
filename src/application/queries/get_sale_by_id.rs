use crate::{
    application::dtos::SaleDTO,
    shared::{
        SharedError,
        input_handler::{Input, InputHandler},
    },
};
use std::sync::Arc;

use crate::domain::repositories::SaleRepository;

pub struct GetSaleByIdQuery {
    pub sale_id: u32,
}
impl Input for GetSaleByIdQuery {
    type Output = SaleDTO;
}
pub struct GetSaleByIdQueryHandler {
    pub sale_repository: Arc<dyn SaleRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<GetSaleByIdQuery> for GetSaleByIdQueryHandler {
    async fn handle(
        &self,
        input: Arc<GetSaleByIdQuery>,
    ) -> Result<<GetSaleByIdQuery as Input>::Output, SharedError> {
        let sale_option = self.sale_repository.get_by_id(&input.sale_id).await;
        match sale_option {
            Some(sale) => Ok(SaleDTO::from(sale)),
            None => Err(SharedError::new(
                &format!("Sale with ID {} not found", input.sale_id),
                404,
            )),
        }
    }
}
