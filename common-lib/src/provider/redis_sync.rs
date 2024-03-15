use crate::error::common_error::{CommonError, CommonErrorMessage};
use r2d2_redis::{r2d2, redis::Commands, RedisConnectionManager};
use std::time::Duration;

/// Redis r2d2 sync pool manager
pub type R2D2Pool = r2d2::Pool<RedisConnectionManager>;
pub struct RedisProviderSync {}

impl RedisProviderSync {
    pub fn new(
        connection_string: String,
        max_size: u32,
        connection_timeout: u64,
    ) -> Result<R2D2Pool, CommonError> {
        let manager = RedisConnectionManager::new(connection_string);
        if manager.is_err() {
            return Err(CommonError {
                message: CommonErrorMessage::RedisPoolError,
            });
        }
        let manager = manager.unwrap();
        let res: Result<R2D2Pool, r2d2::Error> = r2d2::Pool::builder()
            .max_size(max_size)
            .connection_timeout(Duration::from_secs(connection_timeout))
            .build(manager);
        match res {
            Ok(res) => Ok(res),
            Err(_) => Err(CommonError {
                message: CommonErrorMessage::RedisPoolError,
            }),
        }
    }

    pub fn get(pool: &R2D2Pool, key: String) -> Result<String, CommonError> {
        let conn = pool.get();
        if conn.is_err() {
            return Err(CommonError {
                message: CommonErrorMessage::RedisConnectionError,
            });
        }

        let mut conn = conn.unwrap();
        let data = conn.get(key);
        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(CommonError {
                message: CommonErrorMessage::RedisConnectionError,
            }),
        }
    }
}
