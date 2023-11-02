use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::info;
use websocket::actor::websocket::RateWebSocket;
use websocket::config::main_config::{init_logger, init_server_bind};

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(RateWebSocket::new(), &req, stream);
    info!("{:?}", resp);
    resp
}

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
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind((server_bind.addr, server_bind.port))?
        .run()
        .await
}
