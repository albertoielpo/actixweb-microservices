use common_lib::provider::{redis_keys::CURRENT_RATE_VALUE, redis_sync::RedisProviderSync};

pub fn get_rate() -> String {
    let from_redis = RedisProviderSync::get(CURRENT_RATE_VALUE.to_owned());
    return match from_redis {
        Ok(rate) => rate,
        Err(_) => "0.00".to_owned(),
    };
}
