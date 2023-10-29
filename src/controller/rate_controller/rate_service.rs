use crate::common::response::RateDto;

// TODO: sync this rate every 60 seconds
pub fn get_rate() -> RateDto {
    return RateDto {
        rate: fastrand::f32() + 1.00,
    };
}
