use actix_web::web;

/* error routes */
pub mod error_test;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(error_test::rate_error);
    cfg.service(error_test::rate_error_managed);
    cfg.service(error_test::rate_panic);
}
