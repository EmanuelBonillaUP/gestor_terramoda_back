use std::sync::Arc;

use actix_web::{HttpResponse, Responder, route, web};
use serde::Deserialize;

use crate::{
    application::queries::GetCustomerByCcQuery, infrastructure::Mediator,
    presentation::common::CustomerResponse, shared::input_handler::Sender,
};

#[derive(Debug, Deserialize)]
struct GetCustomerByCcRequest {
    cc: String,
}
#[route("/", method = "GET")]
pub async fn get_customer_by_cc(
    query: web::Query<GetCustomerByCcRequest>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let get_customer_query = GetCustomerByCcQuery {
        cc: query.cc.clone(),
    };
    let result = mediator.send(get_customer_query).await;
    match result {
        Ok(o) => {
            let data: CustomerResponse = CustomerResponse::from(o);
            HttpResponse::Ok().json(data)
        }
        Err(err) => HttpResponse::from(err),
    }
}
