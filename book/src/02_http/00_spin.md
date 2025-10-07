# Server Side WebAssembly

We have seen WebAssembly as "shared libraries" and WebAssembly as standalone CLI applications. Now we turn to the primary focus of this workshop: WebAssembly in the server backend role.

WebAssembly has gained in popularity in recent years because unlike traditional container-based deployments, WebAssembly modules start in microseconds rather than seconds, making them ideal for functions that need to scale rapidly from zero. Additionally, their strong sandboxing and efficient resource utilization makes them a perfect fit for serverless and edge computing applications.

A number of big companies now provide support for Wasm, among them Cloudflare, Fastly, and Azure. Each of these hosting providers unfortunately exposes slightly different host APIs and capabilities. This ecosystem fragmentation was one of the original motivations for CloudABI, which then morphed into the WASI standards we have seen earlier!

For this workshop we will be using the `spin` framework built by Fermyon because it has a nice set of APIs. The things you will learn during this workshop should translate easily to other providers as well.

## Spin Framework

`spin` is an open-source framework for building and deploying _serverless_ WebAssembly applications.
For every incoming requests, the runtime will spin up a fresh instance of your Wasm component that is immediately terminated after processing the request. The runtime additionally exposes APIs for common tasks like making HTTP requests, routing, key-value storage, database access, and more. We will go through some these of these APIs in the following sections.

### Prerequisites

To get started, install the `spin` CLI and the `spin-test` plugin for local development:

```bash
curl -fsSL https://spinframework.dev/downloads/install.sh | bash
spin plugin install -u https://github.com/spinframework/spin-test/releases/download/canary/spin-test.json
```

The spin-test plugin allows you to run HTTP tests against your Spin applications locally, `wr` will be using it to check your exercises.

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
source = "../../../target/wasm32-wasip1/release/spin.wasm"

[component.spin.build]
command = "cargo build --target wasm32-wasip1 --release"
watch = ["src/**/*.rs", "Cargo.toml"]

[component.spin.tool.spin-test]
source = "tests/target/wasm32-wasip1/release/tests.wasm"
```

The `[application]` section contains basic metadata. The `[[trigger.http]]` section defines HTTP routing - here `/...` catches all paths and forwards them to the specified component. The `[component]` section points to the compiled WebAssembly module and includes build instructions. The `command` field tells Spin how to compile your Rust code, while `watch` specifies which files should trigger rebuilds during development. The `spin-test` tool configuration enables local testing of your HTTP endpoints.

[`spin`]: https://spinframework.dev
