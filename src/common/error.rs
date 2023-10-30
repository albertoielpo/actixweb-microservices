use actix_http::StatusCode;
use actix_web::{
    dev::ServiceResponse,
    http::{self},
    middleware::ErrorHandlerResponse,
    Result,
};
use log::{error, info};

pub fn add_error_body<B: std::fmt::Debug>(
    mut res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    // all response must be application/json
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("application/json"),
    );

    let status_mut = res.response_mut().status_mut();
    info!("Http status {}", status_mut);

    if *status_mut == StatusCode::NOT_FOUND {
        // Always set +500 http code...
        *status_mut = StatusCode::INTERNAL_SERVER_ERROR;

        let (req, mut res) = res.into_parts();
        error!("Response body: {:?}", res.body());

        let res = match res.error() {
            Some(err) => {
                // I have an error .. it means is raised with ? in the flow...
                let body_str = format!("{{\"message\": \"{}\"}}", err);
                res.set_body(body_str)
            }
            None => {
                // This is a really not found error
                let sm = res.status_mut();
                *sm = StatusCode::NOT_FOUND;
                let body_str = format!("{{\"message\": \"Not found\"}}");
                res.set_body(body_str)
            }
        };

        // modified bodies need to be boxed and placed in the "right" slot
        let res = ServiceResponse::new(req, res)
            .map_into_boxed_body()
            .map_into_right_body();

        Ok(ErrorHandlerResponse::Response(res))
    } else {
        // this is a managed error, with managed http code and body
        Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
    }
}
