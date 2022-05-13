# rust-simple

Compilando um programa Rust e gerando arquivo wasm e depois executando com **wasmtime**

```shell
cargo build --release 
target/release/rust-simple
```

```shell
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
# Ou
# rustc src/simple.rs --target wasm32-wasi
wasmtime target/wasm32-wasi/debug/rust-simple.wasm
```
