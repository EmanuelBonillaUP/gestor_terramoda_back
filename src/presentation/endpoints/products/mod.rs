use actix_web::web::ServiceConfig;

mod edit;
mod get_paginated;
mod register;
mod get_by_sku;

pub fn cfg(cfg: &mut ServiceConfig) {
    cfg.service(register::register_product);
    cfg.service(get_paginated::get_paginated_products);
    cfg.service(edit::edit_product);
    cfg.service(get_by_sku::get_product_by_sku);
}
