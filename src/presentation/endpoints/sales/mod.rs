use actix_web::web::ServiceConfig;

mod get_paginated;
mod register;
mod get_by_id;

pub fn cfg(cfg: &mut ServiceConfig) {
    cfg.service(register::register_sale);
    cfg.service(get_paginated::get_sales_paginated);
    cfg.service(get_by_id::get_sale_by_id);
}
