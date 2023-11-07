use crate::config::bearer_handler::check_bearer;

use self::admin::ADMIN_SCOPE;

use actix_web::{dev::ServiceRequest, web};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
pub mod admin;

pub fn config(cfg: &mut web::ServiceConfig) {
    //bearer check closure...
    let auth =
        HttpAuthentication::bearer(|req: ServiceRequest, credentials: BearerAuth| async move {
            check_bearer(req, credentials)
        });
    //then use as scoped-wrap
    cfg.service(web::scope(ADMIN_SCOPE).wrap(auth).service(admin::get_data));
}
