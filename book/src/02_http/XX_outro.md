# Outro

Everything we built runs on your laptop with `spin up`. Where it can run on the internet depends on the component's imports:

- `00` to `02` import only `wasi:*` and run on anything that implements `wasi:http`: your own service embedding [`wasmtime`], [wasmCloud](https://wasmcloud.com), [`jco`](https://github.com/bytecodealliance/jco) on Node.js, or Spin.
- `03` to `05` import `spin:*` and need a Spin host. Self-hosted that is [SpinKube](https://www.spinkube.dev), a Kubernetes operator that schedules Wasm components next to containers - this is where the containers from the title come back. Hosted that is [Akamai Functions](https://techdocs.akamai.com/akamai-functions), the successor of Fermyon Cloud; it supports the key value store but _not_ SQLite.
- `spin registry push ghcr.io/you/calculator:v1` packages the application like a container image, `spin up --from ghcr.io/you/calculator:v1` runs it anywhere Spin is installed. wasmCloud and `wasmtime` pull from OCI registries as well.

This is orientation, not a recommendation, we have not used the hosted offerings. Fermyon Cloud, which earlier versions of this workshop deployed to, became Akamai Functions when Akamai acquired Fermyon in December 2025.

Deployment targets come and go, the component doesn't change. `wasi:http` is a standard, `wasi:keyvalue` a draft, SQL has none - keep the storage code in a module of its own and the rest moves between hosts as is.

[`wasmtime`]: https://wasmtime.dev
