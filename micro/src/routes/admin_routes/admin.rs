use crate::{
    common::{dto::AdminDto, response::res_ok},
    config::error_handler::AppError,
};
use actix_web::{route, Responder, Result};
use common_lib::{provider::redis::RedisProvider, utils::date::unix_timestamp};
use log::error;

pub const ADMIN_SCOPE: &str = "/admin";

/**
 * GET <base_url>/admin
 */
#[route("", method = "GET", method = "HEAD")]
async fn get_data() -> Result<impl Responder, AppError> {
    let rnd = unix_timestamp();
    let from_redis = RedisProvider::set("key".to_owned(), rnd.to_string(), None).await;

    if from_redis.is_err() {
        error!("{}", from_redis.err().unwrap());
        return Err(AppError::e500("Redis error".to_owned()));
    }

    let from_redis = RedisProvider::get("key".to_owned()).await;
    match from_redis {
        Ok(res) => Ok(res_ok(AdminDto { data: res })),
        Err(err) => {
            error!("{}", err);
            return Err(AppError::e500("Redis error".to_owned()));
        }
    }
}
