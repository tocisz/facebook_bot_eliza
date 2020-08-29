use lambda_http::{handler, lambda, Context, IntoResponse, Request, Response, Body, RequestExt}; // RequestExt,
use serde_json::json;
use lambda_http::http::StatusCode;
use http::Method;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

static VERIFY_TOKEN : &str = "YOUR_VERIFY_TOKEN";

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(route)).await?;
    Ok(())
}

async fn route(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    match req.uri().path() {
        "/" => handle_index(req).await,
        "/webhook" => handle_webhook(req).await,
        _ => handle_404(req).await
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
        _ => handle_404(req).await
    }
}

async fn handle_webhook_get(req: Request) -> Result<Response<Body>, Error> {
    let params = req.query_string_parameters();
    let verify_token = params.get("hub.verify_token");
    let challenge = params.get("hub.challenge");
    let mode = params.get("hub.mode");
    if verify_token.is_some() && challenge.is_some() && mode.is_some() {
        let verify_token = verify_token.unwrap();
        let challenge = challenge.unwrap();
        let mode = mode.unwrap();

        if mode == "subscribe" && verify_token == VERIFY_TOKEN {
            return Ok(challenge.into_response())
        }
    }
    let mut resp = "Failed to verify token!".into_response();
    *resp.status_mut() = StatusCode::FORBIDDEN;
    Ok(resp)
}

async fn handle_webhook_post(req: Request) -> Result<Response<Body>, Error> {
    Ok("POST".into_response())
}

async fn handle_404(_: Request) -> Result<Response<Body>, Error> {
    let mut response = "Not found!".into_response();
    *response.status_mut() = StatusCode::NOT_FOUND;
    Ok(response)
}