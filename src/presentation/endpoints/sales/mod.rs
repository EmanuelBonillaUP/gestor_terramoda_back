use actix_web::web::ServiceConfig;

mod get_paginated;
mod register;

pub fn cfg(cfg: &mut ServiceConfig) {
    cfg.service(register::register_sale);
    cfg.service(get_paginated::get_sales_paginated);
}
