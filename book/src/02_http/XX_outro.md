# Outro

Everything we built runs on your laptop with `spin up`. Putting it on the internet is a hosting decision, and the imports of your component decide how free you are in it. As of October 2026:

- The HTTP-only components (`00` to `02`) run on anything that implements `wasi:http`: your own service embedding [`wasmtime`], [wasmCloud](https://wasmcloud.com), [`jco`](https://github.com/bytecodealliance/jco) on Node.js, or Spin.
- The storage components (`03` to `05`) import `spin:*` interfaces and need a Spin host. Self-hosted that is [SpinKube](https://www.spinkube.dev), a Kubernetes operator that schedules Wasm components next to containers - this is where the `containers` in the title of this workshop come back. Hosted that is [Akamai Functions](https://techdocs.akamai.com/akamai-functions), the successor of Fermyon Cloud, which supports the key value store but _not_ SQLite.
- Any OCI registry works as the distribution channel: `spin registry push ghcr.io/you/calculator:v1` packages the application like a container image, `spin up --from ghcr.io/you/calculator:v1` runs it anywhere Spin is installed. wasmCloud and `wasmtime` pull from OCI registries too.

We list these for orientation, not as a recommendation; we have not used the hosted offerings in anger. Fermyon Cloud, the free service earlier versions of this workshop deployed to, has been folded into Akamai Functions after Akamai acquired Fermyon in December 2025.

[`wasmtime`]: https://wasmtime.dev

Deployment targets come and go, the component you built doesn't change. That is the point, with one caveat you have seen in the imports: `wasi:http` is a standard today, `wasi:keyvalue` is a draft, SQL has none. Keep the storage code behind a small module of your own and the rest of the component moves between hosts as is.
