use std::env;

use common_lib::provider::redis::RedisProvider;

pub struct ServerBind {
    pub addr: String,
    pub port: u16,
}
/**
 * Init logger with env variable
 */
pub fn init_logger() {
    /* init logging library */
    let rust_log: Result<String, env::VarError> = env::var("RUST_LOG");
    if rust_log.is_err() || rust_log.unwrap().is_empty() {
        env::set_var("RUST_LOG", "debug"); //set debug level if not set
    }
    env_logger::init();
}

/**
 * Init server bind with env variables
 */
pub fn init_server_bind() -> ServerBind {
    /* init server bind */
    let addr = match env::var("BIND_ADDR") {
        Ok(v) => v,
        Err(_) => String::from("0.0.0.0"),
    };
    let default_port = 3001;
    let port = match env::var("BIND_PORT") {
        Ok(v) => v.parse::<u16>().unwrap_or(default_port),
        Err(_) => default_port,
    };

    return ServerBind { addr, port };
}

/**
 * Init redis
 */
pub async fn init_redis() {
    let addr = match env::var("REDIS_ADDR") {
        Ok(v) => v,
        Err(_) => "redis://localhost:6379".to_owned(),
    };
    let default_max_size: u32 = 5;
    let pool_max_size = match env::var("REDIS_POOL_MAX_SIZE") {
        Ok(v) => v.parse::<u32>().unwrap_or(default_max_size),
        Err(_) => default_max_size,
    };

    RedisProvider::new(addr, pool_max_size)
        .await
        .expect("Unrecoverable error in RedisProvider::new");
}
