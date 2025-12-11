use std::sync::Arc;

use actix_web::{HttpResponse, Responder, route, web};
use serde::{Deserialize, Serialize};

use crate::{
    application::commands::EditProductCommand, infrastructure::Mediator,
    shared::input_handler::Sender,
};

#[derive(Debug, Deserialize)]
struct EditProductRequest {
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    stock: Option<u32>,
    img_url: Option<String>,
    flags: Option<Vec<String>>,
}

#[route("/{id}", method = "PUT")]
pub async fn edit_product(
    id: web::Path<u32>,
    data: web::Json<EditProductRequest>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let command = EditProductCommand {
        product_id: id.into_inner(),
        name: data.name.clone(),
        description: data.description.clone(),
        price: data.price,
        stock: data.stock,
        img_url: data.img_url.clone(),
        flags: data.flags.clone(),
    };
    let result = mediator.send(command).await;
    if let Err(err) = result {
        return HttpResponse::from(err);
    } else {
        HttpResponse::Ok().finish()
    }
}
