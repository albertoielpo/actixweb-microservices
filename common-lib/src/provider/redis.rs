use std::time::Duration;

use bb8_redis::{
    bb8::{self, RunError},
    RedisConnectionManager,
};

use redis::RedisError;

pub struct RedisProvider {}
pub type BB8Pool = bb8::Pool<RedisConnectionManager>;
// https://github.com/sankaku/sample_actix_with_redis/tree/main

impl RedisProvider {
    pub async fn new(
        connection_string: String,
        max_size: u32,
        connection_timeout: u64,
    ) -> Result<BB8Pool, bb8_redis::redis::RedisError> {
        //init the connection pool
        let manager = RedisConnectionManager::new(connection_string)?;
        let pool = bb8::Pool::builder()
            .max_size(max_size)
            .connection_timeout(Duration::from_secs(connection_timeout))
            .build(manager)
            .await?;
        Ok(pool)
    }

    pub async fn set(
        pool: &BB8Pool,
        key: String,
        val: String,
        ttl: Option<u32>,
    ) -> Result<String, RunError<RedisError>> {
        let mut conn = pool.get().await?;

        match ttl {
            Some(ttl) => {
                let data: String = redis::cmd("SETEX")
                    .arg(key)
                    .arg(ttl)
                    .arg(val)
                    .query_async(&mut *conn)
                    .await?;

                return Ok(data);
            }
            None => {
                let data: String = redis::cmd("SET")
                    .arg(key)
                    .arg(val)
                    .query_async(&mut *conn)
                    .await?;

                return Ok(data);
            }
        }
    }

    pub async fn get(pool: &BB8Pool, key: String) -> Result<String, RunError<RedisError>> {
        let mut conn = pool.get().await?;
        let data: String = redis::cmd("GET").arg(key).query_async(&mut *conn).await?;
        Ok(data)
    }
}
