use std::sync::Arc;

use actix_web::{HttpResponse, Responder, route, web};

use crate::{
    application::queries::GetSaleByIdQuery, infrastructure::Mediator,
    presentation::common::SaleResponse, shared::input_handler::Sender,
};

#[route("/{id}", method = "GET")]
pub async fn get_sale_by_id(
    id: web::Path<u32>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let get_sale_query = GetSaleByIdQuery {
        sale_id: id.into_inner(),
    };
    let result = mediator.send(get_sale_query).await;
    match result {
        Ok(o) => {
            let data: SaleResponse = SaleResponse::from(o);
            HttpResponse::Ok().json(data)
        }
        Err(err) => HttpResponse::from(err),
    }
}
