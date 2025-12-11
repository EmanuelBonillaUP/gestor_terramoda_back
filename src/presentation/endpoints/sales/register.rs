use std::sync::Arc;

use actix_web::{HttpResponse, Responder, route, web};
use serde::{Deserialize, Serialize};

use crate::{
    application::commands::RegisterSaleCommand, infrastructure::Mediator,
    shared::input_handler::Sender,
};

#[derive(Debug, Deserialize, Clone)]
struct ProductSkuQuantity {
    sku: String,
    quantity: u32,
}

#[derive(Debug, Deserialize)]
struct RegisterSaleRequest {
    customer_cc: String,
    product_skus_quantity: Vec<ProductSkuQuantity>,
}

#[derive(Debug, Serialize)]
struct RegisterSaleResponse {
    sale_id: u32,
    total_amount: f64,
}

#[route("", method = "POST")]
pub async fn register_sale(
    request: web::Json<RegisterSaleRequest>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let command = RegisterSaleCommand {
        customer_cc: request.customer_cc.clone(),
        product_skus_quantity: request
            .product_skus_quantity
            .iter()
            .map(|psq| (psq.sku.clone(), psq.quantity))
            .collect(),
    };
    let output = mediator.send(command).await;
    if let Err(err) = output {
        return HttpResponse::from(err);
    } else {
        let output = output.unwrap();
        HttpResponse::Created().json(RegisterSaleResponse {
            sale_id: output.sale_id,
            total_amount: output.total_amount,
        })
    }
}
