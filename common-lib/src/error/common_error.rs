use std::fmt::Debug;

#[derive(Debug)]
pub enum CommonErrorMessage {
    RedisConnectionError,
    RedisPoolError,
}

#[derive(Debug)]
pub struct CommonError {
    pub message: CommonErrorMessage,
}
