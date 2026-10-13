use serde::Deserialize;
use spin_sdk::http::{Request, Response};
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
pub async fn handler(req: Request) -> anyhow::Result<Response<String>> {
    // Hint: use the `serde_querystring` crate that is already added as a dependency to parse the `req.uri().query()` string!
    // see that crates docs for details!

    // TODO: read the base URL of the API from the `random_api` variable (see the book), request
    // "<base url>/api/v1.0/random?min={}&max={}" and return a single number as a result.
    // The API answers with a JSON array containing one number, e.g. `[40]`. `serde_json` is a dependency.

    todo!()
}
