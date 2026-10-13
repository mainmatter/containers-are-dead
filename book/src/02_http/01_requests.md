# Making HTTP requests

When making outbound HTTP requests from a Spin component, you must explicitly grant this capability in the `spin.toml` manifest file. WebAssembly's security model requires explicit permission for components to access external resources, and Spin enforces this through capability declarations in the configuration.

Take a look at the `spin.toml` manifest for this exercise, you will notice a few additions:

```diff
spin_manifest_version = 2

[application]
name = "requests"
version = "1.0.0"

+[variables]
+random_api = { default = "http://www.randomnumberapi.com" }

[[trigger.http]]
route = "/..."
component = "requests"

[component.requests]
source = "../../../target/wasm32-wasip2/debug/requests.wasm"
+allowed_outbound_hosts = ["http://www.randomnumberapi.com", "http://127.0.0.1:3100"]

+[component.requests.variables]
+random_api = "{{ random_api }}"

[component.requests.build]
command = "cargo build -p requests --target wasm32-wasip2"
watch = ["src/**/*.rs", "Cargo.toml"]
```

`allowed_outbound_hosts` is `spin`s way of declaratively requesting access to the specified hosts from the runtime. Requests to any other host fail with a "Destination not allowed" error.

## Variables

The second addition is a _variable_. `[variables]` declares `random_api` with a default value at the application level, `[component.requests.variables]` passes it to the component. Inside the component you read it with:

```rust
let api = spin_sdk::variables::get("random_api").await?;
```

Variables can be overridden when starting the application, e.g. by setting the environment variable `SPIN_VARIABLE_RANDOM_API`. The tests for this exercise use exactly that to point the component at a local stand-in for the real API, so they work without internet access. That is also why `127.0.0.1:3100` is in the list of allowed hosts.

## Making Requests

Let's have a look at an example HTTP endpoint that in-turn makes an HTTP request to `https://example.com`:

```rust
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{EmptyBody, IntoResponse, Method, Request, Response};
use spin_sdk::http_service;

#[http_service]
pub async fn handler(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://example.com")
        .body(EmptyBody::new())?;

    // Ask the Wasm runtime to send off the request!
    let res: Response = spin_sdk::http::send(req).await?;

    // Bodies are streams, collect it into memory
    let body = res.into_body().bytes().await?;

    // do something with the response...
}
```

You will notice the `async` keyword and use of `await` in the handler function; the component yields control back to the host while waiting for the network response, so one instance does not block a thread. This is how WASI 0.3 models I/O in general. Since Spin SDK 6 every handler is `async` and every host API returns a future.

Also note that `Request` and `Response` are the types of the ecosystem's [`http`] crate. Under the hood `spin_sdk::http::send` is backed by the standard `wasi:http` interface, so the same code works on any host that implements WASI HTTP. Other host APIs - key value stores for example - still require a `spin`-specific interface. This exemplifies the relatively early, in-progress nature of WebAssembly: In the absence of standards different hosting providers built out custom APIs and capabilities. The WASI standards address this fragmentation but adoption across providers is still in progress.

## Exercise

Implement the endpoint described in `src/lib.rs`: it takes `min` and `max` query parameters, asks `randomnumberapi.com` for a random number in that range and returns it as plain text. The `serde-querystring` crate is already added as a dependency to parse `req.uri().query()`.

[`http`]: https://docs.rs/http/latest/http/
