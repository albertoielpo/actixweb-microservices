use crate::{
    common::{dto::AdminDto, response::res_ok},
    config::error_handler::AppError,
};
use actix_web::{route, Responder, Result};
use common_lib::provider::redis::RedisProvider;
use log::error;

pub const ADMIN_SCOPE: &str = "/admin";

/**
 * GET <base_url>/admin
 */
#[route("", method = "GET", method = "HEAD")]
async fn get_data() -> Result<impl Responder, AppError> {
    let fetch_from_redis = RedisProvider::get("key".to_owned()).await;
    match fetch_from_redis {
        Ok(res) => Ok(res_ok(AdminDto { data: res })),
        Err(err) => {
            error!("{}", err);
            return Err(AppError::e500("Redis error".to_owned()));
        }
    }
}
