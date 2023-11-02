use crate::common::dto::ErrorDto;
use crate::common::response::res_bad_request;
use crate::common::response::res_ok;
use actix_web::{get, Responder, Result};
use std::fs;

/**
 * GET <base_url>/error-test/panic
 */
#[get("/error-test/panic")]
async fn rate_panic() -> Result<impl Responder> {
    //this file does not exists! panic!
    fs::read_to_string("FAKE.md").expect("Test panic error");
    // never here..
    return Ok(res_ok(""));
}

/**
 * GET <base_url>/error-test/raised
 */
#[get("/error-test/raised")]
async fn rate_error() -> Result<impl Responder> {
    //this file does not exists! propagate error
    fs::read_to_string("FAKE.md")?;
    //never here...
    return Ok(res_ok(""));
}

/**
 * GET <base_url>/error-test/managed
 */
#[get("/error-test/managed")]
async fn rate_error_managed() -> Result<impl Responder> {
    return Ok(res_bad_request(ErrorDto {
        message: "This is an error managed".to_owned(),
    }));
}
