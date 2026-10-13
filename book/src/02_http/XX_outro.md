# Outro

Everything we built runs on your laptop with `spin up`. To put it on the internet, in order of convenience:

- [Akamai Functions](https://techdocs.akamai.com/akamai-functions), the hosted successor of Fermyon Cloud: `spin plugins install aka`, `spin aka login`, `spin aka deploy`. It supports the key value store but _not_ SQLite, so `03_key_value` deploys, `04_databases` doesn't.
- [SpinKube](https://www.spinkube.dev), a Kubernetes operator that schedules Wasm components next to containers - this is where the containers from the title come back. Runs everything we built.
- Any `wasi:http` host for the HTTP-only components (`00` to `02`): [wasmCloud](https://wasmcloud.com), [`jco`](https://github.com/bytecodealliance/jco) on Node.js, or your own service embedding [`wasmtime`].

`spin registry push ghcr.io/you/calculator:v1` packages the application like a container image, `spin up --from ghcr.io/you/calculator:v1` runs it anywhere Spin is installed.

[`wasmtime`]: https://wasmtime.dev
