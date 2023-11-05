use actix_web::web;

use self::error_test::ERROR_TEST_SCOPE;

/* error routes */
pub mod error_test;

pub fn config(cfg: &mut web::ServiceConfig) {
    // The follwing routes are scope with /error-test
    // <base_url>/error-test/<route>
    cfg.service(web::scope(ERROR_TEST_SCOPE).configure(|cfg| {
        cfg.service(error_test::rate_error);
        cfg.service(error_test::rate_error_managed);
        cfg.service(error_test::rate_panic);
    }));
}
