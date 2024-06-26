use std::env;

use common_lib::provider::redis_sync::{R2D2Pool, RedisProviderSync};

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
pub fn init_redis() -> R2D2Pool {
    let addr = match env::var("REDIS_ADDR") {
        Ok(v) => v,
        Err(_) => "redis://localhost:6379".to_owned(),
    };

    return RedisProviderSync::new(addr, 5, 10)
        .expect("Unrecoverable error in RedisProviderSync::new");
}
