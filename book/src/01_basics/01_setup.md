<div class="warning">

Don't jump ahead!\
Complete the exercise for the previous section before you start this one.\
It's located in `exercises/01_basics/00_welcome`, in the [course GitHub's repository](https://github.com/mainmatter/containers-are-dead).\
Use [`wr`](00_welcome.md#wr-the-workshop-runner) to start the course and verify your solutions.

</div>

## Exercise Structure

All exercises in this course follow the same structure:

- a WebAssembly application written in Rust, in the root of the exercise directory
- a `tests` directory that tests the WebAssembly module, you will have to modify the Rust code
  of the application to make the tests pass.

## Project Structure

Let's have a look at the application for this section.

```plaintext
01_setup
├── src
│   └── lib.rs
├── tests
└── Cargo.toml
```

## `Cargo.toml`

The manifest file, `Cargo.toml`, looks like this:

```toml
[package]
name = "setup"
version = "0.1.0"
edition = "2024"

[lib]
name = "setup"
crate-type = ["cdylib", "rlib"]
```

You will notice the `crate-type` attribute standing out compared to a regular Rust project. It tells the Rust compiler what final artifacts to produce. `"rlib"` is the default setting for `crate-type`
and instructs the compiler to emit a special Rust-flavored static library that is meant to be understood and consumed by other Rust projects. `"cdylib"` on the other hand instructs the compiler to emit a dynamic library with a C-compatible interface (**C** **dy**namic **lib**rary). This setting is **required** for Rust to emit WebAssembly (`.wasm`) files. To learn why, keep on reading!

## WebAssembly Binary Format

WebAssembly is a simple, portable abstract machine and an executable format. It is low-level enough to allow languages like C/C++ and Rust to run with near-native performance while providing the strong isolation and sandboxing that is required to run untrusted third-party code on the Web.

It has gained popularity outside the browser in recent years due to its ability to run on a variety of platforms and its ability to be embedded in a variety of contexts. It is used anywhere between, [operating systems](https://github.com/JonasKruckenberg/k23), [font files](https://github.com/harfbuzz/harfbuzz/blob/main/docs/wasm-shaper.md), [database files](https://db.cs.cmu.edu/papers/2025/zeng-sigmod2025.pdf), and - most relevant for today - serverless cloud hosting providers.

WebAssembly operates as a stack-based virtual machine that executes bytecode instructions that manipulate an implicit operand stack - function parameters are pushed onto this stack, operations consume values from the stack, and results are pushed back. This stack is managed by the host and _not_ visible directly to the guest code running in the sandbox.

The VM enforces strong isolation through its linear memory model: each WebAssembly instance has access only to its own contiguous, bounded memory space, with no ability to access host memory directly. All interactions with the host environment must go through explicitly declared imports and exports. This design enables WebAssembly to run untrusted code safely.

This also explains why we had to instruct Rust to emit a dynamic library: All WebAssembly code is loaded and linked dynamically by the host. All WebAssembly modules _are_ shared libraries!

### Key Concepts

- **Module**: A WebAssembly binary, decoded and validated by the host. A module is stateless and explicitly declares imports and exports just like a Rust module/crate does.
- **Memory**: A resizable memory region that contains the linear array of bytes read and written by WebAssembly's low-level memory access instructions. Essentially a `Vec<u8>`.
- **Table**: A resizable typed array of references (e.g., to functions) that could not otherwise be stored as raw bytes in Memory (for safety and portability reasons).
- **Global**: A global value, either mutable or immutable. These are used as global variables by code or e.g. to communicate configuration options from the host to instances.
- **Instance**: A Module paired with all the state it uses at runtime including a Memory, Table, and set of imported values. This instance is _stateful_ and is similar to a shared library loaded into memory.

## Inspect the WebAssembly Module

Head to the exercise for this section and compile it to WebAssembly. This can be done by running `cargo build --target wasm32-unknown-unknown --release`. Note you will need to make some - trivial - changes to the Rust code to make it compile.

You can then find the compiled `.wasm` file in your target folder under `target/wasm32-unknown-unknown/release/setup.wasm`. Note that all exercises share the `target` folder at the root of the repository, not one per exercise. Open this website in your browser: https://webassembly.github.io/wabt/demo/wasm2wat/ and drag your `.wasm` file into the "editor" section. Take a look at the disassembly! It should look something like this:

```wasm
(module $setup.wasm
  (type $t0 (func (result i32)))
  (func $it_works (export "it_works") (type $t0) (result i32)
    (i32.const 1))
  (table $T0 1 1 funcref)
  (memory $memory (export "memory") 16)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global $__data_end (export "__data_end") i32 (i32.const 1048576))
  (global $__heap_base (export "__heap_base") i32 (i32.const 1048576)))
```

Let's break down the WebAssembly Text (WAT) format you're seeing. WAT is the human-readable representation of WebAssembly bytecode, using S-expressions (similar to Lisp syntax) to represent the module structure.

Here's what each section of our compiled module does:

- `(module $setup.wasm ...)`: The root container for our WebAssembly module
- `(type $t0 (func (result i32)))`: Defines a function signature type that takes no parameters and returns a 32-bit integer
- `(func $it_works (export "it_works") ...)`: Our exported function that the host can call, returning the constant value `1`
- `(table $T0 1 1 funcref)`: A table for function references (required by Rust, even if unused)
- `(memory $memory (export "memory") 16)`: Linear memory of 16 pages (1 MiB total) that the host can access too (because of `(export "memory")`).
- `(global $__stack_pointer ...)`: Mutable global for Rust's stack management
- `(global $__data_end ...)` and `(global $__heap_base ...)`: Exported globals marking memory layout boundaries for Rust's allocator.

You may also have noticed the `$__stack_pointer` global and asked yourself why it exists, I thought the WebAssembly stack was implicit and managed by the host??

### Key Differences from Traditional Assembly

Unlike traditional assembly languages, WebAssembly is **strongly typed**. Every value has a specific type (i32, i64, f32, f64, or reference types). Every instruction is typed as well, for example there is a `i64.add` and an `i32.add` instruction, attempting to pass anything but two `i64` values to an `i64.add` instruction will result in an error. This is enforced by the WebAssembly runtime _ahead of time_, programs that don't pass the validation step will not even begin execution. This prevents many classes of bugs that are common in native assembly, e.g. `x86` is happy to interpret your `f64` as an `i64` and do integer arithmetic with it.

### The Stack Pointer Global

WebAssembly technically has 2 different stacks:

1. The operand stack where instructions pop and pushed values.
2. The call stack where function-local variables are stored.

Together they allow _almost_ all programs to be compiled to WebAssembly, with one exception: Some languages allow you to take the _address_ of a stack allocated variable. This can't work since the WebAssembly stack is managed by the host and not directly accessible from WebAssembly code.

```rust
let x = 42;

// this wouldn't work!
println!("{}", &raw const x);
```

Languages that allow this, like Rust, maintain their own small stack in WebAssembly _linear memory_ for the values that a program needs to take the value of. The `__stack_pointer` global tracks the current position in this software-managed stack.
