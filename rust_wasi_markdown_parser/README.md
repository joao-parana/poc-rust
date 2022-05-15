# rust_wasi_markdown_parser

## Usando o Wasmtime

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

Também podemos usar `--invoke`

```shell
wasmtime  --invoke render_mkdn_t1  rust_wasi_markdown_parser.wasm
```

Entretanto ficamos limitados a passagem de parâmetros, que por enquanto 
tem suporte limitado. 

Portanto o uso do método `main` com o suporte do module `structopt` para CLI, 
como mostrado anteriormente, fica muito mais flexivel.

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

## Usando o Wasmer

Veja https://github.com/wasmerio/wasmer

Instalação

```shell
curl https://get.wasmer.io -sSfL | sh
source $HOME/.wasmer/wasmer.sh
```

O Wasmer vem com um gerenciador de pacotes WebAssembly:

```shell
wapm install syrusakbary/cowsay
wapm list
wapm run cowsay "hello wapm"
```

```txt
 ____________
< hello wapm >
 ------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
               ||----w |
                ||     ||
```

```shell
wapm uninstall syrusakbary/cowsay
wapm list
```

```shell
wapm search sql
```

```txt
NAME          | DESCRIPTION                                                                                                                        | DATE       | VERSION
sqlite/sqlite | SQLite is a C-language library that implements a small, fast, self-contained, high-reliability, full-featured, SQL database engine | 2019-10-22 | 0.2.2
```

```shell
wapm search qr2text
```

```txt
 NAME                | DESCRIPTION                               | DATE       | VERSION
 syrusakbary/qr2text | Render a QR code directly in the terminal | 2019-11-27 | 0.0.1
 ```

Um exemplo de pacote WebAssembly é o viu : https://github.com/atanunq/viu 
que também é distribuido como aplicação no ArchLinux (sudo pacman -S viu-git).
Este pacote exibe imagens no kitty e no iTerm (MacOS).

Exemplo de visualização no `iTerm`:

```shell
viu -w 90  ../../arrow/docs/source/developers/images/jira_search_documentation.jpeg
for a in `ls ../../soma-docker-playground/docs/*.png`
do
  echo "********** $a" ; viu -w 90 $a ; echo " " ; echo " " 
done
```

O Post https://paulbutler.org/2021/calling-webassembly-from-rust/ discute como 
incorporar codigo WebAssembly usando Rust (fora do Browser).
