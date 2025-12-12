use crate::{
    application::dtos::ProductDTO,
    shared::{
        SharedError,
        input_handler::{Input, InputHandler},
    },
};
use std::sync::Arc;

use crate::domain::repositories::ProductRepository;

pub struct GetProductBySkuQuery {
    pub sku: String,
}

impl Input for GetProductBySkuQuery {
    type Output = ProductDTO;
}
pub struct GetProductsBySkuQueryHandler {
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<GetProductBySkuQuery> for GetProductsBySkuQueryHandler {
    async fn handle(
        &self,
        input: Arc<GetProductBySkuQuery>,
    ) -> Result<<GetProductBySkuQuery as Input>::Output, SharedError> {
        let product_option = self.product_repository.get_by_sku(&input.sku).await;
        match product_option {
            Some(product) => Ok(ProductDTO::from(product)),
            None => Err(SharedError::new(
                &format!("Product with SKU {} not found", input.sku),
                404,
            )),
        }
    }
}
