use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_service;

#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!(
        "Handling request to {:?}",
        req.headers().get("spin-full-url")
    );

    // TODO return a correct response
    // see the docs for details: https://docs.rs/spin-sdk/latest/spin_sdk/http/index.html
}
