use crate::domain::repositories::{ProductRepository};
use crate::domain::value_objects::Url;
use crate::shared::SharedError;
use crate::shared::input_handler::{Input, InputHandler};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct EditProductCommand {
    pub product_id: u32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub stock: Option<u32>,
    pub price: Option<f64>,
    pub img_url: Option<String>,
    pub flags: Option<Vec<String>>,
}
impl Input for EditProductCommand {
    type Output = ();
}
pub struct EditProductCommandHandler {
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<EditProductCommand> for EditProductCommandHandler {
    async fn handle(
        &self,
        input: Arc<EditProductCommand>,
    ) -> Result<<EditProductCommand as Input>::Output, SharedError> {
        let mut product = match self.product_repository.get_by_id(input.product_id).await {
            Some(prod) => prod,
            None => {
                return Err(SharedError::new(
                    format!("Product with ID {} not found", input.product_id).as_str(),
                    404,
                ));
            }
        };
        if let Some(url) = &input.img_url {
            let url = Url::new(url.clone())?;
            product.set_img_url(Some(url));
        }
        if let Some(name) = &input.name {
            product.set_name(name.clone());
        }
        if let Some(stock) = input.stock {
            product.set_stock(stock);
        }
        if let Some(description) = &input.description {
            product.set_description(Some(description.clone()));
        }
        if let Some(price) = input.price {
            product.set_price(price);
        }
        if let Some(flags) = &input.flags {
            product.set_flags(flags.iter().map(|s| s.to_string()).collect());
        }
        self.product_repository.save(&product).await
    }
}
