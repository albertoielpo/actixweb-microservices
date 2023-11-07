use crate::{
    common::{dto::AdminDto, response::res_ok},
    config::error_handler::AppError,
};
use actix_web::{route, Responder, Result};

pub const ADMIN_SCOPE: &str = "/admin";

/**
 * GET <base_url>/admin
 */
#[route("", method = "GET", method = "HEAD")]
async fn get_data() -> Result<impl Responder, AppError> {
    return Ok(res_ok(AdminDto {
        data: "Welcome to admin data".to_owned(),
    }));
}
