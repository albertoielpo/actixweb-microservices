use actix_cors::Cors;
use actix_web::{
    middleware::{ErrorHandlers, Logger},
    App, HttpServer,
};
use actix_web_lab::middleware::CatchPanic;
use log::info;
use micro::{
    config::error_handler::add_error_body,
    config::main_config::{init_logger, init_server_bind},
    routes::{auth_routes, error_test_routes, rate_routes},
};

/**
 * Startup main
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    init_logger();
    let server_bind = init_server_bind();

    info!(
        "Starting micro in main thread {} {} version {}",
        server_bind.addr, server_bind.port, VERSION
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
            .configure(rate_routes::config)
            .configure(auth_routes::config)
            .configure(error_test_routes::config) //<- test routes.. demonstration purpouses
    })
    .bind((server_bind.addr, server_bind.port))?
    .run()
    .await
}
