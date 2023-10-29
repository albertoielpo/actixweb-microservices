use actix_web::{get, http::StatusCode, web::Json, CustomizeResponder, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct RateDto {
    rate: f32,
}

/** TODO:  */
fn response_ok<T: serde::Serialize>(other_rate: T) -> CustomizeResponder<Json<T>> {
    return Json(other_rate).customize().with_status(StatusCode::OK);
}

#[get("/rate")]
async fn rate() -> Result<impl Responder> {
    let rate = RateDto {
        rate: fastrand::f32() + 1.00,
    };
    return Ok(response_ok(rate));
}
