use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use common_lib::provider::redis_sync::R2D2Pool;
use log::info;
use websocket::actor::websocket::RateWebSocket;
use websocket::config::main_config::{init_logger, init_redis, init_server_bind};

/**
 * Listen ws:// stream
 */
async fn websocket_route_listener(
    pool: web::Data<R2D2Pool>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(RateWebSocket::new(pool), &req, stream);
    info!("{:?}", resp);
    resp
}

/**
 * Startup main
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    init_logger();
    let server_bind = init_server_bind();
    info!(
        "Starting websocket in main thread {} {} version {}",
        server_bind.addr, server_bind.port, VERSION
    );

    // init redis sync
    let pool_redis = init_redis();

    //Start http server and register websocket to route "/"
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool_redis.clone()))
            .route("/", web::get().to(websocket_route_listener))
    })
    .bind((server_bind.addr, server_bind.port))?
    .run()
    .await
}
