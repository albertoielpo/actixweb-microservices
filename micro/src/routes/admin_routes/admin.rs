use crate::{
    common::{dto::AdminDto, response::res_ok},
    config::error_handler::AppError,
};
use actix_web::{route, web, Responder, Result};
use common_lib::{
    provider::{
        redis::{BB8Pool, RedisProvider},
        redis_keys::KEY_EXAMPLE,
    },
    utils::date::unix_timestamp,
};
use log::error;

pub const ADMIN_SCOPE: &str = "/admin";

/**
 * GET <base_url>/admin
 */
#[route("", method = "GET", method = "HEAD")]
async fn get_data(pool: web::Data<BB8Pool>) -> Result<impl Responder, AppError> {
    let rnd = unix_timestamp();
    let from_redis = RedisProvider::set(&pool, KEY_EXAMPLE.to_owned(), rnd.to_string(), None).await;

    if from_redis.is_err() {
        error!("{}", from_redis.err().unwrap());
        return Err(AppError::e500("Redis error".to_owned()));
    }

    let from_redis = RedisProvider::get(&pool, KEY_EXAMPLE.to_owned()).await;
    match from_redis {
        Ok(res) => Ok(res_ok(AdminDto { data: res })),
        Err(err) => {
            error!("{}", err);
            return Err(AppError::e500("Redis error".to_owned()));
        }
    }
}
