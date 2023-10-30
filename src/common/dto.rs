use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct RateDto {
    pub rate: String,
}
