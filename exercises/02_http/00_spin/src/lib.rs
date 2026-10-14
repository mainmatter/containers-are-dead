use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_service;

#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!(
        "Handling request to {:?}",
        req.headers().get("spin-full-url")
    );

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello World!".to_string())?)
}
