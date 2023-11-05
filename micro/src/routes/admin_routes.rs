use actix_web::{guard, web};

use crate::common::jwt::verify;

pub mod admin;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin") //TODO: change it
            .guard(guard::fn_guard(verify))
            .service(admin::get_data),
    );
}
