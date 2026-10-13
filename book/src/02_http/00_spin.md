# Server Side WebAssembly

We have seen WebAssembly as "shared libraries" and WebAssembly as standalone CLI applications. Now we turn to the primary focus of this workshop: WebAssembly in the server backend role.

WebAssembly has gained in popularity in recent years because unlike traditional container-based deployments, WebAssembly modules start in microseconds rather than seconds, making them ideal for functions that need to scale rapidly from zero. Additionally, their strong sandboxing and efficient resource utilization makes them a perfect fit for serverless and edge computing applications.

A number of big companies now provide support for Wasm, among them Cloudflare, Fastly, and Akamai. Each of these hosting providers unfortunately exposes slightly different host APIs and capabilities. The WASI standards we have seen earlier are the ecosystem's answer to this fragmentation, adoption is still in progress.

For this workshop we need a host to run our components. We use [`spin`]: it is open source, runs offline on your laptop, executes standard `wasi:http` components and ships a key value store and SQLite, so we can teach storage without infrastructure. It is an example, not a recommendation - `wasmtime serve` and wasmCloud run the same components, and you will try one of them later. Mainmatter is not affiliated with Spin.

## Spin Framework

`spin` is an open-source framework for building and running _serverless_ WebAssembly applications on top of [`wasmtime`]. It was created by Fermyon, joined the CNCF sandbox in January 2025 and is maintained by Akamai since it acquired Fermyon in December 2025.
For every incoming request, the runtime will spin up a fresh instance of your Wasm component that is immediately terminated after processing the request. The runtime additionally exposes APIs for common tasks like making HTTP requests, routing, key-value storage, database access, and more. We will go through some of these APIs in the following sections and note which are WASI standards and which are Spin's own.

### Prerequisites

To get started, install the `spin` CLI (this workshop was tested with Spin 4.2):

```bash
curl -fsSL https://spinframework.dev/downloads/install.sh | bash
sudo mv ./spin /usr/local/bin/spin
```

> Note: If you use `nix`, `spin` is already provided by the dev shell flake in this repo!

### Spin Configuration

Spin applications are configured through a `spin.toml` manifest file. This file specifies how HTTP requests are routed to WebAssembly components and how those components are built. The `spin.toml` file in this sections exercise looks like so:

```toml
spin_manifest_version = 2

[application]
name = "spin"
version = "1.0.0"

[[trigger.http]]
route = "/..."
component = "spin"

[component.spin]
source = "../../../target/wasm32-wasip2/debug/spin.wasm"

[component.spin.build]
command = "cargo build -p spin --target wasm32-wasip2"
watch = ["src/**/*.rs", "Cargo.toml"]
```

The `[application]` section contains basic metadata. The `[[trigger.http]]` section defines HTTP routing - here `/...` catches all paths and forwards them to the specified component. The `[component]` section points to the compiled WebAssembly component and includes build instructions. The `command` field tells Spin how to compile your Rust code, while `watch` specifies which files should trigger rebuilds during development.

### Running and Testing

`spin up --build` compiles the component and serves it on `http://localhost:3000`:

```bash
spin up --build
curl -i localhost:3000
```

Each exercise in this chapter has a `tests` package. The tests start `spin up --build` themselves (on port 3333, so your own instance doesn't collide), send requests and shut it down again. `wr` runs `cargo test -p spin_tests`.

> Note: The `spin test` plugin is unmaintained and incompatible with current SDKs, so we test from the outside.

## Exercise

Make the component respond with the text `Hello World!`. Have a look at the [`spin_sdk::http`] docs for `Response::builder()`.

[`spin`]: https://spinframework.dev
[`wasmtime`]: https://wasmtime.dev
[`spin_sdk::http`]: https://docs.rs/spin-sdk/latest/spin_sdk/http/index.html
