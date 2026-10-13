# Components

In the previous section, you experienced the friction of passing strings to WebAssembly functions - requiring manual pointer/length pairs and unsafe memory operations. This fundamental limitation exists on purpose to keep the core WebAssembly specification simple, portable and easy to implement.

On top of this core specification the [WebAssembly Component Model](https://component-model.bytecodealliance.org/) introduces a rich type system that supports complex data structures like strings, records, variants, lists, and options. Components can define interfaces using WebAssembly Interface Types (WIT), enabling type-safe communication between WebAssembly modules and their hosts without manual memory management.

Where core WebAssembly modules export functions with basic numeric parameters, components export interfaces with high-level types that are automatically marshaled by the runtime. Below you can see an example of such an interface:

```wit
package wasi:random@0.2.12;

/// WASI Random is a random data API.
///
/// It is intended to be portable at least between Unix-family platforms and
/// Windows.
@since(version = 0.2.0)
interface random {
    // other functions omitted for brevity...

    /// Return a cryptographically-secure random or pseudo-random `u64` value.
    @since(version = 0.2.0)
    get-random-u64: func() -> u64;
}
```

The above interface definition is an excerpt of the real `wasi:random/random` interface that allows WebAssembly access to cryptographically secure random numbers from the host. Notice couple important pieces:

- `package wasi:random@0.2.12;` declares the _namespace_ (`wasi`) and _name_ (`random`) of the current package, as well as the current version (`0.2.12`).
- `interface` denotes a collection of functions and associated types and defines behaviour shareable with the outside world. You can think of it as a `trait` in Rust.
- `get-random-u64: func() -> u64;` declares that - to conform to the `random` interface - one must export a function called `get-random-u64` that takes no arguments and returns a single `u64`.
- `@since(version = 0.2.0)` is a _stability annotation_ that indicates the annotated function is _stable_ since package version `0.2.0` (unstable additions are gated with `@unstable(feature = ...)`). The component model has a first-class story for evolving and updating interfaces.

To find out what "`wasi`" is, keep on reading!

## WebAssembly System Interface (WASI)

WebAssembly System Interface (WASI) defines a set of standard interfaces for common system operations like file I/O, networking, and random number generation. As of the writing of this workshop the interfaces provided by WASI 0.2 are the following:

- `wasi:io` Streams and polling. The building block all other I/O interfaces use.
- `wasi:clocks` Reading the current time and measuring elapsed time.
- `wasi:random` Obtaining pseudo-random, and cryptographically secure data.
- `wasi:filesystem` Accessing host filesystems. Has functions for opening, reading, and writing files, and for working with directories.
- `wasi:sockets` Adds TCP & UDP sockets and domain name lookup.
- `wasi:cli` Command-Line Interface (CLI) environment. Provides Stdio, Command-line arguments, and the concept of a `main` function.
- `wasi:http` Sending and receiving HTTP requests and responses.

WASI 0.3 (released June 2026) replaces `wasi:io` with native `async` functions, `stream<T>` and `future<T>` types in the component model itself. Rust support (`wasm32-wasip3`) is still tier 3, so this workshop targets WASI 0.2.

Today Rust applications can easily access most of these interfaces by simply compiling to the [`wasm32-wasip2`] target. Rust standard library types such as [`std::time::SystemTime`], [`std::fs::File`], or [`std::net::TcpStream`] will automatically make use of the WASI interfaces. Things WASI has no interface for - spawning processes with [`std::process::Command`] for example - return an `Unsupported` error.

## Wasm Component in Rust

### Prerequisites

The Rust ecosystem has strong support for building WebAssembly Components. The `wasm32-wasip2` target produces a component directly, and the [`wit-bindgen`] crate generates the Rust bindings for a WIT file at compile time. No extra tools are required.

> Note: Older material (and this workshop until recently) used the [`cargo-component`] tool instead. Its maintainers are deprecating it in favour of the built-in target, so we won't use it.

### Project Structure

Now, looking at the exercise for this section you will notice a couple new things.

```plaintext
03_components
├── src
│   └── lib.rs
├── wit
│   ├── deps
│   │   └── random
│   │       └── random.wit
│   └── world.wit
├── tests
└── Cargo.toml
```

Let's go over the difference, starting with `Cargo.toml`:

#### `Cargo.toml`

```toml
[package]
name = "components"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wit-bindgen = "0.62.0"
```

The only addition is the `wit-bindgen` dependency, which provides the `generate!` macro we will see below.

#### `./wit/world.wit`

`world.wit` is a _WebAssembly Interface Types_ declaration that defines the `interface` our component is going to export. This is the same syntax you have seen above:

```wit
package workshop:example;

/// An example world for the component to target.
world example {
    import wasi:random/random@0.2.7;

    export add-random: func(num: u64) -> u64;
}
```

This file is quite a bit simpler though, we simply declare we will be _importing_ the `wasi:random/random` interface and _exporting_ a function called `add-random` that takes in a `u64` and returns a `u64`. A `world` describes the complete surface of a component: everything it needs from the host and everything it provides.

#### `./wit/deps/random/random.wit`

The WIT of every imported package has to be available locally. `wit/deps/random/random.wit` is a verbatim copy of the `wasi:random` interface. For a larger project you would fetch dependencies with [`wkg`](https://github.com/bytecodealliance/wasm-pkg-tools) from a registry instead of copying them by hand.

### Rust Bindings

`wit-bindgen` generates **type-safe** bindings to our imports. Even better it generates a Rust `trait` that **requires** our component to adhere to the interface we promised above. Let's have a look!

The exercise already contains the macro invocation at the top of `src/lib.rs`:

```rust
wit_bindgen::generate!({
    path: "wit",
    world: "example",
    generate_all,
});
```

The macro reads the `wit` directory and expands to Rust code at compile time (run `cargo expand` or read the [`wit-bindgen`] docs if you want to see it). The gist:

- bindings to all interfaces we imported are available as submodules. In our case that is `wasi::random::random` to access the random number interface.
- `Guest` is a `trait` that we have to implement in order to be compliant with the interface we promised above.
- `export!` is a macro that lets us declare a Rust type implementing `Guest` as _the_ implementation for this interface. You call it like so `export!(<YourTypeName>);` (substitute `<YourTypeName>` for the actual name of your type).

## Exercise

The exercise for this section is quite simple again, you just need to provide an implementation for the `Guest` trait that uses the functions provided by `wasi:random/random` to add a random number to the provided one.

[`wasm32-wasip2`]: https://doc.rust-lang.org/nightly/rustc/platform-support/wasm32-wasip2.html
[`wasi`]: https://docs.rs/wasi/latest/wasi/
[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`std::net::TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`cargo-component`]: https://github.com/bytecodealliance/cargo-component
[`wit-bindgen`]: https://docs.rs/wit-bindgen/latest/wit_bindgen/
[`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
