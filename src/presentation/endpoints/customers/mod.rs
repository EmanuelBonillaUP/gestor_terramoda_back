use actix_web::web::ServiceConfig;

mod edit;
mod get_by_cc;
mod get_paginated;
mod register;

pub fn cfg(cfg: &mut ServiceConfig) {
    cfg.service(register::register_customer);
    cfg.service(get_paginated::get_paginated_customers);
    cfg.service(edit::edit_customer);
    cfg.service(get_by_cc::get_customer_by_cc);
}
