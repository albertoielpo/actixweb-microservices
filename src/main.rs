use actix_cors::Cors;
use actix_http::StatusCode;
use actix_web::{
    dev::ServiceResponse,
    http::{self},
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger},
    App, HttpServer, Result,
};
use actix_web_lab::middleware::CatchPanic;
use log::{debug, error, info};
use micro_rust::controller::rate_controller;

use std::env;

struct ServerBind {
    addr: String,
    port: u16,
}

fn init_logger() {
    /* init logging library */
    let rust_log: Result<String, env::VarError> = env::var("RUST_LOG");
    debug!("RUST_LOG {:?}", rust_log);
    if rust_log.is_err() || rust_log.unwrap().is_empty() {
        env::set_var("RUST_LOG", "debug"); //set debug level if not set
    }
    env_logger::init();
}

fn init_server_bind() -> ServerBind {
    /* init server bind */
    let addr = match env::var("BIND_ADDR") {
        Ok(v) => v,
        Err(_) => String::from("0.0.0.0"),
    };
    let default_port = 3000;
    let port = match env::var("BIND_PORT") {
        Ok(v) => v.parse::<u16>().unwrap_or(default_port),
        Err(_) => default_port,
    };

    return ServerBind { addr, port };
}

fn add_error_body<B: std::fmt::Debug>(
    mut res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("application/json"),
    );

    //TODO: change it properly
    let status_mut = res.response_mut().status_mut();
    *status_mut = StatusCode::INTERNAL_SERVER_ERROR;

    let (req, res) = res.into_parts();

    error!("Response body: {:?}", res.body());

    let res = match res.error() {
        Some(err) => {
            let body_str = format!("{{\"message\": \"{}\"}}", err);
            res.set_body(body_str)
        }
        None => {
            let body_str = format!("{{\"message\": \"Generic Error\"}}");
            res.set_body(body_str)
        }
    };

    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let server_bind = init_server_bind();
    info!(
        "Starting webserver in main thread {} {}",
        server_bind.addr, server_bind.port
    );

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(
                ErrorHandlers::new()
                    .default_handler_client(add_error_body)
                    .default_handler_server(add_error_body),
            )
            .wrap(CatchPanic::default()) // <- after everything except logger
            .wrap(Logger::default())
            .configure(rate_controller::config)
    })
    .bind((server_bind.addr, server_bind.port))?
    .run()
    .await
}
