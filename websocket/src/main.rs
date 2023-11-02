use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::info;
use websocket::actor::websocket::RateWebSocket;
use websocket::config::main_config::{init_logger, init_server_bind};

/**
 * Listen ws:// stream
 */
async fn websocket_route_listener(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
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
    //Start http server and register websocket to route "/"
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(websocket_route_listener))
    })
    .bind((server_bind.addr, server_bind.port))?
    .run()
    .await
}
