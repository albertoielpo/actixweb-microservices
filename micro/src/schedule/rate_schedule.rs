use common_lib::provider::{
    redis::{BB8Pool, RedisProvider},
    redis_keys::CURRENT_RATE_VALUE,
};
use log::info;
use std::{env, time::Duration};
use tokio::time::sleep;

use crate::routes::rate_routes::rate_service::generate_rate;

pub async fn schedule_rate(pool: BB8Pool) {
    let rate_string = env::var("UPDATE_RATE_INTERVAL").unwrap_or("10".to_owned());
    let update_rate_interval = rate_string.parse::<u64>().unwrap_or(10);
    info!(
        "Starting schedule rate job with update rate interval {} seconds",
        update_rate_interval
    );

    loop {
        // generate rate
        let rate_dto = generate_rate();
        // update data to redis...
        let _ = RedisProvider::set(&pool, CURRENT_RATE_VALUE.to_owned(), rate_dto.rate, None).await;
        sleep(Duration::from_secs(update_rate_interval)).await;
    }
}
