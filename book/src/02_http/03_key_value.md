# Key Value Store

The ephemeral, serverless nature of WebAssembly HTTP components allows for excellent reasoning about a system: Each request starts with a clean slate. What do you do if you actually want to share state between requests though? You need external storage that persists beyond the component's lifecycle.

Most WebAssembly serverless runtimes offer a variety of choices. The first we will look at is the _[Key Value Store]_.

The Key Value Store maps string keys to bytes. It is similar to a Rust [`HashMap`] or Browser [`LocalStorage`]. <br/>
Before you can access the store - similar to exercise [02/01](./01_requests.md) - we need to request access to the store from the runtime. This is done by adding the following line to our `spin.toml` manifest:

```diff
spin_manifest_version = 2

[application]
name = "key-value"
version = "1.0.0"

[[trigger.http]]
route = "/..."
component = "key-value"

[component.key-value]
source = "../../../target/wasm32-wasip2/debug/key_value.wasm"
+key_value_stores = ["default"]

[component.key-value.build]
command = "cargo build -p key_value --target wasm32-wasip2"
watch = ["src/**/*.rs", "Cargo.toml"]
```

This configuration gives the `key-value` component access to the `"default"` store. Spin provides it out of the box, backed by a SQLite file in the `.spin` directory; additional stores can be declared in the runtime configuration. We can then open this store in our Rust code by calling the [`Store::open_default()`] method:

```rust
use spin_sdk::key_value::Store;

let store = Store::open_default().await?;

let val = store
    .get("foo")
    .await?
    .context("no such key in the store")?;

// do something with the value...
```

and write a value back to the store like so:

```rust
store.set("foo", "bar".as_bytes()).await?;
```

All store operations are `async`.

## Where the Standard Ends

Run `wasm-tools component wit` on the finished component and compare with the previous exercise. One new import appeared:

```wit
import spin:key-value/key-value@3.0.0;
```

This is a Spin interface, not a WASI one, and `wasmtime serve` refuses the component with "a matching implementation was not found in the linker". A standard is in the making: [`wasi:keyvalue`] is a phase 2 proposal with the same `get`/`set`/`delete` shape. Spin, wasmtime and wasmCloud all implement a draft of it, but they pin different draft versions (`0.2.0-draft2` vs `0.2.0-draft`) and WIT versions must match exactly to link, so a component using it does not yet move between hosts unchanged. Until that settles, storage is where you pick a vendor - and the HTTP handler around it stays portable.

## JSON

The SDK also provides two convenience methods for storing any [`serde`] serializable data into the store as JSON: [`Store::get_json()`] and [`Store::set_json()`].

```rust
use serde::{Serialize, Deserialize};
use spin_sdk::key_value::Store;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8
}

let store = Store::open_default().await?;

// this presumably came from _somewhere_
let user_name = "john";

// load the user object from the store, it must be deserializable to a `User` struct
let mut val: User = store
    .get_json(user_name)
    .await?
    .context("no such key in the store")?;

// update the age (its the users birthday!)
val.age += 1;

// write the user object back to the store
store.set_json(user_name, &val).await?;
```

## Exercise

Extend the calculator with variables, see `src/lib.rs` for the details. The store persists across requests _and_ across restarts of `spin up` - delete the `.spin` directory in the exercise if you want to start from scratch.

[Key Value Store]: https://spinframework.dev/v4/kv-store-api-guide
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`LocalStorage`]: https://developer.mozilla.org/en-US/docs/Web/API/Storage
[`Store::open_default()`]: https://docs.rs/spin-sdk/latest/spin_sdk/key_value/struct.Store.html#method.open_default
[`serde`]: https://docs.rs/serde/latest/serde/index.html
[`wasi:keyvalue`]: https://github.com/WebAssembly/wasi-keyvalue
[`Store::get_json()`]: https://docs.rs/spin-sdk/latest/spin_sdk/key_value/struct.Store.html#method.get_json
[`Store::set_json()`]: https://docs.rs/spin-sdk/latest/spin_sdk/key_value/struct.Store.html#method.set_json
