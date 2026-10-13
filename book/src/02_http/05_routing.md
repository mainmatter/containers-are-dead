# Routing

So far we have only ever seen a single catch-all (`/...`) endpoint per component. Spin offers two levels of routing: between components in `spin.toml`, and within a component in code.

## Routing between multiple Components

Spin applications can be composed of multiple WebAssembly components, each handling different routes. This is configured in the `spin.toml` manifest by defining multiple `[[trigger.http]]` sections, each mapping a route pattern to a specific component:

```toml
[application]
name = "multi-component-app"
version = "1.0.0"

[[trigger.http]]
route = "/api/users/..."
component = "user-service"

[[trigger.http]]
route = "/api/orders/..."
component = "order-service"

[component.user-service]
source = "target/wasm32-wasip2/release/users.wasm"

[component.order-service]
source = "target/wasm32-wasip2/release/orders.wasm"
```

When a request arrives, Spin's HTTP trigger examines the URL path and matches it against the configured routes, most specific first. The request is then forwarded to the corresponding WebAssembly component.

This allows you to compose applications from modular components - recall the component composition from earlier - that can be developed, tested, and deployed independently. The components don't even have to share a language: a Rust component and a Go component can serve neighbouring routes of the same application.

The exercise for this section does exactly that: `spin.toml` routes `/eval` to the `databases` component from the previous exercise (unchanged, it is built from `../04_databases`) and `/clear/...` to a new `routing` component.

## Routing Within A Component

What if one component should serve multiple endpoints? Instead of manually parsing URLs and HTTP methods in your handler function, use a router. Since `spin_sdk::http` uses the ecosystem's [`http`] types, any router that works with them works in Spin. The exercise uses [`axum`]:

```rust
use axum::extract::{Path, Query};
use axum::routing::{get, post};
use axum::Router;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_service;
use tower_service::Service;

#[http_service]
async fn handle_route(req: Request) -> impl IntoResponse {
    Router::new()
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user))
        .call(req)
        .await
}

async fn get_user(Path(id): Path<String>) -> String {
    // look up the user, or something...
}

async fn create_user(body: String) -> String {
    // create the user, or something...
}
```

The router supports path parameters (`{id}`) that are extracted and passed to handlers via [`axum` extractors] such as `Path` and `Query`. Handlers return anything that implements `axum::response::IntoResponse` - a `String`, a `StatusCode`, a `Result`, ... Error handling moves into the handlers: a `Result<T, (StatusCode, String)>` becomes the error response on `Err`.

> Note: Spin SDK versions before 6 shipped their own `Router`. It is gone, use `axum` or any other `http` based router.

## Exercise

Implement the two `DELETE` endpoints in `src/lib.rs`. Both need the `session` query parameter, requests without one must fail with `400 Bad Request`.

[`http`]: https://docs.rs/http/latest/http/
[`axum`]: https://docs.rs/axum/latest/axum/
[`axum` extractors]: https://docs.rs/axum/latest/axum/extract/index.html
