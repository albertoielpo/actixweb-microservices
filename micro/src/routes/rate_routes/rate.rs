use crate::{common::response::res_ok, routes::rate_routes::rate_service::get_rate};
use actix_web::{route, web, Responder, Result};
use actix_web_lab::sse;
use common_lib::provider::redis::BB8Pool;
use log::info;
use serde_json::json;

use std::time::Duration;
use tokio::time::sleep;

/**
 * GET <base_url>/rate
 */
#[route("/rate", method = "GET", method = "HEAD")]
async fn rate(pool: web::Data<BB8Pool>) -> Result<impl Responder> {
    return Ok(res_ok(get_rate(pool).await));
}

/**
 * GET <base_url>/sse
 * Do not allow HEAD, it's a stream
 */
#[route("/sse", method = "GET")]
async fn sse_event_stream(pool: web::Data<BB8Pool>) -> impl Responder {
    let (sender, receiver) = tokio::sync::mpsc::channel(2);

    actix_web::rt::spawn(async move {
        let mut id: i32 = 1;
        loop {
            let rate_value = json!(get_rate(pool.clone()).await);
            let msg = sse::Data::new(rate_value.to_string())
                .event("message")
                .id(id.to_string());

            if sender.send(msg.into()).await.is_err() {
                info!("Client disconnected! Could not send SSE message");
                break;
            }
            id += 1;

            //send sse payload every 10 seconds
            sleep(Duration::from_secs(10)).await;
        }
    });

    //send keep alive every 3 seconds
    sse::Sse::from_infallible_receiver(receiver).with_keep_alive(Duration::from_secs(3))
}
