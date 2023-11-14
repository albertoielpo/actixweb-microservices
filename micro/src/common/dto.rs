use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct RateDto {
    pub rate: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginDto {
    pub us: String, //username
    pub pa: String, //password
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenDto {
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AdminDto {
    pub data: String,
}
