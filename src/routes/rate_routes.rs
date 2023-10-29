use actix_web::web;

/* rate routes */
pub mod rate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(rate::rate);
}
