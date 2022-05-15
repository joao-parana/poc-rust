# rust_wasi_markdown_parser

Instale o Wasmtime

```shell
curl https://wasmtime.dev/install.sh -sSf | bash

export WASMTIME_HOME="$HOME/.wasmtime"
export PATH="$WASMTIME_HOME/bin:$PATH"
```

Veja: https://docs.wasmtime.dev/examples-markdown.html

O WebAssembly gerado permite executar um parser markdown pela linha de comando

Fazendo o build do nosso WebAssembly escrito em rust:

```shell
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
```

Depois de fazer o build execute:

```shell
ls -la target/wasm32-wasi/debug/rust_wasi_markdown_parser.wasm 
wasmtime --dir . target/wasm32-wasi/debug/rust_wasi_markdown_parser.wasm \
         example-markdown.md
```
Você verá:

```html
<h1>Hello!</h1>
<p>I am example markdown for this demo!</p>
```

Também podemos usar `--invoke`, entretanto ficamos limitados a passagem de 
parâmetros, que por enquanto tem suporte limitado.

```shell
wasmtime  --invoke render_mkdn_t1  rust_wasi_markdown_parser.wasm
```

Neste link https://docs.wasmtime.dev/wasm-rust.html mostra como criar 
WebAssembly a partir de codigo Rust, tanto para aplicacao quanto para lib (biblioteca).

**Observação:** O comando `cat Cargo.toml` mostra o descritor do `cargo` para este modulo e 
a seção `dependencies` define as versões de `pulldown-cmark` (parser Markdown) e 
`structopt` (implementa suporte para CLI)

```toml
[package]
name = "rust_wasi_markdown_parser"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
structopt = "0.3.26"
pulldown-cmark = "0.9.1"
```

