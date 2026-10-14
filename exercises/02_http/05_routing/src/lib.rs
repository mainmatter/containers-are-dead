use axum::Router;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::delete;
use serde::Deserialize;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_service;
use spin_sdk::sqlite::{Connection, Value};
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

/// Deletes the variable `var` from the session.
async fn delete_variable(
    Path(var): Path<String>,
    Query(params): Query<QueryParams>,
) -> Result<(), (StatusCode, String)> {
    let db = Connection::open_default().await.map_err(internal_error)?;
    db.execute(
        "DELETE FROM variables WHERE session_id = ? AND key = ?",
        [Value::Integer(params.session), Value::Text(var)],
    )
    .await
    .map_err(internal_error)?
    .result()
    .await
    .map_err(internal_error)?;
    Ok(())
}

/// Deletes all variables of the session.
async fn clear(Query(params): Query<QueryParams>) -> Result<(), (StatusCode, String)> {
    let db = Connection::open_default().await.map_err(internal_error)?;
    db.execute(
        "DELETE FROM variables WHERE session_id = ?",
        [Value::Integer(params.session)],
    )
    .await
    .map_err(internal_error)?
    .result()
    .await
    .map_err(internal_error)?;
    Ok(())
}

fn internal_error(err: impl std::fmt::Display) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
