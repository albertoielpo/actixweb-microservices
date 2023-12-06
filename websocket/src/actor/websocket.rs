use actix::prelude::{AsyncContext, SpawnHandle};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws::{self, WebsocketContext};
use log::{debug, info};
use std::time::Duration;

use crate::service::rate_service::get_rate;

/// How often messages are sent
const DISPATCH_INTERVAL: Duration = Duration::from_secs(5);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct RateWebSocket {
    //Spawn handle used to run interval
    sh: Option<SpawnHandle>,
}

impl RateWebSocket {
    pub fn new() -> Self {
        Self { sh: None }
    }

    fn do_send_message(ctx: &mut WebsocketContext<RateWebSocket>) {
        let rate = get_rate();
        let x = format!("{{\"rate\":\"{}\"}}", rate);
        debug!("message: {}", x);
        ctx.text(x);
    }

    /// send message with interval
    fn handle_interval_messages(&mut self, ctx: &mut <Self as Actor>::Context, stop: bool) {
        if stop {
            if self.sh.is_some() {
                ctx.cancel_future(self.sh.unwrap());
                debug!("interval cancelled");
            }
        } else {
            let sh: SpawnHandle = ctx.run_interval(
                DISPATCH_INTERVAL,
                |_act: &mut RateWebSocket, ctx: &mut WebsocketContext<RateWebSocket>| {
                    Self::do_send_message(ctx);
                },
            );
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
                info!("received text {}", text);
                let payload = serde_json::from_str::<serde_json::Value>(&text).unwrap();
                let event = &payload["event"];
                info!("{}", event);
                if event.eq("cmd") {
                    let data_type = &payload["data"]["type"];
                    if data_type.eq("start") {
                        info!("this is a start");
                        self.handle_interval_messages(ctx, false);
                        Self::do_send_message(ctx);
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
            Ok(ws::Message::Close(reason)) => {
                if reason.is_some() {
                    info!("connection close: {:?}", reason.unwrap());
                }
                if self.sh.is_some() {
                    ctx.cancel_future(self.sh.unwrap());
                    debug!("interval cancelled");
                }
            }
            _ => (),
        }
    }
}
