use crate::common::response::res_bad_request;
use crate::{common::response::res_ok, controller::rate_controller::rate_service::get_rate};
use actix_web::{get, Responder, Result};
use actix_web_lab::sse;
use log::info;
use serde_json::json;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

#[get("/rate")]
async fn rate() -> Result<impl Responder> {
    return Ok(res_ok(get_rate()));
}

#[get("/sse")]
async fn timestamp() -> impl Responder {
    let (sender, receiver) = tokio::sync::mpsc::channel(2);

    actix_web::rt::spawn(async move {
        let mut id: i32 = 1;
        loop {
            let rate_value = json!(get_rate());
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

// test errors...

#[get("/rate-panic")]
async fn rate_panic() -> Result<impl Responder> {
    //this file does not exists! panic!
    fs::read_to_string("FAKE.md").expect("Test panic error");
    // never here..
    return Ok(res_ok(get_rate()));
}

#[get("/rate-error")]
async fn rate_error() -> Result<impl Responder> {
    //this file does not exists! propagate error
    fs::read_to_string("FAKE.md")?;
    //never here...
    return Ok(res_ok(get_rate()));
}

#[get("/rate-error-managed")]
async fn rate_error_managed() -> Result<impl Responder> {
    return Ok(res_bad_request(get_rate()));
}
