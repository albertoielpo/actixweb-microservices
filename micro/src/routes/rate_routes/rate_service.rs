use std::sync::Mutex;

use common_lib::{
    provider::{redis::RedisProvider, redis_keys::CURRENT_RATE_VALUE},
    utils::date::unix_timestamp,
};
use lazy_static::lazy_static;
use log::debug;

use crate::common::dto::RateDto;

lazy_static! {
    /**
    * This simulate a shared storage
    * It's used for didactical purposes and this practise
    * should not to be used in production environment
    * @see get_rate_old for the usage
    */
    static ref RATE_TUPLA: Mutex<(u128, RateDto)> = {
        Mutex::new((1000, RateDto {rate: String::from("0.000")}))
    };
}

/**
 * not used anymore
 */
pub fn get_rate_old() -> RateDto {
    let mut value = RATE_TUPLA.lock().unwrap();
    let ts = unix_timestamp() / 10000; //1 rate every 10 seconds
    let tsv = value.0 / 10000;
    if ts > tsv {
        debug!("Timeout.. generate another rate");
        value.0 = unix_timestamp();
        value.1 = generate_rate();
    }
    return value.1.clone();
}

pub async fn get_rate() -> RateDto {
    let from_redis = RedisProvider::get(CURRENT_RATE_VALUE.to_owned()).await;
    return match from_redis {
        Ok(rate) => RateDto { rate },
        Err(_) => RateDto {
            rate: "0.00".to_owned(),
        },
    };
}

pub fn generate_rate() -> RateDto {
    let rate = fastrand::f32() + 1.00;
    let rate = rate.to_string();
    if rate.len() >= 5 {
        RateDto {
            rate: String::from(&rate[0..5]),
        }
    } else {
        RateDto {
            rate: String::from(&rate[0..rate.len()]),
        }
    }
}
