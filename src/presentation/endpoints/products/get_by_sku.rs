use std::sync::Arc;

use actix_web::{HttpResponse, Responder, route, web};
use serde::Deserialize;

use crate::{
    application::queries::GetProductBySkuQuery, infrastructure::Mediator,
    presentation::common::ProductResponse, shared::input_handler::Sender,
};

#[derive(Debug, Deserialize)]
struct GetProductBySkuRequest {
    sku: String,
}
#[route("/", method = "GET")]
pub async fn get_product_by_sku(
    query: web::Query<GetProductBySkuRequest>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let get_product_query = GetProductBySkuQuery {
        sku: query.sku.clone(),
    };
    let result = mediator.send(get_product_query).await;
    match result {
        Ok(o) => {
            let data: ProductResponse = ProductResponse::from(o);
            HttpResponse::Ok().json(data)
        }
        Err(err) => HttpResponse::from(err),
    }
}
