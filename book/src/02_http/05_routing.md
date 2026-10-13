# Routing

So far we have only ever seen a single catch-all (`/...`) endpoint per component. There are two levels of routing: between components in `spin.toml`, and within a component in code.

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

This allows you to compose applications from modular components (recall the component composition from earlier) that can be developed, tested, and deployed independently. They don't have to be the same language either, a Rust and a Go component can serve routes together.

The exercise does exactly that: `spin.toml` routes `/eval` to the `databases` component from the previous exercise (built from `../04_databases`, unchanged) and `/clear/...` to a new `routing` component.

## Routing Within A Component

What if one component should serve multiple endpoints? Use a router instead of parsing URLs and methods by hand. `spin_sdk::http` uses the [`http`] crate's types, so any router built on them works. The exercise uses [`axum`]:

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

Path parameters (`{id}`) and query strings arrive through [`axum` extractors] like `Path` and `Query`. Handlers can return anything implementing `axum::response::IntoResponse`: a `String`, a `StatusCode`, or a `Result` whose `Err` becomes the error response.

## Exercise

Implement the two `DELETE` endpoints in `src/lib.rs`. Both need the `session` query parameter, without it they fail with `400`.

[`http`]: https://docs.rs/http/latest/http/
[`axum`]: https://docs.rs/axum/latest/axum/
[`axum` extractors]: https://docs.rs/axum/latest/axum/extract/index.html
