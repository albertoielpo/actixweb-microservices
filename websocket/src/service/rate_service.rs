use common_lib::provider::redis::{RedisProvider, CURRENT_RATE_VALUE};

pub async fn get_rate() -> String {
    let from_redis = RedisProvider::get(CURRENT_RATE_VALUE.to_owned()).await;
    return match from_redis {
        Ok(rate) => rate,
        Err(_) => "0.00".to_owned(),
    };
}
