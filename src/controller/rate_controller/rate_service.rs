use crate::common::response::RateDto;

//TODO: read data from common place...
pub fn get_rate() -> RateDto {
    return RateDto {
        rate: fastrand::f32() + 1.00,
    };
}
