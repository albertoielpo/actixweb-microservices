use std::sync::Mutex;

use bb8_redis::{
    bb8::{self, Pool, RunError},
    RedisConnectionManager,
};
use lazy_static::lazy_static;
use log::info;
use redis::RedisError;

lazy_static! {
    static ref REDIS_POOL: Mutex<(Option<Pool<RedisConnectionManager>>, bool)> =
        Mutex::new((None, true));
}

pub struct RedisProvider {}

impl RedisProvider {
    pub async fn new(connection_string: String, max_size: u32) -> Result<(), RedisError> {
        //init the connection pool
        let mut value = REDIS_POOL.lock().unwrap();
        if value.0.is_some() {
            // check if there are some value...
            return Ok(());
        }

        let manager = RedisConnectionManager::new(connection_string)?;
        let pool = bb8::Pool::builder()
            .max_size(max_size)
            .build(manager)
            .await?;
        let clone: Pool<RedisConnectionManager> = pool.clone();
        value.0 = Some(clone);

        return Ok(());
    }

    pub async fn set(key: String, val: String) -> Result<String, RunError<RedisError>> {
        //TODO: wrap in a function
        let value = REDIS_POOL.lock().unwrap();
        let pool = value.0.clone().unwrap();
        let mut conn = pool.get().await?;

        let data: String = redis::cmd("SET")
            .arg(key)
            //.arg(REDIS_DATE_OF_EXPIRY)
            .arg(val)
            .query_async(&mut *conn)
            .await?;
        Ok(data)
    }

    pub async fn get(key: String) -> Result<String, RunError<RedisError>> {
        //TODO: wrap in a function
        let value = REDIS_POOL.lock().unwrap();
        let pool = value.0.clone().unwrap();
        let mut conn = pool.get().await?;

        let data: String = redis::cmd("GET").arg(key).query_async(&mut *conn).await?;
        Ok(data)
    }
}
pub async fn fetch_async_string() -> Result<String, RunError<RedisError>> {
    let res_1 = RedisProvider::set("key".to_owned(), "val".to_owned()).await?;
    info!("{:?}", res_1);
    let res_2 = RedisProvider::get("key".to_owned()).await;
    return res_2;
}
