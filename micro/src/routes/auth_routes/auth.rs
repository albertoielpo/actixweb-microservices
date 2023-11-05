use crate::common::{dto::LoginDto, response::res_ok};
use actix_web::{route, web, Responder, Result};

// form: web::Form<FormData> for application/x-www-form-urlencoded

/**
 * POST <base_url>/auth/login
 */
#[route("/auth/login", method = "POST")]
async fn login(info: web::Json<LoginDto>) -> Result<impl Responder> {
    // just remap
    let another_login_dto = LoginDto {
        username: info.username.to_string(),
        password: info.password.to_string(),
    };
    return Ok(res_ok(another_login_dto));
}
