use crate::{common::redis::fetch_async_an_integer, config::error_handler::AppError};
use actix_web::{route, Responder, Result};

pub const ADMIN_SCOPE: &str = "/admin";

/**
 * GET <base_url>/admin
 */
#[route("", method = "GET", method = "HEAD")]
async fn get_data() -> Result<impl Responder, AppError> {
    let fetch_from_redis = fetch_async_an_integer().await;
    match fetch_from_redis {
        Ok(res) => Ok(res),
        Err(err) => Err(AppError::e500(format!("Redis error {:?}", err))),
    }
    // info!("info: {:?}", aaa);
    // match int_from_redis {
    //     Ok(res) => Ok(res_ok(AdminDto {
    //         data: res.to_string(),
    //     })),
    //     Err(err) => Err(AppError::e500(format!("Redis error {:?}", err))),
    // }
}
