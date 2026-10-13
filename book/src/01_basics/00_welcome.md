# Welcome

Welcome to **"Containers Are Dead - Long Live WebAssembly"**!

> In this course, we will explore the concept of containers and how they are being replaced by WebAssembly. We will cover the basics of WebAssembly, including its architecture and how it can be used to create lightweight, portable, and efficient applications. We will also discuss the benefits of using WebAssembly over traditional container technologies, such as Docker and Kubernetes.

We assume you are familiar with the basics of Rust but we will provide brief explanations and references whenever we rely on advanced features.

## Methodology

This course is based on the "learn by doing" principle.\
You'll build up your knowledge in small, manageable steps. It has been designed to be interactive and hands-on.

[Mainmatter](https://mainmatter.com/rust-consulting/) developed this course
to be delivered in a classroom setting, over a whole day: each attendee advances
through the lessons at their own pace, with an experienced instructor providing
guidance, answering questions and diving deeper into the topics as needed.\
If you're interested in attending one of our training sessions, or if you'd like to
bring this course to your company, please [get in touch](https://mainmatter.com/contact/).

You can also follow the course on your own, but we recommend you find a friend or
a mentor to help you along the way should you get stuck. You can
also find solutions to all exercises in the
[`solutions` branch of the GitHub repository](https://github.com/mainmatter/containers-are-dead/tree/solutions).

## Prerequisites

To follow this course, you must install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Spin](https://spinframework.dev) (installed in chapter 2)

If you have `nix` installed on your system, you can use the nix flake provided in the [courses Git repository](https://github.com/mainmatter/containers-are-dead) to install the required tools and skip ahead to the next section. Otherwise continue with the installation instructions below.

If Rust is already installed on your machine, make sure to update it to the latest version:

```bash
# If you installed Rust using `rustup`, the recommended way,
# you can update to the latest stable toolchain with:
rustup update stable
```

`rustup` will add the `wasm32-unknown-unknown`, `wasm32-wasip1` and `wasm32-wasip2` targets automatically from `rust-toolchain.toml` the first time `cargo` runs in the repository.

These commands should successfully run on your machine:

```bash
cargo --version
```

Don't start the course until you have these tools installed and working.

## Structure

On the left side of the screen, you can see that the course is divided into sections.\
To verify your understanding, each section is paired with an exercise that you need to solve.

You can find the exercises in the
[companion GitHub repository](https://github.com/mainmatter/containers-are-dead).\
Before starting the course, make sure to clone the repository to your local machine:

```bash
# If you have an SSH key set up with GitHub
git clone git@github.com:mainmatter/containers-are-dead.git
# Otherwise, use the HTTPS URL:
#
#   git clone https://github.com/mainmatter/containers-are-dead.git
```

We recommend you work on a branch, so you can easily track your progress and pull
updates from the main repository if needed:

```bash
cd containers-are-dead
git checkout -b my-solutions
```

All exercises are located in the `exercises` folder.
Each exercise is structured as a Rust package.
The package contains the exercise itself, instructions on what to do (in `src/lib.rs` or `src/main.rs`), and a test suite to
automatically verify your solution.

### `wr`, the workshop runner

To verify your solutions, we've also provided a tool to guide you through the course: the `wr` CLI, short for "workshop runner".
Install `wr` by following the instructions on [its website](https://mainmatter.github.io/rust-workshop-runner/).

Once you have `wr` installed, open a new terminal and navigate to the top-level folder of the repository.
Run the `wr` command to start the course:

```bash
wr
```

`wr` will verify the solution to the current exercise.\
Don't move on to the next section until you've solved the exercise for the current one.

> We recommend committing your solutions to Git as you progress through the course,
> so you can easily track your progress and "restart" from a known point if needed.

Enjoy the course!
