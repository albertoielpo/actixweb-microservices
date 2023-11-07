use crate::common::jwt::verify;

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};

pub fn check_bearer(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let token = credentials.token();

    let verify_res = verify(token);
    match verify_res {
        Ok(_) => Ok(req),
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13"); //TODO: modify this..

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
