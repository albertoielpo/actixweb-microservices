use crate::{common::response::res_ok, config::error_handler::AppError};
use actix_web::{route, Responder, Result};
use log::debug;

/**
 * GET <base_url>/admin
 */
#[route("", method = "GET", method = "HEAD")]
async fn get_data() -> Result<impl Responder, AppError> {
    debug!("Code here it's protected by a guard...");
    return Ok(res_ok("It works"));
}
