use std::sync::Arc;

use actix_web::{HttpResponse, Responder, route, web};
use serde::{Deserialize, Serialize};

use crate::{
    application::commands::RegisterProductCommand, infrastructure::Mediator,
    shared::input_handler::Sender,
};

#[derive(Debug, Deserialize)]
struct RegisterProductRequest {
    sku: String,
    name: String,
    description: Option<String>,
    stock: Option<u32>,
    price: f64,
    img_url: Option<String>,
    flags: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RegisterProductResponse {
    product_id: u32,
}

#[route("", method = "POST")]
pub async fn register_product(
    data: web::Json<RegisterProductRequest>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let command = RegisterProductCommand {
        sku: data.sku.clone(),
        name: data.name.clone(),
        description: data.description.clone(),
        stock: data.stock,
        price: data.price,
        img_url: data.img_url.clone(),
        flags: data.flags.clone(),
    };
    let result = mediator.send(command).await;
    match result {
        Ok(output) => HttpResponse::Created().json(RegisterProductResponse {
            product_id: output.product_id,
        }),
        Err(err) => HttpResponse::from(err),
    }
}
