use std::env;

use actix_cors::Cors;
use actix_web::{
    middleware::{ErrorHandlers, Logger},
    App, HttpServer,
};
use actix_web_lab::middleware::CatchPanic;
use log::info;
use micro::{
    config::error_handler::add_error_body,
    config::main_config::{init_logger, init_redis, init_server_bind},
    routes::{admin_routes, auth_routes, error_test_routes, rate_routes},
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

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

    // init redis connection pool with init lazy mode
    // it does not panic if redis is down
    // it does panic only in case of bug in init phase
    init_redis().await;

    // server configuration:  wrap (middleware), configure (routes)
    let server = HttpServer::new(|| {
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
            .configure(admin_routes::config)
            .configure(error_test_routes::config) //<- test routes.. demonstration purpouses
    });

    //http or https
    let https_enabled: Result<String, env::VarError> = env::var("HTTPS_ENABLED");
    let https_enabled = https_enabled.unwrap_or("false".to_owned());

    if https_enabled == "true" {
        info!("HTTPS mode");
        // Generate test certificate with: `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
            .expect("SslMethod tls builder init failed");
        builder
            .set_private_key_file("resources/key.pem", SslFiletype::PEM)
            .expect("key.pem certificate is expected inside resources/key.pem folder");
        builder
            .set_certificate_chain_file("resources/cert.pem")
            .expect("cert.pem certificate is expected inside resources/cert.pem folder");

        // start up in https (http/2 mode)
        server
            .bind_openssl(
                format!("{}:{}", server_bind.addr, server_bind.port),
                builder,
            )?
            .run()
            .await
    } else {
        info!("HTTP mode");

        // start up in http (http/1.1 mode)
        server
            .bind((server_bind.addr, server_bind.port))?
            .run()
            .await
    }
}
