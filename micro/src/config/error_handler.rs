use std::fmt::Debug;

use actix_http::StatusCode;
use actix_web::{
    dev::ServiceResponse,
    http::{self},
    middleware::ErrorHandlerResponse,
    HttpResponse, Result,
};
use derive_more::{Display, Error};
use derive_new::new;
use log::error;
#[derive(Debug, Display, Error, new)]
#[display(fmt = "{{\"message\": \"{}\"}}", message)]
pub struct AppError {
    pub message: String,
    pub code: StatusCode,
}

/**
 * Use AppError::new to raise a specific http code
 */
impl AppError {
    pub fn e500(message: String) -> AppError {
        AppError::new(message, StatusCode::INTERNAL_SERVER_ERROR)
    }
    pub fn e400(message: String) -> AppError {
        AppError::new(message, StatusCode::BAD_REQUEST)
    }
    pub fn e401(message: String) -> AppError {
        AppError::new(message, StatusCode::UNAUTHORIZED)
    }
    pub fn e403(message: String) -> AppError {
        AppError::new(message, StatusCode::FORBIDDEN)
    }
    pub fn e418(message: String) -> AppError {
        AppError::new(message, StatusCode::IM_A_TEAPOT)
    }
}

/**
 * ResponseError implementation for AppError
 */
impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let body = self.to_string();
        let status = self.status_code();
        error!("Status: {} - Body: {:?}", status, body);
        HttpResponse::build(status).body(body)
    }
    fn status_code(&self) -> StatusCode {
        self.code
    }
}

/**
 * Error handler to be wrapped in main
 */
pub fn add_error_body<B: std::fmt::Debug>(
    mut res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    // all response must be application/json
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("application/json"),
    );

    let status_mut = res.response_mut().status_mut();
    if *status_mut != StatusCode::NOT_FOUND {
        // This is a managed error raised by ResponseError
        return Ok(ErrorHandlerResponse::Response(res.map_into_left_body()));
    }

    // If is not a managed error then set +500 http status code...
    *status_mut = StatusCode::INTERNAL_SERVER_ERROR;

    let (req, mut res) = res.into_parts();

    let res = match res.error() {
        Some(err) => {
            // I have an error .. it means is raised with ? in the flow...
            error!("Status: 500 - Body: {:?}", res.body());
            let d = format!("{}", AppError::e500(err.to_string()));
            res.set_body(d)
        }
        None => {
            // This is a really not found error
            error!("Status: 404 - Resource not found");
            let sm = res.status_mut();
            *sm = StatusCode::NOT_FOUND;
            let d = format!("{}", AppError::e400("Not found".to_owned()));
            res.set_body(d)
        }
    };

    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
