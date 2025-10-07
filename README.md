# Containers Are Dead - Long Live WebAssembly

Welcome to **"Containers Are Dead - Long Live WebAssembly"**!

> In this course, we will explore the concept of containers and how they are being replaced by WebAssembly. We will cover the basics of WebAssembly, including its architecture and how it can be used to create lightweight, portable, and efficient applications. We will also discuss the benefits of using WebAssembly over traditional container technologies, such as Docker and Kubernetes.

We assume you are familiar with the basics of Rust but we will provide brief explanations and references whenever we rely on advanced features.

> [!NOTE]
> This course has been written by [Mainmatter](https://mainmatter.com/rust-consulting/).\
> It's one of the trainings in [our portfolio of Rust workshops](https://mainmatter.com/services/workshops/rust/).\
> Check out our [landing page](https://mainmatter.com/rust-consulting/) if you're looking for Rust consulting or
> training!

## Getting started

Open [the companion book for this course](https://rust-exercises.com/rust-python-interop/) in your browser.
Follow the instructions there to get started.

## Requirements

- **Rust** (follow instructions [here](https://www.rust-lang.org/tools/install)).\
  If `rustup` is already installed on your system, run `rustup update` (or another appropriate command depending on how
  you installed Rust on your system)
  to make sure you're running on the latest stable version.
- _(Optional but recommended)_ An IDE with Rust autocompletion support.
  We recommend one of the following:
  - [RustRover](https://www.jetbrains.com/rust/);
  - [Visual Studio Code](https://code.visualstudio.com) with
    the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.

## Solutions

You can find the solutions to the exercises in
the [`solutions` branch](https://github.com/mainmatter/containers-are-dead/tree/solutions) of this repository.

# License

Copyright © 2024- Mainmatter GmbH (https://mainmatter.com), released under the
[Creative Commons Attribution-NonCommercial 4.0 International license](https://creativecommons.org/licenses/by-nc/4.0/).

- Chapter 02: Building Real Server-Side Applications (3 hours)
  goals:
  - 2.0: cloud hosting providers
    - serverless functions
    - spin framework?
    - install `spin` and install `spin-test` plugin
      ```
      spin plugin install -u https://github.com/spinframework/spin-test/releases/download/canary/spin-test.json
      ```
    exercise: compile and run the simple hello world http handler
  - 2.1: HTTP requests with the spin framework
    goal: demonstrate easy to use, serverless nature, understand permissions, and the manifest
    exercise: fix an http endpoint to return random numbers
  - 2.2: HTTP handlers proper
    goal: understand error handling in wasm components
    exercise: integrate the expression evaluator to evaluate POSTed expressions
  - 2.3: Key Value Store
    goal: demonstrate how simple values can be stored across calls and the usefulness of that. demonstrate limitations. but emphasize cross-platform support.
    exercise: expand expression evaluator to support assigning values to variables. Store the variables in the spin-sdk KV store.
  - 2.4: Key Value Store
    goal: demonstrate how simple values can be stored across calls and the usefulness of that. demonstrate limitations. but emphasize cross-platform support.
    exercise: expand expression evaluator to support assigning values to variables. Store the variables in the spin-sdk KV store.
  - 2.5: Routing
    goal: understand how in-component routing works, understand how composition works, understand how inter-component routing works
    - point to Rust + Go demo for inter-language composition
      exercise: expand expression evaluator to support assigning values to variables. Store the variables in the spin-sdk KV store.
- Chapter 03: Advanced (1.5 hours)
  goals:
  - 3.1: Observability
    goal: with hosted code, host performance becomes a critical part, show off the otel integration. Demonstrate current limitations (i.e. no support for guest o11y).
  - 3.1: Performance Optimization
    - size optimizations (--release, no_std, wasm-opt)
    - runtime optimizations (pre-evaluation of modules)
