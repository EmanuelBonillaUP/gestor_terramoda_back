mod application;
mod domain;
mod infrastructure;
mod presentation;
mod shared;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infrastructure::init().await;
    HttpServer::new(|| {
        App::new()
            .configure(infrastructure::configure)
            .configure(presentation::configure)
    })
    .bind("localhost:8000")?
    .run()
    .await
}
