use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::{debug, info};
use std::time::Duration;
use websocket_rust::config::main_config::{init_logger, init_server_bind};

/// How often messages are sent
const DISPATCH_INTERVAL: Duration = Duration::from_secs(5);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct RateWebSocket {
    //Spawn handle used to run interval
    sh: Option<SpawnHandle>,
}

// TODO: refactor into mod

impl RateWebSocket {
    pub fn new() -> Self {
        Self { sh: None }
    }

    /// send message with interval
    fn handle_interval_messages(&mut self, ctx: &mut <Self as Actor>::Context, stop: bool) {
        if stop {
            if self.sh.is_some() {
                ctx.cancel_future(self.sh.unwrap());
                debug!("interval cancelled");
            }
        } else {
            let sh: SpawnHandle = ctx.run_interval(DISPATCH_INTERVAL, |_act, ctx| {
                let rate = fastrand::f32() + 1.00;
                let x = format!("{{\"rate\":\"{}\"}}", rate);
                ctx.text(x);
                debug!("message sent");
            });
            self.sh = Some(sh);
        }
    }
}

impl Actor for RateWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("WS connected");
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RateWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                info!("ping received {:?}.. send pong", msg);
                ctx.pong(&msg)
            }
            Ok(ws::Message::Text(text)) => {
                //let text_str = print!("{}", text);
                info!("received text {}", text);
                let payload = serde_json::from_str::<serde_json::Value>(&text).unwrap();
                let event = &payload["event"];
                info!("{}", event);
                if event.eq("cmd") {
                    let data_type = &payload["data"]["type"];
                    if data_type.eq("start") {
                        info!("this is a start");
                        self.handle_interval_messages(ctx, false);
                        let rate = fastrand::f32() + 1.00;
                        let x = format!("{{\"rate\":\"{}\"}}", rate);
                        ctx.text(x);
                    } else if data_type.eq("stop") {
                        info!("this is a stop");
                        self.handle_interval_messages(ctx, true);
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                info!("received bytes");
                ctx.binary(bin)
            }
            _ => (),
        }
    }
}

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
