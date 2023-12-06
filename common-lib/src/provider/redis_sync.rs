use std::sync::Mutex;

use lazy_static::lazy_static;
use redis::{Client, Commands};

use crate::error::common_error::{CommonError, CommonErrorMessage};

lazy_static! {
    static ref REDIS_SYNC_CONN: Mutex<(Option<Client>, bool)> = Mutex::new((None, true));
}
/// Redis Provider Sync using single connection
pub struct RedisProviderSync {}

impl RedisProviderSync {
    pub fn new(connection_string: String) -> Result<(), bb8_redis::redis::RedisError> {
        //init single connection
        let client: redis::Client = redis::Client::open(connection_string)?;
        let mut value = REDIS_SYNC_CONN.lock().unwrap();
        value.0 = Some(client);
        return Ok(());
    }

    pub fn get(key: String) -> Result<String, CommonError> {
        let value = REDIS_SYNC_CONN.lock().unwrap();
        if value.0.is_none() {
            return Err(CommonError {
                message: CommonErrorMessage::RedisConnectionError,
            });
        }

        let client = value.0.clone().unwrap();
        match client.get_connection() {
            Ok(mut conn) => {
                let res: String = conn.get(key).unwrap();
                return Ok(res);
            }
            Err(_) => {
                return Err(CommonError {
                    message: CommonErrorMessage::RedisConnectionError,
                });
            }
        }
    }
}
