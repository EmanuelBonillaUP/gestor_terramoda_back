use std::sync::Arc;

use actix_web::{HttpResponse, Responder, route, web};
use serde::{Deserialize, Serialize};

use crate::{
    application::commands::RegisterCustomerCommand, infrastructure::Mediator,
    shared::input_handler::Sender,
};

#[derive(Debug, Deserialize)]
struct RegisterCustomerRequest {
    cc: String,
    name: String,
    email: String,
    phone: Option<String>,
    direction: Option<String>,
}

#[derive(Debug, Serialize)]
struct RegisterCustomerResponse {
    customer_id: u32,
}

#[route("", method = "POST")]
pub async fn register_customer(
    data: web::Json<RegisterCustomerRequest>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let command = RegisterCustomerCommand {
        cc: data.cc.clone(),
        name: data.name.clone(),
        email: data.email.clone(),
        phone: data.phone.clone(),
        direction: data.direction.clone(),
    };
    let result = mediator.send(command).await;
    match result {
        Ok(output) => HttpResponse::Created().json(RegisterCustomerResponse {
            customer_id: output.id,
        }),
        Err(err) => HttpResponse::from(err),
    }
}
