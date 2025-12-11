use std::sync::Arc;

use actix_web::{
    HttpResponse, Responder, route,
    web::{self, ServiceConfig},
};

use crate::{
    application::queries::GenerateCsvReportQuery, infrastructure::Mediator,
    shared::input_handler::Sender,
};

#[route("/csv", method = "GET")]
async fn generate_csv_report(mediator: web::Data<Arc<Mediator>>) -> impl Responder {
    let query = GenerateCsvReportQuery {};
    let output = mediator.send(query).await;
    if let Err(err) = output {
        return HttpResponse::from(err);
    } else {
        let output = output.unwrap();
        HttpResponse::Ok()
            .content_type("text/csv")
            .insert_header(("Content-Disposition", "attachment; filename=\"report.csv\""))
            .body(output.csv_data)
    }
}

pub fn cfg(cfg: &mut ServiceConfig) {
    cfg.service(generate_csv_report);
}
