use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use micro_rust::controller::rate_controller;
use std::env;
use std::result::Result;

use log::{debug, info};

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
            .wrap(Logger::default())
            .configure(rate_controller::config)
    })
    .bind((server_bind.addr, server_bind.port))?
    .run()
    .await
}
