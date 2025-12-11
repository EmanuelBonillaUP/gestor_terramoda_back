use crate::{domain::value_objects::ValueObject, shared::{input_handler::{Input, InputHandler}, Pagination, PaginationResult, SharedError}};
use std::sync::Arc;

use crate::domain::{entities::Sale, repositories::SaleRepository};

pub struct GenerateCsvReportQuery;
pub struct GenerateCsvReportOutput {
    pub csv_data: String,
}
impl Input for GenerateCsvReportQuery {
    type Output = GenerateCsvReportOutput;
}

pub struct GenerateCsvReportQueryHandler {
    pub sale_repository: Arc<dyn SaleRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<GenerateCsvReportQuery> for GenerateCsvReportQueryHandler {
    async fn handle(
        &self,
        _: Arc<GenerateCsvReportQuery>,
    ) -> Result<<GenerateCsvReportQuery as Input>::Output, SharedError> {
        let mut csv_data = String::from("Sale ID, Generated At, Customer CC, Products(Product SKU:Quantity), Total Amount\n");
        let sales = self.sale_repository.get_all().await;
        for sale in sales {
            let products_str = sale.products_sale().iter()
                .map(|(p, quantity)| format!("{}:{}", p.sku(), quantity))
                .collect::<Vec<String>>()
                .join("&");
            let line = format!(
                "{},{},{},{},{}\n",
                sale.id(),
                sale.generated_at(),
                sale.customer().cc().value(),
                products_str,
                sale.total_amount()
            );
            csv_data.push_str(&line);
        }
        Ok(GenerateCsvReportOutput { csv_data } )
    }
}
