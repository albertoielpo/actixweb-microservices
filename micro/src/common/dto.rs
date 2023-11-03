use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct RateDto {
    pub rate: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}
