use crate::common::response::res_ok;
use crate::config::error_handler::AppError;
use actix_web::{
    route,
    web::{self},
    Responder, Result,
};
use std::fs;

/**
 * GET <base_url>/error-test/panic
 */
#[route("/error-test/panic", method = "GET", method = "HEAD")]
async fn rate_panic() -> Result<impl Responder> {
    //this file does not exists! panic!
    fs::read_to_string("FAKE.md").expect("Test panic error");
    // never here..
    return Ok(res_ok(""));
}

/**
 * GET <base_url>/error-test/raised
 */
#[route("/error-test/raised", method = "GET", method = "HEAD")]
async fn rate_error() -> Result<impl Responder> {
    //this file does not exists! propagate error
    fs::read_to_string("FAKE.md")?;
    //never here...
    return Ok(res_ok(""));
}

/**
 * GET <base_url>/error-test/managed/1
 */
#[route("/error-test/managed/{id}", method = "GET", method = "HEAD")]
async fn rate_error_managed(path: web::Path<u32>) -> Result<impl Responder, AppError> {
    let path = path.into_inner();
    match path {
        500 => {
            return Err(AppError::e500("AppError::e500".to_owned()));
        }
        400 => {
            return Err(AppError::e400("AppError::e400".to_owned()));
        }
        401 => {
            return Err(AppError::e401("AppError::e401".to_owned()));
        }
        403 => {
            return Err(AppError::e403("AppError::e403".to_owned()));
        }
        418 => {
            return Err(AppError::e418("AppError::e418".to_owned()));
        }
        _ => {
            return Ok(res_ok(""));
        }
    }
}
