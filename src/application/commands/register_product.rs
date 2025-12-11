use crate::domain::repositories::ProductRepository;
use crate::domain::value_objects::Url;
use crate::shared::SharedError;
use crate::shared::input_handler::{Input, InputHandler};
use std::sync::Arc;

pub struct RegisterProductCommand {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub stock: Option<u32>,
    pub price: f64,
    pub img_url: Option<String>,
    pub flags: Vec<String>,
}

pub struct RegisterProductOutput {
    pub product_id: u32,
}

impl Input for RegisterProductCommand {
    type Output = RegisterProductOutput;
}

pub struct RegisterProductCommandHandler {
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

#[async_trait::async_trait]
impl InputHandler<RegisterProductCommand> for RegisterProductCommandHandler {
    async fn handle(
        &self,
        input: Arc<RegisterProductCommand>,
    ) -> Result<<RegisterProductCommand as Input>::Output, SharedError> {
        let img_url = match &input.img_url {
            Some(u) => Some(Url::new(u.clone())?),
            None => None,
        };
        let product_existing = self.product_repository.get_by_sku(&input.sku).await;
        if let Some(_) = product_existing {
            return Err(SharedError::new(
                format!("Product with SKU {} already exists", input.sku).as_str(),
                400,
            ));
        }
        let product = self
            .product_repository
            .create(
                &input.sku,
                &input.name,
                (input.price * 100.0) as i64,
                input.stock.unwrap_or(0 as u32),
                &input.flags,
                img_url.as_ref(),
                input.description.as_ref(),
            )
            .await?;
        Ok(RegisterProductOutput {
            product_id: product.id(),
        })
    }
}
