use std::sync::Arc;

use actix_web::{HttpResponse, Responder, route, web};
use serde::Deserialize;

use crate::{
    application::commands::EditCustomerCommand, infrastructure::Mediator,
    shared::input_handler::Sender,
};

#[derive(Debug, Deserialize)]
struct EditCustomerRequest {
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    direction: Option<String>,
}

#[route("/{id}", method = "PUT")]
pub async fn edit_customer(
    id: web::Path<u32>,
    data: web::Json<EditCustomerRequest>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let command = EditCustomerCommand {
        customer_id: id.into_inner(),
        name: data.name.clone(),
        email: data.email.clone(),
        phone: data.phone.clone(),
        direction: data.direction.clone(),
    };
    let result = mediator.send(command).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::from(err),
    }
}
