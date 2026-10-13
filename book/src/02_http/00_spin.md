# Server Side WebAssembly

We have seen WebAssembly as "shared libraries" and WebAssembly as standalone CLI applications. Now we turn to the primary focus of this workshop: WebAssembly in the server backend role.

WebAssembly has gained in popularity in recent years because unlike traditional container-based deployments, WebAssembly modules start in microseconds rather than seconds, making them ideal for functions that need to scale rapidly from zero. Additionally, their strong sandboxing and efficient resource utilization makes them a perfect fit for serverless and edge computing applications.

A number of big companies now provide support for Wasm, among them Cloudflare, Fastly, and Akamai. Each of these hosting providers unfortunately exposes slightly different host APIs and capabilities. The WASI standards we have seen earlier are the ecosystem's answer to this fragmentation, and adoption is still in progress.

For this workshop we need _some_ host to run our components in, and we use the [`spin`] framework. It is an example, not a recommendation. We picked it on four criteria: it is open source and free, it runs on your laptop without an account or a network, it executes standard `wasi:http` components, and it bundles a key value store and SQLite so we can teach storage without setting up infrastructure. Other hosts meet some of these - plain `wasmtime serve` and wasmCloud run the same components, the commercial platforms (Fastly, Cloudflare) come with their own ABIs - and you will run one of the exercises on `wasmtime` directly to see what is standard and what is not. Mainmatter has no affiliation with Spin's maintainers.

## Spin Framework

`spin` is an open-source framework for building and running _serverless_ WebAssembly applications. It was created by Fermyon, has been a CNCF sandbox project since January 2025 and is maintained by Akamai since it acquired Fermyon in December 2025. Like every host in this space it is a thin layer over [`wasmtime`], the Bytecode Alliance's runtime.
For every incoming request, the runtime will spin up a fresh instance of your Wasm component that is immediately terminated after processing the request. The runtime additionally exposes APIs for common tasks like making HTTP requests, routing, key-value storage, database access, and more. We will go through some of these APIs in the following sections, and point out for each whether it is a WASI standard or a Spin interface.

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

`spin up --build` compiles the component and serves it on `http://localhost:3000`. Try it in a second terminal:

```bash
spin up --build
curl -i localhost:3000
```

The tests for this chapter live in the `tests` directory of each exercise. They start `spin up --build` themselves (on port 3333, so they don't collide with your own instance), send HTTP requests to it and shut it down again. `wr` runs them with `cargo test -p spin_tests`.

> Note: Spin used to ship a `spin test` plugin that ran tests inside a virtualized Spin host. It is unmaintained and does not work with current SDKs, so we test the application from the outside instead.

## Exercise

Make the component respond with the text `Hello World!`. Have a look at the [`spin_sdk::http`] docs for `Response::builder()`.

[`spin`]: https://spinframework.dev
[`wasmtime`]: https://wasmtime.dev
[`spin_sdk::http`]: https://docs.rs/spin-sdk/latest/spin_sdk/http/index.html
