# Making HTTP requests

When making outbound HTTP requests from a Spin component, you must explicitly grant this capability in the `spin.toml` manifest file. WebAssembly's security model requires explicit permission for components to access external resources, and Spin enforces this through capability declarations in the configuration.

Take a look at the `spin.toml` manifest for this exercise, you will notice it only adds one line:

```diff
spin_manifest_version = 2

[application]
name = "requests"
version = "1.0.0"

[[trigger.http]]
route = "/..."
component = "requests"

[component.requests]
source = "../../../target/wasm32-wasip1/release/requests.wasm"
+ allowed_outbound_hosts = ["http://www.randomnumberapi.com"]

[component.requests.build]
command = "cargo build --target wasm32-wasip1 --release"
watch = ["src/**/*.rs", "Cargo.toml"]

[component.requests.tool.spin-test]
source = "tests/target/wasm32-wasip1/release/tests.wasm"
```

`allowed_outbound_hosts` is `spin`s way of declaratively request access to the specified URLs from the host.

```rust
#[http_component]
pub async fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    todo!()
}
```

Note: making HTTP requests currently requires the use of the `spin`-specific HTTP module. This exemplifies the relatively early, in-progress nature of WebAssembly: In the absencse of standards different hosting providers built out custom APIs and capabilities. As you've seen earlier the WebAssembly System Interface (WASI) standards provides standardized interfaces that address this fragmentation but adoption across providers is still in progress.
