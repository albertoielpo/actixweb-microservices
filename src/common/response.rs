use actix_web::{http::StatusCode, web::Json, CustomizeResponder, Responder};
use serde::Serialize;

pub fn res_ok<T: serde::Serialize>(payload: T) -> CustomizeResponder<Json<T>> {
    return Json(payload).customize().with_status(StatusCode::OK);
}

pub fn res_bad_request<T: serde::Serialize>(payload: T) -> CustomizeResponder<Json<T>> {
    return Json(payload)
        .customize()
        .with_status(StatusCode::BAD_REQUEST);
}

pub fn res_internal_server_error<T: serde::Serialize>(payload: T) -> CustomizeResponder<Json<T>> {
    return Json(payload)
        .customize()
        .with_status(StatusCode::INTERNAL_SERVER_ERROR);
}

pub fn res_status_code<T: serde::Serialize>(
    payload: T,
    status_code: StatusCode,
) -> CustomizeResponder<Json<T>> {
    return Json(payload).customize().with_status(status_code);
}

// TODO: move this...
#[derive(Serialize)]
pub struct RateDto {
    pub rate: f32,
}
