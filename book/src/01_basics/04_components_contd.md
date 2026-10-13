# Components Cont'd

In the previous chapter we built a simple WebAssembly component that demonstrated basic interface definitions and WASI integration. We covered a lot of ground, but the end result wasn't terribly interesting. We'll now return to our calculator project and build something more substantial!

In this chapter we will expand our parser by adding an evaluator that evaluates the parsed mathematical expressions. The evaluator should respect standard operator precedence rules - parentheses first, then multiplication and division, followed by addition and subtraction - and return the computed result as an `f64`.

## Project Structure

Notice how the project now consists of a `main.rs` file instead of a `lib.rs`. We're now building a CLI WebAssembly component!

By compiling to the `wasm32-wasip2` target, Rust automatically implements the [`wasi:cli/run`] interface for us, declaring us compatible with the behavior for traditional CLI executables.

You can run the exercise directly using `cargo run --target wasm32-wasip2`.

> Note: Usually you _cannot_ simply `cargo run` executables compiled to `wasm32-wasip2` the same way you
> cannot simply run executables compiled for `x86` on an `aarch64` machine. For your convenience this workshop provides a runner built on `wasmtime` that allows you to run these as if they were native executables.

## Exercise

Implement `main` so that it reads the expression from the command line, evaluates it and prints the result. A recursive descent evaluator that mirrors the grammar is the simplest approach:

```plaintext
expression := term   (("+" | "-") term)*
term       := factor (("*" | "/") factor)*
factor     := number | "-" factor | "(" expression ")"
```

<details><summary><b>Hint 01</b></summary>

The workshop runner passes _only_ the expression as argument, so `std::env::args().next()` is the expression. A native binary would see the program path first.

</details>

<details><summary><b>Hint 02</b></summary>

Each grammar rule becomes one method on an evaluator struct that holds a `Peekable` token iterator. `eval_factor` recurses into `eval_expression` for parentheses.

</details>

[`wasi:cli/run`]: https://github.com/WebAssembly/wasi-cli/blob/72eb3e163ce0a506a1132a0d3965af0d25aeff65/wit/run.wit
