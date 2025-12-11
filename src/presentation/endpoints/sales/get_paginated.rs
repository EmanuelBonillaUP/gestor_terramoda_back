use actix_web::{HttpResponse, Responder, route, web};
use std::sync::Arc;

use crate::{
    application::queries::GetSalesQuery,
    infrastructure::Mediator,
    presentation::common::{Pagination, PaginationResult, SaleResponse},
    shared::input_handler::Sender,
};

#[route("", method = "GET")]
pub async fn get_sales_paginated(
    pagination_req: web::Query<Pagination>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let query = GetSalesQuery {
        pagination: pagination_req.into_inner().into(),
    };
    let result = mediator.send(query).await;
    match result {
        Ok(o) => {
            let data: PaginationResult<SaleResponse> = PaginationResult::from(o.pagination_result);
            HttpResponse::Ok().json(data)
        }
        Err(err) => HttpResponse::from(err),
    }
}
