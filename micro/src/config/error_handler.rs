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
use serde_json::Value;

#[derive(Debug, Display, Error, new)]
pub enum InternalError {
    JwtDateExpired,
}

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

    let (req, mut res) = res.into_parts();
    let res = match res.error() {
        Some(err) => {
            // I have an error .. it means is raised with ? in the flow...
            error!("Status: {} - Body: {:?}", res.status(), res.body());
            let json = format!("{}", err.to_string());
            let json_check: Result<Value, serde_json::Error> = serde_json::from_str(&json);
            match json_check {
                Ok(_) => {
                    // this is a json error .. then return
                    res.set_body(json)
                }
                Err(_) => {
                    // this is not a json error .. then wrap and return
                    let d = format!("{}", AppError::e500(err.to_string()));
                    res.set_body(d)
                }
            }
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
