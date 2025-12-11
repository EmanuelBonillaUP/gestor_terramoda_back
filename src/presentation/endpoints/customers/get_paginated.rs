use actix_web::{HttpResponse, Responder, route, web};
use std::sync::Arc;

use crate::{
    application::queries::GetCustomersQuery,
    infrastructure::Mediator,
    presentation::common::{CustomerResponse, Pagination, PaginationResult},
    shared::input_handler::Sender,
};

#[route("", method = "GET")]
pub async fn get_paginated_customers(
    pagination: web::Query<Pagination>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let query = GetCustomersQuery {
        pagination: pagination.into_inner().into(),
    };
    let result = mediator.send(query).await;
    match result {
        Ok(o) => {
            let data: PaginationResult<CustomerResponse> =
                PaginationResult::from(o.pagination_result);
            HttpResponse::Ok().json(data)
        }
        Err(err) => HttpResponse::from(err),
    }
}
