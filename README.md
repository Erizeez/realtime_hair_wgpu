## Environment Setup

### Install Rust

Follow the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started) to install Rust.

### Install OS Dependencies

Follow the bevvy [setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies) to install the OS dependencies.

### Install LLVM

Follow the bevvy [setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional) to install LLVM.

### Install WASM Tools

```bash

rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo install wasm-server-runner

```

## How to Run

```bash
make run
```