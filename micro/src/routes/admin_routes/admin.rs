use crate::{
    common::{dto::AdminDto, redis::fetch_async_string, response::res_ok},
    config::error_handler::AppError,
};
use actix_web::{route, Responder, Result};

pub const ADMIN_SCOPE: &str = "/admin";

/**
 * GET <base_url>/admin
 */
#[route("", method = "GET", method = "HEAD")]
async fn get_data() -> Result<impl Responder, AppError> {
    let fetch_from_redis = fetch_async_string().await;
    match fetch_from_redis {
        Ok(res) => Ok(res_ok(AdminDto { data: res })),
        Err(err) => Err(AppError::e500(format!("Redis error {:?}", err))),
    }
}
