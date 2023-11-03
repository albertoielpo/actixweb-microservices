use actix_web::web;

pub mod auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::login);
}
