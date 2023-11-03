use actix_web::{http::StatusCode, web::Json, CustomizeResponder, Responder};

/**
 * JSON response OK
 */
pub fn res_ok<T: serde::Serialize>(payload: T) -> CustomizeResponder<Json<T>> {
    return Json(payload).customize().with_status(StatusCode::OK);
}

/**
 * JSON response CREATED
 */
pub fn res_created<T: serde::Serialize>(payload: T) -> CustomizeResponder<Json<T>> {
    return Json(payload).customize().with_status(StatusCode::CREATED);
}

/**
 * JSON response with custom http status code
 */
pub fn res_status_code<T: serde::Serialize>(
    payload: T,
    status_code: StatusCode,
) -> CustomizeResponder<Json<T>> {
    return Json(payload).customize().with_status(status_code);
}
