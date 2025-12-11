use std::sync::Arc;

use actix_web::{
    HttpResponse, Responder, route,
    web::{self, ServiceConfig},
};
use serde::{Deserialize, Serialize};

use crate::{
    application::queries::LoginQuery, infrastructure::Mediator, shared::input_handler::Sender,
};

#[derive(Deserialize)]
struct LoginRequest {
    pub user: String,
    pub pass: String,
}
#[derive(Serialize)]
struct LoginResponse {
    pub key: String,
}

#[route("/login", method = "POST")]
async fn login(
    request: web::Json<LoginRequest>,
    mediator: web::Data<Arc<Mediator>>,
) -> impl Responder {
    let query = LoginQuery {
        user: request.user.to_string(),
        pass: request.pass.to_string(),
    };
    let output = mediator.send(query).await;
    if let Err(e) = output {
        HttpResponse::from(e)
    } else {
        HttpResponse::Ok().json(LoginResponse {
            key: output.unwrap().key,
        })
    }
}

pub fn cfg(cfg: &mut ServiceConfig) {
    cfg.service(login);
}
