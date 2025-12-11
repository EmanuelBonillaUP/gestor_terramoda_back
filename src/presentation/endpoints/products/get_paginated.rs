use actix_web::{HttpResponse, Responder, route, web};
use std::sync::Arc;

use crate::{
    application::queries::GetProductsQuery,
    infrastructure::Mediator,
    presentation::common::{Pagination, PaginationResult, ProductResponse},
    shared::input_handler::Sender,
};

#[route("", method = "GET")]
pub async fn get_paginated_products(
    pagination: web::Query<Pagination>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let query = GetProductsQuery {
        pagination: pagination.into_inner().into(),
    };
    let result = mediator.send(query).await;
    match result {
        Ok(o) => {
            let data: PaginationResult<ProductResponse> =
                PaginationResult::from(o.pagination_result);
            HttpResponse::Ok().json(data)
        }
        Err(e) => HttpResponse::from(e),
    }
}
