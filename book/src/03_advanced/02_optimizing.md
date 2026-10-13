# Optimizing

## Optimizing for Size

Smaller modules load faster, improving cold start performance and in turn reducing the latency between request arrival and function execution - a critical factor for user-facing applications where every millisecond counts.

> Competition! At the end of this workshop we will compare our final _optimized_ database examples to see who got the smallest!

Let's start with our current databases example. A debug build produces a WebAssembly component of roughly _24 MB_: clearly room for improvement.

We can drastically improve the size by compiling in release mode with optimizations, if we additionally add this config to our _root_ `Cargo.toml` file we can usually squeeze some bytes more:

```toml
[profile.release]
codegen-units = 1 # don't parallelize compilation of crates, this can make optimizations more effective
lto = true # use "fat" link time optimization that operates on the entire dependency graph, again potentially making optimizations more effective
opt-level = "z" # instruct the compiler to aggressively optimize for small code size, sometimes "s" can be a better option here
strip = true # strip debug symbols from the release binary
```

We can instruct `spin` to build our component in release mode by adding the `--release` flag to the build command and pointing it to the release build like so:

```diff
spin_manifest_version = 2

[application]
name = "databases"
version = "1.0.0"

[[trigger.http]]
route = "/..."
component = "databases"

[component.databases]
-source = "../../../target/wasm32-wasip2/debug/databases.wasm"
+source = "../../../target/wasm32-wasip2/release/databases.wasm"
sqlite_databases = ["default"]

[component.databases.build]
-command = "cargo build -p databases --target wasm32-wasip2"
+command = "cargo build -p databases --target wasm32-wasip2 --release"
watch = ["src/**/*.rs", "Cargo.toml"]
```

A plain release build comes in at _620 KB_, the profile above brings it down to _370 KB_. `opt-level = "z"` and `lto` are responsible for most of that.

Binaryen - a WebAssembly optimizer and compiler toolchain - provides a widely used tool called `wasm-opt` that can squeeze core modules further (`wasm-opt input.wasm -Oz -o output.wasm`). It does not understand components yet though: the `.wasm` files we build in this chapter make it fail with a parse error, so it only helps for core modules like the ones from the first chapter. You can download it from the Binaryen GitHub releases [here](https://github.com/WebAssembly/binaryen/releases); it is also in most package repositories under the `binaryen` name and in the `nix` flake of this repo.

Beyond the compiler flags, size is mostly a matter of dependencies: `serde_json`, `axum` and the SQL client each cost tens of kilobytes. Check [`cargo bloat`](https://github.com/RazrFalcon/cargo-bloat) or `twiggy` to see what you are paying for.

## Optimizing for Performance

Size optimization is particularly important for serverless deployments, but nothing beats optimizing for speed. Unfortunately, there are not many WebAssembly specific solutions out there. Lack of profiling tooling has been identified as a major blocker; that unfortunately isn't resolved yet.

In spite of this, some tooling can be adapted to work with Wasm, for example `criterion` the Rust benchmarking framework already works inside WebAssembly today. Wasmtime supports profiling the entire runtime (including your WebAssembly code but also the runtime's code) with `perf` and its built-in guest profiler.

One interesting WebAssembly specific performance optimization tool exists that shouldn't go unmentioned: [`wizer`] is a so-called "preinitializer" for WebAssembly modules. It takes your module or component, runs its initialization function, and then snapshots the bytes of the initialized state into a new Wasm module. Depending on your exact application this weird sounding trick can actually meaningfully improve the startup.

[`criterion`]: https://github.com/bheisler/criterion.rs
[`wizer`]: https://github.com/bytecodealliance/wizer
