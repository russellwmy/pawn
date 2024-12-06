#!/bin/sh

# Install the Rust nightly toolchain with minimal profile (for advanced features)
rustup toolchain install nightly --profile=minimal

# Add the Rust formatter (rustfmt) for the nightly toolchain
rustup component add --toolchain nightly rustfmt 

# Install Wasmtime for running WebAssembly components using the component model
curl https://wasmtime.dev/install.sh -sSf | bash 

# Install cargo-component for WebAssembly component model development
cargo install cargo-component 

# Install wasm-tools for working with WebAssembly binaries
cargo install wasm-tools

# Install Dioxus CLI for building WebAssembly-based front-end applications
cargo install dioxus-cli