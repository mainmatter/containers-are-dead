# Outro

Everything we built runs on your laptop with `spin up`. To put it on the internet you need a host that runs Spin applications. As of October 2026 the options are:

- [SpinKube](https://www.spinkube.dev): a Kubernetes operator that runs Spin applications as pods, in your own cluster. This is where the `containers` in the title of this workshop come back: Wasm components get scheduled next to containers, with the same tooling.
- [Akamai Functions](https://techdocs.akamai.com/akamai-functions): the hosted successor of Fermyon Cloud. Install the plugin with `spin plugins install aka`, log in with `spin aka login` and deploy with `spin aka deploy`. It supports the key value store, but _not_ SQLite - the `03_key_value` exercise deploys, `04_databases` does not.
- Any OCI registry: `spin registry push ghcr.io/you/calculator:v1` packages the application like a container image, `spin up --from ghcr.io/you/calculator:v1` runs it anywhere Spin is installed.

> Note: Fermyon Cloud, the free hosted service earlier versions of this workshop deployed to, has been superseded by Akamai Functions after Akamai acquired Fermyon in December 2025. Spin 4 no longer treats `spin deploy` as an alias for it; you always name the deployment plugin.

Deployment targets come and go, the component you built doesn't change. That is the point.
