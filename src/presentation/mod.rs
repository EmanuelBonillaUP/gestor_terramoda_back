use actix_web::middleware::from_fn;
use actix_web::web::scope;

mod common;
mod endpoints;
mod middlewares;

use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
            .wrap(from_fn(middlewares::loggin_middleware))
            .service(scope("/api").configure(endpoints::api::cfg))
            .configure(endpoints::auth::cfg)
            .service(
                scope("/sales")
                    .wrap(from_fn(middlewares::auth_middleware))
                    .configure(endpoints::sales::cfg),
            )
            .service(
                scope("/customers")
                    .wrap(from_fn(middlewares::auth_middleware))
                    .configure(endpoints::customers::cfg),
            )
            .service(
                scope("/products")
                    .wrap(from_fn(middlewares::auth_middleware))
                    .configure(endpoints::products::cfg),
            )
            .service(
                scope("/reports")
                    .wrap(from_fn(middlewares::auth_middleware))
                    .configure(endpoints::reports::cfg),
            ),
    );
}
