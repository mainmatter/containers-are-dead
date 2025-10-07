# Outro

We've covered quite a lot of ground in this chapter. We have built _core_ WebAssembly modules by compiling to `wasm32-unknown-unknown` and saw the challenges of implementing more complex applications using. We've built two WebAssembly Components and have seen the advantages it has over core WebAssembly.

Before moving on to HTTP and web servers in WebAssembly, let's stick with Components a tiny bit longer and talk about their _one neat trick™️_:

## Component Composition

WebAssembly Components are designed to be _composed together_, combining multiple components into larger applications. Each component could be written in a different language, by a different team, or even a completely different organization and evolve independently over time; The components strongly typed interfaces will ensure that your application always remains correct.

We have seen such an example in the last section: We declared we need _an_, _any_ implementation of the `wasi:random/random` interface regardless of _how_ that is implemented or _who_ implements it. It was the responsibility of the host (our test framework) to figure out how to provide an implementation of the interface.

The second example we have seen in this chapter: By defining an `fn main() {}` in our `main.rs` file and compiling to the `wasm32-wasip2` target we automatically implemented the [`wasi:cli/run`] interface declaring ourselves to provide the behaviour of "a traditional CLI executable".
You can see the test verifier for this workshop depending on this exact interface [here](https://github.com/mainmatter/containers-are-dead/blob/dcfd1080929b72fdf0dfe080ff269c46fb2232f1/verifier/src/wasmtime.rs#L62). The `wasm32-wasip2` target conveniently took care of implementing this interface for us when we declared

This special flavor of loose coupling has one major advantage that e.g. microservices don't have: WebAssembly's Virtual Machine. Since WebAssembly always runs within a host environment, many responsibilities are shifted to the host runtime. WIT interfaces exported by components are **transport agnostic**, meaning a component you import might be statically linked into your executable, dynamically linked at runtime, or even provided over the network. All without a single change to your code.

Combined with core WebAssembly's typed, structured nature, host runtimes can dynamically optimize your application based on the "in-vivo" deployment situation.

We will see this in action in the coming chapter, so let's move on!

[`wasi:cli/run`]: https://github.com/WebAssembly/wasi-cli/blob/72eb3e163ce0a506a1132a0d3965af0d25aeff65/wit/run.wit
