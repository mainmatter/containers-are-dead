use serde::Deserialize;
use serde_querystring::ParseMode;
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{EmptyBody, IntoResponse, Method, Request, Response};
use spin_sdk::http_service;

/// The endpoint should accept the following query parameters
#[derive(Deserialize)]
struct Params {
    /// The smallest random number allowed, defaults to 0
    min: Option<usize>,
    /// The largest random number allowed, defaults to 1000
    max: Option<usize>,
}

#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    let params: Params =
        serde_querystring::from_str(req.uri().query().unwrap_or(""), ParseMode::UrlEncoded)?;

    let api = spin_sdk::variables::get("random_api").await?;
    let req = Request::builder()
        .method(Method::GET)
        .uri(format!(
            "{api}/api/v1.0/random?min={}&max={}",
            params.min.unwrap_or(0),
            params.max.unwrap_or(1000)
        ))
        .body(EmptyBody::new())?;

    let res: Response = spin_sdk::http::send(req).await?;
    let body = res.into_body().bytes().await?;

    // the API answers with a JSON array containing a single number, e.g. `[40]`
    let numbers: Vec<f64> = serde_json::from_slice(&body)?;
    let number = numbers
        .first()
        .ok_or_else(|| anyhow::anyhow!("empty response"))?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(number.to_string())?)
}
