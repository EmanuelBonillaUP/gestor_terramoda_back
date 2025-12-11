use actix_web::{
    HttpResponse, Responder, route,
    web::{self, ServiceConfig},
};

#[route("/status", method = "GET")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

pub fn cfg(cfg: &mut ServiceConfig) {
    cfg.service(status);
}
