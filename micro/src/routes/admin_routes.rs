use actix_web::{guard, web};

use crate::common::jwt::verify;

use self::admin::ADMIN_SCOPE;

pub mod admin;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(ADMIN_SCOPE)
            .guard(guard::fn_guard(verify))
            .service(admin::get_data),
    );
}
