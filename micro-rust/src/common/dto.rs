use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct RateDto {
    pub rate: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorDto {
    pub message: String,
}
