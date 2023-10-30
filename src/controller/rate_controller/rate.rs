use crate::common::response::res_bad_request;
use crate::{common::response::res_ok, controller::rate_controller::rate_service::get_rate};
use actix_web::{get, Responder, Result};
use actix_web_lab::sse;
use log::error;
use serde_json::json;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

#[get("/rate")]
async fn rate() -> Result<impl Responder> {
    return Ok(res_ok(get_rate()));
}

#[get("/rate-panic")]
async fn rate_panic() -> Result<impl Responder> {
    fs::read_to_string("FAKE.md").expect("Should have been able to read the file");
    // never here..
    return Ok(res_ok(get_rate()));
}

#[get("/rate-error")]
async fn rate_error() -> Result<impl Responder> {
    fs::read_to_string("FAKE.md")?;
    return Ok(res_ok(get_rate()));
}

#[get("/rate-error-managed")]
async fn rate_error_managed() -> Result<impl Responder> {
    return Ok(res_bad_request(get_rate()));
}

#[get("/sse")]
async fn timestamp() -> impl Responder {
    let (sender, receiver) = tokio::sync::mpsc::channel(2);

    actix_web::rt::spawn(async move {
        let mut id: i32 = 1;
        loop {
            // TODO: refactor
            let a = json!(get_rate());
            let msg = sse::Data::new(a.to_string())
                .event("message")
                .id(id.to_string());

            if sender.send(msg.into()).await.is_err() {
                error!("Client disconnected! Could not send SSE message");
                break;
            }
            id += 1;

            sleep(Duration::from_secs(10)).await;
        }
    });

    sse::Sse::from_infallible_receiver(receiver).with_keep_alive(Duration::from_secs(3))
}
