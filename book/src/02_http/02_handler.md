# HTTP Handlers

`spin` lets you annotate a function with the `#[http_service]` attribute. This marks it as the
HTTP request entry point, the macro generates the `wasi:http/handler` export (other SDKs do the same, [`wstd`] uses `#[wstd::http_server]`). This handler receives a `Request` object containing the HTTP request data and must return a `Result` with a type that implements `IntoResponse` (such as a `Response` or string). The function must be `async`: the runtime yields control while waiting for I/O operations such as outbound HTTP requests or database calls to complete.

```rust
#[http_service]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
  //...
}
```

When `spin` receives a request matching the route pattern defined in `spin.toml` it will forward the request to the entry point set up above. In our case that pattern is `/...` which matches all paths:

```toml
[[trigger.http]]
route = "/..."
component = "handler"
```

## Reading the Request

`Request` is the [`http`] crate's request type. Method, URI and headers are plain accessors. The body is a stream that has to be collected:

```rust
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::Method;

if req.method() != Method::POST {
    // ...
}

let body = req.into_body().bytes().await?;
let input = str::from_utf8(&body)?;
```

## Error Handling

When Rust code panics inside a WebAssembly component, it triggers a WebAssembly _trap_[^1]. Unlike traditional server applications where this would crash the entire system, WebAssembly's sandbox means all we get is a rather ugly stack trace in the terminal and a `500` response.

Instead of relying on panics, HTTP endpoints should use "proper" error handling using `Result`s. The handler signature already allows us to return `anyhow::Result<impl IntoResponse>`. This means we can propagate server-internal errors (errors that we want to show up as 5xx status code responses) simply using the `?` operator.

_Client errors_ i.e. errors we want to return in response to invalid user input still require us to build and return `Response`s ourselves:

```rust
use spin_sdk::http::{Response, StatusCode};

Ok(Response::builder()
    .status(StatusCode::BAD_REQUEST)
    .header("content-type", "text/plain")
    .body("unexpected end of input".to_string())?)
```

## Exercise

Bring the parser and evaluator from the previous chapter into the HTTP handler. Accept `POST` only (`405` otherwise), evaluate the body and return the result as plain text. Invalid input is a `400`, not a panic - replace the `unwrap`s and `panic!`s in your evaluator with `anyhow::Result`s.

Start the application with `spin up --build` and try it out:

```bash
curl --request POST --data '10 / 2' http://localhost:3000/
```

## One Component, Many Hosts

Time to cash in on the promise from chapter 1. Inspect the component with [`wasm-tools`] (`cargo install wasm-tools`):

```bash
wasm-tools component wit target/wasm32-wasip2/debug/handler.wasm
```

The imports are all `wasi:*`, and there is a single export:

```wit
export wasi:http/handler@0.3.0;
```

Nothing in there is Spin specific, so any host that implements `wasi:http` can run it. Try the reference host, `wasmtime` (`curl https://wasmtime.dev/install.sh -sSf | bash`):

```bash
wasmtime serve -Scli target/wasm32-wasip2/debug/handler.wasm
curl --request POST --data '2 + 3 * 4' http://localhost:8080/
```

Same binary, same answers (`-Scli` lets the component use `println!`). wasmCloud and `jco` (Node.js) load it as well; Fastly, Cloudflare and Wasmer don't, they have their own ABIs. That is the fragmentation WASI is meant to end.

> Note: `wasmtime serve` is a development server: no TLS, no rate limiting, no request size limits.

The next sections add storage. Watch the imports.

[^1]: A trap is similar to a segfault. It cannot be caught and will cause your Wasm component to be terminated.

[`http`]: https://docs.rs/http/latest/http/
[`wasm-tools`]: https://github.com/bytecodealliance/wasm-tools
[`wstd`]: https://github.com/bytecodealliance/wstd
