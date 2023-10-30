use actix_web::web;

/* rate routes */
pub mod rate;
pub mod rate_service;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(rate::rate);
    cfg.service(rate::timestamp);
    cfg.service(rate::rate_error);
    cfg.service(rate::rate_error_managed);
    cfg.service(rate::rate_panic);
}
