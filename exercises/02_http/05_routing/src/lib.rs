use axum::Router;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::delete;
use serde::Deserialize;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_service;
use tower_service::Service;

// curl --request DELETE http://localhost:3000/clear/a?session=<ID>
// curl --request DELETE http://localhost:3000/clear?session=<ID>

#[derive(Deserialize)]
struct QueryParams {
    session: i64,
}

#[http_service]
async fn handle_route(req: Request) -> impl IntoResponse {
    Router::new()
        .route("/clear", delete(clear))
        .route("/clear/{var}", delete(delete_variable))
        .call(req)
        .await
}

// This endpoint should delete the <ident> => value mapping matching the key `var`
// of the session from the SQLite database
// If you get stuck, ask a neighbor or grab a trainer!
async fn delete_variable(
    Path(var): Path<String>,
    Query(params): Query<QueryParams>,
) -> Result<(), (StatusCode, String)> {
    todo!()
}

// This endpoint should clear ALL <ident> => value mappings of the session
// If you get stuck, ask a neighbor or grab a trainer!
async fn clear(Query(params): Query<QueryParams>) -> Result<(), (StatusCode, String)> {
    todo!()
}
