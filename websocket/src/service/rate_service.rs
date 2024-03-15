use actix_web::web;
use common_lib::provider::{
    redis_keys::CURRENT_RATE_VALUE,
    redis_sync::{R2D2Pool, RedisProviderSync},
};

pub fn get_rate(pool: web::Data<R2D2Pool>) -> String {
    let from_redis = RedisProviderSync::get(&pool, CURRENT_RATE_VALUE.to_owned());
    return match from_redis {
        Ok(rate) => rate,
        Err(_) => "0.00".to_owned(),
    };
}
