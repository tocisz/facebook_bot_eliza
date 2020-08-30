mod config;
mod messages;

#[macro_use]
extern crate lazy_static; // used by mod config
#[macro_use]
extern crate log;
use log::Level;

use config::CONFIG;
use messages::Entries;

use env_logger;
use http::Method;
use lambda_http::http::StatusCode;
use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, RequestExt, Response};
use std::ops::Deref;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Config is {}", *CONFIG);
    lambda::run(handler(route)).await?;
    Ok(())
}

async fn route(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    debug!("Request is {} {}", req.method(), req.uri().path());
    match req.uri().path() {
        "/" => handle_index(req).await,
        "/webhook" => handle_webhook(req).await,
        _ => handle_404(req).await,
    }
}

async fn handle_index(_: Request) -> Result<Response<Body>, Error> {
    let response = "Welcome! Please use /webhook.".into_response();
    Ok(response)
}

async fn handle_webhook(req: Request) -> Result<Response<Body>, Error> {
    match *req.method() {
        Method::GET => handle_webhook_get(req).await,
        Method::POST => handle_webhook_post(req).await,
        _ => handle_404(req).await,
    }
}

async fn handle_webhook_get(req: Request) -> Result<Response<Body>, Error> {
    let params = req.query_string_parameters();
    if log_enabled!(Level::Debug) {
        debug!("Request params:");
        for (k, v) in params.iter() {
            debug!(" * {}={}", k, v);
        }
    }
    let verify_token = params.get("hub.verify_token");
    let challenge = params.get("hub.challenge");
    let mode = params.get("hub.mode");
    if verify_token.is_some() && challenge.is_some() && mode.is_some() {
        let verify_token = verify_token.unwrap();
        let challenge = challenge.unwrap();
        let mode = mode.unwrap();

        if mode == "subscribe" && verify_token == CONFIG.verify_token {
            info!("Returning challenge.");
            return Ok(challenge.into_response());
        }
    }
    error!("Verification failed!");
    let mut resp = "Failed to verify token!".into_response();
    *resp.status_mut() = StatusCode::FORBIDDEN;
    Ok(resp)
}

async fn handle_webhook_post(req: Request) -> Result<Response<Body>, Error> {
    let params = req.query_string_parameters();
    if log_enabled!(Level::Debug) {
        debug!("Request params:");
        for (k, v) in params.iter() {
            debug!(" * {}={}", k, v);
        }
    }
    let body_array = req.body().deref();
    let body_string = String::from_utf8_lossy(body_array);
    debug!("Body: {}", body_string);
    let entries: Result<Entries, serde_json::error::Error> = serde_json::from_str(&body_string);
    match entries {
        Ok(entries) => {
            for e in &entries.entry {
                for ems in &e.messaging {
                    let sender = ems.sender.id.as_str();
                    let message = ems.message.text.as_str();
                    messages::send_response(sender, message).await;
                }
            }
            info!("Got {:?}", entries);
        }
        Err(e) => {
            error!("Deserialization failed {:?}", e);
        }
    }
    Ok(().into_response())
}

async fn handle_404(_: Request) -> Result<Response<Body>, Error> {
    let mut response = "Not found!".into_response();
    *response.status_mut() = StatusCode::NOT_FOUND;
    Ok(response)
}
