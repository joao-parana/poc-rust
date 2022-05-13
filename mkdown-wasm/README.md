# mkdown-wasm

> Veja https://rustwasm.github.io/wasm-pack/installer/

Na primeira vez instale o **rustwasm** como root.

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sudo sh
```

```shell
cargo check --quiet

cargo build --release
WASM_INTERFACE_TYPES=1 wasm-pack build

# Your wasm pkg is ready to publish at $PWD/pkg
ls -la pkg
```

**OBS:** os arquivos "mdown.js", "mdown_bg.js" e "mdown.d.ts" nao sao criados pelo 
procedimento de build do wasm-pack mas sao referenciados no arquivo `pkg/package.json`
gerado


> Veja https://wasmtime.dev/

Na primeira vez instale o **wasmtime**

```shell
curl https://wasmtime.dev/install.sh -sSf | bash
export WASMTIME_HOME="$HOME/.wasmtime"
export PATH="$WASMTIME_HOME/bin:$PATH"
```

```shell
# wasmtime --dir=. target=pkg/mdown.wasm --invoke render "# Hello, WebAssembly\!"
wasmtime pkg/mdown.wasm --invoke render "# Hello, WebAssembly\!"
```
