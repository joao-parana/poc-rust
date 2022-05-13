#!/bin/bash

rm -rf target pkg/

cargo check --quiet
cargo build --release
WASM_INTERFACE_TYPES=1 wasm-pack build
# Your wasm pkg is ready to publish at $PWD/pkg
ls -la pkg
# wasmtime --dir=. target=pkg/mdown.wasm --invoke render "# Hello, WebAssembly\!"
wasmtime pkg/mdown.wasm --invoke render "# Hello, WebAssembly\!"
