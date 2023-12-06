use common_lib::provider::{redis::RedisProvider, redis_keys::CURRENT_RATE_VALUE};
use log::{debug, info};
use std::time::Duration;
use tokio::time::sleep;

use crate::routes::rate_routes::rate_service::generate_rate;

pub async fn schedule_rate() {
    info!("Starting schedule rate job");
    loop {
        // generate rate
        let rate_dto = generate_rate();
        // update data to redis...
        debug!("calling redis....");
        let _ = RedisProvider::set(CURRENT_RATE_VALUE.to_owned(), rate_dto.rate, None).await;
        sleep(Duration::from_secs(10)).await;
    }
}
