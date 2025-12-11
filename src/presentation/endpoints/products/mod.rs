use actix_web::web::ServiceConfig;

mod edit;
mod get_paginated;
mod register;

pub fn cfg(cfg: &mut ServiceConfig) {
    cfg.service(register::register_product);
    cfg.service(get_paginated::get_paginated_products);
    cfg.service(edit::edit_product);
}
