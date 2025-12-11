use crate::domain::repositories::{CustomerRepository, ProductRepository, SaleRepository};
use crate::domain::value_objects::CC;
use crate::shared::SharedError;
use crate::shared::input_handler::{Input, InputHandler};
use std::sync::Arc;

pub struct RegisterSaleCommand {
    pub customer_cc: String,
    pub product_skus_quantity: Vec<(String, u32)>,
}
pub struct RegisterSaleOutput {
    pub sale_id: u32,
    pub total_amount: f64,
}
impl Input for RegisterSaleCommand {
    type Output = RegisterSaleOutput;
}

pub struct RegisterSaleCommandHandler {
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
    pub sale_repository: Arc<dyn SaleRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<RegisterSaleCommand> for RegisterSaleCommandHandler {
    async fn handle(
        &self,
        input: Arc<RegisterSaleCommand>,
    ) -> Result<<RegisterSaleCommand as Input>::Output, SharedError> {
        // Fetch customer by CC
        let customer_cc = CC::new(input.customer_cc.clone())?;
        let customer = match self.customer_repository.get_by_cc(&customer_cc).await {
            Some(cust) => cust,
            None => {
                return Err(SharedError::new(
                    format!("Customer with {} CC not found", input.customer_cc).as_str(),
                    404,
                ));
            }
        };
        // Prepare products and quantities
        let products = self
            .product_repository
            .get_many_by_skus(
                input
                    .product_skus_quantity
                    .iter()
                    .map(|(sku, _)| sku.as_str())
                    .collect(),
            )
            .await;
        if products.len() != input.product_skus_quantity.len() {
            return Err(SharedError::new("One or more products not found", 404));
        }
        let mut products_sale = Vec::new();
        for (sku, quantity) in &input.product_skus_quantity {
            if let Some(product) = products.iter().find(|p| *p.sku() == *sku) {
                let mut product = product.clone();
                products_sale.push((product.clone(), *quantity as u32));
                product.set_stock(product.stock() - *quantity);
                self.product_repository.save(&product).await?;
            } else {
                return Err(SharedError::new(
                    format!("Product with SKU {} not found", sku).as_str(),
                    404,
                ));
            }
        }
        let sale = self
            .sale_repository
            .create(
                &customer,
                products_sale.iter().map(|(p, q)| (p, *q)).collect(),
            )
            .await?;
        Ok(RegisterSaleOutput {
            sale_id: sale.id(),
            total_amount: sale.total_amount(),
        })
    }
}
