use crate::{
    common::{
        dto::{LoginDto, TokenDto},
        jwt::sign,
        response::res_ok,
    },
    config::error_handler::AppError,
};
use actix_web::{route, web, Responder, Result};
use log::error;

// form: web::Form<FormData> for application/x-www-form-urlencoded

/**
 * POST <base_url>/auth/login
 */
#[route("/auth/login", method = "POST")]
async fn login(info: web::Json<LoginDto>) -> Result<impl Responder, AppError> {
    // here you can rely to a credential storage (env, database, ... )
    if info.username == "admin" && info.password == "password" {
        let token = sign(&info.username);
        match token {
            Ok(token) => {
                return Ok(res_ok(TokenDto { token }));
            }
            Err(e) => {
                error!("{}", e);
                return Err(AppError::e401("Unauthorized".to_owned()));
            }
        }
    }

    Err(AppError::e401("Unauthorized".to_owned()))
}
