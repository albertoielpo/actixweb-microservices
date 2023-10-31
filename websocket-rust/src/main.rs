use actix_web::{HttpServer, App};
use websocket_rust::config::main_config::{init_logger, init_server_bind};
use log::info;

/**
 * Startup main
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let server_bind = init_server_bind();
    info!(
        "Starting websocket in main thread {} {}",
        server_bind.addr, server_bind.port
    );
    HttpServer::new(|| {App::new()})
    .bind((server_bind.addr, server_bind.port))?
    .run()
    .await
}